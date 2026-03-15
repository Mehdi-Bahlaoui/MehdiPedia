#!/usr/bin/env python3
"""
Strip <br> from <p> tags inside .section-text / .globe-text divs in HTML files.

With `white-space: pre-line`, source newlines render as line breaks.
So for each <p> inside these regions, this script:
  1. Collapses source formatting newlines to spaces
  2. Converts <br> tags to actual newlines
  3. Trims each line

<br> tags OUTSIDE <p> (sidebar, footer, captions, .font-philo, direct text)
are left untouched — they still render as normal HTML line breaks.
"""

import re


def find_div_content_ranges(html, class_name):
    """Return [(start, end), ...] for inner content of divs with given class."""
    pat = re.compile(
        rf'<div\b[^>]*class="[^"]*\b{re.escape(class_name)}\b[^"]*"[^>]*>'
    )
    ranges = []
    for m in pat.finditer(html):
        start = m.end()
        depth = 1
        i = start
        while i < len(html) and depth > 0:
            if re.match(r'<div\b', html[i:]):
                depth += 1
                i += 4
            elif re.match(r'</div\b', html[i:]):
                depth -= 1
                if depth == 0:
                    ranges.append((start, i))
                i += 5
            else:
                i += 1
    return ranges


def reformat_p_content(content):
    """Collapse source whitespace, convert <br> to newlines."""
    # Protect HTML comments
    saved = []
    def save(m):
        saved.append(m.group(0))
        return f'\x01{len(saved) - 1}\x01'
    content = re.sub(r'<!--.*?-->', save, content, flags=re.DOTALL)

    # Remove </br> (invalid HTML)
    content = re.sub(r'</br>', '', content)

    # Mark all <br> variants
    content = re.sub(r'<br\s*/?>', '\x00', content)

    # Collapse all whitespace (newlines, tabs, multi-spaces) → single space
    content = re.sub(r'\s+', ' ', content)

    # Convert marker sequences to newlines
    # 3+ <br> → triple newline (preserve extra spacing intent)
    content = re.sub(r'(\x00 ?){3,}', '\n\n\n', content)
    # 2 <br> → double newline (paragraph break)
    content = re.sub(r'(\x00 ?){2}', '\n\n', content)
    # 1 <br> → single newline
    content = re.sub(r'\x00 ?', '\n', content)

    # Restore comments
    for i, c in enumerate(saved):
        content = content.replace(f'\x01{i}\x01', c)

    # Trim each line
    lines = [l.strip() for l in content.split('\n')]
    return '\n'.join(lines).strip()


def process_html(filepath, class_names):
    with open(filepath) as f:
        html = f.read()

    # Collect target div ranges
    target_ranges = []
    for cls in class_names:
        target_ranges.extend(find_div_content_ranges(html, cls))

    def in_target(pos):
        return any(s <= pos < e for s, e in target_ranges)

    def get_parent_end(pos):
        for s, e in target_ranges:
            if s <= pos < e:
                return e
        return len(html)

    # Find all <p> tags; process only those inside target divs
    replacements = []
    for pm in re.finditer(r'<p\b[^>]*>', html):
        if not in_target(pm.start()):
            continue

        p_start = pm.end()
        parent_end = get_parent_end(pm.start())

        # Find </p> within parent bounds
        close = re.search(r'</p>', html[p_start:parent_end])
        if close:
            p_end = p_start + close.start()
        else:
            # Auto-closed <p> — content runs to parent div end
            p_end = parent_end

        old = html[p_start:p_end]
        new = reformat_p_content(old)

        if old != new:
            replacements.append((p_start, p_end, new))

    # Apply replacements in reverse order (so offsets stay valid)
    for start, end, new in sorted(replacements, key=lambda x: x[0], reverse=True):
        html = html[:start] + '\n' + new + '\n' + html[end:]

    with open(filepath, 'w') as f:
        f.write(html)

    print(f'{filepath}: {len(replacements)} paragraph(s) reformatted')


# ── Run ─────────────────────────────────────────────────────────────
process_html('index.html', ['section-text'])
process_html('articles/articles.html', ['section-text'])
process_html('articles/globe_article/globe.html', ['globe-text'])
