









// lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Token};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use std::collections::HashMap;






