// Bevy/winit's orbit camera ignores pointerType="touch". Patch the
// prototype getter so every read returns "mouse" for touch input.
const origPT = Object.getOwnPropertyDescriptor(PointerEvent.prototype, 'pointerType');
Object.defineProperty(PointerEvent.prototype, 'pointerType', {
  get() { const r = origPT.get.call(this); return r === 'touch' ? 'mouse' : r; },
  configurable: true,
});

// Move Bevy's canvas from <body> into our container as soon as it appears.
const container = document.getElementById('globe-container');
const observer = new MutationObserver(() => {
  const canvas = document.body.querySelector(':scope > canvas');
  if (canvas) {
    observer.disconnect();
    canvas.setAttribute('tabindex', '0');
    container.appendChild(canvas);
  }
});
observer.observe(document.body, { childList: true });

import init from './globe/704496ede9d7912d.js';
// Bevy throws an exception to hand off to the browser's rAF loop.
try { await init({ module_or_path: './globe/704496ede9d7912d.wasm' }); } catch (e) {}

// Fallback in case the observer missed the canvas.
const canvas = document.body.querySelector(':scope > canvas');
if (canvas) { canvas.setAttribute('tabindex', '0'); container.appendChild(canvas); }

// Slide-in animation (desktop only)
const layout = document.querySelector('.globe-layout');
if (layout && window.innerWidth > 850) {
  layout.classList.add('globe-ready');
  setTimeout(() => {
    layout.classList.add('globe-settled');
  }, 350);
}

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
