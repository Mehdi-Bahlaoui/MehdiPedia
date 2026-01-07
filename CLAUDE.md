# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal portfolio website styled as a Wikipedia page, showcasing Mehdi Bahlaoui's biography, articles, and creative work. The site is a static HTML/CSS/JavaScript project with no build system, designed to be deployed directly to GitHub Pages.

## Development Commands

Since this is a static website with no build system:

- **Run locally**: Open `index.html` directly in a browser, or use a simple HTTP server:
  ```bash
  python -m http.server 8000
  # OR
  npx serve .
  ```
- **Deploy**: Push to the `main` branch - the site is hosted via GitHub Pages from the repository root
- **No build/lint/test commands**: This project uses vanilla HTML/CSS/JS with no compilation or testing frameworks

## Architecture Overview

### Core Structure

The site follows a Wikipedia-like layout with three main pages:
- `index.html` - Main biography page (Wikipedia-style profile)
- `articles.html` - Article listing page
- Individual article pages loaded via `articles/template.html` with hash-based routing

### Key Technical Patterns

**1. Dynamic Article System**
- Articles are defined as JavaScript objects in `articles/js/articles-data.js`
- A single template file (`articles/template.html`) renders all articles dynamically
- Hash-based routing (e.g., `template.html#job`) loads specific articles
- Article loader (`articles/js/article-loader.js`) handles rendering, TOC generation, and metadata

**2. Theme System**
The site supports dynamic theming via CSS variables and `data-*` attributes:
- Theme settings: `data-theme` (light/dark)
- Text size: `data-text-size` (small/standard/large)
- Layout width: `data-width` (standard/wide)
- Settings persisted to localStorage
- CSS variables defined in `:root` and `[data-theme="dark"]` selectors in `styles.css`

**3. Mobile-First Collapsible Sections**
- Desktop: All sections expanded by default
- Mobile (≤850px): Sections collapsed by default with toggle arrows
- Toggle behavior handled in `script.js` via `.collapsible-section` and `.section-toggle` classes

**4. Component Structure**
The main page follows this layout hierarchy:
```
<header> - Wikipedia-style header with logo, search, navigation
<body>
  <div class="page-container">
    <div class="sidebar"> - Left TOC (table of contents)
    <div class="main-content"> - Main biography content
      <div class="section-row"> - Two-column layout (text + images)
      <div class="collapsible-section"> - Expandable sections
    <div class="sidebar"> - Right sidebar (Appearance settings)
<footer> - Wikipedia-style footer
```

### File Organization

```
/
├── index.html              # Main biography page
├── articles.html           # Article listing page
├── styles.css              # All CSS (includes theme variables)
├── script.js               # Theme switching, TOC toggle, mobile sections
├── articles/
│   ├── template.html       # Single template for all articles
│   └── js/
│       ├── articles-data.js    # Article content database
│       └── article-loader.js   # Dynamic article rendering
├── Images/                 # Biography images
├── icons/                  # UI icons
└── sfx/                    # Audio files (e.g., Arabic summary)
```

## Development Guidelines

### Adding New Articles

1. Add article data to `articles/js/articles-data.js`:
   ```javascript
   'article-id': {
     title: 'Article Title',
     date: 'MM/DD/YYYY',
     sections: [
       {
         id: 'section-id',
         title: 'Section Title',
         content: `HTML content here`,
         image: {
           url: '../Images/image.jpg',
           alt: 'Description',
           caption: 'Image caption',
           position: 'center' // or 'left', 'right', 'full'
         }
       }
     ]
   }
   ```
2. Link to it using: `template.html#article-id`
3. No need to create individual HTML files - the template handles all articles

### Modifying Biography Content

- Biography sections in `index.html` use `.collapsible-section` for mobile toggles
- Each section needs:
  - `id="section-name-section"` on the container
  - `.section-header` with `data-section="section-name"`
  - `.section-content` with `id="section-name-content"`
  - `.section-toggle` span for the arrow indicator

### Theme/Style Changes

- Modify CSS variables in `styles.css` under `:root` (light) or `[data-theme="dark"]` (dark)
- Key variables: `--text-color`, `--background-color`, `--links-color`, `--border-color`
- The `--svg-filter` variable in dark mode inverts SVG icons for better visibility

