
use juiz_core::{System, Value, jvalue, JuizResult};


fn load() -> () {
    unsafe {
        match libloading::Library::new("./libincrement_process.dylib") {
            Ok(lib) => {
                match lib.get::<libloading::Symbol<unsafe extern fn() -> i32>>(b"main") {
                    Ok(func) => {
                        func();
                    },
                    Err(_) => {
                        eprintln!("Function get error!");
                    }
                }
            },
            Err(_) => {
                eprintln!("Library link error!");
            }
        }
    }

}
pub fn main() -> JuizResult<()> {

    let manifest = jvalue!(
        {

            "name": "test_system",
            "plugins": {
                "process_factories": {
                    "increment_process": {
                        "path": "./"
                    }
                }
            },
            "processes": [
                {
                    "type_name": "increment_process",
                    "name": "increment_a"
                },
                {
                    "type_name": "increment_process",
                    "name": "increment_b"
                },
            ],
            "connections": [
                {
                    "id": "con01",
                    "arg_name": "arg1",
                    "source" : {
                        "type_name": "increment_process",
                        "name": "increment_b"
                    }, 
                    "destination" : {
                        "type_name": "increment_process",
                        "name": "increment_a"
                    }
                }
            ]
        }
    );

    System::new(manifest)?.run_and_do(|system|{
        let proc = system.process_from_id(&"increment_a".to_string())?;
        let retval = proc.try_lock().expect("Lock failed").invoke()?;
        println!("retval = {:?}", retval);
        Ok(())
    })
}