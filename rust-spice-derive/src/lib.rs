/*!
# Procedural macro's crate for **rust-spice**

See:

+ [**rust-spice**][rust-spice link]
+ [proc macro doc][proc-macro link]

[rust-spice link]: https://docs.rs/rust-spice
[proc-macro link]: https://doc.rust-lang.org/reference/procedural-macros.html
*/

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TS2};
use quote::ToTokens;
use std::{boxed::Box, str::FromStr};
use syn::{
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::{Colon, Eq, Let, Semi},
    Expr, FnArg, GenericArgument, ItemFn, Lit, Local, Pat, PatType, PathArguments, ReturnType,
    Token, Type, TypeArray, TypePath,
};

/**
Get the [`String`] representation of a [`Pat::Ident`].
*/
macro_rules! tts {
    ($e: expr) => {
        $e.to_token_stream().to_string()
    };
}

/**
Build a [`Pat::Macro`] from its name as [`String`] and the tokens inside as a [`String`], just like:

```.ignore
path!(tokens);
```
*/
fn pat_macro<S>(path: S, tokens: S) -> Pat
where
    S: Into<String>,
{
    syn::parse_str(&format!("{}!({})", path.into(), tokens.into())).unwrap()
}

/**
Build a [`Pat::Ident`] from [`String`].
*/
fn pat_ident<S>(ident: S) -> Pat
where
    S: Into<String>,
{
    syn::parse_str(&ident.into()).unwrap()
}

/**
Build a [`Pat::Verbatim`] from [`String`].
*/
fn new_pat<S>(expr: S) -> Pat
where
    S: Into<String>,
{
    Pat::Verbatim(TS2::from_str(&expr.into()).unwrap())
}

/**
Build a [`Pat::Type`] from [`String`].
*/
fn new_pat_type<S>(pat: S, ty: S) -> PatType
where
    S: Into<String>,
{
    PatType {
        attrs: vec![],
        pat: Box::new(new_pat(pat)),
        colon_token: Colon::default(),
        ty: Box::new(new_type(ty)),
    }
}

/**
Build an [`Expr] from a [`String`].
*/
#[allow(unused)]
fn expr_ident<S>(ident: S) -> Expr
where
    S: Into<String>,
{
    syn::parse_str(&ident.into()).unwrap()
}

/**
Build an [`Expr::Verbatim`] from [`String`].
*/
#[allow(dead_code)]
fn new_expr<S>(expr: S) -> Expr
where
    S: Into<String>,
{
    Expr::Verbatim(TS2::from_str(&expr.into()).unwrap())
}

/**
Build a [`Type`] from [`String`].
*/
fn new_type<S>(s: S) -> Type
where
    S: Into<String>,
{
    Type::Verbatim(TS2::from_str(&s.into()).unwrap())
}

/**
Build a [`Local`] to declare a variable from its name and mutability as [`String`] and an
optional initial value as [`String`].
*/
fn declare<S>(ident: S, init: Option<S>) -> Local
where
    S: Into<String>,
{
    Local {
        attrs: vec![],
        let_token: Let::default(),
        pat: new_pat(ident),
        init: init.map(|i| (Eq::default(), Box::new(new_expr(i)))),
        semi_token: Semi::default(),
    }
}

/**
Get the size of a [`TypeArray`].
*/
fn array_get_size(arr: &TypeArray) -> usize {
    if let Expr::Lit(el) = &arr.len {
        if let Lit::Int(li) = &el.lit {
            li.base10_parse::<usize>().unwrap()
        } else {
            unreachable!("array size must be syn::LitInt")
        }
    } else {
        unreachable!("array size must be syn::ExprLit")
    }
}

/**
Get the [`String`] representation of the last [`PathSegment`] of a [`TypePath`].
*/
fn path_get_last_s_ident(path: &TypePath) -> (String, PathArguments) {
    let seg = path.path.segments.last().unwrap();
    (seg.ident.to_string(), seg.arguments.clone())
}

#[allow(unused)]
fn semi(b: bool) -> TS2 {
    if b {
        Semi::default().to_token_stream()
    } else {
        TS2::new()
    }
}

