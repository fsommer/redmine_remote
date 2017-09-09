extern crate clap;
extern crate config;
extern crate redmine_api;

#[macro_use]
extern crate serde_derive;

mod arguments;
mod settings;

use redmine_api::RedmineApi;
use redmine_api::time_entries::TimeEntry;
use settings::Settings;

fn main() {
    let mut settings = Settings::new().unwrap();
    let matches = arguments::get_matches();

    if let Some(h) = matches.value_of("host") {
        settings.host = Some(h.to_string());
    }
    if let Some(ak) = matches.value_of("apikey") {
        settings.apikey = Some(ak.to_string());
    }

    if settings.host.is_none() || settings.apikey.is_none() {
        panic!("Please provide host and apikey");
    }

    let redmine = RedmineApi::new(
        settings.host.unwrap(),
        settings.apikey.unwrap(),
    );

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
    }
}
