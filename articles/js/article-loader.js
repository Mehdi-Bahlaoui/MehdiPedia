// Optimized Article Loader - Hash-based routing with image + PDF support
// Works with a SINGLE template file for ALL articles

function loadArticle() {
  let articleId = window.location.hash.substring(1);

  if (!articleId) {
    const urlParams = new URLSearchParams(window.location.search);
    articleId = urlParams.get('id') || urlParams.get('article');
  }

  if (!articleId) {
    showArticleList();
    return;
  }

  if (typeof articlesData === 'undefined') {
    console.error('articlesData not found. Make sure articles-data.js is loaded.');
    showError('Error loading articles database.');
    return;
  }

  const article = articlesData[articleId];

  if (!article) {
    showError(`Article "${articleId}" not found.`);
    return;
  }

  // Standalone page redirect — for articles with their own HTML file
  if (article.redirect) {
    window.location.replace(article.redirect);
    return;
  }

  updatePageMetadata(article);
  renderArticle(article);
  generateTOC(article);
  loadGiscus(articleId, article.title);
}

/* =========================
   GISCUS COMMENTS
========================= */

function getGiscusTheme() {
  const theme = document.documentElement.getAttribute('data-theme') || 'light';
  return theme === 'dark' ? 'dark_dimmed' : 'light';
}

function loadGiscus(articleId, articleTitle) {
  const contentContainer = document.getElementById('articleContent');
  if (!contentContainer) return;

  // Remove any existing giscus container so we can re-init per article
  const existing = document.getElementById('giscus-container');
  if (existing) existing.remove();

  const divider = document.createElement('div');
  divider.className = 'giscus-divider';
  contentContainer.appendChild(divider);

  const wrapper = document.createElement('div');
  wrapper.id = 'giscus-container';
  contentContainer.appendChild(wrapper);

  if (!document.getElementById('giscus-frame-style')) {
    const giscusStyle = document.createElement('style');
    giscusStyle.id = 'giscus-frame-style';
    giscusStyle.textContent = `
      .giscus, .giscus-frame {
        width: 100%;
      }
      .giscus-frame {
        min-height: 360px;
      }
      .giscus-divider {
        height: 1px;
        margin: 48px 0 0;
        border: 0;
        background: linear-gradient(
          to right,
          transparent 0%,
          var(--border-color, #c8ccd1) 20%,
          var(--border-color, #c8ccd1) 80%,
          transparent 100%
        );
      }
      #giscus-container {
        margin: 24px 0 20px;
      }
      @media (min-width: 851px) {
        #giscus-container {
          margin: 24px 0 0;
        }
      }
    `;
    document.head.appendChild(giscusStyle);
  }

  const script = document.createElement('script');
  script.src = 'https://giscus.app/client.js';
  script.async = true;
  script.crossOrigin = 'anonymous';
  script.setAttribute('data-repo', 'Mehdi-Bahlaoui/MehdiPedia');
  script.setAttribute('data-repo-id', 'R_kgDOPQ12eA');
  script.setAttribute('data-category', 'Announcements');
  script.setAttribute('data-category-id', 'DIC_kwDOPQ12eM4C8htD');
  script.setAttribute('data-mapping', 'specific');
  script.setAttribute('data-term', `article:${articleId}`);
  script.setAttribute('data-strict', '1');
  script.setAttribute('data-reactions-enabled', '0');
  script.setAttribute('data-emit-metadata', '0');
  script.setAttribute('data-input-position', 'bottom');
  script.setAttribute('data-theme', getGiscusTheme());
  script.setAttribute('data-lang', 'en');
  script.setAttribute('data-loading', 'lazy');

  wrapper.appendChild(script);
}

// Allow theme switcher to update the giscus iframe live
window.updateGiscusTheme = function () {
  const frame = document.querySelector('iframe.giscus-frame');
  if (!frame) return;
  frame.contentWindow.postMessage(
    { giscus: { setConfig: { theme: getGiscusTheme() } } },
    'https://giscus.app'
  );
};

function updatePageMetadata(article) {
  document.title = `${article.title} - Mehdi Bahlaoui`;

  const lastEditedSpan = document.getElementById('lastEdited');
  if (lastEditedSpan && article.date) {
    lastEditedSpan.textContent = article.date;
  }

  let metaDesc = document.querySelector('meta[name="description"]');
  if (!metaDesc) {
    metaDesc = document.createElement('meta');
    metaDesc.name = 'description';
    document.head.appendChild(metaDesc);
  }

  metaDesc.content = article.description || article.title;
}

