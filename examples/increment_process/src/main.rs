
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
                },
                "ec_factories": {
                    "timer_ec": {
                        "path": "./target/debug"
                    }
                },
                "broker_factories": {
                    "http_broker": {
                        "path": "./target/debug"
                    }
                }
            },
            "brokers": [
                {
                    "type_name": "http",
                    "name": "localhost:3000"
                }  
            ],
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
            "ecs": [
                {
                    "type_name": "TimerEC",
                    "name": "timer0",
                    "rate": 1.0,
                    "bind": [
                        {
                            "target": {
                                "type_name": "increment_process",
                                "name": "increment_a",
                            }
                        }
                    ]
                }
            ],
            
            "connections": [
                {
                    //"id": "con01",
                    "arg_name": "arg1",
                    "source" : {
                        "type_name": "increment_process",
                        "name": "increment_b",
                        //"id": "core://core/Process/increment_b:increment_process"
                    }, 
                    "destination" : {
                        "type_name": "increment_process",
                        "name": "increment_a",
                        "id": "core://core/Process/increment_a::increment_process"
                    }
                }
            ]
        }
    );

    System::new(manifest)?.run_and_do(|system|{
//        let proc = system.process_from_id(&"core://increment_a:increment_process".to_string())?;
//        let retval = proc.try_lock().expect("Lock failed").invoke()?;
//        println!("retval = {:?}", retval);
//        println!("System: {:#}", system.profile_full()?);
        let v = system.broker_proxy(&jvalue!({"type_name":"local"}))?.lock().unwrap().system_profile_full()?;
        println!("System: {:#}", v);
        let v2 = system.broker_proxy(&jvalue!({"type_name":"http", "name": "localhost:3000"}))?.lock().unwrap().system_profile_full()?;
        println!("System2: {:#}", v2);
        let id = "http://localhost:3000/Process/increment_a::increment_process";
        let p1 = system.process_proxy(&id.to_string())?;
        let prof = p1.lock().unwrap().profile_full()?;
        println!("Process: {:#}", prof);
        Ok(())
    })
}