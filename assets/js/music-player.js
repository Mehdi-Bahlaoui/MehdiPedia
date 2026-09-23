// Genius-Style Music Player
// All-in-one file: Loading + Rendering + Playback + Sync Engine

// ============================================
// SECTION 1: Song Loading (Hash-Based Routing)
// ============================================

let currentSong = null;
let currentLyricsArray = [];
let lastActiveLineId = null;

// Monochrome speaker icons (flat MediaWiki-style glyphs, replace the old emoji)
const VOLUME_ICON_PATHS = {
  high: 'M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z',
  low: 'M18.5 12c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM5 9v6h4l5 5V4L9 9H5z',
  muted: 'M16.5 12c0-1.77-1.02-3.29-2.5-4.03v2.21l2.45 2.45c.03-.2.05-.41.05-.63zm2.5 0c0 .94-.2 1.82-.54 2.64l1.51 1.51C20.63 14.91 21 13.5 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06c1.38-.31 2.63-.95 3.69-1.81L19.73 21 21 19.73l-9-9L4.27 3zM12 4L9.91 6.09 12 8.18V4z'
};

function setVolumeIcon(level) {
  const volumeBtn = document.getElementById('volumeBtn');
  if (!volumeBtn) return;
  volumeBtn.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="22" height="22" aria-hidden="true"><path d="${VOLUME_ICON_PATHS[level]}"/></svg>`;
}

function loadSong() {
  try {
    console.log('🎵 loadSong() called');

    // Get song ID from URL hash (music.html#song-id)
    let songId = window.location.hash.substring(1);
    console.log('Song ID from hash:', songId);

    // Fallback to query parameter
    if (!songId) {
      const urlParams = new URLSearchParams(window.location.search);
      songId = urlParams.get('id') || urlParams.get('song');
    }

    // Redirect to music.html if no song specified
    if (!songId) {
      console.log('No song ID - redirecting to track listing');
      window.location.href = 'music.html';
      return;
    }

    // Check if musicData exists
    if (typeof musicData === 'undefined') {
      console.error('❌ musicData not found - did music-data.js load?');
      showError('Error loading music database. Please refresh the page.');
      return;
    }

    console.log('✓ musicData found. Available songs:', Object.keys(musicData));

    const song = musicData[songId];

    if (!song) {
      console.warn(`❌ Song "${songId}" not found in musicData`);
      showError(`Song "${songId}" not found.`);
      return;
    }

    console.log(`✓ Song found: ${song.title}`);

    // Store current song globally (for sync engine)
    currentSong = song;

    // Compute startTime for each lyric from cumulative dur values
    let t = 0;
    (song.lyrics || []).forEach(line => {
      line.startTime = t;
      t += line.dur || 0;
    });
    currentLyricsArray = song.lyrics || [];
    lastActiveLineId = null;

    // Update page metadata
    document.title = `${song.title} - ${song.artist} - Mehdi Bahlaoui`;

    // Keep meta description and social/share tags in sync with the loaded song
    const songDesc = `"${song.title}" by ${song.artist} — lyrics with time-synced annotations.`;
    const metaDesc = document.querySelector('meta[name="description"]');
    if (metaDesc) metaDesc.content = songDesc;
    const setProperty = (prop, value) => {
      const el = document.querySelector(`meta[property="${prop}"]`);
      if (el) el.content = value;
    };
    setProperty('og:title', `${song.title} - ${song.artist}`);
    setProperty('og:description', songDesc);
    setProperty('og:url', `https://mehdibahlaoui.com/music/template.html${window.location.hash}`);

    // Update page header with song info
    const songTitleEl = document.getElementById('songTitle');
    if (songTitleEl) {
      songTitleEl.textContent = song.title;
    }

    const songDescEl = document.getElementById('songDescription');
    if (songDescEl) {
      songDescEl.innerHTML = `<b>${song.title}</b> by ${song.artist}. ${song.album ? `From the album "${song.album}"` : 'Single'}${song.releaseDate ? ` (${song.releaseDate})` : ''}.`;
    }

    // Render song components
    console.log('Rendering lyrics...');
    renderLyrics(song);
    console.log('Rendering annotations...');
    renderAnnotations(song);
    console.log('Updating album art...');
    updateAlbumArt(song);

    // Initialize audio player
    console.log('Initializing audio player...');
    initializeAudioPlayer(song);

    console.log(`✅ Loaded song: ${song.title}`);
  } catch (error) {
    console.error('❌ Error in loadSong():', error);
    console.error('Stack trace:', error.stack);
    showError('An error occurred while loading the song. Check the console for details.');
  }
}

