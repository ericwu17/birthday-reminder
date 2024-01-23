mod parse_birthdays;

use chrono::*;
use dotenv::dotenv;
use parse_birthdays::parse_birthday_entries;
use simple_xml_builder::XMLElement;
use std::fs::File;

#[derive(Clone, Debug)]
struct BirthdayEntry {
    month: i32,
    day: i32,
    name: String,
}

fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    let birthdays_file_dir =
        std::env::var("BIRTHDAYS_FILE_DIR").expect("BIRTHDAYS_FILE_DIR must be set.");

    let xml_out_dir = std::env::var("XML_OUT_DIR").expect("XML_OUT_DIR must be set.");

    let utc_time: DateTime<Utc> = Utc::now();
    let curr_year = utc_time.year();

    let birthdays = parse_birthday_entries(&birthdays_file_dir);
    let recent_birthdays = get_recent_birthdays(birthdays, 5);

    let file = File::create(xml_out_dir).expect("error creating output file");

    let mut rss_element = XMLElement::new("rss");
    rss_element.add_attribute("version", "2.0");

    let mut channel_elem = XMLElement::new("channel");
    let mut title_elem = XMLElement::new("title");
    title_elem.add_text("Birthdays");
    channel_elem.add_child(title_elem);
    channel_elem.add_child(get_empty_link_elem());

    let mut desc_elem = XMLElement::new("description");
    desc_elem.add_text("A personal RSS feed to remind me of friends' birthdays");
    channel_elem.add_child(desc_elem);

    for bd_entry in recent_birthdays {
        let item = generate_birthday_item(&bd_entry, curr_year);
        channel_elem.add_child(item);
    }

    rss_element.add_child(channel_elem);
    rss_element.write(file).expect("error writing to file");
}

fn get_recent_birthdays(
    mut entries: Vec<BirthdayEntry>,
    max_num_birthdays: usize,
) -> Vec<BirthdayEntry> {
    let utc_time: DateTime<Utc> = Utc::now();
    let curr_month = utc_time.month();
    let curr_day = utc_time.day();

    let curr_month_day = curr_month * 100 + curr_day;

    entries.sort_by_key(|bd| bd.month * 100 + bd.day);
    entries
        .iter()
        .filter(|bd| bd.month * 100 + bd.day <= curr_month_day as i32)
        .rev()
        .take(max_num_birthdays)
        .cloned()
        .collect()
}

fn generate_birthday_item(bd_entry: &BirthdayEntry, curr_year: i32) -> XMLElement {
    let BirthdayEntry { name, month, day } = bd_entry;
    let mut item_elem = XMLElement::new("item");
    let mut title_elem = XMLElement::new("title");
    title_elem.add_text(name);
    item_elem.add_child(title_elem);

    let mut desc_elem = XMLElement::new("description");
    desc_elem.add_text(format!("{}'s birthday is {:0>2}-{:0>2}", name, month, day));

    let mut guid_elem = XMLElement::new("guid");
    guid_elem.add_text(format!("{}-{}-{}-{}", name, curr_year, month, day));

    let date_of_birthday = Utc
        .with_ymd_and_hms(curr_year, *month as u32, *day as u32, 0, 0, 0)
        .unwrap();
    let mut pub_date_elem = XMLElement::new("pubDate");
    let pub_date_text = format!("{}", date_of_birthday.format("%a, %d %b %Y %H:%M:%S %Z"));
    pub_date_elem.add_text(pub_date_text);

    item_elem.add_child(desc_elem);
    item_elem.add_child(guid_elem);
    item_elem.add_child(pub_date_elem);
    item_elem.add_child(get_empty_link_elem());

    item_elem
}

fn get_empty_link_elem() -> XMLElement {
    let mut elem = XMLElement::new("link");
    elem.add_text("https://tslyriccompletion.com");
    elem
}
