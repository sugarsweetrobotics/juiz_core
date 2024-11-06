

use std::collections::HashMap;

use quote::{format_ident, quote};
use serde_json::json;
use crate::proc_macro::TokenStream;
use syn::TypePath;


pub(crate) fn manifest_tokenstream() -> TokenStream {
    quote!{
        pub fn manifest2() -> juiz_sdk::prelude::ContainerManifest { 
        }
    }.into()
}

pub(crate) fn component_manifest_tokenstream(proc_type_str: String) -> TokenStream {
    let manifest_function_name_ident = format_ident!("{}", proc_type_str + "_manifest");
    quote!{
        pub fn #manifest_function_name_ident() -> juiz_sdk::prelude::ContainerManifest { 
        }
    }.into()
}

pub(crate) fn construct_manif_tokenstream(function_name: String, manifest_attr: &serde_json::Value, _arg_map: &HashMap<TypePath, syn::Ident>) -> TokenStream {
    // attr変数から読み取った値からdescriptionを取得
    let description = manifest_attr.as_object().unwrap().get("description").and_then(|v| { Some(v.clone()) }).or(Some(json!(format!("Default description of Container({function_name})")))).unwrap().as_str().unwrap().to_owned();
    // ここでmanifestデータの基本データを作成する部分
    
    let construct_manif = quote!{
        let mut manif = ContainerManifest::new( #function_name ).description(#description);
    };
    
    quote!{
        {
            #construct_manif
            manif
        }
    }.into()

}


pub(crate) fn component_construct_manif_tokenstream(function_name: String, manifest_attr: &serde_json::Value, _arg_map: &HashMap<TypePath, syn::Ident>, factory_name: String) -> TokenStream {
    // attr変数から読み取った値からdescriptionを取得
    let description = manifest_attr.as_object().unwrap().get("description").and_then(|v| { Some(v.clone()) }).or(Some(json!(format!("Default description of Container({function_name})")))).unwrap().as_str().unwrap().to_owned();
    // ここでmanifestデータの基本データを作成する部分
    
    let mut construct_manif = quote!{
        let mut manif = ContainerManifest::new( #function_name ).description(#description);
    };
    
    construct_manif = quote!{
        #construct_manif
        manif = manif.factory(#factory_name);
    };

    quote!{
        {
            #construct_manif
            manif
        }
    }.into()

}
