

extern crate proc_macro;
use std::collections::HashMap;

use proc_macro::{Literal, Punct};
use proc_macro::token_stream::IntoIter;
use quote::{format_ident, quote, ToTokens};
use serde_json::{json, Map, Value};
use crate::proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, ItemFn, Stmt, Type, TypePath};
use yaml_rust2::parser::*;
use anyhow::Result;

fn toplevel_key_value(attr: &mut IntoIter, value: &mut Map<String, Value>, first_value:bool) -> anyhow::Result<bool> {
    match attr.next() {
        Some(k) => {
            match k {
                proc_macro::TokenTree::Ident(kident) => {
                    match attr.next() {
                        Some(p) => {
                            match p {
                                proc_macro::TokenTree::Punct(punct) => {
                                    match attr.next() {
                                        Some(v) => {
                                            match v {
                                                proc_macro::TokenTree::Group(group) => {
                                                    let key = kident.to_string();
                                                    let mut it = group.stream().into_iter();
                                                    let mut tmp_val: Map<String, Value> = Map::new();
                                                    toplevel_key_value(&mut it, &mut tmp_val, true)?;
                                                    while toplevel_key_value(&mut it, &mut tmp_val, false)? {}
                                                    value.insert(key, tmp_val.into());
                                                },
                                                proc_macro::TokenTree::Ident(ident) => {
                                                    let key = kident.to_string();
                                                    let val = ident.to_string();
                                                },
                                                proc_macro::TokenTree::Literal(literal) => {
                                                    let key = kident.to_string();
                                                    //slet k = literal.0.kind;
                                                    let val = match litrs::Literal::parse(literal.to_string().as_str())? {
                                                        litrs::Literal::Bool(bool_lit) => {
                                                            json!(bool_lit.value())
                                                        }
                                                        litrs::Literal::Integer(integer_lit) => {
                                                            json!(integer_lit.raw_input().parse::<i64>().unwrap())
                                                        }
                                                        litrs::Literal::Float(float_lit) => {
                                                            json!(float_lit.raw_input().parse::<f64>().unwrap())
                                                        }
                                                        litrs::Literal::Char(char_lit) => {
                                                            json!(char_lit.value())
                                                        }
                                                        litrs::Literal::String(string_lit) => {
                                                            json!(string_lit.value())
                                                        }
                                                        litrs::Literal::Byte(byte_lit) => {
                                                            json!(byte_lit.value())
                                                        }
                                                        litrs::Literal::ByteString(byte_string_lit) =>{
                                                            json!(byte_string_lit.value())
                                                        }
                                                    };
                                                    //let val = literal.to_string();
                                                    value.insert(key, val.into());
                                                },
                                                _ => {
                                                    panic!("Expected value after '='")
                                                }
                                            }
                                        }
                                        None => {
                                            panic!("Expected value after '='")
                                        }
                                    }
                                }
                                _ => {
                                    panic!("Expected '=' punct. but {p:?}")
                                }
                            }
                        }
                        None => {
                            panic!("Expected '=' punct. But none.")
                        }
                    }
                }
                proc_macro::TokenTree::Punct(punct) => {
                    if first_value {
                        panic!("Unexpected punct {punct:?}")
                    }
                }
                _v => {
                    panic!("Unexpected token. {_v:?}")
                }
            }
        }
        None => return Ok(false)
    }
    Ok(true)
}

fn parse_attr(attr: TokenStream) -> Value {
    let mut description = "Default Process description".to_owned();
    let mut c =  attr.into_iter();
    let mut va = json!({});
    let m = va.as_object_mut().unwrap();
    let _ = toplevel_key_value(&mut c, m, true);
    while toplevel_key_value(&mut c, m, false).unwrap() {}
    println!("value is {va:?}");
    va
}

