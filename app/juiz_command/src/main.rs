
mod process;
mod setup;
mod default_juiz_conf;
mod container;
mod container_process;

use container::{on_container, ContSubCommands};
use container_process::{on_container_process, ContProcSubCommands};
use juiz_core::{ yaml_conf_load, JuizResult, System};
use crate::process::{on_process, ProcSubCommands};
use crate::setup::{on_setup, SetupSubCommands};

use clap::{Parser, Subcommand};


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(
    name = "juiz",
    author = "Yuki Suga",
    version = "v1.0.0",
    about = "JUIZ command-line tool"
)]
struct Args {
    /// Name of the person to greet
    #[arg(short = 'd', help = "Daemonize JUIZ server")]
    daemonize: bool,

    #[arg(short = 'f', default_value = "./juiz.conf", help = "Input system definition file path")]
    filepath: String,

    #[clap(subcommand)]
    subcommand: Option<SubCommands>,
}


#[derive(Debug, Subcommand)]
enum SubCommands {
    // Setup tools
    #[clap(arg_required_else_help = false)]
    Setup {
        #[clap(subcommand)]
        subcommand: SetupSubCommands
    },

    // Process tools
    #[clap(arg_required_else_help = false)]
    Process {
        #[clap(subcommand)]
        subcommand: ProcSubCommands
    },

    // Container tools
    #[clap(arg_required_else_help = false)]
    Container {
        #[clap(subcommand)]
        subcommand: ContSubCommands
    },

    // Container tools
    #[clap(arg_required_else_help = false)]
    ContainerProcess {
        #[clap(subcommand)]
        subcommand: ContProcSubCommands
    },
}



fn do_task_once(_system: &mut System) -> JuizResult<()> {
    println!("System started once");
    Ok(())
}

fn do_task(_system: &mut System) -> JuizResult<()> {
    println!("System started");
    Ok(())
}

fn main() -> () {
    match do_once() {
        Ok(_) => (),
        Err(e) => println!("Error:{:?}", e)
    }
}

fn do_once() -> JuizResult<()>{
    let args = Args::parse();
    let manifest = yaml_conf_load(args.filepath)?;

    if args.subcommand.is_none() {
        if args.daemonize {
            return System::new(manifest)?.run_and_do(do_task);
        } else {
            return System::new(manifest)?.run_and_do_once(do_task_once);
        }
    }
    match args.subcommand.unwrap() {
        SubCommands::Setup{ subcommand } => { 
            on_setup(manifest, subcommand)
        },
        SubCommands::Process { subcommand } => {
            on_process(manifest, subcommand)
        },
        SubCommands::Container { subcommand } => {
            on_container(manifest, subcommand)
        },
        SubCommands::ContainerProcess { subcommand } => {
            on_container_process(manifest, subcommand)
        },
        /* _ => {
            return Ok(())
        } */
    }
}