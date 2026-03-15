#!/usr/bin/env python3
"""
Strip <br> tags from articles-data.js content template literals.

With `white-space: pre-line` on the article page, newlines in the source
render as line breaks automatically, making <br> tags redundant.

Rules:
  - In `content:` template literals: each <br> becomes a \n, and if the
    sequence is already followed by a \n that trailing newline is absorbed
    (so we don't double-space).
  - In `title:` strings: <br> is simply removed (titles shouldn't break).
  - </br> (invalid HTML) is removed everywhere.
"""

import re

FILE = 'articles/js/articles-data.js'

with open(FILE, 'r') as f:
    text = f.read()

# ── 1. Remove </br> everywhere ──────────────────────────────────────
text = re.sub(r'</br>', '', text)

# ── 2. Process `content:` template literals ─────────────────────────

def process_content(content_str):
    """Replace <br> tags with newlines inside a content template literal."""
    # Normalise <br/>, <br />, etc. → <br>
    s = re.sub(r'<br\s*/?>', '<br>', content_str)

    # Replace sequences of one or more <br> (with optional horizontal
    # whitespace between them) optionally followed by a single newline.
    # N <br> tags → N newline characters; the trailing \n is consumed.
    def _replace(m):
        count = len(re.findall(r'<br>', m.group(0)))
        return '\n' * count

    s = re.sub(r'(?:<br>[ \t]*)+\n?', _replace, s)
    return s

# Walk through the file, find `content: \`...\`` regions, and process them.
content_re = re.compile(r'content\s*:\s*`')
out = []
pos = 0

while pos < len(text):
    m = content_re.search(text, pos)
    if not m:
        out.append(text[pos:])
        break

    # Everything before the template literal body (including the opening `)
    out.append(text[pos:m.end()])

    # Find the closing backtick (unescaped)
    j = m.end()
    while j < len(text):
        if text[j] == '`' and text[j - 1] != '\\':
            break
        j += 1

    body = text[m.end():j]
    out.append(process_content(body))
    out.append('`')
    pos = j + 1

text = ''.join(out)

# ── 3. Strip <br> from title strings ────────────────────────────────
text = re.sub(r"(title:\s*['\"`][^'\"]*?)<br\s*/?>([^'\"]*?['\"`])", r'\1\2', text)

# ── 4. Write back ───────────────────────────────────────────────────
with open(FILE, 'w') as f:
    f.write(text)

print('Done – <br> tags stripped from', FILE)
