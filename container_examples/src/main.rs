use juiz_core::{System, jvalue, JuizResult, Broker};

pub fn main() -> JuizResult<()> {

    let manifest = jvalue!(
        {

            "name": "test_system",
            "plugins": {
                "container_factories": {
                    "example_container": {
                        "path": "./target/debug",
                        "processes": {
                            "example_container_increment": {
                                "path": "./target/debug",
                            },
                            "example_container_get": {
                                "path": "./target/debug",
                            }
                        }
                    }
                }
            },
            "containers": [
                {
                    "type_name": "example_container",
                    "name": "container_a",
                    "processes": [
                        {
                            "type_name": "example_container_increment",
                            "name": "increment_a",
                        },
                        {
                            "type_name": "example_container_get",
                            "name": "get_a"
                        }
                    ]
                }
            ],
        }
    );

    System::new(manifest)?.run_and_do(|system|{
        println!("JuizSystem started!!");
        let v = system.core_broker().lock().unwrap().profile_full()?;
        println!("System: {:#}", v);
        //let c = system.container_from_id(&"container_a".to_string())?;
        //let retval = proc.try_lock().expect("Lock failed").invoke()?;
        //println!("retval = {}", c.try_lock().unwrap().to_string());
        Ok(())
    })
}