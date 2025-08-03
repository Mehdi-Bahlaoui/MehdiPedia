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
        radio.addEventListener('change', function() {
            if (this.checked) {
                const theme = this.value;
                document.documentElement.setAttribute('data-theme', theme);
                localStorage.setItem('theme', theme);
            }
        });
    });

    const expandableItems = document.querySelectorAll(".toc-list li.expandable");

    expandableItems.forEach(item => {
      item.addEventListener("click", function (e) {
        // Prevent click from navigating if <a> is inside
        if (e.target.tagName.toLowerCase() === "a") return;

        item.classList.toggle("expanded");
        item.classList.toggle("collapsed");
      });
    });

    // Improved TOC highlighting logic
    const sections = document.querySelectorAll(".main-content .section-row");
    const tocLinks = document.querySelectorAll(".toc-list a");
    
    // Create a map of section IDs to their corresponding TOC links
    const sectionToTocMap = new Map();
    sections.forEach(section => {
      const sectionId = section.id || section.querySelector('.section_title')?.id;
      if (sectionId) {
        const tocLink = document.querySelector(`.toc-list a[href="#${sectionId}"]`);
        if (tocLink) {
          sectionToTocMap.set(sectionId, tocLink);
        }
      }
    });
    
    // Add the top section manually
    const overviewSection = document.getElementById('overview-early-life-section');
    if (overviewSection) {
      const topLink = document.querySelector('.toc-list a[href="#top"]');
      if (topLink) {
        sectionToTocMap.set('overview-early-life-section', topLink);
      }
    }

    // Add early life section manually (section title inside merged section)
    const earlyLifeTitle = document.getElementById('early-life');
    if (earlyLifeTitle) {
      const earlyLifeLink = document.querySelector('.toc-list a[href="#early-life"]');
      if (earlyLifeLink) {
        // Map the early life title to its TOC link, but we'll need to handle this specially
        sectionToTocMap.set('early-life', earlyLifeLink);
      }
    }

    // Add rap career section manually (section ID doesn't match TOC link)
    const rapCareerSection = document.getElementById('rap-career-section');
    if (rapCareerSection) {
      const rapCareerLink = document.querySelector('.toc-list a[href="#rap-career"]');
      if (rapCareerLink) {
        sectionToTocMap.set('rap-career-section', rapCareerLink);
      }
    }

    let currentActiveSection = null;
    let isScrolling = false;
    let scrollTimer;

    function updateActiveSection(newActiveSection) {
      if (newActiveSection !== currentActiveSection) {
        // Remove active class from all links
        tocLinks.forEach(link => {
          link.classList.remove("active-link");
        });
        
        // Collapse all expandable items first
        expandableItems.forEach(item => {
          item.classList.remove("expanded");
          item.classList.add("collapsed");
        });
        
        // Add active class to the new active section's link
        if (newActiveSection && sectionToTocMap.has(newActiveSection)) {
          const activeLink = sectionToTocMap.get(newActiveSection);
          activeLink.classList.add('active-link');
          
          // Check if the active link is inside a sublist (hidden under an expandable item)
          const parentSublist = activeLink.closest('.toc-sublist');
          if (parentSublist) {
            // Find the parent expandable item and expand it
            const parentExpandableItem = parentSublist.closest('li.expandable');
            if (parentExpandableItem) {
              parentExpandableItem.classList.add("expanded");
              parentExpandableItem.classList.remove("collapsed");
            }
          }
        }
        
        currentActiveSection = newActiveSection;
      }
    }

    function determineActiveSection() {
      const scrollTop = window.scrollY;
      let activeSection = 'overview-early-life-section';
      
      // If we're near the top of the page, always show the first section
      if (scrollTop < 100) {
        return 'overview-early-life-section';
      }
      
      // Check if we're in the early life portion of the merged section
      const earlyLifeTitle = document.getElementById('early-life');
      if (earlyLifeTitle) {
        const earlyLifeRect = earlyLifeTitle.getBoundingClientRect();
        // If the early life title is in the viewport area (with some tolerance)
        if (earlyLifeRect.top <= 300 && earlyLifeRect.bottom >= -50) {
          return 'early-life';
        }
      }
      
      // Find the section whose top is closest to the viewport center
      let closestSection = null;
      let closestDistance = Infinity;
      
      sections.forEach(section => {
        const rect = section.getBoundingClientRect();
        const sectionId = section.id || section.querySelector('.section_title')?.id;
        
        if (sectionId) {
          // Calculate distance from section top to a point 200px from viewport top
          const distance = Math.abs(rect.top - 200);
          
          // If section is visible and closer than previous closest
          if (rect.top <= 300 && rect.bottom >= 100 && distance < closestDistance) {
            closestDistance = distance;
            closestSection = sectionId;
          }
        }
      });
      
      return closestSection || activeSection;
    }

    const observer = new IntersectionObserver((entries) => {
      // Only process intersection changes if we're not actively scrolling
      if (isScrolling) return;
      
      const newActiveSection = determineActiveSection();
      updateActiveSection(newActiveSection);
    }, {
      root: null,
      rootMargin: "-10% 0px -70% 0px",
      threshold: [0, 0.25, 0.5, 0.75, 1.0]
    });

    // Observe all sections
    sections.forEach(section => {
      observer.observe(section);
    });

    // Handle scroll events with debouncing
    window.addEventListener('scroll', () => {
      isScrolling = true;
      
      clearTimeout(scrollTimer);
      scrollTimer = setTimeout(() => {
        isScrolling = false;
        
        // Update active section based on current scroll position
        const newActiveSection = determineActiveSection();
        updateActiveSection(newActiveSection);
      }, 50); // Increased debounce time for smoother transitions
    });

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
document.addEventListener('click', function(event) {
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