function showError(message) {
  const contentDiv = document.querySelector('.music-player-container');
  if (contentDiv) {
    contentDiv.innerHTML = `<div class="error-message"><p>${message}</p></div>`;
  }
}

// ============================================
// SECTION 2: Rendering Functions
// ============================================

function renderLyrics(song) {
  try {
    const lyricsContainer = document.getElementById('lyricsContainer');
    if (!lyricsContainer) {
      console.warn('❌ lyricsContainer not found in DOM');
      return;
    }

    if (!song.lyrics || song.lyrics.length === 0) {
      lyricsContainer.innerHTML = '<p class="placeholder-lyrics">Lyrics coming soon...</p>';
      return;
    }

    let html = '';

    song.lyrics.forEach(line => {
    if (!line.text || !line.text.trim()) return;
    const hasAnnotation = line.annotations && line.annotations.length > 0;

    html += `
      <div class="lyric-line"
           id="${line.id}"
           data-start="${line.startTime}"
           ${hasAnnotation ? 'data-has-annotation="true"' : ''}>
        <span class="lyric-text">${line.text}</span>
      </div>
    `;
  });

    lyricsContainer.innerHTML = html;

    // Initial slot machine state: first 3 rendered lyrics visible, first one centered
    requestAnimationFrame(() => {
      const initPos = ['active', 'next1', 'next2', 'next3'];
      const renderedLines = song.lyrics.filter(l => document.getElementById(l.id));
      renderedLines.forEach((line, i) => {
        const el = document.getElementById(line.id);
        if (i < 4) el.setAttribute('data-lyric-pos', initPos[i]);
      });

      const firstEl = renderedLines[0] && document.getElementById(renderedLines[0].id);
      if (firstEl) {
        centerLyricLine(firstEl, 'instant');
      }
    });

    // Add click handlers for seeking + annotation scroll
    document.querySelectorAll('.lyric-line').forEach(lineEl => {

      lineEl.addEventListener('click', () => {


        const startTime = parseFloat(lineEl.dataset.start);
        const audioPlayer = document.getElementById('audioPlayer');
        const playPauseBtn = document.getElementById('playPauseBtn');

        audioPlayer.currentTime = startTime;
        playPauseBtn.textContent = '⏸';
        audioPlayer.play();

        // Also scroll to matching annotation within sidebar (not the page)
        syncAnnotationToLine(lineEl.id);
      });
    });
    console.log(`✓ Rendered ${song.lyrics.length} lyric lines`);
  } catch (error) {
    console.error('❌ Error in renderLyrics():', error);
    throw error;
  }
}

function renderAnnotations(song) {
  try {
    const annotationsSidebar = document.getElementById('annotationsSidebar');
    if (!annotationsSidebar) {
      console.warn('❌ annotationsSidebar not found in DOM');
      return;
    }

    if (!song.annotations || Object.keys(song.annotations).length === 0) {
      annotationsSidebar.innerHTML = '<h3 class="annotations-title">Annotations</h3><p class="placeholder-annotations">No annotations yet.</p>';
      return;
    }

    let html = '<h3 class="annotations-title">Annotations</h3>';

  // Sort annotations by line order
  const annotationIds = Object.keys(song.annotations);

  annotationIds.forEach(annId => {
    const ann = song.annotations[annId];
    const verifiedClass = ann.verified ? 'verified' : '';
    const verifiedBadge = ann.verified ? ' ✓' : '';

    html += `
      <div class="annotation-card" data-line-id="${ann.lineId}" data-annotation-id="${annId}">
        <div class="annotation-header">
          <span class="annotation-author ${verifiedClass}">${ann.author}${verifiedBadge}</span>
          <span class="annotation-date">${ann.date}</span>
        </div>
        <div class="annotation-quote">"${getLineText(ann.lineId, song)}"</div>
        <div class="annotation-content">
          ${ann.content}
        </div>
      </div>
    `;
  });

    annotationsSidebar.innerHTML = html;

    // Add click handlers for annotation → lyric scroll (within lyrics container)
    document.querySelectorAll('.annotation-card').forEach(card => {
      card.addEventListener('click', (e) => {
        if (e.target.classList.contains('vote-btn')) return; // Don't scroll on vote click

        const lineId = card.dataset.lineId;
        const lyricLine = document.getElementById(lineId);
        if (!lyricLine) return;

        // Seek playback to the line so the user hears it alongside the annotation
        const audioPlayer = document.getElementById('audioPlayer');
        const playPauseBtn = document.getElementById('playPauseBtn');
        const startTime = parseFloat(lyricLine.dataset.start);
        if (audioPlayer && !isNaN(startTime)) {
          audioPlayer.currentTime = startTime;
          if (playPauseBtn) playPauseBtn.textContent = '⏸';
          audioPlayer.play().catch(() => {
            if (playPauseBtn) playPauseBtn.textContent = '▶';
          });
        }

        // Center the line and mark this annotation as the active one
        centerLyricLine(lyricLine);
        document.querySelectorAll('.annotation-card.active-annotation').forEach(c => {
          c.classList.remove('active-annotation');
        });
        card.classList.add('active-annotation');

        // Add temporary highlight
        lyricLine.classList.add('annotation-focused');
        setTimeout(() => lyricLine.classList.remove('annotation-focused'), 2000);
      });
    });
    console.log(`✓ Rendered ${Object.keys(song.annotations).length} annotations`);
  } catch (error) {
    console.error('❌ Error in renderAnnotations():', error);
    throw error;
  }
}

