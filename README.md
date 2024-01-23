# birthday RSS feed generator

This is a simple script to generate a RSS feed that I use to
remember my friends' birthdays.

The .env file contains information about where to write the output
xml file. This should be the directory where apache web server
looks for the file.

The `birthdays.txt` contains text such as:

```text
// This is the birthdays file
// comments are lines that begin with a newline
// comments and empty lines are ignored.

// Each line has a specific format:
// characters 0..=1 are the month
// characters 3..=4 are the day
// characters 6.. are the person's name

01-01 John Smith
01-04 TEST USER 2
01-23 TEST USER 23
01-24 TEST USER 24
01-25 TEST USER 25
09-09 Another user's birthday

```

the `birthdays.txt` file is ignored by git.

The `.env` file contains two keys:

```
BIRTHDAYS_FILE_DIR=/Users/ericwu/Desktop/birthdays-rss/birthdays.txt
XML_OUT_DIR=/Users/ericwu/Desktop/birthdays-rss/sample.xml
```
