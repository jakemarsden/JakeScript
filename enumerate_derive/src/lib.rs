#![feature(derive_default_enum)]

use darling::{FromDeriveInput, FromMeta, FromVariant};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, LitStr, Variant};

#[proc_macro_derive(Enumerate)]
pub fn derive_enumerate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let output = derive_enumerate_impl(&input);
    proc_macro::TokenStream::from(output)
}

#[proc_macro_derive(EnumerateStr, attributes(enumerate_str))]
pub fn derive_enumerate_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let output = derive_enumerate_str_impl(&input);
    proc_macro::TokenStream::from(output)
}

fn derive_enumerate_impl(DeriveInput { ident, data, .. }: &DeriveInput) -> TokenStream {
    let DataEnum { variants, .. } = match data {
        Data::Enum(data) => data,
        _ => panic!("Expected an enum type"),
    };

    let variant_idents: Vec<_> = variants
        .iter()
        .map(|variant| variant.ident.to_owned())
        .collect();

    let fn_body = quote! {
        const VALUES: &[#ident] = &[
            #(#ident::#variant_idents, )*
        ];
        VALUES
    };
    quote! {
        impl enumerate::Enumerate for #ident {
            fn enumerate() -> &'static [Self] {
                #fn_body
            }
        }
    }
}

fn derive_enumerate_str_impl(input @ DeriveInput { ident, data, .. }: &DeriveInput) -> TokenStream {
    let opts: EnumerateStrOpts = FromDeriveInput::from_derive_input(input).unwrap();
    let DataEnum { variants, .. } = match data {
        Data::Enum(data) => data,
        _ => panic!("Expected an enum type"),
    };

    let variant_idents: Vec<_> = variants.iter().map(|variant| &variant.ident).collect();
    let variant_values: Vec<_> = variants
        .iter()
        .map(|variant| variant_name(variant, opts.rename_all, ident.span()))
        .collect();

    let as_str_body = if !variants.is_empty() {
        quote! {
            match self {
                #(Self::#variant_idents => #variant_values, )*
            }
        }
    } else {
        quote! {
            match *self {}
        }
    };
    let from_str_body = if !variants.is_empty() {
        quote! {
            Ok(match s {
                #(#variant_values => Self::#variant_idents, )*
                _ => return Err(enumerate::NoSuchVariantError),
            })
        }
    } else {
        quote! {
            Err(enumerate::NoSuchVariantError)
        }
    };
    quote! {
        impl enumerate::EnumerateStr for #ident {
            fn as_str(&self) -> &'static str {
                #as_str_body
            }
        }
        impl std::str::FromStr for #ident {
            type Err = enumerate::NoSuchVariantError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #from_str_body
            }
        }
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    }
}

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(enumerate_str), forward_attrs(allow, doc, fg))]
struct EnumerateStrOpts {
    rename_all: RenameAll,
}

#[derive(FromVariant, Default)]
#[darling(default, attributes(enumerate_str), forward_attrs(allow, doc, cfg))]
struct EnumerateStrVariantOpts {
    rename: Option<String>,
}

#[derive(FromMeta, Copy, Clone, Default, Eq, PartialEq, Debug)]
#[darling(default)]
enum RenameAll {
    #[default]
    None,
    #[darling(rename = "lowercase")]
    Lowercase,
    #[darling(rename = "UPPERCASE")]
    Uppercase,
}

impl RenameAll {
    fn apply(&self, s: String) -> String {
        match self {
            Self::None => s,
            Self::Lowercase => s.to_lowercase(),
            Self::Uppercase => s.to_uppercase(),
        }
    }
}

fn variant_name(variant: &Variant, fallback_strategy: RenameAll, span: Span) -> LitStr {
    let opts: EnumerateStrVariantOpts = darling::FromVariant::from_variant(variant).unwrap();
    opts.rename
        .as_ref()
        .map(|s| LitStr::new(s, span))
        .unwrap_or_else(|| LitStr::new(&fallback_strategy.apply(variant.ident.to_string()), span))
}
