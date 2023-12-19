use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

use crate::derive_struct_fields::derive_struct_fields;

mod derive_struct_fields;

#[proc_macro_derive(FieldNames)]
pub fn impl_derive_struct_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_struct_fields(input)
}
