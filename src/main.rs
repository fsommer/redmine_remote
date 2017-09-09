#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate clap;
extern crate config;
extern crate redmine_api;

mod arguments;
mod errors;
mod settings;

use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;
use redmine_api::time_entries::TimeEntry;
use settings::Settings;

quick_main!(run);

fn run() -> Result<i32> {
    let settings = Settings::new()?;
    let matches = arguments::get_matches();

    let (host, apikey) = get_host_and_apikey(&settings, &matches)?;
    let redmine = RedmineApi::new(host, apikey);

    match matches.subcommand() {
        ("issues", Some(matches)) => match matches.subcommand() {
            ("create", Some(matches)) => {
                let mut watcher_user_ids = Vec::new();
                watcher_user_ids.push(matches.value_of("watcher-user-ids").unwrap().parse::<u32>().unwrap());

                let result = redmine.issues().create(
                        matches.value_of("project-id").unwrap().parse::<u32>().unwrap(),
                        matches.value_of("tracker-id").unwrap().parse::<u32>().unwrap(),
                        matches.value_of("status-id").unwrap().parse::<u32>().unwrap(),
                        matches.value_of("priority-id").unwrap().parse::<u32>().unwrap(),
                        matches.value_of("subject").unwrap()
                    )
                    .description(matches.value_of("description").unwrap())
                    .category_id(matches.value_of("category-id").unwrap().parse::<u32>().unwrap())
                    .fixed_version_id(matches.value_of("version-id").unwrap().parse::<u32>().unwrap())
                    .assigned_to_id(matches.value_of("assigned-to-id").unwrap().parse::<u32>().unwrap())
                    .parent_issue_id(matches.value_of("parent-issue-id").unwrap().parse::<u32>().unwrap())
                    .watcher_user_ids(watcher_user_ids)
                    .is_private(matches.value_of("is-private").unwrap().parse::<bool>().unwrap())
                    .estimated_hours(matches.value_of("estimated-hours").unwrap().parse::<f32>().unwrap())
                    .execute();
                println!("Result: {:?}", result);
            },
            _ => println!("nothing here yet"),
        },
        ("time_entries", Some(matches)) => match matches.subcommand() {
            ("create", Some(matches)) => {
                let issue_id = matches.value_of("issue-id").unwrap();
                let hours = matches.value_of("hours").unwrap();
                let activity_id = matches.value_of("activity-id").unwrap();
                let comments = matches.value_of("comments").unwrap();

                let time_entry = TimeEntry::new(
                        issue_id.parse::<u32>().unwrap(),
                        hours.parse::<f32>().unwrap(),
                        activity_id.parse::<u8>().unwrap()
                    )
                    .comments(comments);

                let result = redmine.time_entries().create(&time_entry);
                println!("Result: {:?}", result);
            },
            _ => println!("nothing here yet"),
        },
        _ => println!("nothing here yet"),
    };
    Ok(0)
}

fn get_host_and_apikey(settings: &Settings, matches: &ArgMatches) -> Result<(String, String)> {
    let host: String;
    if let Some(ref h) = matches.value_of("host") {
        host = h.to_string();
    } else if let Some(ref h) = settings.host {
        host = h.to_string();
    } else {
        bail!("host is missing");
    }

    let apikey: String;
    if let Some(ref ak) = matches.value_of("apikey") {
        apikey = ak.to_string();
    } else if let Some(ref ak) = settings.apikey {
        apikey = ak.to_string();
    } else {
        bail!("apikey is missing");
    }

    Ok((host, apikey))
}
