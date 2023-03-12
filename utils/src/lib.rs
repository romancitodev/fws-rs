use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// a derive error to make easier the implementation with `std::error::Error` and `std::fmt::Display`
#[proc_macro_derive(Error, attributes(msg))]
pub fn error_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let output = quote! {
        impl std::error::Error for #struct_name {}

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt("Error: {&self.msg}", f)
            }
        }
    };

    output.into()
}
