#!/usr/bin/env python3

import os
import re
from pathlib import Path

INFO_FILE = Path("Cargo.toml")
TEMPLATE_FILE = Path("docs") / Path("printr.txt.tpl")
OUTPUT_FILE = Path("dist") / Path("printr.txt")

with open(INFO_FILE) as file:
    string = file.read()
    version_pattern = r'version = ["](?:(\d+\.[.\d]*\d+))["]'
    try:
        VERSION_STRING = re.findall(version_pattern, string)[0]
    except IndexError:
        print("Unable to determine version, exiting...")
        exit(1)

    # this regex extracts the string between the `[` following the `authors`
    # key in the file and the matching `]`
    authors_pattern = r"(?<=authors[ ]=[ ]\[)[^\]]*"
    try:
        # we try to find the authors, and if it is found, we can safely assume
        # the authors are present in the 0th index
        matches = re.search(authors_pattern, string)[0]
        # we now have a string that contains the authors
        # we convert this string to a list of authors
        matches = matches.split(",")
        # we remove all empty strings from the list
        matches = list(filter(None, matches))
        # by now, we have all authors; each author is a separate element of
        # the list
        # we remove all newlines and spaces from each string
        matches = list(map(lambda s: s.strip(), matches))
        # we remove all double-quotes from the string
        matches = list(map(lambda s: s.strip(r'"'), matches))
        # we join all elements of the list using two newlines, for some weird
        # reason, a single newline does not seem to work
        AUTHORS = f"{os.linesep}{os.linesep}".join(matches)
    except IndexError:
        print("Unable to get authors, exiting...")
        exit(1)

with open(TEMPLATE_FILE) as file:
    document = file.read()
    document = document.replace(r"{{VERSION}}", VERSION_STRING)
    document = document.replace(r"{{AUTHORS}}", AUTHORS)

# we create the directory if it does not exist
OUTPUT_FILE.parent.mkdir(exist_ok=True)

with open(OUTPUT_FILE, "w") as file:
    file.write(document)
