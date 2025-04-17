//! NOTE: This crate is in early development. Please be aware that it currently 
//! has very poor error handling, and using it in production environments is not
//! recommended. Contributions and improvements are welcome.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use platform_aware_features::*;
use platform_aware_amd::*;
use platform_aware_intel::*;
use platform_aware_nvidia::*;

use std::{collections::HashMap, fmt::Debug};

use proc_macro::Punct;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, *};

/// Represents a function declared in the module passsed 
/// to the `platformaware` macro. Uses only the function's
/// body (block) and the platforms/features it supports.
/// Isn't meant to be used directly. Always in a supergroup.
struct FunctionData {
    block: Block,
    kernelv: Option<TokenStream>,
}

/// A group of functions with the same name, passed as attr
/// to the `platformaware` macro.
struct FunctionGroup {
    ident: Ident,
    params: Vec<TokenStream>,
    functions: Vec<FunctionData>,
    match_arms: Vec<TokenStream>,
}

/// All functions passed to the `platformaware` macro.
struct FunctionSupergroup(Vec<FunctionGroup>);

impl FunctionData {
    fn new(block: Block, kernelv: Option<TokenStream>) -> FunctionData {
        FunctionData {
            block,
            kernelv,
        }
    }

    fn new_from_itemfn(item_fn: ItemFn) -> FunctionData {
        FunctionData::new(
            get_block_from_item_fn(&item_fn),
            get_kernel_version_from_item_fn(&item_fn),
        )
    }

    fn build_function
        (&self, params: Vec<TokenStream>, mut function_name: Ident, n: i32) 
        -> TokenStream {

        let block_ = &self.block;

        function_name = Ident::new(
            &format!("{}{}", function_name, n), Span::call_site().into());

        quote! {
            fn #function_name (#(#params),*) #block_
        }
    }
}

impl FunctionGroup {
    fn new
        (ident: Ident, params: Vec<TokenStream>, functions: Vec<FunctionData>, 
        match_arms: Vec<TokenStream>) -> FunctionGroup {

        FunctionGroup { 
            ident,
            params,
            functions,
            match_arms,
        }
    }   

    fn build_arms
        (&mut self, function_name: Ident, params_name: Vec<Ident>, n: i32) {

        self.match_arms = create_arms(function_name, params_name, n);
    }

    /// Generates a dispatch function for the group, which matches against a 
    /// resolved platform value `V` and executes the appropriate function 
    /// variant based on the match arms. Each match arm corresponds to a 
    /// specific implementation of the function identified by `ident`,
    /// and the function is called with the specified 
    /// parameters.
    fn build_group_dispatch(&self) -> TokenStream {
        let ident_ = &self.ident;
        let params_ = &self.params;
        let arms_ = &self.match_arms;

        let dispatch = quote! {
            pub fn #ident_ (#(#params_),*) {
                match *V {
                    #(#arms_),*
                }
            }
        };

        dispatch
    }

    fn build_platforms(&mut self, item_fn: &ItemFn) {
        for f in &mut self.functions {
            f.kernelv = get_kernel_version_from_item_fn(item_fn);
        }
    }
}

