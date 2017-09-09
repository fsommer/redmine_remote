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

pub fn create(redmine: &RedmineApi, arguments: &ArgMatches) -> Result<()> {
    let mut watcher_user_ids = Vec::new();
    watcher_user_ids.push(arguments.value_of("watcher-user-ids").unwrap().parse::<u32>().unwrap());

    let result = redmine.issues().create(
            arguments.value_of("project-id").unwrap().parse::<u32>().unwrap(),
            arguments.value_of("tracker-id").unwrap().parse::<u32>().unwrap(),
            arguments.value_of("status-id").unwrap().parse::<u32>().unwrap(),
            arguments.value_of("priority-id").unwrap().parse::<u32>().unwrap(),
            arguments.value_of("subject").unwrap()
        )
        .description(arguments.value_of("description").unwrap())
        .category_id(arguments.value_of("category-id").unwrap().parse::<u32>().unwrap())
        .fixed_version_id(arguments.value_of("version-id").unwrap().parse::<u32>().unwrap())
        .assigned_to_id(arguments.value_of("assigned-to-id").unwrap().parse::<u32>().unwrap())
        .parent_issue_id(arguments.value_of("parent-issue-id").unwrap().parse::<u32>().unwrap())
        .watcher_user_ids(watcher_user_ids)
        .is_private(arguments.value_of("is-private").unwrap().parse::<bool>().unwrap())
        .estimated_hours(arguments.value_of("estimated-hours").unwrap().parse::<f32>().unwrap())
        .execute();
    println!("Result: {:?}", result);
    Ok(())
}
