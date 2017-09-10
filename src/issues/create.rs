use clap::ArgMatches;
use csv;
use csv::DeserializeRecordsIter;
use errors::*;
use redmine_api::RedmineApi;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(Debug, Deserialize, StructOpt)]
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
impl Issue {
    pub fn merge(&mut self, issue: &Issue) -> &mut Self {
        if let Some(id) = issue.project_id {
            self.project_id = Some(id);
        }

        if let Some(id) = issue.tracker_id {
            self.tracker_id = Some(id);
        }

        if let Some(id) = issue.status_id {
            self.status_id = Some(id);
        }

        if let Some(id) = issue.priority_id {
            self.priority_id = Some(id);
        }

        if let Some(ref s) = issue.subject {
            self.subject = Some(s.to_string());
        }

        if let Some(ref d) = issue.description {
            self.description = Some(d.to_string());
        }

        if let Some(id) = issue.category_id {
            self.category_id = Some(id);
        }

        if let Some(id) = issue.version_id {
            self.version_id = Some(id);
        }

        if let Some(id) = issue.assigned_to_id {
            self.assigned_to_id = Some(id);
        }

        if let Some(id) = issue.parent_issue_id {
            self.parent_issue_id = Some(id);
        }

        if let Some(ids) = issue.watcher_user_ids {
            self.watcher_user_ids = Some(ids);
        }

        if let Some(p) = issue.is_private {
            self.is_private = Some(p);
        }

        if let Some(h) = issue.estimated_hours {
            self.estimated_hours = Some(h);
        }

        self
    }
}

pub fn execute(redmine: &RedmineApi, clap: &ArgMatches) -> Result<()> {
    let issue_args = Issue::from_clap(clap.clone());

    if let Some(batch_file) = clap.value_of("batch_file") {
        let mut file = File::open(batch_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(contents.as_bytes());

        let iterator: DeserializeRecordsIter<&[u8], Issue> = reader.deserialize();
        for line in iterator {
            let mut issue = line?;
            issue.merge(&issue_args);

            create_single(redmine, issue)?;
        }
    } else {
        return create_single(redmine, issue_args);
    }

    Ok(())
}

fn create_single(redmine: &RedmineApi, issue: Issue) -> Result<()> {
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
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
