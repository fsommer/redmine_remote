use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;

pub fn execute(redmine: &RedmineApi, args: &ArgMatches) -> Result<()> {
    let result = redmine.issues().list().execute()?;
    for i in result {
        println!("(#{}) {}", i.id, i.subject);
    }

    Ok(())
}
