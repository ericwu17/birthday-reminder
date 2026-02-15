pub mod parse_birthdays;
pub mod print_usage;
pub mod send_reminder;
pub mod test_email;
pub mod test_json;

use chrono::*;
use dotenv::dotenv;
use parse_birthdays::load_birthday_entries;
use serde::{Deserialize, Serialize};

use crate::send_reminder::send_birthday_reminder;
use print_usage::print_usage;
use test_email::test_email;
use test_json::test_json;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BirthdayEntry {
    name: String,
    month: i32,
    day: i32,
    year: Option<i32>,
    phone: Option<String>,
    email: Option<String>,
    other_contact: Option<String>,
}

fn get_todays_birthdays(entries: &[BirthdayEntry]) -> Vec<&BirthdayEntry> {
    let utc_time: DateTime<Utc> = Utc::now();
    let current_month = utc_time.month() as i32;
    let current_day = utc_time.day() as i32;

    entries
        .iter()
        .filter(|entry| entry.month == current_month && entry.day == current_day)
        .collect()
}

fn remind_birthdays() {
    let birthdays_file_dir =
        std::env::var("BIRTHDAYS_FILE_DIR").expect("BIRTHDAYS_FILE_DIR must be set.");

    let birthdays = load_birthday_entries(&birthdays_file_dir);

    let todays_birthdays = get_todays_birthdays(&birthdays);

    let utc_time: DateTime<Utc> = Utc::now();
    let curr_year = utc_time.year() as i32;

    for birthday in todays_birthdays {
        send_birthday_reminder(birthday, curr_year);
    }
}

fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "--test-json" => {
            test_json();
        }
        "--test-email" => {
            test_email();
        }
        "--remind-birthdays" => {
            remind_birthdays();
        }
        _ => {
            println!("Error: Unknown argument '{}'", args[1]);
            print_usage();
        }
    }
}
