
use juiz_core::{processes::proc_lock, JuizResult, System, Value};


use clap::Subcommand;


#[derive(Debug, Subcommand)]
pub(crate) enum ProcSubCommands {

    /// get logs
    #[clap(arg_required_else_help = false)]
    List {
        #[arg(short = 's', default_value = "localhost:8080", help = "Host of server (ex., localhost:8080)")]
        server: String,

        #[arg(short = 'a', help = "Any process includes")]
        any_process: bool,
        
    },

    /// get logs
    #[clap(arg_required_else_help = false)]
    Info {
        #[arg(help = "ID of process")]
        identifier: String
    },

}


pub(crate) fn on_process(manifest: Value, subcommand: ProcSubCommands) -> JuizResult<()> {
    match on_process_inner(manifest, subcommand) {
        Ok(_) => return Ok(()),
        Err(e) => println!("Error: {e:?}")
    };
    Ok(())
}
pub(crate) fn on_process_inner(manifest: Value, subcommand: ProcSubCommands) -> JuizResult<()> {
    match subcommand {
        ProcSubCommands::List { server, any_process } => {
            System::new(manifest)?.run_and_do_once( |system| { 
                if any_process {
                    on_any_process_list(system, server)
                } else {
                    on_process_list(system, server)
                } 
            }) 
        },
        ProcSubCommands::Info { identifier } => {
            System::new(manifest)?.run_and_do_once( |system| { 
                on_process_info(system, identifier)
            }) 
        } 
    }
}

fn on_process_list(system: &mut System, _server: String) -> JuizResult<()> {
    let proc_manifests: Vec<Value> = system.process_list()?;
    let mut ids: Vec<String> = Vec::new();
    for v in proc_manifests.iter() {
        ids.push(v.as_str().unwrap().to_owned());
    }
    println!("{ids:?}");
    Ok(())
}

fn on_any_process_list(system: &mut System, _server: String) -> JuizResult<()> {
    let proc_manifests: Vec<Value> = system.any_process_list()?;
    let mut ids: Vec<String> = Vec::new();
    for v in proc_manifests.iter() {
        ids.push(v.as_str().unwrap().to_owned());
    }
    println!("{ids:?}");
    Ok(())
}

fn on_process_info(system: &mut System, id: String) -> JuizResult<()> {
    //println!("processes:");
    let p = system.any_process_from_id(&id);
    match p {
        Ok(ps) => println!("{:}", proc_lock(&ps)?.profile_full()?),
        Err(e) => println!("Error: {e:?}"),
    }
    Ok(())
}