/* =========================
   IMAGE RENDERING
========================= */

function renderImage(imageData) {
  if (!imageData || !imageData.url) return '';

  const position = imageData.position || 'center';
  const width = imageData.width || 'auto';
  const alt = imageData.alt || 'Article image';
  const caption = imageData.caption || '';

  let containerClass = 'image-container';
  let imageStyle = '';

  switch (position) {
    case 'left':
      containerClass += ' image-left';
      imageStyle = width !== 'auto' ? `max-width: ${width};` : 'max-width: 400px;';
      break;
    case 'right':
      containerClass += ' image-right';
      imageStyle = width !== 'auto' ? `max-width: ${width};` : 'max-width: 400px;';
      break;
    case 'full':
      containerClass += ' image-full';
      imageStyle = 'width: 100%;';
      break;
    default:
      containerClass += ' image-center';
      imageStyle = width !== 'auto' ? `max-width: ${width};` : 'max-width: 600px;';
  }

  return `
    <div class="${containerClass}">
      <img src="${imageData.url}" 
           alt="${alt}" 
           class="article-image"
           style="${imageStyle}"
           loading="lazy">
      ${caption ? `<p class="image-caption">${caption}</p>` : ''}
    </div>
  `;
}

/* =========================
   PDF RENDERING
========================= */


function renderPDF(pdfData) {
  if (!pdfData || !pdfData.url) return '';

  const containerId = 'pdf-render-' + Math.random().toString(36).substring(7);

  // Wait for container to be in the DOM and have a computed width
  function waitForContainer(callback, attempts) {
    attempts = attempts || 0;
    const container = document.getElementById(containerId);
    if (container && container.clientWidth > 0) {
      callback(container);
    } else if (attempts < 50) {
      requestAnimationFrame(() => waitForContainer(callback, attempts + 1));
    }
  }

  waitForContainer(function (container) {
    const pdfjsLib = window['pdfjs-dist/build/pdf'];
    pdfjsLib.GlobalWorkerOptions.workerSrc =
      'https://cdnjs.cloudflare.com/ajax/libs/pdf.js/3.11.174/pdf.worker.min.js';

    pdfjsLib.getDocument(pdfData.url).promise.then(pdf => {

      const containerWidth = container.clientWidth;

      for (let pageNum = 1; pageNum <= pdf.numPages; pageNum++) {
        pdf.getPage(pageNum).then(page => {
          // Get natural PDF page size at scale 1
          const unscaledViewport = page.getViewport({ scale: 1 });

          // Fit page to container width
          const fitScale = (containerWidth) / unscaledViewport.width;

          const qualityMultiplier = 2; // your high quality factor

          const finalScale = fitScale * qualityMultiplier;

          const viewport = page.getViewport({ scale: finalScale });

          const outputScale = Math.min(window.devicePixelRatio || 1, 2);

          const canvas = document.createElement('canvas');
          const context = canvas.getContext('2d');

          // High-resolution internal pixels
          canvas.width = Math.floor(viewport.width * outputScale);
          canvas.height = Math.floor(viewport.height * outputScale);

          canvas.classList.add('pdf-canvas');

          container.appendChild(canvas);

          const transform = outputScale !== 1
            ? [outputScale, 0, 0, outputScale, 0, 0]
            : null;

          page.render({
            canvasContext: context,
            transform: transform,
            viewport: viewport
          });

        });

      }
    });
  });

  return `<div id="${containerId}" class="pdf-container"></div>`;
}


/* =========================
   MEDIA RENDERING (Images + PDF)
========================= */

function renderMedia(section) {
  let mediaHTML = '';

  if (section.image) {
    mediaHTML += renderImage(section.image);
  }

  if (section.images && Array.isArray(section.images)) {
    mediaHTML += section.images.map(img => renderImage(img)).join('');
  }

  if (section.pdf) {
    mediaHTML += renderPDF(section.pdf);
  }

  return mediaHTML;
}

