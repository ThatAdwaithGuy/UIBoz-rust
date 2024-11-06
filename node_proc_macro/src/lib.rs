use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Node)]
pub fn node_trait_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl Node for #name {}
    };

    // Convert back to token stream and return
    TokenStream::from(expanded)
}
