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
                Self(ObjectId::new())
            }
        }

        impl Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0.to_string())
            }
        }

        #[Scalar]
        impl ScalarType for #name {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                if let async_graphql::Value::String(maybe_id) = value.clone() {
                    if let Ok(object_id) = mongodb::bson::oid::ObjectId::parse_str(&maybe_id) {
                        return Ok(Self(object_id));
                    }

                    return Err(async_graphql::InputValueError::expected_type(value));
                } else {
                    return Err(async_graphql::InputValueError::expected_type(value))
                }
            }

            fn to_value(&self) -> async_graphql::Value {
                async_graphql::Value::String(self.to_string())
            }
        }

        impl From<#name> for tokenspan_utils::pagination::Cursor {
            fn from(value: #name) -> Self {
                Self::new(value.to_string())
            }
        }

        // impl TryFrom<String> for #name {
        //     type Error = anyhow::Error;
        //
        //     fn try_from(value: String) -> Result<Self, Self::Error> {
        //         if let Ok(object_id) = mongodb::bson::oid::ObjectId::parse_str(&value) {
        //             return Ok(Self(object_id));
        //         }
        //
        //         Err(anyhow::anyhow!("invalid id"))
        //     }
        // }

        impl From<#name> for String {
            fn from(value: #name) -> Self {
                value.0.to_string()
            }
        }

        impl From<String> for #name {
            fn from(value: String) -> Self {
                let oid = mongodb::bson::oid::ObjectId::parse_str(&value).unwrap();
                Self(oid)
            }
        }

        impl From<&str> for #name {
            fn from(value: &str) -> Self {
                let oid = mongodb::bson::oid::ObjectId::parse_str(&value).unwrap();
                Self(oid)
            }
        }

        impl From<#name> for mongodb::bson::oid::ObjectId {
            fn from(value: #name) -> Self {
                value.0
            }
        }

        impl From<&#name> for mongodb::bson::oid::ObjectId {
            fn from(value: &#name) -> Self {
                value.0
            }
        }

        impl From<mongodb::bson::oid::ObjectId> for #name {
            fn from(value: mongodb::bson::oid::ObjectId) -> Self {
                Self(value)
            }
        }

        // impl serde::Serialize for #name {
        //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        //     where
        //         S: serde::Serializer,
        //     {
        //         serializer.serialize_str(&self.0.to_string())
        //     }
        // }
        //
        // impl<'de> serde::Deserialize<'de> for #name {
        //     fn deserialize<D>(deserializer: D) -> Result<#name, D::Error>
        //     where
        //         D: serde::Deserializer<'de>,
        //     {
        //         let s = String::deserialize(deserializer)?;
        //         let id = #name::try_from(s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        //         Ok(id)
        //     }
        // }
    };

    gen.into()
}