impl FunctionSupergroup {
    /// This function takes a `syn::ItemMod` and a vector of `syn::Ident`s as 
    /// input. It then iterates over the vector of `syn::Ident`s and creates a
    /// `FunctionGroup` for each one by iterating over the content of the
    /// `syn::ItemMod` and finding all the functions with the same name as the
    /// syn::Ident`. It then builds the arms for each `FunctionGroup` and adds
    /// it to the `FunctionSupergroup`.
    fn build_supergroup(item_mod: &ItemMod, idents: Vec<Ident>) 
        -> FunctionSupergroup {

        let mut fsg = FunctionSupergroup(Vec::new());

        for ident in idents {
            let ident_ = ident.clone();
            let mut fg = FunctionGroup::new(
                ident, Vec::new(), Vec::new(), Vec::new());
            let mut params_names: Vec<Ident> = Vec::new();

            if let Some(tuple) = &item_mod.content {
                let content_array = tuple.clone().1;
                
                content_array.into_iter().for_each(|item| {
                    match item {
                        Item::Fn(item_fn) if item_fn.sig.ident == ident_ => {
                            fg.functions.push(
                                FunctionData::new_from_itemfn(item_fn.clone()));
                            fg.params = get_inputs_from_itemfn(&item_fn); 
                            params_names = get_params_name_from_itemfn(&item_fn);
                            fg.build_platforms(&item_fn);
                        },
                        _ => {},
                    }
                })
            }

            if fg.functions.len() == 0 {
                continue
            }

            fg.build_arms(
                fg.ident.clone(), 
                params_names,
                fg.functions.len() as i32);

            fsg.0.push(fg);
        }

        fsg
    }

    fn build_all_match_arms(&self) -> Vec<TokenStream> {
        let mut arms: Vec<TokenStream> = Vec::new();

        for fg in &self.0 {
            arms.push(fg.build_group_dispatch());
        }

        arms 
    }

    fn build_all_functions(&self) -> Vec<TokenStream> {
        let mut functions: Vec<TokenStream> = Vec::new();

        for fg in &self.0 {
            for (i, f) in fg.functions.iter().enumerate() {
                functions.push(
                    f.build_function(
                        fg.params.clone(), fg.ident.clone(), i as i32));
            }
        }

        functions
    }

    fn get_all_platforms(&self) -> Vec<TokenStream> {
        let mut platforms: Vec<TokenStream> = Vec::new();

        for fg in &self.0 {
            for f in &fg.functions {
                match &f.kernelv {
                    None => continue,
                    Some(kv) => {
                        platforms.push(kv.clone());
                    },
                }
            }
        }

        platforms
    }
}

