// Timing markers (same protocol for baseline vs optimized comparison).
// tLoaderStart: this module began executing (loader JS downloaded + parsed).
const tLoaderStart = performance.now();
const mark = (name) => console.info(`[globe] ${name}: ${Math.round(performance.now() - tLoaderStart)}ms`);

// Bevy/winit's orbit camera ignores pointerType="touch". Patch the
// prototype getter so every read returns "mouse" for touch input.
const origPT = Object.getOwnPropertyDescriptor(PointerEvent.prototype, 'pointerType');
Object.defineProperty(PointerEvent.prototype, 'pointerType', {
  get() { const r = origPT.get.call(this); return r === 'touch' ? 'mouse' : r; },
  configurable: true,
});

// Move Bevy's canvas from <body> into our container as soon as it appears.
// NOTE: the skeleton stays until first-frame below — canvas attach only
// means Bevy started, while ~1.6s of main-thread shader/pipeline setup
// still blocks before real pixels appear.
const container = document.getElementById('globe-container');
const layout = document.querySelector('.globe-layout');
function globeLive(source) {
  mark(`canvas-visible (${source})`);
  // This rAF can only run once Bevy's synchronous startup unblocks the main
  // thread, so it doubles as the first-pixels signal: hide the skeleton and
  // start the slide-in here, not at canvas attach.
  requestAnimationFrame(() => {
    container.classList.add('globe-live');
    mark('first-frame');
    // Slide-in animation (desktop only)
    if (layout && window.innerWidth > 850) {
      layout.classList.add('globe-ready');
      setTimeout(() => {
        layout.classList.add('globe-settled');
      }, 350);
    }
  });
}
const observer = new MutationObserver(() => {
  const canvas = document.body.querySelector(':scope > canvas');
  if (canvas) {
    observer.disconnect();
    canvas.setAttribute('tabindex', '0');
    container.appendChild(canvas);
    globeLive('observer');
  }
});
observer.observe(document.body, { childList: true });

import init from '/articles/globe-vendor/globe_app.js';
// NOTE: the glue already streams via fetch + instantiateStreaming when the
// server serves application/wasm (GitHub Pages does), so we keep the
// { module_or_path } form and only changed the path to the stable filename.
mark('wasm-fetch-start');
// Bevy throws an exception to hand off to the browser's rAF loop.
try { await init({ module_or_path: '/articles/globe-vendor/globe_app.wasm' }); mark('wasm-init-resolved'); } catch (e) { mark('wasm-init-handed-off'); }

// Fallback in case the observer missed the canvas.
const canvas = document.body.querySelector(':scope > canvas');
if (canvas) { canvas.setAttribute('tabindex', '0'); container.appendChild(canvas); globeLive('fallback'); }

// Prime Bevy's input state so the first touch-drag works immediately.
// Without this, Bevy needs a manual tap to focus + initialize its
// pointer tracking before orbit rotation responds.
setTimeout(() => {
  const cv = container.querySelector('canvas');
  if (!cv) return;
  const rect = cv.getBoundingClientRect();
  const opts = {
    pointerId: 1, pointerType: 'mouse', isPrimary: true, button: 0,
    clientX: rect.left + rect.width / 2, clientY: rect.top + rect.height / 2,
    bubbles: true, cancelable: true,
  };
  cv.dispatchEvent(new PointerEvent('pointerdown', { ...opts, buttons: 1 }));
  cv.dispatchEvent(new PointerEvent('pointerup',   { ...opts, buttons: 0 }));
  cv.focus();
}, 500);
