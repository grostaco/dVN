use parse::{inner_ident, is_option};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

use crate::parse::get_kv;

mod errors;
mod parse;

#[proc_macro_attribute]
pub fn directive(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let kv = get_kv(args.clone()).unwrap();

    let keyword = match kv.get("keyword") {
        Some(kw) => kw,
        None => {
            return syn::Error::new(ast.ident.span(), "Expected keyword attribute, none found")
                .into_compile_error()
                .into()
        }
    };

    let ident = ast.ident.clone();

    let s = match ast.data {
        Data::Struct(s) => s,
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                "`directive` attribute is exclusive to struct",
            )
            .into_compile_error()
            .into()
        }
    };

    let (
        f_idents,
        (f_opt_idents, f_opt_tys),
        (f_nonopt_idents, f_nonopt_tys),
        (f_int_idents, f_int_tys),
    ) = match s.fields {
        Fields::Named(named) => {
            let mut idents = Vec::new();
            let mut opt = Vec::new();
            let mut nonopt = Vec::new();
            let mut ints = Vec::new();
            for field in named.named.into_iter() {
                match &field.ty {
                    ty if is_option(&field.ty) => {
                        opt.push((field.ident.clone().unwrap(), ty.clone()))
                    }
                    ty => nonopt.push((field.ident.clone().unwrap(), ty.clone())),
                }
                match inner_ident(&field.ty).unwrap().to_string().as_str() {
                    "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" => {
                        ints.push((field.ident.clone().unwrap(), field.ty.clone()))
                    }
                    _ => {}
                }

                idents.push(field.ident.unwrap());
            }
            (
                idents,
                opt.into_iter().unzip::<_, _, Vec<_>, Vec<_>>(),
                nonopt.into_iter().unzip::<_, _, Vec<_>, Vec<_>>(),
                ints.into_iter().unzip::<_, _, Vec<_>, Vec<_>>(),
            )
        }
        _ => todo!(),
    };

    let tokens = quote! {
        use thiserror::Error;




        pub struct Jump {
            #(#f_nonopt_idents: #f_nonopt_tys,)*
            #(#f_opt_idents: #f_opt_tys,)*
        }

        impl Jump {
            fn parse(ctx: &std::primitive::str) -> std::result::Result<Self, DirectiveParseError> {
                let mut tokens = ctx.split(",").map(std::primitive::str::trim);


                #(let #f_idents = tokens.next();)*

                #(let #f_int_idents = #f_int_idents.map(|i| i.parse::<#f_int_tys>().unwrap());)*

                #(let #f_nonopt_idents = #f_nonopt_idents.unwrap().to_owned();)*
                #(let #f_opt_idents = #f_opt_idents.map(|x| x.to_owned());)*

                Ok(Self {
                    #(#f_nonopt_idents: #f_nonopt_idents,)*
                    #(#f_opt_idents: #f_opt_idents,)*
                })
            }
        }
    };

    tokens.into()
}

/*
#[directive(keyword = "jump")]
struct Jump {
    endpointA: String,
    endpointB: Option<String>
    endpoint: Option<>
}
*/
