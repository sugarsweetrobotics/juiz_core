use proc_macro::TokenStream;
use serde_json::{Map, Value};
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, Stmt};

use crate::util::parse_attr;


pub(crate) fn juiz_component_manifest_inner(attr: TokenStream) -> TokenStream {
    let manifest_attr = parse_attr(attr);
    let container_name = match manifest_attr.get("container_name") {
        Some(cn_value) => {
            match cn_value.as_str() {
                Some(cn) => {
                    cn.to_owned()
                }
                None => {
                    panic!("container_nameは文字列出なければなりません");
                }
            }
        }
        None => {
            panic!("container_nameを指定してください")
        }
    };

    
    let token_stream: TokenStream = quote!{
        #[no_mangle]
        pub unsafe extern "Rust" fn component_manifest() -> ComponentManifest {
            //env_logger::init();
            let mut manif = ComponentManifest::new(#container_name);

        }
    }.into();

    let mut item_fn: ItemFn = syn::parse_macro_input!(token_stream as ItemFn);

    let proc_attrs: Vec<Value> = match manifest_attr.get("processes") {
        Some(v) => {
            match v.as_array() {
                Some(arr) => {
                    arr.clone()
                }
                None => {
                    panic!("processesアトリビュートはリストでなければなりません。")
                },
            }
        }
        None => {
            Vec::new()
        }
    };
    for proc_attr in proc_attrs.iter() {
        let proc_name = proc_attr.as_str().unwrap();
        let process_manif_function_ident = format_ident!("{}_manifest", proc_name);
        let token_stream: TokenStream = quote!{
            manif = manif.add_process(#process_manif_function_ident());
        }.into();
        item_fn.block.stmts.push(parse_macro_input!(token_stream as Stmt));

    }

    let cont_attrs: Map<String, Value> = match manifest_attr.get("containers") {
        Some(v) => {
            match v.as_object() {
                Some(arr) => {
                    arr.clone()
                }
                None => {
                    panic!("containersアトリビュートは辞書でなければなりません。")
                },
            }
        }
        None => {
            Map::new()
        }
    };
    for (cont_name, cont_proc_list) in cont_attrs.iter() {
        //let cont_name = cont_name.as_str().unwrap();
        let cont_manif_function_ident = format_ident!("{}_manifest", cont_name);
        let token_stream: TokenStream = quote!{
            manif = manif.add_container(#cont_manif_function_ident());
        }.into();
        item_fn.block.stmts.push(parse_macro_input!(token_stream as Stmt));


        let cont_proc_attrs: Vec<Value> = match cont_proc_list.as_array() {
            Some(arr) => {
                arr.clone()
            }
            None => {
                panic!("containersアトリビュートは辞書でなければなりません。")
            },
        };
        for proc_attr in cont_proc_attrs.iter() {
            let proc_name = proc_attr.as_str().unwrap();
            let process_manif_function_ident = format_ident!("{}_manifest", proc_name);
            let token_stream: TokenStream = quote!{
                manif = manif.add_process(#process_manif_function_ident());
            }.into();
            item_fn.block.stmts.push(parse_macro_input!(token_stream as Stmt));
        }
    }

    let return_token_stream: TokenStream = quote!{
        return manif;
    }.into();
    item_fn.block.stmts.push(parse_macro_input!(return_token_stream as Stmt));

    // println!("fn: {}", item_fn.to_token_stream().to_string());

    // 最後の最後に全部の関数を並べる。
    quote! { 
        #item_fn
    }.into()
}