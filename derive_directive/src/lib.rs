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

    let kv = get_kv(args).unwrap();

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

    let prefix = &keyword.value.trim_matches('\"');

    let tokens = quote! {
        use directive_errors::DirectiveParseError;

        pub struct #ident {
            #(#f_nonopt_idents: #f_nonopt_tys,)*
            #(#f_opt_idents: #f_opt_tys,)*
        }

        impl #ident {
            fn parse(ctx: &std::primitive::str) -> std::option::Option<std::result::Result<Self, DirectiveParseError>> {
                if !ctx.starts_with(&format!("@{}", #prefix)) {
                    return None;
                }

                let mut tokens = ctx.get(ctx.find('(').unwrap() + 1..ctx.rfind(')').unwrap()).unwrap().split(",").map(std::primitive::str::trim);

                #(let #f_idents = tokens.next();)*

                #(let #f_int_idents = match #f_int_idents.map(|i| i.parse::<#f_int_tys>()) {
                    Some(Err(e)) => return Some(Err(DirectiveParseError::CannotParse(format!("\"{}\"", #f_int_idents.unwrap().to_string()), stringify!(#f_int_tys).to_string(), e.to_string()))),
                    Some(v) => Some(v.unwrap()),
                    None => None,
                };)*

                let mut pos = 1;
                #(let #f_nonopt_idents = match #f_nonopt_idents {
                    Some(v) => v.to_owned(),
                    None => return Some(Err(DirectiveParseError::ExpectPositionalArgument(format!("\"{}\"", stringify!(#f_nonopt_idents)).to_string(), pos))),
                }; pos += 1;)*
                #(let #f_opt_idents = #f_opt_idents.map(|x| x.to_owned());)*

                Some(Ok(Self {
                    #(#f_nonopt_idents: #f_nonopt_idents,)*
                    #(#f_opt_idents: #f_opt_idents,)*
                }))
            }
        }
    };

    tokens.into()
}
