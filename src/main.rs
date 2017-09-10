extern crate clap;
extern crate csv;
extern crate config;
#[macro_use]
extern crate error_chain;
extern crate redmine_api;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod args;
mod errors;
mod issues;
mod settings;
mod time_entries;

use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;
use settings::Settings;

quick_main!(run);

fn run() -> Result<()> {
    let settings = Settings::new()?;
    let args = args::get_matches();

    let (host, apikey) = get_host_and_apikey(&settings, &args)?;
    let redmine = RedmineApi::new(host, apikey);

    match args.subcommand() {
        ("issues", Some(args)) => issues::handle(&redmine, &args),
        ("time_entries", Some(args)) => time_entries::handle(&redmine, &args),
        _ => Ok(println!("{}", args.usage())),
    }
}

fn get_host_and_apikey(settings: &Settings, args: &ArgMatches) -> Result<(String, String)> {
    let host: &str;
    if let Some(ref h) = args.value_of("host") {
        host = h;
    } else if let Some(ref h) = settings.host {
        host = h;
    } else {
        bail!("host is missing");
    }

    let apikey: &str;
    if let Some(ref ak) = args.value_of("apikey") {
        apikey = ak;
    } else if let Some(ref ak) = settings.apikey {
        apikey = ak;
    } else {
        bail!("apikey is missing");
    }

    Ok((host.to_string(), apikey.to_string()))
}