function getLineText(lineId, song) {
  const line = song.lyrics.find(l => l.id === lineId);
  return line ? line.text : '[Line not found]';
}

function updateAlbumArt(song) {
  const albumArtImg = document.getElementById('controlAlbumArt');
  if (albumArtImg) {
    albumArtImg.src = song.coverArt || 'Images/boss.jpg';
    albumArtImg.alt = `${song.title} album art`;
  } else {
    console.warn('controlAlbumArt element not found in DOM');
  }
}

// ============================================
// SECTION 3: Audio Playback + Sync Engine
// ============================================

// How many lyric lines before the saved position to resume from on reload.
// Set to 0 to resume from the exact saved time.
const RESTORE_LINES_BACK = 2;

function initializeAudioPlayer(song) {
  const audioPlayer = document.getElementById('audioPlayer');
  const playPauseBtn = document.getElementById('playPauseBtn');
  const progressBarContainer = document.getElementById('progressBarContainer');
  const progressBarFilled = document.getElementById('progressBarFilled');
  const timeCurrent = document.getElementById('timeCurrent');
  const timeTotal = document.getElementById('timeTotal');
  const prevBtn = document.getElementById('prevBtn');
  const nextBtn = document.getElementById('nextBtn');
  const volumeSlider = document.getElementById('volumeSlider');
  const volumeBtn = document.getElementById('volumeBtn');

  // Validate all required elements exist
  if (!audioPlayer || !playPauseBtn || !progressBarContainer) {
    console.error('Missing required audio player elements in DOM');
    console.log('audioPlayer:', audioPlayer);
    console.log('playPauseBtn:', playPauseBtn);
    console.log('progressBarContainer:', progressBarContainer);
    return;
  }

  // Load audio source
  audioPlayer.src = song.audioUrl;
  console.log('Audio source set to:', audioPlayer.src);

  // Update duration when metadata loads; restore saved position on reload
  audioPlayer.addEventListener('loadedmetadata', () => {
    timeTotal.textContent = formatTime(audioPlayer.duration);
    const savedTime = parseFloat(sessionStorage.getItem(`music_time_${song.id}`));
    if (!isNaN(savedTime) && savedTime > 0) {
      let restoreTime = savedTime;
      if (RESTORE_LINES_BACK > 0 && currentLyricsArray.length > 0) {
        let activeIndex = 0;
        for (let i = 0; i < currentLyricsArray.length; i++) {
          if (currentLyricsArray[i].startTime <= savedTime) activeIndex = i;
          else break;
        }
        restoreTime = currentLyricsArray[Math.max(0, activeIndex - RESTORE_LINES_BACK)].startTime;
      }
      audioPlayer.currentTime = restoreTime;
      audioPlayer.play().then(() => {
        playPauseBtn.textContent = '⏸';
      }).catch(() => {});
    }
  });

  // init progress bar and volume - always start at 100% on every device
  progressBarFilled.style.width = `0%`;
  audioPlayer.currentTime = 0;
  audioPlayer.volume = 1;
  volumeSlider.value = 100;
  setVolumeIcon('high');

  // SYNC ENGINE: Listen to playback time updates (~4 times/second)
  audioPlayer.addEventListener('timeupdate', () => {
    const currentTime = audioPlayer.currentTime;

    // Update progress bar and time display
    const progress = (currentTime / audioPlayer.duration) * 100;
    progressBarFilled.style.width = `${progress}%`;
    timeCurrent.textContent = formatTime(currentTime);

    // Highlight current lyric line
    highlightActiveLine(currentTime);

    // Persist position so reload resumes from here
    sessionStorage.setItem(`music_time_${song.id}`, currentTime);
  });

  // Play/Pause button - update icon IMMEDIATELY for better UX
  playPauseBtn.addEventListener('click', () => {
    if (audioPlayer.paused) {
      // Update icon immediately before play() resolves
      playPauseBtn.textContent = '⏸';
      audioPlayer.play().catch(() => {
        // Revert icon if play fails
        playPauseBtn.textContent = '▶';
      });
    } else {
      // Update icon immediately before pause
      playPauseBtn.textContent = '▶';
      audioPlayer.pause();
    }
  });

  // Update play button on ended
  audioPlayer.addEventListener('ended', () => {
    playPauseBtn.textContent = '▶';
  });

  // Progress bar seeking - handle click, drag, and immediate visual feedback
  let isDragging = false;

  const updateProgressFromEvent = (e) => {
    const rect = progressBarContainer.getBoundingClientRect();
    const clickX = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
    const percentage = clickX / rect.width;

    // Update visual immediately for responsive feel
    progressBarFilled.style.width = `${percentage * 100}%`;

    // Update audio time
    if (audioPlayer.duration) {
      audioPlayer.currentTime = audioPlayer.duration * percentage;
    }
  };

  progressBarContainer.addEventListener('mousedown', (e) => {
    isDragging = true;
    updateProgressFromEvent(e);
  });

  document.addEventListener('mousemove', (e) => {
    if (isDragging) {
      updateProgressFromEvent(e);
    }
  });

  document.addEventListener('mouseup', () => {
    isDragging = false;
  });

  // Touch support for mobile
  progressBarContainer.addEventListener('touchstart', (e) => {
    isDragging = true;
    updateProgressFromEvent(e.touches[0]);
  });

  document.addEventListener('touchmove', (e) => {
    if (isDragging) {
      updateProgressFromEvent(e.touches[0]);
    }
  });

  document.addEventListener('touchend', () => {
    isDragging = false;
  });

  // Volume control
  volumeSlider.addEventListener('input', (e) => {
    if (window.matchMedia('(max-width: 850px)').matches) {
      e.target.value = 100;
      audioPlayer.volume = 1;
      setVolumeIcon('high');
      return;
    }
    const volume = e.target.value / 100;
    audioPlayer.volume = volume;

    // Update volume button icon
    if (volume === 0) {
      setVolumeIcon('muted');
    } else if (volume < 0.5) {
      setVolumeIcon('low');
    } else {
      setVolumeIcon('high');
    }
  });

  // Mute/unmute on volume button click - restore previous level on unmute
  let volumeBeforeMute = 1;
  volumeBtn.addEventListener('click', () => {
    if (audioPlayer.volume === 0) {
      const restored = volumeBeforeMute > 0 ? volumeBeforeMute : 1;
      audioPlayer.volume = restored;
      volumeSlider.value = Math.round(restored * 100);
      setVolumeIcon(restored < 0.5 ? 'low' : 'high');
    } else {
      volumeBeforeMute = audioPlayer.volume;
      audioPlayer.volume = 0;
      volumeSlider.value = 0;
      setVolumeIcon('muted');
    }
  });

  // Keyboard shortcuts
  document.addEventListener('keydown', (e) => {
    if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;

    switch (e.code) {
      case 'Space':
        e.preventDefault();
        playPauseBtn.click();
        break;
      case 'ArrowLeft':
        e.preventDefault();
        audioPlayer.currentTime = Math.max(0, audioPlayer.currentTime - 5);
        break;
      case 'ArrowRight':
        e.preventDefault();
        audioPlayer.currentTime = Math.min(audioPlayer.duration, audioPlayer.currentTime + 5);
        break;
    }
  });

  // Prev/Next buttons (for multi-song support in future)
  prevBtn.addEventListener('click', () => {
    const songIds = Object.keys(musicData);
    const currentIndex = songIds.indexOf(currentSong.id);
    if (currentIndex > 0) {
      window.location.hash = songIds[currentIndex - 1];
    }
  });

  nextBtn.addEventListener('click', () => {
    const songIds = Object.keys(musicData);
    const currentIndex = songIds.indexOf(currentSong.id);
    if (currentIndex < songIds.length - 1) {
      window.location.hash = songIds[currentIndex + 1];
    }
  });
}

