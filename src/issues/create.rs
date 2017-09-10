use clap::ArgMatches;
use errors::*;
use redmine_api::RedmineApi;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Issue {
    project_id: Option<u32>,
    tracker_id: Option<u32>,
    status_id : Option<u32>,
    priority_id: Option<u32>,
    subject: Option<String>,
    description: Option<String>,
    category_id: Option<u32>,
    version_id: Option<u32>,
    assigned_to_id: Option<u32>,
    parent_issue_id: Option<u32>,
    watcher_user_ids: Option<u32>,
    is_private: Option<bool>,
    estimated_hours: Option<f32>,
}

pub fn execute(redmine: &RedmineApi, clap: &ArgMatches) -> Result<()> {
    let issue = Issue::from_clap(clap.clone());

    if issue.project_id.is_none()
        || issue.tracker_id.is_none()
        || issue.status_id.is_none()
        || issue.priority_id.is_none()
        || issue.subject.is_none() {
        bail!("Please provide atleast project-id, tracker-id, status-id, \
              priority-id and subject");
    }

    let subject = issue.subject.unwrap();
    let mut i = redmine.issues().create(
        issue.project_id.unwrap(),
        issue.tracker_id.unwrap(),
        issue.status_id.unwrap(),
        issue.priority_id.unwrap(),
        &subject);

    if let Some(ref d) = issue.description {
        i = i.description(d);
    }

    if let Some(id) = issue.category_id {
        i = i.category_id(id);
    }

    if let Some(id) = issue.version_id {
        i = i.fixed_version_id(id);
    }

    if let Some(id) = issue.assigned_to_id {
        i = i.assigned_to_id(id);
    }

    if let Some(id) = issue.parent_issue_id {
        i = i.parent_issue_id(id);
    }

    if let Some(ids) = issue.watcher_user_ids {
        let mut watcher_user_ids = Vec::new();
        watcher_user_ids.push(ids); // TODO: comma-spearated list
        i = i.watcher_user_ids(watcher_user_ids);
    }

    if let Some(p) = issue.is_private {
        i = i.is_private(p);
    }

    if let Some(h) = issue.estimated_hours {
        i = i.estimated_hours(h);
    }

    match i.execute() {
        Ok(l) => println!("{}", l),
        _ => bail!("Can't create issue"),
    }

    Ok(())
}
