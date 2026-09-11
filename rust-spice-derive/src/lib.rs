/*!
# Procedural macros for **rust-spice**

This crate is an implementation detail of [**rust-spice**][rust-spice link]; it is not meant to be
used on its own.

[`cspice_proc!`] turns the *signature* of a CSPICE routine into a safe, idiomatic Rust wrapper. The
macro itself knows nothing about the SPICE types: everything type specific lives in the `spice::ffi`
module of **rust-spice**, where the `SpiceArg`/`SpiceRet` traits describe how a Rust type is handed
to, and read back from, a C routine. The macro only has to decide *where* each value goes.

```ignore
cspice_proc! {
    pub fn spkpos(targ: &str, et: f64, frame: &str, abcorr: &str, obs: &str) -> ([f64; 3], f64) {}
}
```

expands, roughly, to

```ignore
pub fn spkpos(targ: &str, et: f64, frame: &str, abcorr: &str, obs: &str) -> ([f64; 3], f64) {
    let mut arg_0 = crate::core::ffi::In::new(targ);
    // ...
    let mut out_0 = crate::core::ffi::Out::<[f64; 3]>::new();
    let mut out_1 = crate::core::ffi::Out::<f64>::new();
    unsafe {
        crate::c::spkpos_c(arg_0.raw() as _, /* .. */ out_0.raw() as _, out_1.raw() as _);
    }
    (out_0.get(), out_1.get())
}
```

## Attributes understood by [`cspice_proc!`]

+ `#[return_output]`: the C routine returns its result instead of writing it through a pointer.
+ `#[cname(foo_c)]`: call `foo_c` instead of the default `<fn name>_c`.
+ `#[lenout]` on an *argument*: that argument is the size, in bytes, of the string output buffer.

[rust-spice link]: https://docs.rs/rust-spice
*/

#![deny(missing_docs)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Attribute, Error, FnArg, Ident,
    ItemFn, Pat, PatType, Path, Result, ReturnType, Signature, Token, Type, Visibility,
};

/// Marks a wrapper whose C routine returns its output rather than writing it through a pointer.
const RETURN_OUTPUT: &str = "return_output";
/// Overrides the name of the C routine to call.
const CNAME: &str = "cname";
/// Marks the argument holding the size of the string output buffer.
const LENOUT: &str = "lenout";

/// One argument of the wrapper being generated.
struct Arg {
    /// Identifier the caller binds, e.g. `targ`.
    ident: Ident,
    /// Local holding the marshalled value for the duration of the call.
    local: Ident,
    /// Argument with every helper attribute stripped, ready to be re-emitted.
    clean: FnArg,
    /// Whether the argument carries `#[lenout]`.
    is_lenout: bool,
}

/// Everything [`cspice_proc`] needs to know about the declaration it was handed.
struct Wrapper {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    cfunc: Ident,
    return_output: bool,
    args: Vec<Arg>,
    outputs: Vec<Type>,
    /// `true` when the declared return type is a tuple, so the outputs must be re-assembled as one.
    tuple_output: bool,
}

