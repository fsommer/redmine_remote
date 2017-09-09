use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;
use redmine_api::time_entries::TimeEntry;

pub fn create(redmine: &RedmineApi, args: &ArgMatches) -> Result<()> {
    let issue_id = args.value_of("issue-id").unwrap();
    let hours = args.value_of("hours").unwrap();
    let activity_id = args.value_of("activity-id").unwrap();
    let comments = args.value_of("comments").unwrap();

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
