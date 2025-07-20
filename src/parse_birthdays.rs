use crate::BirthdayEntry;
use std::fs;

pub fn load_birthday_entries(file_path: &str) -> Vec<BirthdayEntry> {
    // Read the file
    let file_content = fs::read_to_string(file_path).expect("Error reading file from path");

    // Find the "==========" separator and extract everything after it
    let separator = "==========\n";
    let json_content = if let Some(separator_pos) = file_content.find(separator) {
        // Find the end of the line containing "=========="
        &file_content[(separator_pos + separator.len())..]
    } else {
        // No separator found, treat entire file as JSON
        &file_content
    };

    let entries: Vec<BirthdayEntry> =
        serde_json::from_str(&json_content).expect("Error loading JSON");

    entries
}
