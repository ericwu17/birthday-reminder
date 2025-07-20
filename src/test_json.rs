use crate::{parse_birthdays::load_birthday_entries, BirthdayEntry};

pub fn test_json() {
    let birthdays_file_dir =
        std::env::var("BIRTHDAYS_FILE_DIR").expect("BIRTHDAYS_FILE_DIR must be set.");

    let birthdays = load_birthday_entries(&birthdays_file_dir);

    println!("JSON loaded successfully.");

    for birthday in birthdays {
        sanity_checks(&birthday);
    }
}

fn sanity_checks(birthday: &BirthdayEntry) {
    let name = &birthday.name;
    let day = birthday.day;
    let month = birthday.month;
    let year = birthday.year;
    if month < 1 || month > 12 {
        println!("warning: name {} has month {}", name, month);
    }
    if day < 1 || day > 31 {
        println!("warning: name {} has day {}", name, day);
    }
    if let Some(year) = year {
        if year < 1925 || year > 2030 {
            // Will need to change this after I start knowing people born after 2030
            println!("warning: name {} has year {}", name, year);
        }
    }
}
