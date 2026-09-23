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

  // Article titles/descriptions may contain HTML - strip it for meta tags
  const plainTitle = article.title.replace(/<[^>]*>/g, '');
  metaDesc.content = (article.description || plainTitle).replace(/<[^>]*>/g, '');

  // Keep social/share tags in sync with the loaded article
  const setProperty = (prop, value) => {
    const el = document.querySelector(`meta[property="${prop}"]`);
    if (el) el.content = value;
  };
  setProperty('og:title', `${plainTitle} - Mehdi Bahlaoui`);
  setProperty('og:description', metaDesc.content);
  setProperty('og:url', `https://mehdibahlaoui.com/articles/article.html${window.location.hash}`);
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
      if (section.audio) {
        contentHTML += `<div class="section-row"><div class="section-text">${renderAudio(section.audio)}</div></div>`;
        return;
      }

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
  initVoiceovers(contentContainer);
  window.scrollTo(0, 0);
}

/* =========================
   AUDIO RENDERING (Voiceover player)
   Vanilla replica of Substack's article voiceover player.
   Usage in articles-data.js: { audio: { url: '...', title: '...' } }
========================= */

const VOICEOVER_RATES = [1, 1.25, 1.5, 1.75, 2];

function renderAudio(audioData) {
  if (!audioData || !audioData.url) return '';

  const title = audioData.title || 'Article voiceover';

  const rateItems = VOICEOVER_RATES.map(rate => `
    <button type="button" role="menuitemradio" data-rate="${rate}" aria-checked="${rate === 1}">
      <span>${rate}×</span>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M20 6 9 17l-5-5"/></svg>
    </button>
  `).join('');

  return `
    <div class="voiceover">
      <div class="voiceover-icon" aria-hidden="true">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M11.8518 3.31518C12.2459 4.26329 13.551 7.10985 14.4542 8.3467C15.3764 9.60967 12.8285 9.95184 12 10.5"/>
          <path d="M12 10.5C12 10.5 12.4662 11.2332 12.7627 11.7326C13.0592 12.232 13 12.5 11.5 13C10.7772 13.2409 9.5 13.5 9.5 13.5C10.3447 13.9272 12.2664 14.1393 12.4041 14.3167C12.763 14.779 12.3241 15.4618 11.8617 15.8208C11.3994 16.1797 12.0724 18.7353 10.5 19.5C8.9276 20.2647 5.70772 19.432 3 18"/>
          <path d="M18.5 6.5L20.3544 4.8526"/>
          <path d="M18.5 15.5L20.3544 17.2545"/>
          <path d="M19.5 11.0326L22 11.0326"/>
        </svg>
      </div>

      <div class="voiceover-player" role="region" aria-label="Voiceover player" data-state="paused">
        <button type="button" class="voiceover-play" aria-label="Play">
          <svg class="voiceover-play-icon" width="20" height="20" viewBox="0 0 20 20" aria-hidden="true"><path d="M5.04688 18.5527C5.4375 18.5527 5.76953 18.3965 6.16016 18.1719L17.5469 11.5898C18.3574 11.1113 18.6406 10.7988 18.6406 10.2812C18.6406 9.76367 18.3574 9.45117 17.5469 8.98242L6.16016 2.39063C5.76953 2.16602 5.4375 2.01953 5.04688 2.01953C4.32422 2.01953 3.875 2.56641 3.875 3.41602V17.1465C3.875 17.9961 4.32422 18.5527 5.04688 18.5527Z"/></svg>
          <svg class="voiceover-pause-icon" width="20" height="20" viewBox="0 0 20 20" aria-hidden="true"><path d="M5.29883 17.9082H7.52539C8.375 17.9082 8.82422 17.459 8.82422 16.5996V3.29883C8.82422 2.41016 8.375 2 7.52539 2H5.29883C4.44922 2 4 2.44922 4 3.29883V16.5996C4 17.459 4.44922 17.9082 5.29883 17.9082ZM12.3984 17.9082H14.6152C15.4746 17.9082 15.9141 17.459 15.9141 16.5996V3.29883C15.9141 2.41016 15.4746 2 14.6152 2H12.3984C11.5391 2 11.0898 2.44922 11.0898 3.29883V16.5996C11.0898 17.459 11.5391 17.9082 12.3984 17.9082Z"/></svg>
        </button>

        <div class="voiceover-body">
          <div class="voiceover-title">${title}</div>
          <div class="voiceover-progress">
            <span class="voiceover-time">0:00</span>
            <div class="voiceover-track" style="--progress: 0">
              <div class="voiceover-bar"><div class="voiceover-fill"></div></div>
              <div class="voiceover-playhead"></div>
            </div>
            <span class="voiceover-time voiceover-remaining">-0:00</span>
          </div>
        </div>

        <div class="voiceover-speed">
          <button type="button" class="voiceover-speed-btn" aria-label="Playback speed" aria-haspopup="menu" aria-expanded="false">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="m12 14 4-4"/><path d="M3.34 19a10 10 0 1 1 17.32 0"/></svg>
          </button>
          <div class="voiceover-menu" role="menu" hidden>
            <div class="voiceover-menu-label">Playback speed</div>
            ${rateItems}
          </div>
        </div>

        <audio src="${audioData.url}" preload="metadata"></audio>
      </div>
    </div>
  `.trim();
}