### Mobile Responsiveness

- Breakpoint: 850px (`@media (max-width: 850px)`)
- Mobile-specific logic in `script.js` checks `window.innerWidth <= 850`
- Sections auto-collapse on mobile via `DOMContentLoaded` event

### Maintaining Shared Headers and Footers

Due to the static nature of this project (no build system), headers and footers are duplicated across three files to ensure instant page rendering without JavaScript loading flash. Follow this workflow to maintain consistency:

#### Header Locations
- `index.html` - Lines 32-97 (between sync markers)
- `articles.html` - Lines 23-92 (between sync markers)
- `articles/template.html` - Lines 25-96 (between sync markers)

#### Footer Locations
- `index.html` - Lines 647-711 (between sync markers)
- `articles.html` - Lines 314-356 (between sync markers)
- `articles/template.html` - Lines 214-256 (between sync markers)

#### Variation Points

When syncing headers, preserve these intentional differences:

**1. Logo Link (`<a class="logo-link">` in `.logo-area`)**
- `index.html`: **No `href` attribute** (stays on current page)
- `articles.html`: `href="index.html"`
- `articles/template.html`: `href="../index.html"` (relative path from articles/ directory)

**2. Left Navigation Menu (`<nav class="left-nav">`)**
- `index.html`: Simple structure - only hamburger button, **omits** overlay and links container
- `articles.html` & `template.html`: Full navigation including:
  - `<label id="overlay-left" for="sidebar-active-left"></label>`
  - `<div class="links-container-left">` with navigation links

**3. Right Navigation Menu (`<nav>` in `.right-header`)**
- `index.html`: Entire `<nav>` block is **commented out**
- `articles.html`: Active but **without** `<div class="links-container-right">`
- `template.html`: Active **with** empty `<div class="links-container-right">`

#### Sync Workflow

When updating headers or footers (e.g., changing search placeholder, modifying SVG icons, updating footer links):

1. **Identify change scope**: Is it universal or specific to one variation?

2. **Make changes in canonical file** (recommend `articles.html` as it has the most complete structure)

3. **Copy to other files** and apply variations:
   - Select the content between sync markers (look for `<!-- ========== SHARED HEADER START ==========`)
   - Copy to the corresponding file
   - Apply file-specific variations using the matrix above
   - Check inline `<!-- VARIATION: ... -->` comments for guidance

4. **Verify variation points** using this checklist:
   - [ ] index.html: Logo has NO href attribute
   - [ ] index.html: Left nav omits overlay-left and links-container-left
   - [ ] index.html: Right nav is commented out
   - [ ] articles.html: Logo `href="index.html"`
   - [ ] articles.html: Left nav includes overlay + links container
   - [ ] articles.html: Right nav is active without links-container-right
   - [ ] template.html: Logo `href="../index.html"`
   - [ ] template.html: Left nav includes overlay + links container
   - [ ] template.html: Right nav is active with empty links-container-right

5. **Update sync marker dates** in all three files:
   ```html
   <!-- Last updated: 2026-01-07 -->
   ```

6. **Test all pages** to ensure no visual regressions

#### Footer Variations

Footer content differs primarily in:
- Last edited dates
- Disclaimer text length
- Available links

Follow the same sync workflow but pay attention to content-specific differences.

#### Quick Tips

- Sync markers make it easy to select the entire header/footer for copying
- Inline variation comments guide you to critical differences
- Use a diff tool to compare headers if unsure what changed
- When in doubt, check the variation matrix above

## Important Conventions

- **No build tools**: Direct file editing, no transpilation or bundling
- **Wikipedia aesthetic**: Maintain the Wikipedia visual style (fonts, colors, layout)
- **Link styling**: Use class `links` for Wikipedia-style blue links
- **Personal voice**: The content uses a personal, sometimes philosophical writing style
- **Image styling**: Images use `.profile-image` class and are wrapped in `.image_container` with captions
- **Git workflow**: Commit messages are casual and descriptive (see git history for examples)

## Deployment

The site is deployed via GitHub Pages from the `main` branch. Any push to `main` automatically updates the live site at the domain specified in `CNAME`.
