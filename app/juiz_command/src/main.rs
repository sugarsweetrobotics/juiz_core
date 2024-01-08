use juiz_core::{System, jvalue, JuizResult};
use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Juiz Command")]
enum Juiz {
    Process(JuizPorcess),
}



#[derive(StructOpt)]
#[structopt(about = "Juiz Command")]
enum JuizProcess {
    List {
        #[structopt(short, long)]
        debug: bool,
    }
}

fn main() -> JuizResult<()>{

    let manifest = jvalue!({

        "name": "test_system",
        "plugins": {
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
        ]
    });


    fn juiz_command_process(system: &mut System, args: &[String]) -> JuizResult<()> {
        fn show_process_usage() -> JuizResult<()> {
            Ok(())
        }

        fn show_process_list(system: &mut System, args: &[String]) -> JuizResult<()> {
            println!("show_process_list({:?})", args);
            Ok(())
        }

        match args.get(1) {
            Some(subcmd) => {
                match subcmd.as_str() {
                    "list" => show_process_list(system, args), 
                    _ => show_process_usage()
                }
            }
            _ => show_process_usage()
        }
    }

    Ok(System::new(manifest)?.run_and_do_once(|system|{
        fn show_usage() -> JuizResult<()> {
            Ok(())
        }

        let args: Vec<String> = env::args().collect();
        match args.get(1) {
            Some(subcmd) => {
                match subcmd.as_str() {
                    "process" => {
                        juiz_command_process(system, &args[1..])
                    },
                    _ => show_usage()
                }
            },
            _ => show_usage()
        }
    }).expect("Error in System::run_and_do()"))
}