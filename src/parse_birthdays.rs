use std::fs::File;
use std::io::Read;

use crate::BirthdayEntry;

pub fn parse_birthday_entries(dir: &str) -> Vec<BirthdayEntry> {
    let mut file = File::open(dir).expect("error opening birthday entries file");

    let mut s = Vec::new();
    file.read_to_end(&mut s).unwrap();
    let s = String::from_utf8(s).expect("birthdays file contains invalid utf-8");

    let mut birthdays = Vec::new();

    for line in s.split('\n') {
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let month = str::parse(&line[0..=1]).expect("invalid birthday file format");
        let day = str::parse(&line[3..=4]).expect("invalid birthday file format");
        let name = line[6..].to_string();
        birthdays.push(BirthdayEntry { month, day, name });
    }

    birthdays
}
