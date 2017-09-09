use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;
use redmine_api::time_entries::TimeEntry;

pub fn create(redmine: &RedmineApi, arguments: &ArgMatches) -> Result<()> {
    let issue_id = arguments.value_of("issue-id").unwrap();
    let hours = arguments.value_of("hours").unwrap();
    let activity_id = arguments.value_of("activity-id").unwrap();
    let comments = arguments.value_of("comments").unwrap();

    let time_entry = TimeEntry::new(
            issue_id.parse::<u32>().unwrap(),
            hours.parse::<f32>().unwrap(),
            activity_id.parse::<u8>().unwrap()
        )
        .comments(comments);

    let result = redmine.time_entries().create(&time_entry);
    println!("Result: {:?}", result);

    Ok(())
}
