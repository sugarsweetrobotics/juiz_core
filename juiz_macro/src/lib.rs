

extern crate proc_macro;

mod util;
mod process;
mod container;
mod container_process;
mod component;

use crate::proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn juiz_container(attr: TokenStream, item: TokenStream) -> TokenStream {
    container::juiz_container_inner(attr, item)
}

#[proc_macro_attribute]
pub fn juiz_container_process(attr: TokenStream, item: TokenStream) -> TokenStream {
    container_process::juiz_container_process_inner(attr, item)
}

#[proc_macro_attribute]
pub fn juiz_process(attr: TokenStream, item: TokenStream) -> TokenStream {
    process::juiz_process_inner(attr, item)
}

#[proc_macro]
pub fn juiz_component_manifest(attr: TokenStream) -> TokenStream {
    component::juiz_component_manifest_inner(attr)
}

#[proc_macro_attribute]
pub fn juiz_component_process(attr: TokenStream, item: TokenStream) -> TokenStream {
    process::juiz_component_process_inner(attr, item)
}

#[proc_macro_attribute]
pub fn juiz_component_container(attr: TokenStream, item: TokenStream) -> TokenStream {
    container::juiz_component_container_inner(attr, item)
}

#[proc_macro_attribute]
pub fn juiz_component_container_process(attr: TokenStream, item: TokenStream) -> TokenStream {
    container_process::juiz_component_container_process_inner(attr, item)
}