function setVoiceoverMenu(speed, open) {
  speed.querySelector('.voiceover-menu').hidden = !open;
  speed.querySelector('.voiceover-speed-btn').setAttribute('aria-expanded', open);
}

function initVoiceovers(root) {
  root.querySelectorAll('.voiceover-player').forEach(player => {
    const audio = player.querySelector('audio');
    const playBtn = player.querySelector('.voiceover-play');
    const track = player.querySelector('.voiceover-track');
    const [elapsed, remaining] = player.querySelectorAll('.voiceover-time');
    const speed = player.querySelector('.voiceover-speed');

    const syncState = () => {
      const playing = !audio.paused;
      player.dataset.state = playing ? 'playing' : 'paused';
      playBtn.setAttribute('aria-label', playing ? 'Pause' : 'Play');
    };

    const syncTime = () => {
      const duration = audio.duration || 0;
      const current = audio.currentTime;
      track.style.setProperty('--progress', duration ? current / duration : 0);
      elapsed.textContent = formatTime(current);
      remaining.textContent = `-${formatTime(duration - current)}`;
    };

    const seek = (e) => {
      if (!audio.duration) return;
      const rect = track.getBoundingClientRect();
      const ratio = Math.min(Math.max((e.clientX - rect.left) / rect.width, 0), 1);
      audio.currentTime = ratio * audio.duration;
      syncTime();
    };

    playBtn.addEventListener('click', () => (audio.paused ? audio.play() : audio.pause()));
    ['play', 'pause', 'ended'].forEach(type => audio.addEventListener(type, syncState));
    ['loadedmetadata', 'timeupdate'].forEach(type => audio.addEventListener(type, syncTime));

    // Pointer events cover mouse + touch; capture keeps the drag alive outside the track
    track.addEventListener('pointerdown', (e) => {
      track.setPointerCapture(e.pointerId);
      seek(e);
    });
    track.addEventListener('pointermove', (e) => {
      if (track.hasPointerCapture(e.pointerId)) seek(e);
    });

    speed.querySelector('.voiceover-speed-btn').addEventListener('click', () => {
      setVoiceoverMenu(speed, speed.querySelector('.voiceover-menu').hidden);
    });

    speed.querySelector('.voiceover-menu').addEventListener('click', (e) => {
      const item = e.target.closest('[data-rate]');
      if (!item) return;
      audio.playbackRate = Number(item.dataset.rate);
      speed.querySelectorAll('[data-rate]').forEach(el => el.setAttribute('aria-checked', el === item));
      setVoiceoverMenu(speed, false);
    });
  });
}

// Close any open speed menu when clicking elsewhere (registered once)
document.addEventListener('click', (e) => {
  document.querySelectorAll('.voiceover-speed').forEach(speed => {
    if (!speed.contains(e.target)) setVoiceoverMenu(speed, false);
  });
});

// Format seconds as M:SS (same helper as assets/js/music-player.js)
function formatTime(seconds) {
  if (!isFinite(seconds) || seconds < 0) return '0:00';
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
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
      /* Override inline max-width set in renderImage so wide images never overflow on mobile */
      .article-image {
        max-width: 100% !important;
        height: auto;
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
        <p><a href="/articles/">View all articles</a></p>
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