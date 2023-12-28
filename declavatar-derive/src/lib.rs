use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Error as SynError, Fields, LitStr};

#[proc_macro_derive(EnumLog, attributes(log_error, log_warn, log_info))]
pub fn enum_log_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match enum_log_impl(&input) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}

fn enum_log_impl(derive_input: &DeriveInput) -> Result<TokenStream, SynError> {
    let enum_tree = match &derive_input.data {
        Data::Enum(e) => e,
        _ => {
            return Err(SynError::new_spanned(
                &derive_input.ident,
                "must be implemented for enum",
            ))
        }
    };
    let (impl_generics, _, impl_where) = derive_input.generics.split_for_impl();

    for variant in &enum_tree.variants {
        let Some((key_literal, severity_ts2, erroneous)) = enum_log_attribute(&variant.attrs)?
        else {
            return Err(SynError::new_spanned(
                &variant.ident,
                "variant must have one of log_error, log_warn, log_info attr",
            ));
        };
        let fields_length = match &variant.fields {
            Fields::Unnamed(un) => un.unnamed.len(),
            Fields::Unit => 0,
            Fields::Named(_) => {
                return Err(SynError::new_spanned(
                    &variant.ident,
                    "variant cannot use named (struct-style) fields",
                ))
            }
        };
    }

    let expanded = quote! {};
    Ok(expanded.into())
}

fn enum_log_attribute(
    attrs: &[Attribute],
) -> Result<Option<(LitStr, TokenStream2, bool)>, SynError> {
    for attr in attrs {
        if attr.path().is_ident("log_error") {
            return Ok(Some((attr.parse_args()?, quote!(Error), true)));
        } else if attr.path().is_ident("log_warn") {
            return Ok(Some((attr.parse_args()?, quote!(Warning), false)));
        } else if attr.path().is_ident("log_info") {
            return Ok(Some((attr.parse_args()?, quote!(Information), false)));
        } else {
            continue;
        }
    }
    Ok(None)
}
