
use std::path::Path;

use juiz_core::{JuizResult, System, Value};


use clap::Subcommand;


#[derive(Debug, Subcommand)]
pub(crate) enum ContProcSubCommands {
    /// get logs
    #[clap(arg_required_else_help = false)]
    List {
        #[arg(short = 's', default_value = "localhost:8080", help = "Host of server (ex., localhost:8080)")]
        server: String,
        
    }
}

pub(crate) fn on_container_process(manifest: Value, working_dir: &Path, subcommand: ContProcSubCommands) -> JuizResult<()> {
    match subcommand {
        ContProcSubCommands::List { server } => {
            System::new(manifest)?
            .set_working_dir(working_dir)
            .run_and_do_once( |system| { on_container_process_list(system, server) }) 
        }
    }
}

fn on_container_process_list(system: &mut System, _server: String) -> JuizResult<()> {
    //println!("processes:");
    let proc_manifests: Vec<Value> = system.container_process_list()?;
    //println!("proc_manifests: {proc_manifests:?}");
    let mut ids: Vec<String> = Vec::new();
    for v in proc_manifests.iter() {
        //ids.push(obj_get_str(v, "identifier")?.to_owned());
        ids.push(v.as_str().unwrap().to_owned());
    }
    //println!("process list");
    println!("{ids:?}");
    Ok(())
}