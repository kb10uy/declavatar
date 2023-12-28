use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Error as SynError, Fields, FieldsUnnamed,
    LitStr,
};

#[proc_macro_derive(EnumLog, attributes(log_error, log_warn, log_info))]
pub fn enum_log_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match enum_log_generate(&input) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}

fn enum_log_generate(derive_input: &DeriveInput) -> Result<TokenStream, SynError> {
    let enum_tree = match &derive_input.data {
        Data::Enum(e) => e,
        _ => {
            return Err(SynError::new_spanned(
                &derive_input.ident,
                "must be implemented for enum",
            ))
        }
    };
    let enum_name = &derive_input.ident;
    let (impl_generics, ty_generics, where_where) = derive_input.generics.split_for_impl();

    if enum_tree.variants.is_empty() {
        return Ok(quote! {
            impl #impl_generics crate::log::Log for #enum_name #ty_generics #where_where {
                fn erroneous(&self) -> bool {
                    unreachable!("no variant generated");
                }

                fn serialize_log<'a, C: std::iter::Iterator<Item = &'a dyn crate::log::Context>>(&self, context: C) -> crate::log::SerializedLog {
                    unreachable!("no variant generated");
                }
            }
        }.into());
    }

    let mut erroneous_arms = vec![];
    let mut serialize_log_arms = vec![];
    for variant in &enum_tree.variants {
        let variant_ident = &variant.ident;
        let Some((key_literal, severity_ts2, erroneous_ts2)) =
            enum_log_find_attribute(&variant.attrs)?
        else {
            return Err(SynError::new_spanned(
                &variant.ident,
                "variant must have one of log_error, log_warn, log_info attr",
            ));
        };
        match &variant.fields {
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let fields: Vec<_> = unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format_ident!("_f{i}",))
                    .collect();
                erroneous_arms.push(quote!(Self::#variant_ident(#(#fields),*) => #erroneous_ts2));
                serialize_log_arms.push(quote!(
                    Self::#variant_ident(#(#fields),*) => (
                        crate::log::Severity::#severity_ts2,
                        #key_literal,
                        vec![(#(#fields.to_string()),*)],
                    )
                ));
            }
            Fields::Unit => {
                erroneous_arms.push(quote!(Self::#variant_ident => #erroneous_ts2));
                serialize_log_arms.push(quote!(
                    Self::#variant_ident => (
                        crate::log::Severity::#severity_ts2,
                        #key_literal,
                        vec![],
                    )
                ));
            }
            Fields::Named(_) => {
                return Err(SynError::new_spanned(
                    &variant.ident,
                    "variant cannot use named (struct-style) fields",
                ))
            }
        };
    }

    let expanded = quote! {
        impl #impl_generics crate::log::Log for #enum_name #ty_generics #where_where {
            fn erroneous(&self) -> bool {
                match self {
                    #(#erroneous_arms),*
                }
            }

            fn serialize_log<'a, C: std::iter::Iterator<Item = &'a dyn crate::log::Context>>(&self, context: C) -> crate::log::SerializedLog {
                let (severity, kind, args) = match self {
                    #(#serialize_log_arms),*
                };
                let context = context.into_iter().map(|c| c.to_string()).collect();
                crate::log::SerializedLog {
                    severity,
                    kind: kind.into(),
                    args,
                    context,
                }
            }
        }
    };
    Ok(expanded.into())
}

fn enum_log_find_attribute(
    attrs: &[Attribute],
) -> Result<Option<(LitStr, TokenStream2, TokenStream2)>, SynError> {
    for attr in attrs {
        if attr.path().is_ident("log_error") {
            return Ok(Some((attr.parse_args()?, quote!(Error), quote!(true))));
        } else if attr.path().is_ident("log_warn") {
            return Ok(Some((attr.parse_args()?, quote!(Warning), quote!(false))));
        } else if attr.path().is_ident("log_info") {
            return Ok(Some((
                attr.parse_args()?,
                quote!(Information),
                quote!(false),
            )));
        } else {
            continue;
        }
    }
    Ok(None)
}
