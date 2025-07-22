# Birthday reminder

This program is meant to be run once a day from a cron job,
to send emails to remind me when people's birthdays are happening.

The .env file contains the following information:

- `BIRTHDAYS_FILE_DIR` is the path to the input json file containing everyone's birthday.
This file is ignored by git because I didn't want to push everyone's birthday information to github.
- `EMAIL_ADDRESS`, `EMAIL_RECIPIENT`, and `EMAIL_PASS` are information to facilitate sending emails.

