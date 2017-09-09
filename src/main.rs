#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate clap;
extern crate config;
extern crate redmine_api;

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

fn run() -> Result<i32> {
    let settings = Settings::new()?;
    let args = args::get_matches();

    let (host, apikey) = get_host_and_apikey(&settings, &args)?;
    let redmine = RedmineApi::new(host, apikey);

    match args.subcommand() {
        ("issues", Some(args)) => match args.subcommand() {
            ("list", Some(args)) => { issues::list(&redmine, &args)?; },
            ("create", Some(args)) => { issues::create(&redmine, &args)?; },
            _ => println!("{}", args.usage()),
        },
        ("time_entries", Some(args)) => match args.subcommand() {
            ("create", Some(args)) => { time_entries::create(&redmine, &args)?; },
            _ => println!("{}", args.usage()),
        },
        _ => println!("{}", args.usage()),
    };

    Ok(0)
}

fn get_host_and_apikey(settings: &Settings, args: &ArgMatches) -> Result<(String, String)> {
    let host: String;
    if let Some(ref h) = args.value_of("host") {
        host = h.to_string();
    } else if let Some(ref h) = settings.host {
        host = h.to_string();
    } else {
        bail!("host is missing");
    }

    let apikey: String;
    if let Some(ref ak) = args.value_of("apikey") {
        apikey = ak.to_string();
    } else if let Some(ref ak) = settings.apikey {
        apikey = ak.to_string();
    } else {
        bail!("apikey is missing");
    }

    Ok((host, apikey))
}
