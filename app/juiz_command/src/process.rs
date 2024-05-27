
use juiz_core::{processes::proc_lock, JuizResult, System, Value};
use opencv::imgcodecs::imwrite;
use opencv::core::Vector;
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

    /// get logs
    #[clap(arg_required_else_help = false)]
    Call {
        #[arg(help = "ID of process")]
        identifier: String,


        #[arg(help = "Argument")]
        argument: String,

        #[arg(short = 'o', help = "Output Filename")]
        fileout: Option<String>,
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
        },
        ProcSubCommands::Call { identifier, argument , fileout} => {
            System::new(manifest)?.run_and_do_once( |system| { 
                on_process_call(system, identifier, argument, fileout)
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

fn on_process_call(system: &mut System, id: String, arg: String, fileout: Option<String>) -> JuizResult<()> {
    //println!("processes:");
    let p = system.any_process_from_id(&id);
    match p {
        Ok(ps) => {
            let argv = juiz_core::load_str(arg.as_str())?;
            // println!("Value is {argv:?}");
            let value = proc_lock(&ps)?.call(argv.try_into()?)?;
            if value.is_value()? {
                println!("{:?}", value);
            } else if value.is_mat()? {
                let _ = value.lock_as_mat(|mat| {

                    let params: Vector<i32> = Vector::new();
                    match fileout {
                        Some(filepath) => {
                            match imwrite(filepath.as_str(), mat, &params) {
                                Ok(_) => {
                                    //println!("ok");
                                },
                                Err(e) => {
                                    println!("error: {e:?}");
                                }
                            }
                        },
                        None => {
                            println!("{:?}", mat);
                        }
                    }
                } );
            }
        },
        Err(e) => println!("Error: {e:?}"),
    }
    Ok(())
}