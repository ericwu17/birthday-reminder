use crate::send_reminder::send_email;

pub fn test_email() {
    println!("Sending test email...");

    send_email(
        "Test email",
        "This test email is sent from Birthday Reminders",
    );

    println!("Test email has been sent");
}
