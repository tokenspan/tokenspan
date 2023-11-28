use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    let name = &ast.ident;

    let gen = quote! {
        impl #name {
            fn hello_world() {
                println!("Hello, world! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(TeraId)]
pub fn impl_tokenspan_id(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    let name = &ast.ident;

    let gen = quote! {
        impl Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0.to_string())
            }
        }

        #[Scalar]
        impl ScalarType for #name {
            fn parse(value: Value) -> InputValueResult<Self> {
                if let Value::String(value) = value {
                    Ok(Self(value))
                } else {
                    Err(InputValueError::expected_type(value))
                }
            }

            fn to_value(&self) -> Value {
                Value::String(self.0.clone())
            }
        }

        impl From<#name> for Cursor {
            fn from(value: #name) -> Self {
                Self::new(value.0)
            }
        }

        impl From<#name> for String {
            fn from(value: #name) -> Self {
                value.0
            }
        }
    };

    gen.into()
}