// Center a lyric line inside the lyrics container.
// Uses getBoundingClientRect deltas instead of offsetTop: neither container is a
// positioned ancestor, so offsetTop is measured against the page, not the scroller.
function centerLyricLine(lineEl, behavior = 'smooth') {
  const container = document.getElementById('lyricsContainer');
  if (!container || !lineEl) return;
  const delta = lineEl.getBoundingClientRect().top - container.getBoundingClientRect().top;
  const target = container.scrollTop + delta - container.clientHeight / 2 + lineEl.offsetHeight / 2;
  container.scrollTo({ top: Math.max(0, target), behavior });
}

// Scroll the annotations sidebar so the card for lineId sits near the top,
// and highlight it as the active annotation. No-op if the line has no annotation.
function syncAnnotationToLine(lineId) {
  const card = document.querySelector(`.annotation-card[data-line-id="${lineId}"]`);

  // Always clear first so a card unfocuses once playback passes its line
  document.querySelectorAll('.annotation-card.active-annotation').forEach(c => {
    c.classList.remove('active-annotation');
  });
  if (!card) return;

  card.classList.add('active-annotation');

  const sidebar = document.getElementById('annotationsSidebar');
  if (!sidebar) return;
  const delta = card.getBoundingClientRect().top - sidebar.getBoundingClientRect().top;
  const target = sidebar.scrollTop + delta - 20; // 20px padding from top
  sidebar.scrollTo({ top: Math.max(0, target), behavior: 'smooth' });
}

