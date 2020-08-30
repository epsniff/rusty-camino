
#[macro_use]
extern crate derive_more;
extern crate log;

use clap::{App, AppSettings, Arg, SubCommand, ArgMatches};
use std::io::Write;
use log::LevelFilter;
use env_logger::Builder;

mod camserver;
use crate::camserver::run_cam_server;
// mod commands;
// use self::commands::*;

fn main() {

    // Initialize logging, and log the "info" level for this crate only, unless
    // the environment contains `RUST_LOG`.
    let mut builder = Builder::from_default_env();

    builder //.format(|buf, record| writeln!(buf, "{}:{} -- {}", record.level(), record.target(), record.args()))
           .filter(None, LevelFilter::Info)
           .default_format()
           .default_format_timestamp(true)
           .default_format_level(true)
           .default_format_module_path(true)
           .init();

    let cli_options = App::new("rusty-camino")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Eric Sniff <esniff@gmail.com>")
        .about("Rusty Camino's command line interface.")
        .subcommand(            
            SubCommand::with_name("version")
            .about("get the version")
            .arg(
                Arg::with_name("queries")
                .short("v")
                .long("version")
            )
        )
        .subcommand(            
            SubCommand::with_name("server")
            .about("run the server")
            .arg(
                Arg::with_name("port")
                .short("p")
                .long("port")
            )
        ).get_matches();

    let (subcommand, some_options) = cli_options.subcommand();
    let options = some_options.unwrap(); 
    let run_cli = match subcommand {
        "server" => run_server,
        "version" => run_version,
        _ => panic!("Subcommand {} is unknown", subcommand),
    };

    if let Err(ref e) = run_cli(options) {
        let stderr = &mut std::io::stderr();
        let errmsg = "Error writing ot stderr";
        writeln!(stderr, "{}", e).expect(errmsg); 
        std::process::exit(1);
    }
}

pub fn run_server(matches: &ArgMatches) -> Result<(), String> {
    let _ =  match run_cam_server(matches){
     Ok(_) => {
         return Ok(())
     },
     Err(e) => panic!("Server exited with error:{} ", e),
    };
 }


struct Version {
    version :i32,
}
impl Version {
    pub fn new() -> Version {
        Version{version:42}
    }

    pub fn version(&self) -> i32 {
        self.version
    }
}
pub fn run_version(_: &ArgMatches) -> Result<(), String>{
    let ver = Version::new();
    println!("version: {}", ver.version());
    Ok(())
}
 