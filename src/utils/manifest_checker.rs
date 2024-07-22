
use anyhow::Context;

use crate::value::CapsuleMap;
use crate::JuizError;
use crate::JuizResult;
use crate::value::*;

use super::get_hashmap;
use super::get_value;
use super::manifest_util::get_hashmap_mut;

pub fn check_process_factory_manifest(manifest: Value) -> JuizResult<Value> {
    let _ = obj_get_str(&manifest, "type_name").context("check_process_factory_manifest failed.")?;
    Ok(manifest)
}


pub fn check_corebroker_manifest(manifest: Value) -> Result<Value, JuizError> {
    return Ok(manifest);
} 

fn default_value_type_str(def_value: &Value) -> Result<&str, JuizError> {
    if def_value.is_i64() || def_value.is_u64(){
        return Ok("int");
    } else if def_value.is_f64() {
        return Ok("float");
    } else if def_value.is_string() {
        return Ok("string");
    } else if def_value.is_object() {
        return Ok("object");
    } else if def_value.is_array() {
        return Ok("array");
    }
    return Err(JuizError::ManifestArgumentDefaultValueIsNotAvailableValueTypeError{value: def_value.clone()});
}


fn check_process_manifest_argument(arg_manifest: &mut Value) -> JuizResult<()> {
    
    
    let def_value =get_value(arg_manifest, "default")?.clone();
    // デフォルトの型を表す文字列を取得
    let def_value_type = default_value_type_str(&def_value)?;

    //let arg_map = get_hashmap_mut(arg_manifest)?;
    match  obj_get_str(arg_manifest, "type") {
        Err(_) => {},
        Ok(type_str) => {
            if type_str != def_value_type && !(type_str == "image" && def_value_type == "object" ){
                return Err(anyhow::Error::from(JuizError::ManifestDefaultValueIsNotExpectedTypeError{value: arg_manifest.clone(), key: "type".to_string(), set_type: type_str.to_string(), expected_type: def_value_type.to_string()}));
            }
        }
    }
    let arg_map = get_hashmap_mut(arg_manifest)?;
    arg_map.insert("type".to_string(), jvalue!(def_value_type));
    match arg_map.get("description") {
        Some(desc_v) => {
            if !desc_v.is_string() {
                return Err(anyhow::Error::from(JuizError::ManifestValueIsNotExpectedTypeError{value: arg_manifest.clone(), key: "description".to_string(), expected_type: "str".to_string()}));
            }
        },
        None => {}
    }
    Ok(())
        
}

fn check_process_manifest_arguments(args_manifest: &mut Value) -> JuizResult<()> {
    let args_map = get_hashmap_mut(args_manifest)?;
    for (_arg_name, arg_manif) in args_map.iter_mut() {
        let _ = check_process_manifest_argument(arg_manif).with_context(||format!("check_process_manifest_arguments(name={}, {:?}) function", _arg_name, arg_manif))?;
    }
    Ok(())
}


pub fn check_connection_manifest(connection_manifest: Value) -> Result<Value, JuizError> {
    Ok(connection_manifest)
}

pub fn check_process_manifest(mut process_manifest: Value) -> JuizResult<Value> {
    let _ = obj_get_str(&process_manifest, "name").context("check_process_manifest failed.")?;
    let _ = obj_get_str(&process_manifest, "type_name").context("check_process_manifest failed.")?;
    let arg_v  = obj_get_mut(&mut process_manifest, "arguments").context("check_process_manifest failed.")?;
    check_process_manifest_arguments(arg_v).with_context(||format!("check_process_manifest({})", process_manifest))?;
    Ok(process_manifest)
}

fn check_arguments(args_manifest: &Value, argument: &CapsuleMap) -> JuizResult<()> {
    //let arg_map = get_hashmap(argument).context("check_arguments")?;
    for (arg_name, _v) in get_hashmap(args_manifest).context("check_arguments")? {
        match argument.get(arg_name) {
            Err(_) => {
                log::error!("In Process Manifest there is argument named '{arg_name}', but can not be found in argument:CapsuleMap ({argument:?}).");
                
                return Err(
                anyhow::Error::from(JuizError::ArgumentMissingWhenCallingError{
                    process_manifest: args_manifest.clone(), 
                    missing_arg_name: arg_name.clone()}));
                },
            Ok(_) => {}
            };
    }
    Ok(())
}

pub fn check_manifest_before_call(manifest: &Value, argument: &CapsuleMap) -> JuizResult<()> {
    check_arguments(obj_get(manifest, "arguments")?, argument)
}