/// This macro attribute is used to generate a dispatch function, which
/// determines the correct function variant to call based on the platform
/// assumptions. The macro takes a single argument, which is a comma-separated
/// list of names of functions that should be generated as variants of the
/// dispatch function.
#[proc_macro_attribute]
pub fn platformaware
    (attr: proc_macro::TokenStream, item: proc_macro::TokenStream) 
    -> proc_macro::TokenStream {

    let item_mod = parse_macro_input!(item as ItemMod);

    let fn_ids = get_function_ids_from_attr(attr);
    let mod_name = get_name_from_itemmod(&item_mod);

    let fsg = FunctionSupergroup::build_supergroup(&item_mod, fn_ids);

    let dispatches = fsg.build_all_match_arms();
    let functions = fsg.build_all_functions();
    let platforms_list = fsg.get_all_platforms();

    let expanded = quote! {       
        mod #mod_name {
            use platform_aware_features::*;
            use platform_aware_amd::*;
            use platform_aware_intel::*;
            use platform_aware_nvidia::*;
            use std::collections::HashMap;
            use lazy_static::lazy_static;
    
            lazy_static! {
                static ref V: i32 = {
                    resolve(vec![#(#platforms_list),*])
                };
            }
    
            #(#dispatches)*
    
            #(#functions)*
        }
    };

    eprintln!("{}", proc_macro::TokenStream::from(expanded.clone()));

    proc_macro::TokenStream::from(expanded)
}

fn get_function_ids_from_attr(attr: proc_macro::TokenStream) -> Vec<Ident> {
    let mut ids: Vec<Ident> = Vec::new();
    
    attr.into_iter().for_each(|token| {
        if let proc_macro::TokenTree::Ident(ident) = token {
            ids.push(Ident::new(&ident.to_string(), Span::call_site()));
        }
    });

    ids
}

fn get_kernel_version_from_item_fn(item_fn: &ItemFn) -> Option<TokenStream> {
    let tokens = if let Meta::List(list) = &item_fn.attrs[0].meta {
        list.tokens.clone()
    } else {
        return None;
    };

    let mut is_new = true;
    let mut is_punct = false;
    let mut is_sep = false;

    //(Platform, Feature)
    let mut kernv: Vec<(TokenStream, TokenStream)> = Vec::new(); 
    let mut platform: TokenStream = TokenStream::new();
    let mut feature: TokenStream = TokenStream::new();

    // not the most efficient way of doing it... but works
    tokens.into_iter().for_each(|token| {
        if is_sep { 
            is_sep = false;
        }
        else if is_new {
            platform = token.into();
            is_punct = true;
            is_new = false;
        } else if is_punct {
            is_punct = false;
        } else {
            feature = token.into();
            is_new = true;
            kernv.push((platform.clone(), feature.clone()));
            is_punct = false;
            is_sep = true;
        }
    });

    if kernv.len() == 0 {
        return None;
    }

    let mut tokens = TokenStream::new();
    let mut tokens_len = kernv.len();
    for (platform, feature) in kernv {
        if tokens_len == 1 {
            tokens.extend(quote! {
                (PlatformParameters::#platform, &#feature as &dyn Feature)
            });
        } else {
            tokens.extend(quote! {
                (PlatformParameters::#platform, &#feature as &dyn Feature),
            })
        }

        tokens_len -= 1;
    }

    Some(quote! {
        HashMap::from([#tokens])
    })
}

fn create_arms(function_name: Ident,
    params_name: Vec<Ident>, n: i32) -> Vec<TokenStream> {

    let mut arms: Vec<TokenStream> = Vec::new();

    for i in 0..n {
        let new_function_name = Ident::new(
            &format!("{}{}", function_name, i), Span::call_site().into());
        let expr = quote! {
            #i => #new_function_name (#(#params_name),*)
        };

        arms.push(expr);
    }

    let fallback_name = Ident::new(
        &format!("{}0", function_name), Span::call_site().into());
    let expr = quote! {
        _ => #fallback_name (#(#params_name),*)
    };

    arms.push(expr);

    arms
}

fn get_block_from_item_fn(item_fn: &ItemFn) -> Block {
    *item_fn.block.clone()
}

fn get_inputs_from_itemfn(item_fn: &ItemFn) -> Vec<TokenStream>{
    let list = item_fn.sig.inputs.clone();
    let mut inputs: Vec<TokenStream> = Vec::new();

    for (i, _) in list.iter().enumerate()
    {
        let pat = if let FnArg::Typed(pat) = list.get(i).unwrap() {
            *pat.clone().pat
        } else {
            continue
        };

        let function_param = if let Pat::Ident(ident) = pat {
            ident.ident
        } else {
            continue
        };

        let param_type = {
            let typepath = if let FnArg::Typed(pattype) = list.get(i).unwrap() {
                *pattype.clone().ty
            } else {
                continue
            };

            let param_type = if let Type::Path(path) = typepath {
                path.path.segments[0].ident.clone()
            } else {
                continue
            };

            param_type
        };

        inputs.push(quote! {
            #function_param : #param_type
        });
    }

    inputs
}

fn get_name_from_itemfn(item_fn: &ItemFn) -> Ident {
    item_fn.sig.ident.clone()
}

fn get_name_from_itemmod(item_mod: &ItemMod) -> Ident {
    item_mod.ident.clone()
}

fn get_params_name_from_itemfn(item_fn: &ItemFn) -> Vec<Ident> {
    let mut params: Vec<Ident> = Vec::new();

    let list = &item_fn.sig.inputs;

    for (i, _) in list.iter().enumerate()
    {
        let pat = if let FnArg::Typed(pat) = list.get(i).unwrap() {
            *pat.clone().pat
        } else {
            continue
        };

        let function_param = if let Pat::Ident(ident) = pat {
            ident.ident
        } else {
            continue
        };

        params.push(function_param);
    }

    params
}
