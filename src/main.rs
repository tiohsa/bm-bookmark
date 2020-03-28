#[macro_use]
extern crate clap;
use bookmark::process::Process;
use clap::{App, Arg, ArgMatches, SubCommand};
use failure::Error;
use std::env;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("NAME")
                .help("get bookmark by name.")
                .index(1),
        )
        .subcommand(SubCommand::with_name("list").about("show bookmark list"))
        .subcommand(
            SubCommand::with_name("add")
                .about("add directory to bookmark")
                .arg(Arg::with_name("NAME").help("bookmark name").index(1))
                .arg(Arg::with_name("PATH").help("directory  path").index(2)),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("remove bookmark")
                .arg(Arg::with_name("NAME").help("bookmark name").index(1)),
        )
        .get_matches();

    match run(&matches) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn run(matches: &ArgMatches) -> Result<(), Error> {
    if let Some(name) = matches.value_of("NAME") {
        Process::change_directory(name)?;
    } else if let Some(_) = matches.subcommand_matches("list") {
        Process::show_list()?;
    } else if let Some(sub_matches) = matches.subcommand_matches("add") {
        let name = sub_matches.value_of("NAME").unwrap();
        let path = sub_matches.value_of("PATH").unwrap();
        Process::add_bookmark(name, path)?;
    } else if let Some(sub_matches) = matches.subcommand_matches("remove") {
        let name = sub_matches.value_of("NAME").unwrap();
        Process::remove_bookmark(name)?;
    }
    Ok(())
}
