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
use std::{boxed::Box, str::FromStr};
use syn::{
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::{Eq, Let, Semi},
    Expr, FnArg, ItemFn, Lit, Local, Pat, ReturnType, Token, Type, TypeArray, TypePath,
};

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
Build an [`Expr] from a [`String`].
*/
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
fn expr_verbatim<S>(expr: S) -> Expr
where
    S: Into<String>,
{
    Expr::Verbatim(TS2::from_str(&expr.into()).unwrap())
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
        pat: pat_ident(ident),
        init: init.map_or(None, |i| Some((Eq::default(), Box::new(expr_ident(i))))),
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
Get the [`String`] representation of a [`Pat::Ident`].
*/
fn pat_get_s_ident(pat: &Pat) -> String {
    if let Pat::Ident(i) = pat.clone() {
        i.ident.to_string()
    } else {
        unreachable!("spice_derive::pat_get_s_ident must only take Pat::Ident")
    }
}
/**
Get the [`String`] representation of the last [`PathSegment`] of a [`TypePath`].
*/
fn path_get_last_s_ident(path: &TypePath) -> String {
    path.path.segments.last().unwrap().ident.to_string()
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

    let cspice_func = Ident::new(&format!("{}_c", fname.to_string()), Span::call_site());

    let inputs = sig.inputs;

    // Build CSPICE inputs from function inputs and reference to function outputs.
    let mut cspice_inputs = Punctuated::<Pat, Token![,]>::new();
    // Function inpus into CSPICE inputs.
    cspice_inputs.extend(inputs.iter().map(|arg| -> Pat {
        match arg {
            FnArg::Typed(pt) => {
                let pat = *pt.clone().pat;
                let ty = *pt.clone().ty;

                let ident = pat_get_s_ident(&pat);

                match ty {
                    Type::Path(tp) => match path_get_last_s_ident(&tp).as_str() {
                        "S" => pat_macro("crate::cstr", &format!("{}.into()", ident)),
                        "String" => pat_macro("crate::cstr", &format!("{}", ident)),
                        "f64" => pat_ident(ident),
                        "usize" => new_pat(format!("{} as i32", ident)),
                        _ => panic!("->1"),
                    },
                    Type::Reference(tr) => match *tr.elem {
                        Type::Path(tp) => match path_get_last_s_ident(&tp).as_str() {
                            "str" => pat_macro("crate::cstr", &format!("{}.to_string()", ident)),
                            _ => panic!("->2"),
                        },
                        _ => panic!("->3"),
                    },
                    _ => panic!("->4"),
                }
            }
            FnArg::Receiver(_) => panic!("->5"),
        }
    }));

    // Needed allocations declarations for the function ouputs that will be converted to pointers for CSPICE function.
    let mut vars_out_decl = Vec::<Local>::new();
    let mut vars_out = Vec::<Pat>::new();

    // Get function ouputs
    let o_ty: syn::Type = match sig.output.clone() {
        ReturnType::Default => parse_quote! { () },
        ReturnType::Type(_, ty) => {
            // Reference to function ouputs into CSPICE inputs.
            match *ty.clone() {
                Type::Tuple(tt) => tt.elems.iter().for_each(|e| match e {
                    Type::Path(tp) => match path_get_last_s_ident(tp).as_str() {
                        "f64" => {
                            let ident = format!("varout_{}", vars_out_decl.len());
                            vars_out_decl.push(declare(
                                format!("mut {}", ident.clone()),
                                Some("0.0f64".to_string()),
                            ));
                            cspice_inputs.push(pat_ident(format!("&mut {}", ident.clone())));
                            vars_out.push(pat_ident(ident.clone()));
                        }
                        _ => panic!("->6"),
                    },
                    Type::Array(ta) => {
                        let ident = format!("varout_{}", vars_out_decl.len());
                        let pat_ident_fc = new_pat(format!("{}.as_mut_ptr()", ident));
                        let size = array_get_size(&ta);
                        let init = format!("{:?}", vec![0.0f64; size]);
                        vars_out_decl.push(declare(format!("mut {}", ident.clone()), Some(init)));
                        cspice_inputs.push(pat_ident_fc);
                        vars_out.push(pat_ident(ident.clone()));
                    }
                    _ => panic!("->7"),
                }),
                Type::Path(tp) => match path_get_last_s_ident(&tp).as_str() {
                    "f64" => {
                        let ident = format!("varout_{}", vars_out_decl.len());
                        vars_out_decl.push(declare(
                            format!("mut {}", ident.clone()),
                            Some("0.0f64".to_string()),
                        ));
                        cspice_inputs.push(pat_ident(format!("&mut {}", ident.clone())));
                        vars_out.push(pat_ident(ident.clone()));
                    }
                    "String" => {
                        let ident = format!("varout_{}", vars_out_decl.len());
                        vars_out_decl.push(declare(
                            format!("{}", ident.clone()),
                            Some("crate::cstr!()".to_string()),
                        ));
                        cspice_inputs.push(pat_ident(ident.clone()));
                        vars_out.push(new_pat(format!("crate::fcstr!({})", ident.clone())));
                    }
                    _ => panic!("->8"),
                },
                Type::Array(ta) => match *ta.clone().elem {
                    Type::Path(tp) => match path_get_last_s_ident(&tp).as_str() {
                        "f64" => {
                            let ident = format!("varout_{}", vars_out_decl.len());
                            let pat_ident_fc = new_pat(format!("{}.as_mut_ptr()", ident));
                            let size = array_get_size(&ta);
                            let init = format!("{:?}", vec![0.0f64; size]);
                            vars_out_decl
                                .push(declare(format!("mut {}", ident.clone()), Some(init)));
                            cspice_inputs.push(pat_ident_fc);
                            vars_out.push(pat_ident(ident.clone()));
                        }
                        _ => panic!("->12"),
                    },
                    Type::Array(ta_2) => match *ta_2.clone().elem {
                        Type::Path(tp) => match path_get_last_s_ident(&tp).as_str() {
                            "f64" => {
                                let ident = format!("varout_{}", vars_out_decl.len());
                                let pat_ident_fc = new_pat(format!("{}.as_mut_ptr()", ident));
                                let size = array_get_size(&ta);
                                let size_2 = array_get_size(&ta_2);
                                let init = format!("{:?}", vec![vec![0.0f64; size_2]; size]);
                                vars_out_decl
                                    .push(declare(format!("mut {}", ident.clone()), Some(init)));
                                cspice_inputs.push(pat_ident_fc);
                                vars_out.push(pat_ident(ident.clone()));
                            }
                            _ => panic!("->13"),
                        },
                        _ => panic!("->11"),
                    },
                    _ => panic!("->10"),
                },
                _ => panic!("->9"),
            }

            // Fill `o_ty` for function ouput.
            *ty
        }
    };

    let tokens = quote! {
        #(#attrs)*
        #vis fn #fname#generics(#inputs) -> #o_ty {
            #(#vars_out_decl)*
            unsafe {
                crate::c::#cspice_func(#cspice_inputs);
            }
            (#(#vars_out),*)
        }
    };
    // println!("{}", tokens);
    tokens.into()
}
