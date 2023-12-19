use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

pub fn derive_struct_fields(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => panic!("Only structs with named fields are supported"),
    };

    let field_names = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string())
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl tokenspan_extra::FieldNamesExt for #struct_name {
            const FIELDS: &'static [&'static str] = &[#(#field_names),*];
        }
    };

    expanded.into()
}
