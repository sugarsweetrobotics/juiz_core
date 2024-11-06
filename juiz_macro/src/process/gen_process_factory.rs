
use quote::{format_ident, quote};

pub(crate) fn factory_tokenstream(function_ident: syn::Ident) -> proc_macro::TokenStream {
    // まず土台となる関数定義
    quote!{
        #[no_mangle]
        pub unsafe extern "Rust" fn process_factory() -> JuizResult<ProcessFactoryStruct> {
            env_logger::init();
            Ok(juiz_sdk::process_factory(manifest2(), #function_ident))
        }
    }.into()
}


pub(crate) fn component_factory_tokenstream(function_ident: syn::Ident, factory_name: String) -> proc_macro::TokenStream {
    let factory_name_ident = format_ident!("{}", factory_name);

    let manifest_function_name_ident = format_ident!("{}", function_ident.to_string() + "_manifest");
    // まず土台となる関数定義
    quote!{
        #[no_mangle]
        pub unsafe extern "Rust" fn #factory_name_ident() -> JuizResult<ProcessFactoryStruct> {
            env_logger::init();
            Ok(juiz_sdk::process_factory(#manifest_function_name_ident(), #function_ident))
        }
    }.into()
}