function renderArticle(article) {
  const contentContainer = document.getElementById('articleContent');
  if (!contentContainer) {
    console.error('Article content container not found');
    return;
  }

  injectStyles();

  let contentHTML = `
    <div class="section-row">
      <div class="section-text">
        <p class="section_title">${article.title}</p>
        <br>
      </div>
    </div>
  `;

  if (article.sections && Array.isArray(article.sections)) {
    article.sections.forEach(section => {
      const mediaHTML = renderMedia(section);

      contentHTML += `
        <div class="section-row" id="${section.id}">
          <div class="section-text">
            ${section.title && section.title !== 'Introduction'
          ? `<p class="section_title">${section.title}</p><br>`
          : ''}
            ${mediaHTML}
            <p>${section.content}</p>
          </div>
        </div>
      `;
    });
  }

  contentContainer.innerHTML = contentHTML;
  window.scrollTo(0, 0);
}

/* =========================
   STYLES
========================= */

function injectStyles() {
  if (document.getElementById('article-media-styles')) return;

  const styleSheet = document.createElement('style');
  styleSheet.id = 'article-media-styles';
  styleSheet.textContent = `
    .image-container {
      margin: 20px 0;
    }

    .image-center img {
      display: block;
      margin: 0 auto;
      max-width: 100%;
      height: auto;
    }

    .image-left {
      float: left;
      margin: 0 20px 20px 0;
      max-width: 45%;
    }

    .image-right {
      float: right;
      margin: 0 0 20px 20px;
      max-width: 45%;
    }

    .image-full img {
      width: 100%;
      height: auto;
    }

    .article-image {
      border-radius: 8px;
      box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    }

    .image-caption {
      margin-top: 8px;
      font-size: 0.9em;
      color: #666;
      font-style: italic;
      text-align: center;
    }

    .pdf-container {
      margin: -20px 0;
      width: 100%;
      min-height: 200px;
    }

    .pdf-canvas {
      width: 90%;
      height: auto;
      display: block;
      margin: 0 auto 0 -5px;
      padding: 0;
    }

    .section-row::after {
      content: "";
      display: table;
      clear: both;
    }

    @media (max-width: 1200px) {
      .pdf-container {
        margin: 0;
        width: 100%;
        overflow: hidden; /* Prevents the page from expanding horizontally */
      }
      .pdf-canvas {
        width: 110%;
        max-width: none;
        margin: 0 0 0 -5%; /* Trims 5% of white space on both sides */
      }
    }

    @media (max-width: 768px) {
      .pdf-canvas {
        width: 110%;
        margin: 0 -5% 0 -5%; /* Trims 10% on mobile to make text even larger */
      }
      .image-left,
      .image-right {
        float: none;
        max-width: 100%;
        margin: 0px 0;
      }
    }
  `;

  document.head.appendChild(styleSheet);
}

/* =========================
   TOC + NAVIGATION
========================= */

function generateTOC(article) {
  const tocList = document.getElementById('tocList');
  if (!tocList) return;

  let tocHTML = '<li><a href="#top">(Top)</a></li>';

  if (article.sections && Array.isArray(article.sections)) {
    article.sections.forEach(section => {
      if (section.title && section.id) {
        tocHTML += `<li><a href="#${section.id}" onclick="scrollToSection('${section.id}'); return false;">${section.title}</a></li>`;
      }
    });
  }

  tocList.innerHTML = tocHTML;
}

function scrollToSection(sectionId) {
  const element = document.getElementById(sectionId);
  if (element) {
    element.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }
}

function showArticleList() {
  const contentContainer = document.getElementById('articleContent');
  if (!contentContainer || typeof articlesData === 'undefined') return;

  let listHTML = `
    <div class="section-row">
      <div class="section-text">
        <p class="section_title">Available Articles</p>
        <br>
        <ul style="list-style: none; padding: 0;">
  `;

  Object.keys(articlesData).forEach(id => {
    const article = articlesData[id];
    listHTML += `
      <li style="margin-bottom: 20px;">
        <a href="#${id}" style="text-decoration: none;">
          <strong>${article.title}</strong>
        </a>
        <br>
        <small>${article.date || 'No date'}</small>
      </li>
    `;
  });

  listHTML += `
        </ul>
      </div>
    </div>
  `;

  contentContainer.innerHTML = listHTML;
}

function showError(message) {
  const contentContainer = document.getElementById('articleContent');
  if (!contentContainer) return;

  contentContainer.innerHTML = `
    <div class="section-row">
      <div class="section-text">
        <p class="section_title">Error</p>
        <p>${message}</p>
        <p><a href="../articles/articles.html">View all articles</a></p>
      </div>
    </div>
  `;
}

/* =========================
   EVENT LISTENERS
========================= */

window.addEventListener('hashchange', loadArticle);

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', loadArticle);
} else {
  loadArticle();
}