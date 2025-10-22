// Optimized Article Loader - Hash-based routing
// Works with a SINGLE template file for ALL articles

function loadArticle() {
  // Get article ID from URL hash (e.g., article.html#passion-job)
  let articleId = window.location.hash.substring(1); // Remove the '#'
  
  // Fallback to query parameter if no hash
  if (!articleId) {
    const urlParams = new URLSearchParams(window.location.search);
    articleId = urlParams.get('id') || urlParams.get('article');
  }
  
  // Default article if none specified
  if (!articleId) {
    showArticleList();
    return;
  }
  
  // Check if articlesData exists
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
  
  // Update page metadata
  updatePageMetadata(article);
  
  // Render article content
  renderArticle(article);
  
  // Generate table of contents
  generateTOC(article);
}

function updatePageMetadata(article) {
  // Update page title
  document.title = `${article.title} - Mehdi Bahlaoui`;
  
  // Update last edited date
  const lastEditedSpan = document.getElementById('lastEdited');
  if (lastEditedSpan && article.date) {
    lastEditedSpan.textContent = article.date;
  }
  
  // Update meta description (good for SEO)
  let metaDesc = document.querySelector('meta[name="description"]');
  if (!metaDesc) {
    metaDesc = document.createElement('meta');
    metaDesc.name = 'description';
    document.head.appendChild(metaDesc);
  }
  metaDesc.content = article.description || article.title;
}

function renderArticle(article) {
  const contentContainer = document.getElementById('articleContent');
  if (!contentContainer) {
    console.error('Article content container not found');
    return;
  }
  
  // Build article HTML
  let contentHTML = `
    <div class="section-row">
      <div class="section-text">
        <p class="section_title">${article.title}</p>
        <br>
      </div>
    </div>
  `;
  
  // Add each section
  if (article.sections && Array.isArray(article.sections)) {
    article.sections.forEach(section => {
      contentHTML += `
        <div class="section-row" id="${section.id}">
          <div class="section-text">
            ${section.title && section.title !== 'Introduction' 
              ? `<p class="section_title">${section.title}</p><br>` 
              : ''}
            <p>${section.content}</p>
          </div>
        </div>
      `;
    });
  }
  
  contentContainer.innerHTML = contentHTML;
  
  // Scroll to top when new article loads
  window.scrollTo(0, 0);
}

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
        <p><a href="article.html">View all articles</a></p>
      </div>
    </div>
  `;
}

// Listen for hash changes (when user clicks article links)
window.addEventListener('hashchange', loadArticle);

// Load article on page load
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', loadArticle);
} else {
  loadArticle();
}