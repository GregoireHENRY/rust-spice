/*!
Doc

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
    token::{And, Bang, Bracket, Eq, Let, Mut, Paren, Semi},
    BareFnArg, Expr, ExprArray, ExprLit, ExprMacro, FnArg, Lit, LitFloat, Local, Macro,
    MacroDelimiter, Pat, PatIdent, PatLit, PatReference, Path, PathArguments, PathSegment,
    ReturnType, Signature, Token, Type, TypeBareFn,
};

#[proc_macro]
pub fn create_double_function(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as BareFnArg);

    let tokens = quote! {
        fn demo(arg: #f) -> #f {
            arg * 2 as #f
        }
    };
    println!("{}", tokens);
    tokens.into()
}

#[proc_macro]
pub fn create_bare_function(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as TypeBareFn);

    let arg_type = f.inputs.first().unwrap();

    let ty: syn::Type = match f.output {
        ReturnType::Type(_, ty) => *ty,
        ReturnType::Default => parse_quote! { () },
    };

    let tokens = quote! {
        fn demo(arg: #arg_type) -> #ty {
            (arg * 2 as #arg_type) as #ty
        }
    };
    println!("{}", tokens);
    tokens.into()
}

#[proc_macro]
pub fn create_function(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as Signature);

    let (arg, ty) = match f.inputs.first().unwrap() {
        FnArg::Typed(pt) => (pt.clone().pat, pt.clone().ty),
        FnArg::Receiver(_) => panic!("NONONO"),
    };

    let o_ty: syn::Type = match f.output.clone() {
        ReturnType::Type(_, ty) => *ty,
        ReturnType::Default => parse_quote! { () },
    };
    let func = f.ident;

    let tokens = quote! {
        fn #func(#arg: #ty) -> #o_ty {
            (#arg * 2 as #ty) as #o_ty
        }
    };
    println!("{}", tokens);
    tokens.into()
}

fn call_macro(seg: Punctuated<PathSegment, Token![::]>, tokens: &str) -> Pat {
    Pat::Lit(PatLit {
        attrs: vec![],
        expr: Box::new(Expr::Macro(ExprMacro {
            attrs: vec![],
            mac: Macro {
                path: Path {
                    leading_colon: None,
                    segments: seg,
                },
                bang_token: Bang::default(),
                delimiter: MacroDelimiter::Paren(Paren::default()),
                tokens: TS2::from_str(tokens).unwrap(),
            },
        })),
    })
}

fn send_ident(by_ref: bool, by_mut: bool, ident: Ident) -> Pat {
    let _mut;
    if !by_mut {
        _mut = None;
    } else {
        _mut = Some(Mut::default());
    }
    let pat = Pat::Ident(PatIdent {
        attrs: vec![],
        by_ref: None,
        mutability: _mut,
        ident,
        subpat: None,
    });
    if !by_ref {
        pat
    } else {
        Pat::Reference(PatReference {
            attrs: vec![],
            and_token: And::default(),
            mutability: None,
            pat: Box::new(pat),
        })
    }
}

fn declare_ident(by_mut: bool, ident: Ident, init: Option<Expr>) -> Local {
    let _init;
    if init.is_some() {
        _init = Some((Eq::default(), Box::new(init.unwrap())));
    } else {
        _init = None;
    }
    Local {
        attrs: vec![],
        let_token: Let::default(),
        pat: send_ident(false, by_mut, ident),
        init: _init,
        semi_token: Semi::default(),
    }
}

#[proc_macro]
pub fn cspice_proc(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as Signature);

    let func = f.ident.clone();
    let generics = f.generics;

    let cspice_func = Ident::new(
        &format!("{}_c", f.ident.clone().to_string()),
        Span::call_site(),
    );

    let inputs = f.inputs;

    // Build CSPICE inputs from function inputs and reference to function outputs.
    let mut cspice_inputs = Punctuated::<Pat, Token![,]>::new();
    // Function inpus into CSPICE inputs.
    cspice_inputs.extend(inputs.iter().map(|arg| match arg {
        FnArg::Typed(pt) => {
            let pat = *pt.clone().pat;
            let ty = *pt.clone().ty;

            let (ident, s_ident) = match pat.clone() {
                Pat::Ident(i) => (i.ident.clone(), i.ident.to_string()),
                _ => panic!("->4"),
            };

            let mut macro_cstr = Punctuated::<PathSegment, Token![::]>::new();
            macro_cstr.push_value(PathSegment {
                ident: Ident::new("cstr", Span::call_site()),
                arguments: PathArguments::None,
            });

            let new_pat = match ty {
                Type::Path(tp) => {
                    match tp.path.segments.first().unwrap().ident.to_string().as_str() {
                        "S" => call_macro(macro_cstr, &format!("{}.into()", s_ident)),
                        "String" => call_macro(macro_cstr, &format!("{}", s_ident)),
                        "f64" => send_ident(false, false, ident),
                        _ => panic!("->2"),
                    }
                }
                Type::Reference(tr) => match *tr.elem {
                    Type::Path(tp) => {
                        match tp
                            .clone()
                            .path
                            .segments
                            .first()
                            .unwrap()
                            .ident
                            .to_string()
                            .as_str()
                        {
                            "str" => call_macro(macro_cstr, &format!("{}.to_string()", s_ident)),
                            _ => panic!("->5"),
                        }
                    }
                    _ => panic!("->6"),
                },
                _ => panic!("->3"),
            };

            new_pat
        }
        FnArg::Receiver(_) => panic!("->1"),
    }));

    // Needed allocations declarations for the function ouputs that will be converted to pointers for CSPICE function.
    let mut vars_out = Vec::<Pat>::new();
    let mut vars_out_decl = Vec::<Local>::new();

    // Get function ouputs
    let o_ty: syn::Type = match f.output.clone() {
        ReturnType::Default => parse_quote! { () },
        ReturnType::Type(_, ty) => {
            // Reference to function ouputs into CSPICE inputs.
            match *ty.clone() {
                Type::Tuple(tt) => {
                    tt.elems.iter().for_each(|e| match e {
                        Type::Path(tp) => {
                            let typ = tp.path.segments.first().unwrap().ident.to_string();
                            match typ.as_str() {
                                "f64" => {
                                    let ident = Ident::new(
                                        &format!("varout_{}", vars_out_decl.len()),
                                        Span::call_site(),
                                    );
                                    let init = Some(Expr::Lit(ExprLit {
                                        attrs: vec![],
                                        lit: Lit::Float(LitFloat::new("0.0f64", Span::call_site())),
                                    }));
                                    vars_out.push(send_ident(false, false, ident.clone()));
                                    vars_out_decl.push(declare_ident(true, ident.clone(), init));
                                    cspice_inputs.push(send_ident(true, true, ident));
                                }
                                _ => panic!("->9: {}", typ),
                            }
                        }
                        Type::Array(ta) => {
                            let ident = Ident::new(
                                &format!("varout_{}", vars_out_decl.len()),
                                Span::call_site(),
                            );
                            let ident_fc = Pat::Verbatim(
                                TS2::from_str(&format!(
                                    "varout_{}.as_mut_ptr()",
                                    vars_out_decl.len()
                                ))
                                .unwrap(),
                            );
                            // ::new(&format!("varout_{}.as_mut_ptr()", vars_out_decl.len()), Span::call_site());
                            let init_value = Expr::Lit(ExprLit {
                                attrs: vec![],
                                lit: Lit::Float(LitFloat::new("0.0f64", Span::call_site())),
                            });
                            let mut elems = Punctuated::<Expr, Token![,]>::new();
                            let size = match &ta.len {
                                Expr::Lit(el) => match &el.lit {
                                    Lit::Int(li) => li.base10_parse::<usize>().unwrap(),
                                    _ => panic!("->11"),
                                },
                                _ => panic!("->10"),
                            };
                            for _ in 0..size {
                                elems.push(init_value.clone());
                            }
                            let init = Some(Expr::Array(ExprArray {
                                attrs: vec![],
                                bracket_token: Bracket::default(),
                                elems,
                            }));
                            vars_out.push(send_ident(false, false, ident.clone()));
                            vars_out_decl.push(declare_ident(true, ident.clone(), init));
                            cspice_inputs.push(ident_fc);
                        }
                        _ => panic!("->8"),
                    })
                }
                Type::Path(tp) => {
                    match tp.path.segments.first().unwrap().ident.to_string().as_str() {
                        "f64" => {
                            let ident = Ident::new(
                                &format!("varout_{}", vars_out_decl.len()),
                                Span::call_site(),
                            );
                            let init = Some(Expr::Lit(ExprLit {
                                attrs: vec![],
                                lit: Lit::Float(LitFloat::new("0.0f64", Span::call_site())),
                            }));
                            vars_out.push(send_ident(false, false, ident.clone()));
                            vars_out_decl.push(declare_ident(true, ident.clone(), init));
                            cspice_inputs.push(send_ident(true, true, ident));
                        }
                        _ => panic!("->2"),
                    }
                }
                _ => panic!("->7"),
            }

            // Fill o_ty for function ouput.
            *ty
        }
    };

    let tokens = quote! {
        fn #func#generics(#inputs) -> #o_ty {
            #(#vars_out_decl)*
            // (#arg * 2 as #ty) as #o_ty
            unsafe {
                crate::c::#cspice_func(#cspice_inputs);
            }
            (#(#vars_out),*)
        }
    };
    println!("{}", tokens);
    tokens.into()
}

/*
fn load<S: Into<String>>(name: S) {
    unsafe {
        crate::c::furnsh_c(cstr!(name.into()));
    }
}
*/
