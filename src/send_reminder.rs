use crate::BirthdayEntry;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

pub fn send_birthday_reminder(birthday: &BirthdayEntry, curr_year: i32) {
    let name = &birthday.name;
    let month = &birthday.month;
    let day = &birthday.day;
    let mut message = format!("{name}'s birthday is {month}-{day}. That's today! ",);

    if let Some(year) = &birthday.year {
        let age = curr_year - *year;

        message += &format!(
            "They were born in {} and they are turning {} years old today. ",
            year, age
        );
    }

    message += "\n\nTheir full record in the JSON file is: ";
    message += &serde_json::to_string_pretty(birthday).unwrap();

    println!("{}", message);

    let subject = format!("{}'s Birthday", name);

    send_email(&subject, &message).expect("Error sending email");
}

pub fn send_email(subject: &str, body: &str) -> Result<(), Box<dyn Error>> {
    let gmail_user = std::env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS must be set.");
    let gmail_password = std::env::var("EMAIL_PASS").expect("EMAIL_PASS must be set.");
    let to_email = std::env::var("EMAIL_RECIPIENT").expect("EMAIL_RECIPIENT must be set.");

    // Create email message
    let email = Message::builder()
        .from(gmail_user.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())?;

    // Gmail SMTP configuration
    let creds = Credentials::new(gmail_user.to_string(), gmail_password.to_string());

    // Create SMTP transport for Gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully to {}", to_email);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to send email: {}", e);
            Err(Box::new(e))
        }
    }
}
