
use std::path::Path;

use juiz_core::prelude::*;
use juiz_core::yaml_conf_load;
use juiz_core::log;


use clap::Subcommand;

use crate::Args;


#[derive(Debug, Subcommand, Clone)]
pub(crate) enum ContProcSubCommands {
    /// get logs
    #[clap(arg_required_else_help = false)]
    List {
        #[arg(short = 'f', default_value = "./juiz.conf", help = "Input system definition file path")]
        filepath: String,
    }
}

pub(crate) fn on_container_process(_manifest: Value, working_dir: &Path, subcommand: ContProcSubCommands, args: Args) -> JuizResult<()> {
    match subcommand {
        ContProcSubCommands::List { filepath} => {
            log::trace!("container-process list command is selected.");
            let manifest2 = yaml_conf_load(filepath.clone())?;
            let server = args.server;
            let recursive = args.recursive;
            System::new(manifest2)?
            .set_working_dir(working_dir)
            .start_http_broker(args.start_http_broker)
            .setup()?
            .run_and_do_once( |system| { on_container_process_list(system, server, recursive) }) 
        }
    }
}

fn on_container_process_list(system: &mut System, _server: Option<String>, recursive: bool) -> JuizResult<()> {
    log::trace!("on_container_process_list() called");
    let proc_manifests: Vec<Value> = system.container_process_list(recursive)?;
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