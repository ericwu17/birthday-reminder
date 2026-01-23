# This script generates a file "anki_notes.csv"
# which can be imported into Anki with the "import" function
# You can search the help menu to find the "import function",
# which prompts you to select a file and then tells you what note type it interprets the file as.

import json
import os

output_file = "anki_notes.csv"
input_file = "birthdays.json"

data = None
with open (input_file) as f:
    file_string = ""
    divider = "="*10
    while f.readline().strip() != divider:
        pass
    data = json.loads(f.read())

if os.path.exists(output_file):
    input(f"press enter to overwrite {output_file} ")

with open(output_file, "w") as f:
    for entry in data:
        name = entry["name"]
        month = entry["month"]
        day = entry["day"]
        year = ""
        if "year" in entry:
            year = entry["year"]
        f.write(f"{name},{year},{month},{day}\n")

