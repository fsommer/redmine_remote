use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;

pub fn execute(redmine: &RedmineApi, args: &ArgMatches) -> Result<()> {
    let project_id = args.value_of("set-project-id");
    let tracker_id = args.value_of("set-tracker-id");
    let status_id = args.value_of("set-status-id");
    let priority_id = args.value_of("set-priority-id");
    let subject = args.value_of("set-subject");

    if project_id.is_none() || tracker_id.is_none() || status_id.is_none() ||
       priority_id.is_none() || subject.is_none() {
        bail!("Please provide atleast project-id, tracker-id, status-id, priority-id and subject");
    }

    let mut issue = redmine.issues().create(
        project_id.unwrap().parse::<u32>()?,
        tracker_id.unwrap().parse::<u32>()?,
        status_id.unwrap().parse::<u32>()?,
        priority_id.unwrap().parse::<u32>()?,
        subject.unwrap());

    if let Some(d) = args.value_of("set-description") {
        issue = issue.description(d);
    }

    if let Some(c) = args.value_of("set-category-id") {
        issue = issue.category_id(c.parse::<u32>().chain_err(|| "category-id must be numeric")?);
    }

    if let Some(v) = args.value_of("set-version-id") {
        issue = issue.fixed_version_id(v.parse::<u32>().chain_err(|| "version-id must be numeric")?);
    }

    if let Some(a) = args.value_of("set-assigned-to-id") {
        issue = issue.assigned_to_id(a.parse::<u32>().chain_err(|| "assigned-to-id must be numeric")?);
    }

    if let Some(p) = args.value_of("set-parent-issue-id") {
        issue = issue.parent_issue_id(p.parse::<u32>().chain_err(|| "parent-issue-id must be numeric")?);
    }

    if let Some(w) = args.value_of("set-watcher-user-ids") {
        let mut watcher_user_ids = Vec::new();
        watcher_user_ids.push(w.parse::<u32>().chain_err(|| "watcher-user-ids must be numeric")?); // TODO: comma-spearated list
        issue = issue.watcher_user_ids(watcher_user_ids);
    }

    if let Some(p) = args.value_of("set-is-private") {
        issue = issue.is_private(p.parse::<bool>().chain_err(|| "is-private must be boolean")?);
    }

    if let Some(eh) = args.value_of("set-estimated-hours") {
        issue = issue.estimated_hours(eh.parse::<f32>().chain_err(|| "estimated-hours must be floating point number")?);
    }

    match issue.execute() {
        Ok(l) => println!("{}", l),
        _ => bail!("Can't create issue"),
    }

    Ok(())
}
