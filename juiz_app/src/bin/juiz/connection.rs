//
// juiz connect http://localhost:8001/rust_talker0 http://localhost:8000/rust_listener0/arg1 --type push 


use std::path::Path;
use juiz_core::utils::yaml_conf_load;
use juiz_core::log;

use juiz_core::prelude::*;

// #[cfg(feature="opencv4")]
// use juiz_core::opencv::{imgcodecs::imwrite, core::{Mat, Vector}};
use clap::Subcommand;

use crate::Args;

#[derive(Debug, Subcommand, Clone)]
pub(crate) enum ConnectionSubCommands {

    #[clap(arg_required_else_help = false)]
    Create {
        #[arg(long = "type", short = 't', help = "Type of connection. Default is PUSH", default_value="push")]
        connection_type: String,


        #[arg(help = "ID of source process")]
        source: String,

        #[arg(help = "Inlet name of destination process")]
        arg_name: String,

        #[arg(help = "ID of destination process")]
        destination: String
    },

    #[clap(arg_required_else_help = false)]
    Delete {
        
        #[arg(help = "ID of Conection")]
        connection_id: String
    },

    /// get logs
    #[clap(arg_required_else_help = false)]
    List {
    }
}

pub(crate) fn on_connection(manifest: Value, working_dir: &Path, subcommand: ConnectionSubCommands, args: Args) -> JuizResult<()> {
    match on_connection_inner(manifest, working_dir, subcommand, args) {
        Ok(_) => return Ok(()),
        Err(e) => println!("Error: {e:?}")
    };
    Ok(())
}

pub(crate) fn on_connection_inner(manifest: Value, working_dir: &Path, subcommand: ConnectionSubCommands, args: Args) -> JuizResult<()> {
    let server = args.server.clone();
    let recursive = args.recursive;
    match subcommand {
        ConnectionSubCommands::Create { connection_type, source, arg_name, destination} => {
            log::trace!("connection connect command is selected. args={args:?}");
            //let manifest2 = yaml_conf_load(filepath.clone())?;
            System::new(manifest)?
                .set_working_dir(working_dir)
                .start_http_broker(args.start_http_broker)
                .setup()?
                .add_systemproxy_by_id(Some(server.clone()))?
                .run_and_do_once( |system| { 
                    on_connection_create(system, source, arg_name, destination, connection_type)
                // if any_process {
                //     //on_any_process_list(system, Some(server), recursive)
                // } else {
                //     //on_process_list(system, Some(server), recursive)
                // } 
                    //Ok(())
                }
            ) 
        },
        ConnectionSubCommands::Delete { connection_id} => {
            log::trace!("connection connect command is selected. args={args:?}");
            //let manifest2 = yaml_conf_load(filepath.clone())?;
            System::new(manifest)?
                .set_working_dir(working_dir)
                .start_http_broker(args.start_http_broker)
                .setup()?
                .add_systemproxy_by_id(Some(server.clone()))?
                .run_and_do_once( |system| { 
                // if any_process {
                //     //on_any_process_list(system, Some(server), recursive)
                // } else {
                //     //on_process_list(system, Some(server), recursive)
                // } 
                    Ok(())
                }
            ) 
        },
        ConnectionSubCommands::List {} => {
            log::trace!("connection list command is selected. args={args:?}");
            // let manifest2 = yaml_conf_load(filepath.clone())?;
            System::new(manifest)?
                .set_working_dir(working_dir)
                .start_http_broker(args.start_http_broker)
                .setup()?
                .add_systemproxy_by_id(Some(server.clone()))?
                .run_and_do_once( |system| { 
                    
                // if any_process {
                //     //on_any_process_list(system, Some(server), recursive)
                // } else {
                //     //on_process_list(system, Some(server), recursive)
                // } 
                    Ok(())
                }
            ) 
        }
    }
}


fn on_connection_create(system: &mut System, source_id: String, arg_name: String, destination_id: String, connection_type: String) -> JuizResult<()> {
    log::info!("Connecting {source_id} -({arg_name}:{connection_type:})-> {destination_id}");
    let manifest = ConnectionManifest::new(
        connection_type.as_str().try_into()?,
        source_id,
        arg_name,
        destination_id,
        None,
    );
    system.core_broker().lock_mut()?.connection_create(manifest.into())?;
    Ok(())
}