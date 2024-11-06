
use proc_macro::token_stream::IntoIter;
use serde_json::{json, Map, Value};
use crate::proc_macro::TokenStream;

pub(crate) fn toplevel_key_value(attr: &mut IntoIter, value: &mut Map<String, Value>, first_value:bool) -> anyhow::Result<bool> {
    match attr.next() {
        Some(k) => {
            match k {
                proc_macro::TokenTree::Ident(kident) => {
                    match attr.next() {
                        Some(p) => {
                            match p {
                                proc_macro::TokenTree::Punct(_punct) => {
                                    match attr.next() {
                                        Some(v) => {
                                            match v {
                                                proc_macro::TokenTree::Group(group) => {
                                                    match group.delimiter() {
                                                        proc_macro::Delimiter::Brace => {
                                                            let key = kident.to_string();
                                                            let mut it = group.stream().into_iter();
                                                            let mut tmp_val: Map<String, Value> = Map::new();
                                                            toplevel_key_value(&mut it, &mut tmp_val, true)?;
                                                            while toplevel_key_value(&mut it, &mut tmp_val, false)? {}
                                                            value.insert(key, tmp_val.into());
                                                        }
                                                        proc_macro::Delimiter::Bracket => {
                                                            // リストです。
                                                            let key = kident.to_string();
                                                            let _it = group.stream().into_iter();
                                                            let mut val: Vec<Value> = Vec::new();
                                                            for token in  group.stream().into_iter() {
                                                                match token {
                                                                    proc_macro::TokenTree::Ident(ident) => {
                                                                        val.push(json!(ident.to_string()));
                                                                    },
                                                                    proc_macro::TokenTree::Punct(_punct) => {
                                                                        // カンマは無視するよ
                                                                    },
                                                                    _var => {
                                                                        panic!("リストの中身は単一の識別子に限られています ({_var:?})")
                                                                    }
                                                                }
                                                            }
                                                            value.insert(key, val.into());
                                                        },
                                                        _par => {
                                                            panic!("予期しないかっこ ({_par:?})です");
                                                        }
                                                    }
                                                },
                                                proc_macro::TokenTree::Ident(ident) => {
                                                    let _key = kident.to_string();
                                                    let _val = ident.to_string();
                                                    todo!("まだ実装していない箇所です ({ident:})")
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

pub(crate) fn parse_attr(attr: TokenStream) -> Value {
    let _description = "Default Process description".to_owned();
    let mut c =  attr.into_iter();
    let mut va = json!({});
    let m = va.as_object_mut().unwrap();
    let _ = toplevel_key_value(&mut c, m, true);
    while toplevel_key_value(&mut c, m, false).unwrap() {}
    // println!("value is {va:?}");
    va
}
