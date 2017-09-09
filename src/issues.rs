use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;

pub fn list(redmine: &RedmineApi, arguments: &ArgMatches) -> Result<()> {
    let result = redmine.issues().list().execute()?;
    for i in result {
        println!("(#{}) {}", i.id, i.subject);
    }
    Ok(())
}