#[proc_macro_attribute]
pub fn juiz_process(attr: TokenStream, item: TokenStream) -> TokenStream {
    let manifest_attr = parse_attr(attr);
    let mut ast = parse_macro_input!(item as ItemFn);
    let function_name = ast.sig.ident.clone().to_string();
    // ここで引数の名前と型名のリストを受け取っておく
    let mut arg_map: HashMap<TypePath, String> = HashMap::new();
    for input in ast.sig.inputs.iter() {
        if let syn::FnArg::Typed(pat_type) = input {
            //println!("Typed: {:?}", pat_type);
            let arg_name = if let syn::Pat::Ident(arg_name) = pat_type.pat.as_ref() {
                arg_name.ident.to_string()
            } else {
                panic!("引数の名前宣言が不正");
            };
            let _type_name = if let Type::Path(type_path) = pat_type.ty.as_ref() {
                arg_map.insert(type_path.clone(), arg_name);
                type_path.path.segments.iter().map(|seg| {seg.ident.to_string()}).collect::<Vec<String>>().join("::")
            } else {
                panic!("引数の型宣言が不正");
            };
        }
    }

    // ここで全部 CapsuleMapにしてしまう。唯一の引数は __the_onlyなんちゃらかんちゃら
    ast.sig.inputs.clear();
    let ass: ItemFn = parse_quote!( fn __unused_function_name__( __the_only_argument_modified_by_juiz_process_macro__: juiz_sdk::prelude::CapsuleMap) {} );
    let u = ass.sig.inputs;
    ast.sig.inputs.push(u.first().unwrap().clone());

    // このあとclearしちゃうのでまずbodyを保存。
    let mut body = quote! {};
    for s in &ast.block.stmts {
        body = quote! {
            #body
            #s
        };
    }
    let body: TokenStream = quote! {
        let body = || { #body };
    }.into();

    ast.block.stmts.clear();

    // 最初に取得した引数のリストから、CapsuleMapから引数それぞれにtry_intoするコードを生成する。
    for (k, v) in arg_map.iter() {
        let type_name = k;// format_ident!("{}", k);
        let arg_name = format_ident!("{}", v);
        let set_original_argument_statement: TokenStream = quote!(
            let #arg_name : #type_name = __the_only_argument_modified_by_juiz_process_macro__.get(stringify!(#arg_name))?.try_into()?;
        ).into();
        ast.block.stmts.push(parse_macro_input!(set_original_argument_statement as Stmt));
    }
    
    // bodyを返す
    ast.block.stmts.push(parse_macro_input!(body as Stmt));
    // 値を返す
    let exit: TokenStream = quote! { { body() } }.into();
    ast.block.stmts.push(parse_macro_input!(exit as Stmt));


    // ここからmanifest関数を自動生成する。
    // まず土台となる関数定義
    let manifest2_tokenstream: TokenStream = quote!{
        fn manifest2() -> juiz_sdk::prelude::ProcessManifest { 
        }
    }.into();
    let mut manifest2_item_fn : ItemFn = syn::parse_macro_input!(manifest2_tokenstream as ItemFn);
    // attr変数から読み取った値からdescriptionを取得
    let description = manifest_attr.as_object().unwrap().get("description").and_then(|v| { Some(v.clone()) }).or(Some(json!(format!("Default description of Process({function_name})")))).unwrap().as_str().unwrap().to_owned();
    // ここでmanifestデータの基本データを作成する部分
    let mut construct_manif = quote!{
        let mut manif = ProcessManifest::new( #function_name ).description(#description);
    };

    // ここから収集したattrデータおよび引数データを使ってmanifestを自動生成する。
    let empty_value = json!({}); // 空っぽのMapは使い回す。
    // attrから受け取ったmanifest情報を使いやすいMapに変更してからforに飛び込む！
    let argument_value = manifest_attr.as_object().unwrap().get("arguments").or( Some(&empty_value)).unwrap().as_object().unwrap();
    let argument_default_value = argument_value.get("default").or( Some(&empty_value)).unwrap().as_object().unwrap();
    let argument_description_value = argument_value.get("description").or( Some(&empty_value)).unwrap().as_object().unwrap();
    // itemから受け取った引数情報をイテレートしつつ、attrからもらった部分にデフォルト引数およびデスクリプションの情報があれば、それを追加していく
    for (type_path, arg_name) in arg_map.into_iter() {
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
            "Value" => {
                let default_value_default = json!({});
                let default_value = argument_default_value.get(arg_name.as_str()).or( Some(&default_value_default)).unwrap();
                let value_str = serde_json::to_string(default_value).unwrap();
                construct_manif = quote!{
                    #construct_manif
                    manif = manif.add_object_arg(#arg_name, #description, serde_json::from_str(#value_str).unwrap());
                }
            },
            _ => {
                panic!("自動マニフェスト生成に失敗。対応するデータ型ではありません。 ({type_name:})")
            }
        }
    }
    // 最後にtokenを組み立てる。
    let return_manifest: TokenStream = quote!{
        {
            #construct_manif
            manif
        }
    }.into();
    manifest2_item_fn.block.stmts.push(parse_macro_input!(return_manifest as Stmt));

    // factoryを自動生成する
    // まず土台となる関数定義
    let fstr = ast.sig.ident.clone();
    println!("fstr: {fstr}");
    let factory_tokenstream: TokenStream = quote!{
        #[no_mangle]
        pub unsafe extern "Rust" fn process_factory() -> JuizResult<ProcessFactoryStruct> {
            env_logger::init();
            Ok(juiz_sdk::process_factory(manifest2(), #fstr))
        }
    }.into();
    let mut factory_item_fn : ItemFn = syn::parse_macro_input!(factory_tokenstream as ItemFn);
    
    // 最後の最後に全部の関数を並べる。
    quote! { 
        #factory_item_fn 
        #ast
        #manifest2_item_fn
        //#factory_item_fn
    }.into()
}