/**
Get list of [`GenericArgument`]s in [`PathArguments`].
*/
#[allow(unused)]
fn generic_arguments(pargs: &PathArguments) -> Vec<GenericArgument> {
    if let PathArguments::AngleBracketed(abga) = &pargs {
        abga.args.iter().cloned().collect::<Vec<_>>()
    } else {
        unreachable!("ParthArguments is not an AngleBracketed.")
    }
}

/**
I write Rust idiomatic interface for CSPICE.
*/
#[proc_macro]
pub fn cspice_proc(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    let attrs = f.attrs;
    let vis = f.vis;
    let sig = f.sig;
    let _block = f.block;

    let fname = sig.ident.clone();
    let generics = sig.generics;

    let return_output = attrs.iter().any(|attr| tts!(attr.path) == "return_output");
    let return_result = attrs
        .iter()
        .any(|attr| tts!(attr.tokens).contains("feature = \"results\", return_result"));

    let semi_call = semi(!return_output);

    let cspice_func = Ident::new(&format!("{}_c", fname.to_string()), Span::call_site());

    // Update wrapper input.
    let inputs = sig
        .inputs
        .iter()
        .map(|arg| {
            FnArg::Typed(match arg {
                FnArg::Typed(pt) => {
                    let pat = *pt.clone().pat;
                    let ty = *pt.clone().ty;

                    match ty.clone() {
                        Type::Path(tp) => match path_get_last_s_ident(&tp).0.as_str() {
                            "DLADSC" => new_pat_type(format!("mut {}", tts!(pat)), tts!(ty)),
                            _ => pt.clone(),
                        },
                        Type::Array(_) => new_pat_type(format!("mut {}", tts!(pat)), tts!(ty)),
                        _ => pt.clone(),
                    }
                }
                FnArg::Receiver(_) => panic!("->5 bis"),
            })
        })
        .collect::<Punctuated<_, Token![,]>>();

    // Build CSPICE inputs from function inputs and reference to function outputs.
    let mut cspice_inputs = Punctuated::<Pat, Token![,]>::new();
    // Function inpus into CSPICE inputs.
    cspice_inputs.extend(sig.inputs.iter().map(|arg| -> Pat {
        match arg {
            FnArg::Typed(pt) => {
                let pat = *pt.clone().pat;
                let ty = *pt.clone().ty;

                let ident = tts!(&pat);

                match ty {
                    Type::Path(tp) => match path_get_last_s_ident(&tp).0.as_str() {
                        "String" => pat_macro("crate::cstr", &ident),
                        "f64" | "i32" => new_pat(ident),
                        "usize" => new_pat(format!("{} as i32", ident)),
                        "bool" => new_pat(format!("{} as i32", ident)),
                        "DLADSC" => new_pat(format!("&mut {}", ident)),
                        _ => panic!("->1"),
                    },
                    Type::Reference(tr) => match *tr.elem {
                        Type::Path(tp) => match path_get_last_s_ident(&tp).0.as_str() {
                            "str" => pat_macro("crate::cstr", &format!("{}.to_string()", ident)),
                            _ => panic!("->2"),
                        },
                        Type::Slice(_) => new_pat(format!("{}.as_mut_ptr()", ident)),
                        _ => panic!("->3"),
                    },
                    Type::Array(_) => new_pat(format!("{}.as_mut_ptr()", ident)),
                    _ => panic!("->4"),
                }
            }
            FnArg::Receiver(_) => panic!("->5"),
        }
    }));

    // Needed allocations declarations for the function ouputs that will be converted to pointers for CSPICE function.
    let mut vars_out_decl = Vec::<Local>::new();
    let mut vars_out = Vec::<Pat>::new();
    let mut out_is_bool = false;
    // Get function ouputs
    let output = match sig.output {
        ReturnType::Type(_, ty) => {
            // Reference to function ouputs into CSPICE inputs.
            if !return_output {
                match *ty.clone() {
                    Type::Tuple(tt) => tt.elems.iter().for_each(|e| match e {
                        Type::Path(tp) => {
                            let tpp = path_get_last_s_ident(tp);
                            match tpp.0.as_str() {
                                "f64" => {
                                    let ident = format!("varout_{}", vars_out_decl.len());
                                    vars_out_decl.push(declare(
                                        format!("mut {}", ident),
                                        Some("0.0f64".to_string()),
                                    ));
                                    cspice_inputs.push(pat_ident(format!("&mut {}", ident)));
                                    vars_out.push(pat_ident(ident));
                                }
                                "i32" => {
                                    let ident = format!("varout_{}", vars_out_decl.len());
                                    vars_out_decl.push(declare(
                                        format!("mut {}", ident),
                                        Some("0i32".to_string()),
                                    ));
                                    cspice_inputs.push(pat_ident(format!("&mut {}", ident)));
                                    vars_out.push(pat_ident(ident));
                                }
                                "String" => {
                                    let ident = format!("varout_{}", vars_out_decl.len());
                                    vars_out_decl.push(declare(
                                        &ident,
                                        Some(&"crate::mallocstr!(crate::MAX_LEN_OUT)".to_string()),
                                    ));
                                    cspice_inputs.push(pat_ident(ident.clone()));
                                    vars_out.push(new_pat(format!("crate::fcstr!({})", ident)));
                                }
                                "bool" => {
                                    let ident = format!("varout_{}", vars_out_decl.len());
                                    vars_out_decl.push(declare(
                                        format!("mut {}", ident),
                                        Some("0i32".to_string()),
                                    ));
                                    cspice_inputs.push(pat_ident(format!("&mut {}", ident)));
                                    vars_out.push(new_pat(format!("{} != 0", ident)));
                                }
                                "DLADSC" => {
                                    let ident = format!("varout_{}", vars_out_decl.len());
                                    vars_out_decl.push(declare(
                                        format!("mut {}", ident),
                                        Some("std::mem::MaybeUninit::uninit()".to_string()),
                                    ));
                                    cspice_inputs.push(new_pat(format!("{}.as_mut_ptr()", ident)));
                                    vars_out.push(new_pat(format!("{}.assume_init()", ident)));
                                }
                                _ => panic!("->6: {}", tpp.0.as_str()),
                            }
                        }
                        Type::Array(ta) => {
                            let ident = format!("varout_{}", vars_out_decl.len());
                            let pat_ident_fc = new_pat(format!("{}.as_mut_ptr()", ident));
                            let size = array_get_size(ta);
                            let init = format!("{:?}", vec![0.0f64; size]);
                            vars_out_decl.push(declare(format!("mut {}", ident), Some(init)));
                            cspice_inputs.push(pat_ident_fc);
                            vars_out.push(pat_ident(ident));
                        }
                        _ => panic!("->7"),
                    }),
                    Type::Path(tp) => {
                        let a = path_get_last_s_ident(&tp);
                        let b = a.0.as_str();
                        // println!("{}: {}", fname, b);
                        match b {
                            "f64" => {
                                let ident = format!("varout_{}", vars_out_decl.len());
                                vars_out_decl.push(declare(
                                    format!("mut {}", ident),
                                    Some("0.0f64".to_string()),
                                ));
                                cspice_inputs.push(pat_ident(format!("&mut {}", ident)));
                                vars_out.push(pat_ident(ident));
                            }
                            "i32" => {
                                let ident = format!("varout_{}", vars_out_decl.len());
                                vars_out_decl.push(declare(
                                    format!("mut {}", ident),
                                    Some("0i32".to_string()),
                                ));
                                cspice_inputs.push(pat_ident(format!("&mut {}", ident)));
                                vars_out.push(pat_ident(ident));
                            }
                            "String" => {
                                let ident = format!("varout_{}", vars_out_decl.len());
                                vars_out_decl.push(declare(
                                    &ident,
                                    Some(&"mallocstr!(crate::MAX_LEN_OUT)".to_string()),
                                ));
                                cspice_inputs.push(pat_ident(ident.clone()));
                                vars_out.push(new_pat(format!("crate::fcstr!({})", ident)));
                            }
                            "bool" => {
                                let ident = format!("varout_{}", vars_out_decl.len());
                                vars_out_decl.push(declare(
                                    format!("mut {}", ident),
                                    Some("0i32".to_string()),
                                ));
                                cspice_inputs.push(pat_ident(format!("&mut {}", ident)));
                                vars_out.push(new_pat(format!("{} != 0", ident)));
                            }
                            "DSKDSC" => {
                                let ident = format!("varout_{}", vars_out_decl.len());
                                vars_out_decl.push(declare(
                                    format!("mut {}", ident),
                                    Some("std::mem::MaybeUninit::uninit()".to_string()),
                                ));
                                cspice_inputs.push(new_pat(format!("{}.as_mut_ptr()", ident)));
                                vars_out.push(new_pat(format!("{}.assume_init()", ident)));
                            }
                            "Cell" => {
                                let ident = format!("varout_{}", vars_out_decl.len());
                                vars_out_decl.push(declare(
                                    format!("mut {}", ident),
                                    Some("Cell::new_int()".to_string()),
                                ));
                                cspice_inputs.push(new_pat(format!("&mut {}.0", ident)));
                                vars_out.push(new_pat(ident));
                            }
                            _ => panic!("->8"),
                        }
                    }
                    Type::Array(ta) => match *ta.clone().elem {
                        Type::Path(tp) => match path_get_last_s_ident(&tp).0.as_str() {
                            "f64" => {
                                let ident = format!("varout_{}", vars_out_decl.len());
                                let pat_ident_fc = new_pat(format!("{}.as_mut_ptr()", ident));
                                let size = array_get_size(&ta);
                                let init = format!("{:?}", vec![0.0f64; size]);
                                vars_out_decl.push(declare(format!("mut {}", ident), Some(init)));
                                cspice_inputs.push(pat_ident_fc);
                                vars_out.push(pat_ident(ident));
                            }
                            _ => panic!("->12"),
                        },
                        Type::Array(ta_2) => match *ta_2.clone().elem {
                            Type::Path(tp) => match path_get_last_s_ident(&tp).0.as_str() {
                                "f64" => {
                                    let ident = format!("varout_{}", vars_out_decl.len());
                                    let pat_ident_fc = new_pat(format!("{}.as_mut_ptr()", ident));
                                    let size = array_get_size(&ta);
                                    let size_2 = array_get_size(&ta_2);
                                    let init = format!("{:?}", vec![vec![0.0f64; size_2]; size]);
                                    vars_out_decl
                                        .push(declare(format!("mut {}", ident), Some(init)));
                                    cspice_inputs.push(pat_ident_fc);
                                    vars_out.push(pat_ident(ident));
                                }
                                _ => panic!("->13"),
                            },

                            _ => panic!("->11"),
                        },
                        _ => panic!("->10"),
                    },
                    _ => panic!("->9"),
                }
            } else {
                let ty_token = ty.to_token_stream().to_string();
                let ty_str = ty_token.as_str();
                if ty_str == "bool" {
                    out_is_bool = true;
                }
            }
            *ty
        }
        ReturnType::Default => parse_quote! {()},
    };

    let function_output = match return_output {
        true => match out_is_bool {
            true => new_pat("!= 0".to_string()).to_token_stream(),
            false => TS2::new(),
        },
        false => match vars_out.is_empty() {
            true => quote! {()},
            false => quote! { ( #(#vars_out),* )},
        },
    };

    let tokens = match return_result {
        false => {
            quote! {
                #(#attrs)*
                #vis fn #fname#generics(#inputs) -> #output {
                    #(#vars_out_decl)*
                    #[allow(unused_unsafe)]
                    unsafe {
                        crate::c::#cspice_func(#cspice_inputs)#semi_call
                        #function_output
                    }
                }
            }
        }
        true => {
            quote! {
                #(#attrs)*
                #vis fn #fname#generics(#inputs) -> std::result::Result<#output, crate::spice_results::error::SpiceError> {
                    #(#vars_out_decl)*
                    #[allow(unused_unsafe)]
                    unsafe {
                        crate::c_raw::erract("SET", crate::MAX_LEN_OUT as i32, "RETURN");
                        crate::c::#cspice_func(#cspice_inputs)#semi_call
                        if crate::c_raw::failed() {
                            let short = crate::c_raw::getmsg("SHORT", crate::MAX_LEN_OUT as i32);
                            let long = crate::c_raw::getmsg("LONG", crate::MAX_LEN_OUT as i32);
                            let e = spice_results::SpiceError::new(short, long);
                            crate::c_raw::reset();
                            return Err(e);
                        } else {
                            return Ok(#function_output);
                        }
                    }
                }
            }
        }
    };
    if [].contains(&fname.to_string().as_str()) {
        println!("{}", tokens);
    }
    tokens.into()
}

#[proc_macro_attribute]
pub fn return_output(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn return_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
