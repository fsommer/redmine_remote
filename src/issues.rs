use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;

pub fn list(redmine: &RedmineApi, args: &ArgMatches) -> Result<()> {
    let result = redmine.issues().list().execute()?;
    for i in result {
        println!("(#{}) {}", i.id, i.subject);
    }

    Ok(())
}

pub fn create(redmine: &RedmineApi, args: &ArgMatches) -> Result<()> {
    let mut watcher_user_ids = Vec::new();
    watcher_user_ids.push(args.value_of("watcher-user-ids").unwrap().parse::<u32>().unwrap());

    let result = redmine.issues().create(
            args.value_of("project-id").unwrap().parse::<u32>().unwrap(),
            args.value_of("tracker-id").unwrap().parse::<u32>().unwrap(),
            args.value_of("status-id").unwrap().parse::<u32>().unwrap(),
            args.value_of("priority-id").unwrap().parse::<u32>().unwrap(),
            args.value_of("subject").unwrap()
        )
        .description(args.value_of("description").unwrap())
        .category_id(args.value_of("category-id").unwrap().parse::<u32>().unwrap())
        .fixed_version_id(args.value_of("version-id").unwrap().parse::<u32>().unwrap())
        .assigned_to_id(args.value_of("assigned-to-id").unwrap().parse::<u32>().unwrap())
        .parent_issue_id(args.value_of("parent-issue-id").unwrap().parse::<u32>().unwrap())
        .watcher_user_ids(watcher_user_ids)
        .is_private(args.value_of("is-private").unwrap().parse::<bool>().unwrap())
        .estimated_hours(args.value_of("estimated-hours").unwrap().parse::<f32>().unwrap())
        .execute();
    println!("Result: {:?}", result);

    Ok(())
}
