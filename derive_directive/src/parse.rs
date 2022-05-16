use std::collections::HashMap;

use proc_macro::Span;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Ident;
use syn::{Type, TypePath};

use crate::errors::KVParseError;

#[derive(Debug)]
pub struct Value {
    pub key_span: Span,
    pub value: String,
    pub value_span: Span,
}
pub fn get_kv(input: TokenStream) -> Result<HashMap<String, Value>, KVParseError> {
    let mut input = input.into_iter();

    let mut kv = HashMap::new();
    loop {
        let key = match input.next() {
            Some(key) => match key {
                TokenTree::Ident(ident) => (ident.to_string(), ident.span()),
                tree => return Err(KVParseError::UnexpectedTokenTree(tree)),
            },
            None => return Ok(kv),
        };

        let punct = match input.next() {
            Some(punct) => match punct {
                TokenTree::Punct(punct) => (punct.to_string(), punct.span()),
                tree => return Err(KVParseError::UnexpectedTokenTree(tree)),
            },
            _ => return Err(KVParseError::ExpectedPunct(key.1)),
        };

        let value = match input.next() {
            Some(val) => match val {
                TokenTree::Literal(literal) => (literal.to_string(), literal.span()),
                val => return Err(KVParseError::UnexpectedTokenTree(val)),
            },
            None => return Err(KVParseError::ExpectedLiteral(punct.1)),
        };

        kv.insert(
            key.0,
            Value {
                key_span: key.1,
                value: value.0,
                value_span: value.1,
            },
        );

        let _ = match input.next() {
            Some(punct) => match punct {
                TokenTree::Punct(punct) => (punct.to_string(), punct.span()),
                tree => return Err(KVParseError::UnexpectedTokenTree(tree)),
            },
            _ => return Ok(kv),
        };
    }
}

pub fn inner_ident(ty: &Type) -> Option<&Ident> {
    match ty {
        Type::Path(TypePath { path, .. }) => Some(&path.segments.first().unwrap().ident),
        _ => None,
    }
}

pub fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath { path, .. }) => path.segments.first().unwrap().ident == "Option",
        _ => false,
    }
}
