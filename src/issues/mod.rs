mod create;
mod list;

use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;

pub fn handle(redmine: &RedmineApi, args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        ("list", Some(args)) => list::execute(&redmine, &args),
        ("create", Some(args)) => create::execute(&redmine, &args),
        _ => Ok(println!("{}", args.usage())),
    }
}

