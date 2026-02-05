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

  // Width switching functionality
  const widthRadios = document.querySelectorAll('input[name="width"]');

  // Load saved width or default to standard
  const savedWidth = localStorage.getItem('width') || 'standard';
  document.documentElement.setAttribute('data-width', savedWidth);

  // Set the correct radio button based on saved width
  const widthRadio = document.querySelector(`input[name="width"][value="${savedWidth}"]`);
  if (widthRadio) {
    widthRadio.checked = true;
  }

  // Add event listeners to width radio buttons
  widthRadios.forEach(radio => {
    radio.addEventListener('change', function () {
      if (this.checked) {
        const width = this.value;
        document.documentElement.setAttribute('data-width', width);
        localStorage.setItem('width', width);
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

// Music Paywall Modal Functions
function showMusicPaywall(event) {
  event.preventDefault();
  const modal = document.getElementById('musicPaywallModal');
  modal.style.display = 'flex';
  // Reset to stage 1
  resetMusicPaywall();
}

function closeMusicPaywall() {
  const modal = document.getElementById('musicPaywallModal');
  modal.style.display = 'none';
  resetMusicPaywall();
}

function resetMusicPaywall() {
  document.getElementById('musicModalText').textContent =
    "Access to Mehdi's exclusive music collection requires payment.";
  document.getElementById('musicBtn1').style.display = 'inline-block';
  document.getElementById('musicBtn2').style.display = 'none';
  document.getElementById('musicBtn3').style.display = 'none';
  document.getElementById('musicBtn4').style.display = 'none';
}

function handleMusicPaywall(stage) {
  const modalText = document.getElementById('musicModalText');
  const btn1 = document.getElementById('musicBtn1');
  const btn2 = document.getElementById('musicBtn2');
  const btn3 = document.getElementById('musicBtn3');
  const btn4 = document.getElementById('musicBtn4');

  if (stage === 1) {
    modalText.textContent = "Okay okay, maybe that's too much. How about a discount?";
    btn1.style.display = 'none';
    btn2.style.display = 'inline-block';
  } else if (stage === 2) {
    modalText.textContent = "Still too expensive? Fine, final offer!";
    btn2.style.display = 'none';
    btn3.style.display = 'inline-block';
  } else if (stage === 3) {
    modalText.textContent = "You know what... just come in. But tell your friends about this site!";
    btn3.style.display = 'none';
    btn4.style.display = 'inline-block';
  } else if (stage === 4) {
    // Actually redirect or show music content
    alert('Music section coming soon! \n\nWe are actively working on uploading his tracks. Check back later!');
    closeMusicPaywall();
  }
}

// Close modal when clicking outside
window.addEventListener('click', function(event) {
  const modal = document.getElementById('musicPaywallModal');
  if (event.target === modal) {
    closeMusicPaywall();
  }
});