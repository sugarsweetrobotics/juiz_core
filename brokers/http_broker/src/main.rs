use juiz_core::{System, jvalue, JuizResult};

#[tokio::main]
async fn main() -> JuizResult<()>{

    let manifest = jvalue!(
        {

            "name": "test_system",
            "plugins": {
                "broker_factories": {
                    "http_broker": {
                        "path": "./target/debug"
                    }
                }
            },
            
        }
    );

    Ok(System::new(manifest)?.run_and_do(|system|{
        println!("JuizSystem started!!");
        let v = system.broker_proxy(&jvalue!({"type_name":"http", "name": "localhost:3000"}))?.lock().unwrap().system_profile_full()?;
        println!("System: {:#}", v);
        Ok(())
    }).expect("Error in System::run_and_do()"))
}