/// Write an idiomatic Rust interface for a CSPICE routine.
///
/// See the [crate documentation][crate] for the accepted attributes and the shape of the expansion.
#[proc_macro]
pub fn cspice_proc(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    match Wrapper::parse(item).and_then(|w| w.expand()) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Marker attribute consumed by [`cspice_proc!`]; a no-op on its own.
#[proc_macro_attribute]
pub fn return_output(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marker attribute consumed by [`cspice_proc!`]; a no-op on its own.
#[proc_macro_attribute]
pub fn lenout(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Re-expose a free function as an inherent method of `struct_path`, with an added `&self`.
///
/// Used to build the guarded API of the `lock` feature out of the unguarded one.
#[proc_macro_attribute]
pub fn impl_for(struct_path: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);
    match expand_impl_for(struct_path.into(), function) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_impl_for(struct_path: TokenStream2, function: ItemFn) -> Result<TokenStream2> {
    let struct_path: Path = syn::parse2(struct_path)?;

    let Signature {
        ident,
        generics,
        inputs,
        output,
        ..
    } = function.sig.clone();

    let idents = inputs
        .iter()
        .map(arg_ident)
        .collect::<Result<Punctuated<Ident, Token![,]>>>()?;
    let inputs = inputs
        .iter()
        .map(strip_helper_attrs)
        .collect::<Punctuated<FnArg, Token![,]>>();

    // Documentation and lints belong on the method too, but `cfg`s have already been resolved by
    // the time this attribute runs, so only the harmless ones are carried over.
    let attrs = function
        .attrs
        .iter()
        .filter(|attr| !is_helper(attr))
        .collect::<Vec<_>>();
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let mut out = function.to_token_stream();
    out.extend(quote! {
        impl #struct_path {
            #(#attrs)*
            pub fn #ident #impl_generics (&self, #inputs) #output #where_clause {
                #ident(#idents)
            }
        }
    });
    Ok(out)
}

impl Wrapper {
    fn parse(item: ItemFn) -> Result<Self> {
        let ItemFn {
            attrs, vis, sig, ..
        } = item;

        if let Some(variadic) = &sig.variadic {
            return Err(Error::new(
                variadic.span(),
                "variadic wrappers are not supported",
            ));
        }

        let cfunc = match find_attr(&attrs, CNAME) {
            Some(attr) => attr.parse_args::<Ident>()?,
            None => Ident::new(&format!("{}_c", sig.ident), Span::call_site()),
        };
        let return_output = find_attr(&attrs, RETURN_OUTPUT).is_some();

        let args = sig
            .inputs
            .iter()
            .enumerate()
            .map(|(index, arg)| {
                Ok(Arg {
                    ident: arg_ident(arg)?,
                    local: format_ident!("arg_{}", index),
                    clean: strip_helper_attrs(arg),
                    is_lenout: match arg {
                        FnArg::Typed(PatType { attrs, .. }) => find_attr(attrs, LENOUT).is_some(),
                        FnArg::Receiver(_) => false,
                    },
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let (outputs, tuple_output) = match &sig.output {
            ReturnType::Default => (vec![], false),
            ReturnType::Type(_, ty) => match &**ty {
                Type::Tuple(tuple) => (tuple.elems.iter().cloned().collect(), true),
                other => (vec![other.clone()], false),
            },
        };

        if return_output && outputs.len() != 1 {
            return Err(Error::new(
                sig.output.span(),
                "`#[return_output]` requires exactly one, non-tuple, return type",
            ));
        }

        let attrs = attrs.into_iter().filter(|a| !is_helper(a)).collect();

        Ok(Self {
            attrs,
            vis,
            sig,
            cfunc,
            return_output,
            args,
            outputs,
            tuple_output,
        })
    }

    fn expand(self) -> Result<TokenStream2> {
        let Self {
            attrs,
            vis,
            sig,
            cfunc,
            return_output,
            args,
            outputs,
            tuple_output,
        } = self;

        let name = &sig.ident;
        let generics = &sig.generics;
        let where_clause = &sig.generics.where_clause;
        let ret = &sig.output;
        let inputs = args.iter().map(|arg| &arg.clean);

        let bindings = args.iter().map(|Arg { ident, local, .. }| {
            quote! { let mut #local = crate::core::ffi::In::new(#ident); }
        });
        let mut call = args
            .iter()
            .map(|Arg { local, .. }| quote! { #local.raw() as _ })
            .collect::<Vec<_>>();

        // The buffer of a string output is sized by the `#[lenout]` argument when there is one, so
        // that the wrapper never hands CSPICE a buffer smaller than the length it was promised.
        let lenout = args.iter().find(|arg| arg.is_lenout).map(|arg| &arg.ident);

        let body = if return_output {
            let ty = &outputs[0];
            quote! {
                #(#bindings)*
                unsafe {
                    let returned = crate::c::#cfunc(#(#call),*);
                    <#ty as crate::core::ffi::SpiceReturn>::from_c(returned)
                }
            }
        } else {
            let locals = (0..outputs.len())
                .map(|index| format_ident!("out_{}", index))
                .collect::<Vec<_>>();
            let allocations = outputs
                .iter()
                .zip(&locals)
                .map(|(ty, local)| match (is_string(ty), lenout) {
                    (true, Some(len)) => {
                        quote! {
                            let mut #local =
                                crate::core::ffi::Out::<#ty>::with_len((#len).max(0) as usize);
                        }
                    }
                    _ => quote! { let mut #local = crate::core::ffi::Out::<#ty>::new(); },
                })
                .collect::<Vec<_>>();
            call.extend(locals.iter().map(|local| quote! { #local.raw() as _ }));

            let collect = match (locals.is_empty(), tuple_output) {
                (true, _) => quote! {},
                (false, true) => quote! { ( #(#locals.get()),* ) },
                (false, false) => {
                    let local = &locals[0];
                    quote! { #local.get() }
                }
            };
            quote! {
                #(#bindings)*
                #(#allocations)*
                unsafe { crate::c::#cfunc(#(#call),*); }
                #collect
            }
        };

        Ok(quote! {
            #(#attrs)*
            #vis fn #name #generics (#(#inputs),*) #ret #where_clause {
                #body
            }
        })
    }
}

/// Pull the bare identifier out of an argument, rejecting patterns the wrapper cannot forward.
fn arg_ident(arg: &FnArg) -> Result<Ident> {
    match arg {
        FnArg::Typed(PatType { pat, .. }) => match &**pat {
            Pat::Ident(pat) if pat.subpat.is_none() => Ok(pat.ident.clone()),
            other => Err(Error::new(
                other.span(),
                "only bare identifiers are allowed as argument patterns",
            )),
        },
        FnArg::Receiver(receiver) => Err(Error::new(
            receiver.span(),
            "CSPICE wrappers are free functions and cannot take a receiver",
        )),
    }
}

/// Remove the attributes this crate introduces, so the argument can be re-emitted as written.
fn strip_helper_attrs(arg: &FnArg) -> FnArg {
    match arg.clone() {
        FnArg::Typed(mut pat) => {
            pat.attrs.retain(|attr| !is_helper(attr));
            FnArg::Typed(pat)
        }
        receiver => receiver,
    }
}

fn find_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident(name))
}

fn is_helper(attr: &Attribute) -> bool {
    [RETURN_OUTPUT, CNAME, LENOUT]
        .iter()
        .any(|name| attr.path().is_ident(name))
}

fn is_string(ty: &Type) -> bool {
    matches!(ty, Type::Path(path) if path.qself.is_none()
        && path.path.segments.last().is_some_and(|s| s.ident == "String"))
}
