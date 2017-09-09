mod create;

use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;

pub fn handle(redmine: &RedmineApi, args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        ("list", Some(args)) => list(&redmine, &args),
        ("create", Some(args)) => create::execute(&redmine, &args),
        _ => Ok(println!("{}", args.usage())),
    }
}

fn list(redmine: &RedmineApi, args: &ArgMatches) -> Result<()> {
    let result = redmine.issues().list().execute()?;
    for i in result {
        println!("(#{}) {}", i.id, i.subject);
    }

    Ok(())
}
