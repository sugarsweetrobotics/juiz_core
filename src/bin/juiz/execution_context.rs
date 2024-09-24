

use std::path::Path;
use juiz_core::prelude::juiz_lock;
use juiz_core::yaml_conf_load;
use juiz_core::log;

use juiz_core::prelude::*;
use clap::Subcommand;

use crate::Args;

#[derive(Debug, Subcommand, Clone)]
pub(crate) enum EcSubCommands {
    /// get logs
    #[clap(arg_required_else_help = false)]
    List {
        #[arg(short = 'f', default_value = "./juiz.conf", help = "Input system definition file path")]
        filepath: String,
    },

    #[clap(arg_required_else_help = false)]
    Start {
        #[arg(help = "ID of Execution context")]
        identifier: String,

        #[arg(short = 'f', default_value = "./juiz.conf", help = "Input system definition file path")]
        filepath: String,
    },
}



pub(crate) fn on_execution_context(manifest: Value, working_dir: &Path, subcommand: EcSubCommands, args: Args) -> JuizResult<()> {
    match on_ec_inner(manifest, working_dir, subcommand, args) {
         Ok(_) => return Ok(()),
         Err(e) => println!("Error: {e:?}")
     };
    Ok(())
}

pub(crate) fn on_ec_inner(_manifest: Value, working_dir: &Path, subcommand: EcSubCommands, args: Args) -> JuizResult<()> {
    match subcommand {
        EcSubCommands::List {filepath} => {
            log::trace!("ec list command is selected.");
            let manifest2 = yaml_conf_load(filepath.clone())?;
            let server = args.server;
            System::new(manifest2)?
                .set_working_dir(working_dir)
                .start_http_broker(args.start_http_broker)
                .setup()?
                .run_and_do_once( |system| { 
                
                    on_ec_list(system, server)
                
            }) 
        },

        EcSubCommands::Start { identifier, filepath} => {
            log::trace!("ec start command is selected.");
            let manifest2 = yaml_conf_load(filepath.clone())?;
            let server = args.server;
            System::new(manifest2)?
                .set_working_dir(working_dir)
                .start_http_broker(args.start_http_broker)
                .setup()?
                .run_and_do_once( |system| { 
                
                    on_ec_start(system, server, identifier)
                
            }) 
        },
    }
}


fn on_ec_list(system: &mut System, _server: String) -> JuizResult<()> {
    log::info!("on_ec_list() called");
    let ec_manifests: Vec<Value> = system.ec_list()?;
    let mut ids: Vec<String> = Vec::new();
    for v in ec_manifests.iter() {
        ids.push(v.as_str().unwrap().to_owned());
    }
    println!("{ids:?}");
    Ok(())
}

fn on_ec_start(system: &mut System, _server: String, id: String) -> JuizResult<()> {
    //println!("processes:");
    let e = system.ec_from_id(&id);
    match e {
        Ok(ec) => {
            juiz_lock(&ec)?.start()?;
        },
        Err(e) => println!("Error: {e:?}"),
    }
    Ok(())
}