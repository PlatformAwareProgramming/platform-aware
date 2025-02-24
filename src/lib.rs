// lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Token};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use std::collections::HashMap;

struct PlatformAssumptions{
    args:HashMap<Ident, Ident>
}

/* Esse parser vai processar os argumentos passados para a macro "platformaware", os quais definem 
   as suposições de plataforma (platform assumptions). Como sugerido na definição acima da "struct"
   PlatformAssumptions, cada "platform assumption" é um par de dois identificadores (Ident), 
   onde o primeiro é o nome do parâmetro e o segundo é a suposição aplicada.
   Ex: #[platformaware(accelerator_backend:CUDA_5_0, 
                       processor_microarchitecture: IntelLionCoveProcessor)]
*/
// ESSE CÓDIGO ABAIXO NÃO ESTÁ IMPLEMENTADO AINDA
impl Parse for PlatformAssumptions{
    fn parse(input: ParseStream) -> Result<Self> {
        let args = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(PlatformAssumptions {
            args: HashMap::new(), //args.into_iter().collect(),
        })
    }
}


// Definição da macro platformaware (por enquanto, apenas "cospe" a definição de uma função)
#[proc_macro_attribute]
pub fn platformaware(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    TokenStream::from(quote!{fn my_transformed_kernel_function() {}})
}


