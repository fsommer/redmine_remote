extern crate docopt;
extern crate redmine_api;
extern crate rustc_serialize;

use docopt::Docopt;
use redmine_api::RedmineApi;
use redmine_api::time_entries::TimeEntry;

const USAGE: &'static str = "
Redmine Remote.

Usage:
  rr time-entries create --host=<host> --apikey=<apikey> <issue-id> <hours> <activity-id> <comments>
  rr (-h | --help)

Options:
  -h --help          Show this screen.
  --host=<host>      Redmine host.
  --apikey=<apikey>  Redmine API key.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_time_entries: bool,
    cmd_create: bool,
    flag_host: String,
    flag_apikey: String,
    arg_issue_id: u32,
    arg_hours: f32,
    arg_activity_id: u8,
    arg_comments: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);

    let redmine = RedmineApi::new(
    //    "http://localhost:10083".to_string(),
    //    "9d61c6c2696289c545673daad62272a3ea91f3ef".to_string(),
        args.flag_host,
        args.flag_apikey,
    );

    let time_entry = TimeEntry {
        issue_id: args.arg_issue_id,
        hours: args.arg_hours,
        activity_id: args.arg_activity_id,
        comments: args.arg_comments,
    };

    redmine.time_entries().create(&time_entry);
}
