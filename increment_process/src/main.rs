
use juiz_core::{System, jvalue, JuizResult};

#[tokio::main]
async fn main() -> JuizResult<()> {

    let manifest = jvalue!(
        {

            "name": "test_system",
            "plugins": {
                "process_factories": {
                    "increment_process": {
                        "path": "./target/debug"
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
                        "name": "increment_b",
                        //"id": "core://core/Process/increment_b:increment_process"
                    }, 
                    "destination" : {
                        "type_name": "increment_process",
                        "name": "increment_a",
                        "id": "core://core/Process/increment_a:increment_process"
                    }
                }
            ]
        }
    );

    System::new(manifest)?.run_and_do(|_system|{
//        let proc = system.process_from_id(&"core://increment_a:increment_process".to_string())?;
//        let retval = proc.try_lock().expect("Lock failed").invoke()?;
//        println!("retval = {:?}", retval);
//        println!("System: {:#}", system.profile_full()?);
        Ok(())
    })
}