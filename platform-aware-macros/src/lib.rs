use configurable_features::__internal_configurable;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn platformaware(_: TokenStream, item: TokenStream) -> TokenStream {
    __internal_configurable(item.into(), "platformaware", "kernelversion", "platform_aware").into()
}