// SYNC ENGINE: Core logic for highlighting current lyric
function highlightActiveLine(currentTime) {
  if (!currentLyricsArray || currentLyricsArray.length === 0) return;

  // Only work with lines that are actually rendered in the DOM (empty-text lines are skipped)
  const renderedLines = currentLyricsArray.filter(line => document.getElementById(line.id));
  if (renderedLines.length === 0) return;

  // Find the last rendered line whose startTime <= currentTime
  let activeIndex = -1;
  for (let i = renderedLines.length - 1; i >= 0; i--) {
    if (renderedLines[i].startTime <= currentTime) {
      activeIndex = i;
      break;
    }
  }

  // Remove previous highlight class
  document.querySelectorAll('.lyric-line.active-line').forEach(el => {
    el.classList.remove('active-line');
  });

  // Before first lyric: show first 4 rendered lines, none highlighted
  if (activeIndex === -1) {
    const initPos = ['active', 'next1', 'next2', 'next3'];
    renderedLines.forEach((line, i) => {
      const el = document.getElementById(line.id);
      if (i < 4) el.setAttribute('data-lyric-pos', initPos[i]);
      else el.removeAttribute('data-lyric-pos');
    });
    return;
  }

  // Assign slot machine positions based on rendered-line index distance
  const posMap = { '-2': 'prev2', '-1': 'prev1', '0': 'active', '1': 'next1', '2': 'next2', '3': 'next3' };
  renderedLines.forEach((line, i) => {
    const el = document.getElementById(line.id);
    if (!el) return;
    const pos = posMap[String(i - activeIndex)];
    if (pos) el.setAttribute('data-lyric-pos', pos);
    else el.removeAttribute('data-lyric-pos');
  });

  // Highlight + center-scroll active line
  const activeLineId = renderedLines[activeIndex].id;
  const lineElement = document.getElementById(activeLineId);
  if (lineElement) {
    lineElement.classList.add('active-line');

    clearTimeout(lineElement.dataset.scrollTimeout);
    lineElement.dataset.scrollTimeout = setTimeout(() => {
      centerLyricLine(lineElement);
    }, 50);

    // When focus moves to a new line, bring its annotation along (if it has one)
    if (activeLineId !== lastActiveLineId) {
      lastActiveLineId = activeLineId;
      syncAnnotationToLine(activeLineId);
    }
  }
}

// Utility function to format time (MM:SS)
function formatTime(seconds) {
  if (!isFinite(seconds)) return '0:00';
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}


// ============================================
// Event Listeners & Initialization
// ============================================

// Listen for hash changes (enables navigation between songs)
window.addEventListener('hashchange', loadSong);

// Load on page load - handle both cases:
// 1. If DOM is still loading, wait for DOMContentLoaded
// 2. If DOM is already loaded (common with defer), call immediately
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', loadSong);
} else {
  loadSong();
}
