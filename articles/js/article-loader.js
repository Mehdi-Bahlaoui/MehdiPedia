// Optimized Article Loader - Hash-based routing with image support
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

function renderImage(imageData) {
  if (!imageData || !imageData.url) return '';
  
  const position = imageData.position || 'center';
  const width = imageData.width || 'auto';
  const alt = imageData.alt || 'Article image';
  const caption = imageData.caption || '';
  
  let imageClass = 'article-image';
  let containerClass = 'image-container';
  let imageStyle = '';
  
  // Handle different positions
  switch(position) {
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
    case 'center':
    default:
      containerClass += ' image-center';
      imageStyle = width !== 'auto' ? `max-width: ${width};` : 'max-width: 600px;';
  }
  
  return `
    <div class="${containerClass}">
      <img src="${imageData.url}" 
           alt="${alt}" 
           class="${imageClass}"
           style="${imageStyle}"
           loading="lazy">
      ${caption ? `<p class="image-caption">${caption}</p>` : ''}
    </div>
  `;
}

function renderImages(section) {
  let imagesHTML = '';
  
  // Handle single image
  if (section.image) {
    imagesHTML = renderImage(section.image);
  }
  
  // Handle multiple images
  if (section.images && Array.isArray(section.images)) {
    const multiImageContainer = section.images.length > 1 ? 
      '<div class="multi-image-container">' : '';
    const multiImageClose = section.images.length > 1 ? '</div>' : '';
    
    imagesHTML = multiImageContainer + 
                 section.images.map(img => renderImage(img)).join('') + 
                 multiImageClose;
  }
  
  return imagesHTML;
}

function renderArticle(article) {
  const contentContainer = document.getElementById('articleContent');
  if (!contentContainer) {
    console.error('Article content container not found');
    return;
  }
  
  // Add image styles if not already present
  injectImageStyles();
  
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
      const imagesHTML = renderImages(section);
      
      contentHTML += `
        <div class="section-row" id="${section.id}">
          <div class="section-text">
            ${section.title && section.title !== 'Introduction' 
              ? `<p class="section_title">${section.title}</p><br>` 
              : ''}
            ${imagesHTML}
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

function injectImageStyles() {
  // Check if styles already injected
  if (document.getElementById('article-image-styles')) return;
  
  const styleSheet = document.createElement('style');
  styleSheet.id = 'article-image-styles';
  styleSheet.textContent = `
    .image-container {
      margin: 20px 0;
    }
    
    .image-center {
      text-align: center;
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
    
    .image-left img {
      width: 100%;
      height: auto;
    }
    
    .image-right {
      float: right;
      margin: 0 0 20px 20px;
      max-width: 45%;
    }
    
    .image-right img {
      width: 100%;
      height: auto;
    }
    
    .image-full {
      clear: both;
      margin: 30px 0;
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
    
    .multi-image-container {
      display: flex;
      flex-wrap: wrap;
      gap: 20px;
      justify-content: center;
      margin: 20px 0;
    }
    
    .multi-image-container .image-container {
      flex: 1;
      min-width: 250px;
      max-width: 45%;
    }
    
    /* Clear floats after sections */
    .section-row::after {
      content: "";
      display: table;
      clear: both;
    }
    
    @media (max-width: 768px) {
      .image-left,
      .image-right {
        float: none;
        max-width: 100%;
        margin: 20px 0;
      }
      
      .multi-image-container .image-container {
        max-width: 100%;
      }
    }
  `;
  
  document.head.appendChild(styleSheet);
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