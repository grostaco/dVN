use parse::{inner_ident, is_option};
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

use crate::parse::{get_kv, inner_option};

mod errors;
mod parse;

#[proc_macro_attribute]
pub fn directive(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let kv = get_kv(args).unwrap();

    let keyword = match kv.get("keyword") {
        Some(kw) => match &kw.value {
            TokenTree::Literal(literal) => literal.to_string(),
            tt => {
                return syn::Error::new(tt.span().into(), "Keyword must be a literal")
                    .into_compile_error()
                    .into()
            }
        },
        None => {
            return syn::Error::new(ast.ident.span(), "Expected keyword attribute, none found")
                .into_compile_error()
                .into()
        }
    };

    let verify = match kv.get("verify") {
        Some(kw) => match kw.value.clone() {
            TokenTree::Ident(ident) => Ident::new(&ident.to_string(), ident.span().into()),
            tt => {
                return syn::Error::new(tt.span().into(), "Expected ident")
                    .into_compile_error()
                    .into()
            }
        },
        None => Ident::new("dummy", Span::call_site()),
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
        (f_parse_idents, f_parse_tys),
        (f_parseopt_idents, f_parseopt_tys),
    ) = match s.fields {
        Fields::Named(named) => {
            let mut idents = Vec::new();
            let mut opt = Vec::new();
            let mut nonopt = Vec::new();
            let mut parsable = Vec::new();
            let mut parsable_opt: Vec<(Ident, Type)> = Vec::new();
            for field in named.named.into_iter() {
                match &field.ty {
                    ty if is_option(&field.ty) => {
                        //println!("{:#?} {:#?}", ty, inner_option(ty).unwrap().to_string().as_str());
                        // opt.push((field.ident.clone().unwrap(), ty.clone()));

                        let inner_ty = inner_option(ty).unwrap();
                        match inner_ident(inner_ty).unwrap().to_string().as_str() {
                            "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "f32"
                            | "f64" => {
                                // parsable_opt.push((field.ident.clone().unwrap(), ty.clone()))
                                parsable_opt.push((field.ident.clone().unwrap(), inner_ty.clone()))
                            }
                            _ => {}
                        }
                        opt.push((field.ident.clone().unwrap(), ty.clone()))
                    }
                    ty => nonopt.push((field.ident.clone().unwrap(), ty.clone())),
                }
                match inner_ident(&field.ty).unwrap().to_string().as_str() {
                    "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "f32" | "f64" => {
                        parsable.push((field.ident.clone().unwrap(), field.ty.clone()))
                    }
                    _ => {}
                }

                idents.push(field.ident.unwrap());
            }
            (
                idents,
                opt.into_iter().unzip::<_, _, Vec<_>, Vec<_>>(),
                nonopt.into_iter().unzip::<_, _, Vec<_>, Vec<_>>(),
                parsable.into_iter().unzip::<_, _, Vec<_>, Vec<_>>(),
                parsable_opt.into_iter().unzip::<_, _, Vec<_>, Vec<_>>(),
            )
        }
        _ => todo!(),
    };
    //::directive_errors::Directive;
    let prefix = keyword.trim_matches('\"');

    let tokens = quote! {
        pub struct #ident {
            #(pub #f_nonopt_idents: #f_nonopt_tys,)*
            #(pub #f_opt_idents: #f_opt_tys,)*
        }

        impl ::directive_errors::Directive for #ident {
            fn parse(ctx: &std::primitive::str) -> std::option::Option<std::result::Result<Self, ::directive_errors::ParseError>> {
                use directive_errors::Directive;
                if !ctx.starts_with(&format!("@{}", #prefix)) {
                    return None;
                }

                let mut tokens = ctx.get(ctx.find('(').unwrap() + 1..ctx.rfind(')').unwrap()).unwrap().split(",").map(std::primitive::str::trim);
                if ctx.rfind(')').unwrap() - ctx.find('(').unwrap() == 1 {
                    tokens.next();
                }

                #(let #f_idents = tokens.next();)*

                #(let #f_parseopt_idents = match #f_parseopt_idents.map(|i| i.parse::<#f_parseopt_tys>()) {
                    Some(Err(e)) => return Some(Err(::directive_errors::ParseError::CannotParse(format!("\"{}\"", #f_parseopt_idents.unwrap().to_string()), stringify!(#f_parseopt_tys).to_string(), e.to_string()))),
                    Some(v) => Some(v.unwrap()),
                    None => None,
                };)*

                #(let #f_parse_idents = match #f_parse_idents.map(|i| i.parse::<#f_parse_tys>()) {
                    Some(Err(e)) => return Some(Err(::directive_errors::ParseError::CannotParse(format!("\"{}\"", #f_parse_idents.unwrap().to_string()), stringify!(#f_parse_tys).to_string(), e.to_string()))),
                    Some(v) => Some(v.unwrap()),
                    None => None,
                };)*

                // #(let #f_parseopt_idents = match #f_parseopt_idents.map(|i| i.parse::<#f_parseopt_tys>()) {
                //     Some(Err(e)) => return Some(Err(::directive_errors::ParseError::CannotParse(format!("\"{}\"", #f_parse_idents.unwrap().to_string()), stringify!(#f_parse_tys).to_string(), e.to_string()))),
                //     Some(v) => Some(v.unwrap()),
                //     None => None,
                // };)*

                let mut pos = 1;
                #(let #f_nonopt_idents = match #f_nonopt_idents {
                    Some(v) => { v.to_owned() },
                    None => return Some(Err(::directive_errors::ParseError::ExpectPositionalArgument(format!("\"{}\"", stringify!(#f_nonopt_idents)).to_string(), pos))),
                }; pos += 1;)*
                #(let #f_opt_idents = #f_opt_idents.map(|x| x.to_owned());)*

                Some(Ok(Self {
                    #(#f_nonopt_idents: #f_nonopt_idents,)*
                    #(#f_opt_idents: #f_opt_idents,)*

                }))
            }

            fn verify(&self) -> std::result::Result<(), ::directive_errors::VerifyError> {
                fn dummy(s: &#ident) -> std::result::Result<(), ::directive_errors::VerifyError> {
                    Ok(())
                }

                #verify(&self)
            }
        }
    };

    tokens.into()
}

#[proc_macro_derive(DirectiveEnum)]
pub fn directive_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let e = match ast.data {
        Data::Enum(e) => e,
        _ => {
            return syn::Error::new(
                ast.ident.span(),
                "`DirectiveEnum` proc-macro is exclusive to enum",
            )
            .into_compile_error()
            .into()
        }
    };

    let (f_variants, f_tys) = e
        .variants
        .into_iter()
        .map(|v| {
            (
                v.ident,
                match v.fields {
                    Fields::Unnamed(unnamed) => unnamed.unnamed.first().unwrap().ty.clone(),
                    _ => todo!(),
                },
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    let tokens = quote! {
        impl #ident {
            pub fn parse(ctx: &std::primitive::str) -> std::option::Option<std::result::Result<Self, ::directive_errors::ParseError>> {
               use directive_errors::Directive;
               #(if let Some(directive) = #f_tys::parse(ctx) {
                    match directive {
                        Ok(directive) => return Some(
                            match directive.verify() {
                            Ok(_) => Ok(Self::#f_variants(directive)),
                            Err(e) => Err(::directive_errors::ParseError::VerifyError(e)),
                        }),
                        Err(e) => return Some(Err(e)),
                    }}
                )*
               None
            }
        }

    };
    tokens.into()
}

// #(let #f_float_idents = match #f_float_idents.map(|i| i.parse::<#f_float_tys>()) {
//     Some(Err(e)) => return Some(Err(ParseError::CannotParse(format!("\"{}\"", #f_float_idents.unwrap().to_string()), stringify!(#f_float_tys).to_string(), e.to_string()))),
//     Some(v) => Some(v.unwrap()),
//     None => None,
// };)*
