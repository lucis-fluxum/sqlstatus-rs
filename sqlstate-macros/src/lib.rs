use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Ident, LitStr};

#[proc_macro_attribute]
pub fn class(_attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut class_enum = syn::parse_macro_input!(item as syn::ItemEnum);
    let mut subclasses: HashMap<Ident, LitStr> = HashMap::new();

    for variant in class_enum.variants.iter_mut() {
        let attrs = &mut variant.attrs;
        // TODO: Replace with attrs.drain_filter(...)
        let mut i = 0;
        while i < attrs.len() {
            if attrs[i].path.is_ident("subclass") {
                let attr = attrs.remove(i);
                let code: LitStr = attr.parse_args().unwrap();
                subclasses.insert(variant.ident.clone(), code);
            } else {
                i += 1;
            }
        }
    }

    let attributes = &class_enum.attrs;
    let visibility = &class_enum.vis;
    let class_ident = &class_enum.ident;
    let variants = &class_enum.variants;
    let from_str_arms = subclasses
        .iter()
        .map(|(variant, code)| quote! { #code => Ok(Self::#variant), });
    let as_str_arms = subclasses
        .iter()
        .map(|(variant, code)| quote! { Self::#variant => #code, });

    quote!(
        #(#attributes)*
        #visibility enum #class_ident {
            #variants
            Other(::std::string::String),
        }

        impl ::std::str::FromStr for #class_ident {
            type Err = ::std::convert::Infallible;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    #(#from_str_arms)*
                    other => Ok(Self::Other(other.to_string())),
                }
            }
        }

        impl #class_ident {
            pub fn as_str(&self) -> &str {
                match self {
                    #(#as_str_arms)*
                    Self::Other(subclass) => subclass.as_str(),
                }
            }
        }
    )
    .into()
}

#[proc_macro_attribute]
pub fn state(_attr_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut state_enum = syn::parse_macro_input!(item as syn::ItemEnum);
    // Mapping of class code -> (variant ident, whether it's a tuple variant)
    let mut classes: HashMap<LitStr, (Ident, bool)> = HashMap::new();

    for variant in state_enum.variants.iter_mut() {
        let attrs = &mut variant.attrs;
        // TODO: Replace with attrs.drain_filter(...)
        let mut i = 0;
        while i < attrs.len() {
            if attrs[i].path.is_ident("class") {
                let attr = attrs.remove(i);
                let code: LitStr = attr.parse_args().unwrap();
                classes.insert(
                    code,
                    (
                        variant.ident.clone(),
                        matches!(variant.fields, Fields::Unnamed(_)),
                    ),
                );
            } else {
                i += 1;
            }
        }
    }

    let attributes = &state_enum.attrs;
    let visibility = &state_enum.vis;
    let state_ident = &state_enum.ident;
    let variants = &state_enum.variants;
    let from_str_arms = classes.iter().map(|(code, (variant, is_tuple))| {
        if *is_tuple {
            quote! { #code => Ok(Self::#variant(subclass.parse().unwrap())) }
        } else {
            quote! { #code => Ok(Self::#variant), }
        }
    });

    quote!(
        #(#attributes)*
        #visibility enum #state_ident {
            #variants
            Other(::std::string::String),
        }

        impl ::std::str::FromStr for #state_ident {
            type Err = crate::error::ParseError;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                // SQL standard requires length to be 5 bytes
                if s.len() != 5 {
                    return Err(crate::error::ParseError::InvalidLength(s.len()));
                }

                let (class, subclass) = s.split_at(2);

                match class {
                    #(#from_str_arms)*
                    _ => Ok(Self::Other(value.to_string())),
                }
            }
        }
    )
    .into()
}
