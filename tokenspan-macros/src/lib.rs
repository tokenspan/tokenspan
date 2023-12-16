use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(ID)]
pub fn impl_tokenspan_id(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn new() -> Self {
                let mut snowflake = rustflake::Snowflake::new(1702720026606, 1, 1);
                let id = snowflake.generate();
                Self(id)
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0)
            }
        }

        #[async_graphql::Scalar]
        impl async_graphql::ScalarType for #name {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                use base64ct::Encoding;
                if let async_graphql::Value::String(maybe_id) = value.clone() {
                    let decoded = base64ct::Base64::decode_vec(maybe_id.as_str()).unwrap();
                    let id = String::from_utf8(decoded).unwrap();
                    let id = id.parse::<i64>().unwrap();

                    return Ok(Self(id))
                }

                Err(async_graphql::InputValueError::expected_type(value))
            }

            fn to_value(&self) -> async_graphql::Value {
                use base64ct::Encoding;
                let id = base64ct::Base64::encode_string(format!("{}", self.0).as_bytes());
                async_graphql::Value::String(id)
            }
        }

        impl From<#name> for tokenspan_extra::pagination::Cursor {
            fn from(value: #name) -> Self {
                Self::new(value.0)
            }
        }

        impl From<i64> for #name {
            fn from(value: i64) -> Self {
                Self(value)
            }
        }

        impl From<#name> for i64 {
            fn from(value: #name) -> Self {
                value.0
            }
        }

    };

    gen.into()
}
