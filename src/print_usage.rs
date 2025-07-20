use std::env;

pub fn print_usage() {
    println!(
        "Usage: {} [OPTION]",
        env::args().next().unwrap_or("program".to_string())
    );
    println!();
    println!("Options:");
    println!("  --test-json         Load a JSON file and verify its format");
    println!("  --test-email        Send a test email with fixed subject and body");
    println!("  --remind-birthdays  Send birthday reminders for today's birthdays");
}
