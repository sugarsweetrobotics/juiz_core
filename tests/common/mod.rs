
use juiz_core::{jvalue, processes::process_impl::ProcessImpl, Capsule, CapsuleMap, JuizError, JuizResult};




#[allow(dead_code)]
pub fn increment_function(v: CapsuleMap) -> JuizResult<Capsule> {
    //let i = arg(&v, "arg1")?.as_i64().unwrap();
    let iv = v.get("arg1").ok_or_else(|| { anyhow::Error::from(JuizError::ArgumentCanNotFoundByNameError{ name: "arg1".to_owned() })})?;
    let i = iv.as_value().ok_or_else(|| {
        anyhow::Error::from(JuizError::ArguemntTypeIsInvalidError{})
    })?.as_i64().ok_or_else(|| {
        anyhow::Error::from(JuizError::ArguemntTypeIsInvalidError{})
    })?;
    return Ok(jvalue!(i+1).into());
}

#[allow(dead_code)]
pub fn new_increment_process<'a> (name: &str) -> ProcessImpl {
    let manifest = serde_json::json!({
        "name": name,
        "type_name": "increment",
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, increment_function);
    assert!(p.is_ok(), "ProcessImpl::new() failed. Error is '{:?}'", p.err());
    p.unwrap()
}

static mut COUNTER: i64 = 0;

#[allow(dead_code)]
pub fn execution_function(_v: CapsuleMap) -> JuizResult<Capsule> {
    #[allow(unused)]
    let mut val: i64 = 0;
    unsafe {
        COUNTER = COUNTER + 1;
        val = COUNTER;
    }
    return Ok(jvalue!(val).into());
}

#[allow(dead_code)]
pub fn new_execution_process<'a> (name: &str) -> ProcessImpl {
    let manifest = serde_json::json!({
        "name": "test_function",
        "type_name": name,
        "arguments" : {
            "arg1": {
                "description": "test_argument",
                "default": 1,
            }, 
        }, 
    });
    let p = ProcessImpl::new(manifest, execution_function);
    assert!(p.is_ok(), "ProcessImpl::new() failed. Error is {:?}", p.err());
    p.unwrap()
}
