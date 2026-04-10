use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// Procedural macro to generate a builder for an enum
/*
#[proc_macro_derive(Style)]
pub fn style_trait_derive(input: TokenStream) -> TokenStream {
     let input = parse_macro_input!(input as DeriveInput);
     let enum_name = &input.ident;
     let builder_name = format_ident!("{}Builder", enum_name);

     Collect variant information
     let variants = match &input.data {
         Data::Enum(data_enum) => &data_enum.variants,
         _ => panic!("EnumBuilder can only be used with enums"),
     };

    quote! {}.into()
}*/

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
