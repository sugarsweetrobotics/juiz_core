use crate::error::*;
use crate::process;
use crate::value::*;

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
    return Err(JuizError::ManifestArgumentDefaultValueIsInvalidTypeError{});
}


fn check_process_manifest_argument(arg_manifest: &mut Value) -> Result<(), JuizError> {
    match arg_manifest.as_object_mut() {
        None => return Err(JuizError::ManifestArgumentsInvalidError{}),
        Some(arg_map) => {
            // デフォルトの値をチェック
            let def_result = arg_map.get("default");
            if def_result.is_none() {
                return Err(JuizError::ManifestArgumentDefaultValueMissingError{});
            }
            let def_value = def_result.unwrap().clone();
            let def_value_type_result = default_value_type_str(&def_value);
            if def_value_type_result.is_err() {
                return Err(def_value_type_result.unwrap_err());
            }
            // デフォルトの型を表す文字列を取得
            let def_value_type = def_value_type_result.unwrap();

            // 引数の型宣言チェック
            let type_result = arg_map.get_mut("type");
            if type_result.is_some() {
                // 型宣言がある場合はデフォルトとタイプが合っているかを確認
                let type_v = type_result.unwrap();
                match type_v.as_str() {
                    None => return Err(JuizError::ArgumentTypeIsNotStringError{}), 
                    Some(type_str) => {
                        if type_str != def_value_type {
                            return Err(JuizError::ArgumentTypeIsNotMatchWithDefaultError{});
                        }
                    }
                }
            } else {
                arg_map.insert("type".to_string(), jvalue!(def_value_type));
            }



            let desc_result = arg_map.get("description");
            if desc_result.is_some() {
                let desc_v = desc_result.unwrap();
                if !desc_v.is_string() {
                    return Err(JuizError::ArgumentDescriptionIsNotStringError{});
                }
            }
            Ok(())
        }
    }
}

fn check_process_manifest_arguments(args_manifest: &mut Value) -> Result<(), JuizError> {
    match args_manifest.as_object_mut() {
        None => return Err(JuizError::ManifestArgumentsInvalidError{}),
        Some(args_map) => {
            for (arg_name, arg_manif) in args_map.iter_mut() {
                match check_process_manifest_argument(arg_manif) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
            }
            Ok(())
        }
    }
}


pub fn check_process_manifest(mut process_manifest: Value) -> Result<Value, JuizError> {
    
    match process_manifest.as_object_mut() {
        None => return Err(JuizError::ProcessManifestError{}),
        Some(hash_map) => {
            match hash_map.get("name") {
                None => return Err(JuizError::ManifestNameMissingError{}),
                Some(_) => { /* Do Nothing */ }
            }

            match hash_map.get_mut("arguments") {
                None => return Err(JuizError::ManifestArgumentsMissingError{}),
                Some(args_value) => {
                    match check_process_manifest_arguments(args_value) {
                        Err(err) => return Err(err),
                        Ok(_) => {

                        }
                    }
                }
            }

            return Ok(process_manifest)
        }
    }
}

fn check_arguments(args_manifest: &Value, argument: &Value) -> Result<(), JuizError> {
    match args_manifest.as_object() {
        None => return Err(JuizError::ArgumentManifestIsInvalidError{}),
        Some(manifest_map) => {
            match argument.as_object() {
                None => return Err(JuizError::ArgumentIsNotObjectError{}),
                Some(arg_map) => {
                    for (manif_key, _) in manifest_map {
                        match arg_map.get(manif_key) {
                            Some(_) => {},
                            None => return Err(JuizError::ArgumentMissingWhenCallingError{})
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn check_manifest_before_call(manifest: &Value, argument: &Value) -> Result<(), JuizError> {
    match manifest.get("arguments") {
        None => return Err(JuizError::ArgumentIsNotObjectError{}),
        Some(args_manifest) => {
            check_arguments(args_manifest, argument)
        }
    }
}