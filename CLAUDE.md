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

## Important Conventions

- **No build tools**: Direct file editing, no transpilation or bundling
- **Wikipedia aesthetic**: Maintain the Wikipedia visual style (fonts, colors, layout)
- **Link styling**: Use class `links` for Wikipedia-style blue links
- **Personal voice**: The content uses a personal, sometimes philosophical writing style
- **Image styling**: Images use `.profile-image` class and are wrapped in `.image_container` with captions
- **Git workflow**: Commit messages are casual and descriptive (see git history for examples)

## Deployment

The site is deployed via GitHub Pages from the `main` branch. Any push to `main` automatically updates the live site at the domain specified in `CNAME`.
