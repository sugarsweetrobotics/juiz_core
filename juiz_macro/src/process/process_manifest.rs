use quote::{format_ident, quote, ToTokens};
use serde_json::json;
use crate::proc_macro::TokenStream;
use syn::TypePath;


pub(crate) fn manifest_tokenstream() -> TokenStream {
    quote!{
        pub(crate) fn manifest() -> juiz_sdk::prelude::ProcessManifest { 
        }
    }.into()
}


pub(crate) fn component_manifest_tokenstream(proc_type_str: String) -> TokenStream {
    let manifest_function_name_ident = format_ident!("{}", proc_type_str + "_manifest");
    quote!{
        pub(crate) fn #manifest_function_name_ident() -> juiz_sdk::prelude::ProcessManifest { 
        }
    }.into()
}


fn construct_manif_inner(function_name: String, manifest_attr: &serde_json::Value, arg_map: &Vec<(TypePath, syn::Ident)>) -> proc_macro2::TokenStream {
    // attr変数から読み取った値からdescriptionを取得
    let description = manifest_attr.as_object().unwrap().get("description").and_then(|v| { Some(v.clone()) }).or(Some(json!(format!("Default description of Process({function_name})")))).unwrap().as_str().unwrap().to_owned();
    
    let mut construct_manif = quote!{
        let mut manif = ProcessManifest::new( #function_name ).description(#description);
    };
    
    let empty_value = json!({}); // 空っぽのMapは使い回す。
    // attrから受け取ったmanifest情報を使いやすいMapに変更してからforに飛び込む！
    let argument_value = manifest_attr.as_object().unwrap().get("arguments").or( Some(&empty_value)).unwrap().as_object().unwrap();
    let argument_default_value = argument_value.get("default").or( Some(&empty_value)).unwrap().as_object().unwrap();
    let argument_description_value = argument_value.get("description").or( Some(&empty_value)).unwrap().as_object().unwrap();
    // itemから受け取った引数情報をイテレートしつつ、attrからもらった部分にデフォルト引数およびデスクリプションの情報があれば、それを追加していく
    for (type_path, arg_name_ident) in arg_map.into_iter() {
        let arg_name = arg_name_ident.to_string();
        let default_desc = format!("Default description for argument {arg_name}");
        let description = argument_description_value.get(arg_name.as_str()).and_then(|v| { v.as_str() }).or(Some(default_desc.as_str())).unwrap();
        let type_name = type_path.path.segments.iter().map(|seg| {seg.ident.to_string()}).collect::<Vec<String>>().join("::");
        match type_name.as_str() { // 引数のタイプで分岐。タイプはitemから受け取ったタイプが優先。defaultが別のタイプだったらデフォルトのデフォルトが振られる。
            "bool" => {
                let default_value_default = false;
                let default_value = argument_default_value.get(arg_name.as_str()).and_then(|v| { v.as_bool() }).or( Some(default_value_default)).unwrap();
                construct_manif = quote!{
                    #construct_manif
                    manif = manif.add_bool_arg(#arg_name, #description, #default_value);
                }
            },
            "i64" => {
                let default_value_default = 0;
                let default_value = argument_default_value.get(arg_name.as_str()).and_then(|v| { v.as_i64() }).or( Some(default_value_default)).unwrap();
                construct_manif = quote!{
                    #construct_manif
                    manif = manif.add_int_arg(#arg_name, #description, #default_value);
                }
            },
            "f64" => {
                let default_value_default = 0.0;
                let default_value = argument_default_value.get(arg_name.as_str()).and_then(|v| { v.as_f64() }).or( Some(default_value_default)).unwrap();
                construct_manif = quote!{
                    #construct_manif
                    manif = manif.add_float_arg(#arg_name, #description, #default_value);
                }
            },
            "String" => {
                let default_value_default = "";
                let default_value = argument_default_value.get(arg_name.as_str()).and_then(|v| { v.as_str() }).or( Some(default_value_default)).unwrap();
                construct_manif = quote!{
                    #construct_manif
                    manif = manif.add_string_arg(#arg_name,  #description, #default_value);
                }
            },
            "Vec" => {
                let arguments = type_path.path.segments.first().unwrap().arguments.clone();
                // println!("args: {arguments:?}");
                let vec_element_typename = match arguments.into_token_stream().into_iter().find_map(|v|{ match v {
                    proc_macro2::TokenTree::Ident(ident) => Some(ident.to_string()),
                    _ => None,
                }}) {
                    None => panic!("Vecの引数が見つかりません"),
                    Some(v) => v,
                };
                match vec_element_typename.as_str() {
                    "Value" => { // Vec<Value>の場合
                        construct_manif = quote!{
                            #construct_manif
                            manif = manif.add_array_arg(#arg_name, #description, serde_json::from_str("[]").unwrap());
                        }
                    }
                    "f64" => {
                        construct_manif = quote!{
                            #construct_manif
                            manif = manif.add_array_arg(#arg_name, #description, serde_json::from_str("[]").unwrap());
                        }
                    }
                    "i64" => {
                        construct_manif = quote!{
                            #construct_manif
                            manif = manif.add_array_arg(#arg_name, #description, serde_json::from_str("[]").unwrap());
                        }
                    }
                    _ => {
                        panic!("対応していないVecのElement({vec_element_typename}) です。")
                    }
                }
            }
            "Value" => {
                let default_value_default = json!({});
                let default_value = argument_default_value.get(arg_name.as_str()).or( Some(&default_value_default)).unwrap();
                let value_str = serde_json::to_string(default_value).unwrap();
                construct_manif = quote!{
                    #construct_manif
                    manif = manif.add_object_arg(#arg_name, #description, serde_json::from_str(#value_str).unwrap());
                }
            },
            "DynamicImage" => {
                construct_manif = quote!{
                    #construct_manif
                    manif = manif.add_image_arg(#arg_name, #description);
                }
            },
            _ => {
                panic!("[process_manifest.rs]自動マニフェスト生成に失敗。対応するデータ型ではありません。 ({type_name:}/(type_path={type_path:?}))")
            }
        }
    }
    construct_manif
}


pub(crate) fn construct_manif_tokenstream(function_name: String, manifest_attr: &serde_json::Value, arg_map: &Vec<(TypePath, syn::Ident)>) -> TokenStream {
    
    let construct_manif: proc_macro2::TokenStream = construct_manif_inner(function_name, manifest_attr, arg_map);

    quote!{
        {
            #construct_manif
            manif
        }
    }.into()

}

pub(crate) fn component_construct_manif_tokenstream(function_name: String, manifest_attr: &serde_json::Value, arg_map: &Vec<(TypePath, syn::Ident)>, factory_name: String) -> TokenStream {
    
    let mut construct_manif: proc_macro2::TokenStream = construct_manif_inner(function_name, manifest_attr, arg_map);
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
