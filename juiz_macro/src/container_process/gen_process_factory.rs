
use quote::quote;

pub(crate) fn factory_tokenstream(function_ident: syn::Ident) -> proc_macro::TokenStream {
    // まず土台となる関数定義
    quote!{
       #[no_mangle]
        pub unsafe extern "Rust" fn container_process_factory() -> JuizResult<ContainerProcessFactoryStruct> {
            env_logger::init();
            Ok(juiz_sdk::container_process_factory(manifest2(), #function_ident))
        }
    }.into()
}

