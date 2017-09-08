extern crate clap;
extern crate redmine_api;

use clap::{Arg, App, SubCommand};
use redmine_api::RedmineApi;
use redmine_api::time_entries::TimeEntry;

fn main() {
    let matches = App::new("Redmine Remote")
        .version("0.0.1")
        .author("Florian Sommer <fsommer1986@gmail.com>")
        .about("Command line program for using the redmine api (see http://www.redmine.org/).")
        .arg(Arg::with_name("host")
             .long("host")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("apikey")
             .long("apikey")
             .takes_value(true)
             .required(true))
        .subcommand(SubCommand::with_name("issues")
                    .about("Handles the issues sub api.")
                    .subcommand(SubCommand::with_name("create")
                                .about("Creates new issue.")
                                .arg(Arg::with_name("project-id")
                                     .required(true))
                                .arg(Arg::with_name("tracker-id")
                                     .required(true))
                                .arg(Arg::with_name("status-id")
                                     .required(true))
                                .arg(Arg::with_name("priority-id")
                                     .required(true))
                                .arg(Arg::with_name("subject")
                                     .required(true))
                                .arg(Arg::with_name("description")
                                     .required(true))
                                .arg(Arg::with_name("category-id")
                                     .required(true))
                                .arg(Arg::with_name("version-id")
                                     .required(true))
                                .arg(Arg::with_name("assigned-to-id")
                                     .required(true))
                                .arg(Arg::with_name("parent-issue-id")
                                     .required(true))
                                .arg(Arg::with_name("watcher-user-ids")
                                     .required(true))
                                .arg(Arg::with_name("is-private")
                                     .required(true))
                                .arg(Arg::with_name("estimated-hours")
                                     .required(true))))
        .subcommand(SubCommand::with_name("time_entries")
                    .about("Handles the time entries sub api.")
                    .subcommand(SubCommand::with_name("create")
                                .about("Creates new time entry.")
                                .arg(Arg::with_name("issue-id")
                                     .required(true))
                                .arg(Arg::with_name("hours")
                                     .required(true))
                                .arg(Arg::with_name("activity-id")
                                     .required(true))
                                .arg(Arg::with_name("comments")
                                     .required(true))))
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let apikey = matches.value_of("apikey").unwrap();

    let redmine = RedmineApi::new(
        host.to_string(),
        apikey.to_string(),
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
