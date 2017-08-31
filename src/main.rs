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

    if let Some(matches) = matches.subcommand_matches("time_entries") {
        if let Some(matches) = matches.subcommand_matches("create") {
            let issue_id = matches.value_of("issue-id").unwrap();
            let hours = matches.value_of("hours").unwrap();
            let activity_id = matches.value_of("activity-id").unwrap();
            let comments = matches.value_of("comments").unwrap();

            let time_entry = TimeEntry {
                issue_id: issue_id.parse::<u32>().unwrap(),
                hours: hours.parse::<f32>().unwrap(),
                activity_id: activity_id.parse::<u8>().unwrap(),
                comments: comments.to_string(),
            };

            let result = redmine.time_entries().create(&time_entry);
            println!("Result: {:?}", result);
        } else {
        println!("nothing here yet");
    }
    } else {
        println!("nothing here yet");
    }
}
