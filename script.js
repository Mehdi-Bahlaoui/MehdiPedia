document.addEventListener("DOMContentLoaded", function () {
  // Theme switching functionality
  const colorRadios = document.querySelectorAll('input[name="color"]');

  // Load saved theme or default to light
  const savedTheme = localStorage.getItem('theme') || 'light';
  document.documentElement.setAttribute('data-theme', savedTheme);

  // Set the correct radio button based on saved theme
  const themeRadio = document.querySelector(`input[name="color"][value="${savedTheme}"]`);
  if (themeRadio) {
    themeRadio.checked = true;
  }

  // Add event listeners to theme radio buttons
  colorRadios.forEach(radio => {
    radio.addEventListener('change', function () {
      if (this.checked) {
        const theme = this.value;
        document.documentElement.setAttribute('data-theme', theme);
        localStorage.setItem('theme', theme);
      }
    });
  });

  // Text size switching functionality
  const textSizeRadios = document.querySelectorAll('input[name="text-size"]');

  // Load saved text size or default to standard
  const savedTextSize = localStorage.getItem('textSize') || 'standard';
  document.documentElement.setAttribute('data-text-size', savedTextSize);

  // Set the correct radio button based on saved text size
  const textSizeRadio = document.querySelector(`input[name="text-size"][value="${savedTextSize}"]`);
  if (textSizeRadio) {
    textSizeRadio.checked = true;
  }

  // Add event listeners to text size radio buttons
  textSizeRadios.forEach(radio => {
    radio.addEventListener('change', function () {
      if (this.checked) {
        const textSize = this.value;
        document.documentElement.setAttribute('data-text-size', textSize);
        localStorage.setItem('textSize', textSize);
      }
    });
  });

  // Initialize mobile sections as collapsed on load
  if (window.innerWidth <= 850) {
    const sectionContents = document.querySelectorAll('.section-content');
    const toggles = document.querySelectorAll('.section-toggle');

    sectionContents.forEach(content => {
      content.classList.remove('expanded');
    });

    toggles.forEach(toggle => {
      toggle.classList.remove('expanded');
    });
  }
});

// Handle section clicks for mobile toggling - MOVED OUTSIDE DOMContentLoaded for persistence
document.addEventListener('click', function (e) {
  // Only work on mobile
  if (window.innerWidth > 850) return;

  // Check if click is on a section header, title, or toggle button
  const sectionHeader = e.target.closest('.section-header');
  const sectionTitle = e.target.closest('.section_title');
  const sectionToggle = e.target.closest('.section-toggle');

  if (sectionHeader || sectionTitle || sectionToggle) {
    e.preventDefault();
    e.stopPropagation();

    // Find the collapsible section
    const section = e.target.closest('.collapsible-section');
    if (section) {
      const sectionId = section.id.replace('-section', '');
      toggleSection(sectionId);
    }
  }
});

// Toggle TOC visibility function
function toggleToc(button) {
  const toc = button.closest('.toc');
  const tocList = toc.querySelector('.toc-list');

  if (tocList.style.display === 'none') {
    tocList.style.display = 'block';
    button.textContent = 'hide';
  } else {
    tocList.style.display = 'none';
    button.textContent = 'show';
  }
}

// Languages dropdown functionality
function toggleLanguagesDropdown() {
  const dropdown = document.querySelector('.languages-dropdown');
  dropdown.classList.toggle('show');
}

// Close dropdown when clicking outside
document.addEventListener('click', function (event) {
  const dropdown = document.querySelector('.languages-dropdown');
  if (!dropdown.contains(event.target)) {
    dropdown.classList.remove('show');
  }
});

// Play Arabic version audio
function playArabicVersion() {
  // Create audio element and play the Arabic version
  const audio = new Audio('sfx/dajajatan.mp3');
  audio.play().catch(error => {
    console.log('Audio playback failed:', error);
    alert('Sorry, the Arabic version audio could not be played.');
  });

  // Close the dropdown after selection
  document.querySelector('.languages-dropdown').classList.remove('show');
}

// Mobile section toggle functionality
function toggleSection(sectionId) {
  // Only function on mobile screens
  if (window.innerWidth > 850) return;

  const content = document.getElementById(sectionId + '-content');
  const toggle = document.querySelector(`#${sectionId}-section .section-toggle`);

  if (content && toggle) {
    content.classList.toggle('expanded');
    toggle.classList.toggle('expanded');
  }
}