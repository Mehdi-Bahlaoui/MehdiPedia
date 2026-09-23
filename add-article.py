#!/usr/bin/env python3
"""Add a new article entry to assets/js/articles-data.js (and llms.txt).

Usage:
    python3 add-article.py "My New Article"   # title as argument
    python3 add-article.py                      # prompts for the title

The article id is generated from the title by replacing whitespace runs
with underscores (e.g. "My New Article" -> "My_New_Article"), matching the
existing convention in articles-data.js. The new entry is inserted before
the closing of the articlesData object with today's date and a starter
'introduction' section for you to fill in.
"""

import datetime
import pathlib
import re
import shutil
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent
DATA_FILE = ROOT / "assets" / "js" / "articles-data.js"
LLMS_FILE = ROOT / "llms.txt"


def make_id(title):
    """Title -> id: strip, collapse whitespace runs to single underscores."""
    return re.sub(r"\s+", "_", title.strip())


def main():
    if len(sys.argv) > 1:
        title = " ".join(sys.argv[1:]).strip()
    else:
        title = input("Article title: ").strip()

    if not title:
        print("Error: title cannot be empty.")
        sys.exit(1)

    article_id = make_id(title)
    if not article_id:
        print("Error: could not derive an id from that title.")
        sys.exit(1)

    today = datetime.date.today()
    date_str = f"{today.day}/{today.month}/{today.year}"  # e.g. 23/9/2026
    llms_date = f"{today.day} {today.strftime('%b')} {today.year}"  # 23 Sep 2026

    safe_key = article_id.replace("\\", "\\\\").replace("'", "\\'")
    safe_title = title.replace("\\", "\\\\").replace('"', '\\"')

    text = DATA_FILE.read_text()
    if f"'{safe_key}':" in text:
        print(f"Error: id '{article_id}' already exists in {DATA_FILE.name}.")
        sys.exit(1)
    if re.search(r"[^A-Za-z0-9_\-?]", article_id):
        print(f"Warning: id '{article_id}' contains unusual characters; "
              "only letters, digits, _, - and ? are typical.")

    today = datetime.date.today()
    date_str = f"{today.day}/{today.month}/{today.year}"  # e.g. 23/9/2026
    llms_date = f"{today.day} {today.strftime('%b')} {today.year}"  # 23 Sep 2026

    entry = (
        f"  '{safe_key}': {{\n"
        f'    title: "{safe_title}",\n'
        f"    date: '{date_str}',\n"
        f"    status: '',\n"
        f"    sections: [\n"
        f"      {{\n"
        f"        id: 'introduction',\n"
        f"        title: 'Introduction',\n"
        f"        content: \n"
        f"          `Write your article here.\n"
        f"\n"
        f"          `,\n"
        f"      }}\n"
        f"    ]\n"
        f"  }},\n"
    )

    # Insert before the final closing of the articlesData object.
    closing = text.rfind("};")
    if closing == -1:
        print(f"Error: could not find closing '}};' in {DATA_FILE}.")
        sys.exit(1)
    DATA_FILE.write_text(text[:closing] + entry + text[closing:])
    print(f"Added '{article_id}' to {DATA_FILE.relative_to(ROOT)}.")

    # Keep llms.txt in sync (insert at the end of the Articles list).
    llms = LLMS_FILE.read_text()
    marker = "## Music"
    line = (f"- [{title}](https://mehdibahlaoui.com/articles/article.html"
            f"#{article_id}): {llms_date}\n")
    if f"article.html#{article_id}" not in llms and marker in llms:
        llms = llms.replace(marker, line + "\n" + marker, 1)
        LLMS_FILE.write_text(llms)
        print(f"Listed '{article_id}' in {LLMS_FILE.name}.")
    else:
        print(f"Skipped {LLMS_FILE.name} update (already listed or section missing).")

    # Syntax-check the edited JS if node is available.
    node = shutil.which("node")
    if node:
        result = subprocess.run(
            [node, "--check", str(DATA_FILE)],
            capture_output=True, text=True)
        if result.returncode == 0:
            print("node --check: OK.")
        else:
            print("node --check FAILED:\n" + result.stderr)
            sys.exit(1)
    else:
        print("node not found, skipping syntax check.")

    print(f"\nNext: edit the 'introduction' content, then open "
          f"articles/article.html#{article_id}")


if __name__ == "__main__":
    main()
