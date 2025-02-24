// lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

// using proc_macro_attribute to declare an attribute like procedural macro
#[proc_macro_attribute]
// _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
pub fn platformaware(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    // returing a simple TokenStream for Struct
    TokenStream::from(quote!{fn my_transformed_kernel_function() {}})
}


