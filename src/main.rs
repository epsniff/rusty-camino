use clap::{App, AppSettings, Arg, SubCommand};
use std::io::Write;

mod commands;
use self::commands::*;


fn main() {
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
                .short("s")
                .long("server")
            )
        ).get_matches();

    let (subcommand, some_options) = cli_options.subcommand();
    let options = some_options.unwrap();
    let run_cli = match subcommand {
        "version" => run_version_cli,
        "server" => run_server,
        _ => panic!("Subcommand {} is unknown", subcommand),
    };

    if let Err(ref e) = run_cli(options) {
        let stderr = &mut std::io::stderr();
        let errmsg = "Error writing ot stderr";
        writeln!(stderr, "{}", e).expect(errmsg);
        std::process::exit(1);
    }
}
