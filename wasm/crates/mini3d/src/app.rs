use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCanvasElement, Node, Window};

use crate::camera::{install_orbit_controls, OrbitCamera, OrbitConfig};
use crate::gfx::Gl;
use crate::math::Mat4;
use crate::render::Renderer;
use crate::scene::Scene;

type FrameClosure = Closure<dyn FnMut(f64)>;
type FrameSlot = Rc<RefCell<Option<FrameClosure>>>;

const DEFAULT_FALLBACK_HTML: &str =
    "<p style=\"max-width:32rem;margin:2rem auto;font-family:sans-serif;\
     color:#888;text-align:center;\">This interactive 3D scene needs WebGL2. \
     Try a current version of Chrome, Firefox, Safari or Edge.</p>";

/// Per-frame timing, viewport, and camera matrices passed to [`Scene::update`] and [`Scene::render`].
pub struct Frame {
    pub now_ms: f64,
    pub dt: f32,
    pub width: u32,
    pub height: u32,
    pub view: Mat4,
    pub proj: Mat4,
    pub view_proj: Mat4,
    pub size_scale: f32,
}

impl Frame {
    pub fn aspect(&self) -> f32 {
        (self.width.max(1) as f32) / (self.height.max(1) as f32)
    }
}

/// Optional DOM class transitions triggered on the first rendered frame and after a settle delay.
#[derive(Clone, Debug)]
pub struct Lifecycle {
    pub live_class: Option<&'static str>,
    pub layout_selector: Option<&'static str>,
    pub ready_class: Option<&'static str>,
    pub settled_class: Option<&'static str>,
    pub settle_delay_ms: f64,
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::none()
    }
}

impl Lifecycle {
    pub const fn none() -> Self {
        Self {
            live_class: None,
            layout_selector: None,
            ready_class: None,
            settled_class: None,
            settle_delay_ms: 0.0,
        }
    }

    /// Standard slide-in contract used by `articles/globe/index.html` (`globe-live`, `globe-ready`, `globe-settled`).
    pub const fn slide_in(layout_selector: &'static str, settle_delay_ms: f64) -> Self {
        Self {
            live_class: Some("globe-live"),
            layout_selector: Some(layout_selector),
            ready_class: Some("globe-ready"),
            settled_class: Some("globe-settled"),
            settle_delay_ms,
        }
    }

    pub const fn with_classes(
        mut self,
        live_class: &'static str,
        ready_class: &'static str,
        settled_class: &'static str,
    ) -> Self {
        self.live_class = Some(live_class);
        self.ready_class = Some(ready_class);
        self.settled_class = Some(settled_class);
        self
    }
}

/// Builder that creates the `<canvas>`, WebGL2 context, orbit camera, and animation loop.
pub struct App {
    container_id: &'static str,
    aria_label: &'static str,
    orbit: OrbitConfig,
    lifecycle: Lifecycle,
    fallback_html: &'static str,
}

impl App {
    pub const fn new(container_id: &'static str, aria_label: &'static str) -> Self {
        Self {
            container_id,
            aria_label,
            orbit: OrbitConfig {
                yaw: 0.0,
                pitch: 20.0 * std::f32::consts::PI / 180.0,
                radius: 3.0,
                target: crate::math::Vec3::ZERO,
                fov_y: std::f32::consts::PI / 4.0,
                near: 0.1,
                far: 100.0,
                auto_rotate: 0.0,
                drag_sensitivity: 0.005,
                pitch_limit: 85.0 * std::f32::consts::PI / 180.0,
                interactive: true,
            },
            lifecycle: Lifecycle::none(),
            fallback_html: DEFAULT_FALLBACK_HTML,
        }
    }

    pub const fn orbit(mut self, orbit: OrbitConfig) -> Self {
        self.orbit = orbit;
        self
    }

    pub const fn lifecycle(mut self, lifecycle: Lifecycle) -> Self {
        self.lifecycle = lifecycle;
        self
    }

    pub const fn fallback_html(mut self, html: &'static str) -> Self {
        self.fallback_html = html;
        self
    }

    pub fn run<S, F>(self, init_scene: F)
    where
        S: Scene,
        F: FnOnce(&Gl) -> Result<S, JsValue>,
    {
        let container_id = self.container_id.trim_start_matches('#');
        let fallback_html = self.fallback_html;
        if let Err(error) = self.try_run(init_scene) {
            web_sys::console::error_1(&error);
            show_fallback(container_id, fallback_html);
        }
    }

    pub fn try_run<S, F>(self, init_scene: F) -> Result<(), JsValue>
    where
        S: Scene,
        F: FnOnce(&Gl) -> Result<S, JsValue>,
    {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("no document"))?;

        let id = self.container_id.trim_start_matches('#');
        let container = document
            .get_element_by_id(id)
            .ok_or_else(|| JsValue::from_str(&format!("#{id} not found")))?;

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;

        let canvas_element: &Element = canvas.unchecked_ref();
        canvas_element.set_attribute("aria-label", self.aria_label)?;
        canvas_element.set_attribute("role", "img")?;

        container.append_with_node_1(canvas.unchecked_ref::<Node>())?;

        let gl = canvas
            .get_context("webgl2")?
            .ok_or_else(|| JsValue::from_str("WebGL2 is not available in this browser"))?
            .dyn_into::<Gl>()?;

        let scene = init_scene(&gl)?;
        let renderer = Renderer::new(gl);

        let layout = match self.lifecycle.layout_selector {
            Some(selector) => document.query_selector(selector)?,
            None => None,
        };

        let interactive = self.orbit.interactive;
        let camera = Rc::new(RefCell::new(OrbitCamera::new(self.orbit)));
        if interactive {
            install_orbit_controls(&window, &canvas, camera.clone())?;
        }

        let runner = Rc::new(RefCell::new(Runner {
            window: window.clone(),
            canvas,
            container,
            layout,
            lifecycle: self.lifecycle,
            renderer,
            camera,
            scene,
            css_w: 0.0,
            css_h: 0.0,
            dpr: 0.0,
            last_time: 0.0,
            first_frame: false,
            first_frame_at: 0.0,
            settled: false,
        }));

        runner.borrow_mut().update_size();
        start_animation_loop(&window, runner)
    }
}

struct Runner<S: Scene> {
    window: Window,
    canvas: HtmlCanvasElement,
    container: Element,
    layout: Option<Element>,
    lifecycle: Lifecycle,
    renderer: Renderer,
    camera: Rc<RefCell<OrbitCamera>>,
    scene: S,
    css_w: f64,
    css_h: f64,
    dpr: f64,
    last_time: f64,
    first_frame: bool,
    first_frame_at: f64,
    settled: bool,
}

impl<S: Scene> Runner<S> {
    fn update_size(&mut self) {
        let dpr = self.window.device_pixel_ratio().clamp(1.0, 2.0);
        let css_w = self.container.client_width().max(1) as f64;
        let css_h = self.container.client_height().max(1) as f64;

        if (css_w - self.css_w).abs() < 0.5
            && (css_h - self.css_h).abs() < 0.5
            && (dpr - self.dpr).abs() < 0.01
        {
            return;
        }

        self.css_w = css_w;
        self.css_h = css_h;
        self.dpr = dpr;

        let width = (css_w * dpr).round().max(1.0) as u32;
        let height = (css_h * dpr).round().max(1.0) as u32;

        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.renderer.resize(width as i32, height as i32);
    }

    fn frame(&mut self, now_ms: f64) {
        self.update_size();

        let dt = if self.last_time > 0.0 {
            ((now_ms - self.last_time) / 1000.0).clamp(0.0, 0.05) as f32
        } else {
            0.0
        };
        self.last_time = now_ms;

        let width = self.canvas.width().max(1);
        let height = self.canvas.height().max(1);
        let aspect = (width as f32) / (height as f32);

        let (view, proj, view_proj, size_scale) = {
            let mut cam = self.camera.borrow_mut();
            cam.step(dt);
            let (view, proj, view_proj) = cam.view_projection(aspect);
            let size_scale = cam.size_scale(height as f32);
            (view, proj, view_proj, size_scale)
        };

        let frame = Frame {
            now_ms,
            dt,
            width,
            height,
            view,
            proj,
            view_proj,
            size_scale,
        };

        self.scene.update(self.renderer.gl(), &frame);
        self.scene.render(&mut self.renderer, &frame);

        if !self.first_frame {
            self.first_frame = true;
            self.first_frame_at = now_ms;

            if let Some(live_class) = self.lifecycle.live_class {
                let _ = self.container.class_list().add_1(live_class);
            }
            if let (Some(layout), Some(ready_class)) = (&self.layout, self.lifecycle.ready_class) {
                let _ = layout.class_list().add_1(ready_class);
            }
        }

        if self.first_frame
            && !self.settled
            && now_ms - self.first_frame_at >= self.lifecycle.settle_delay_ms
        {
            if let (Some(layout), Some(settled_class)) =
                (&self.layout, self.lifecycle.settled_class)
            {
                let _ = layout.class_list().add_1(settled_class);
            }
            self.settled = true;
        }
    }
}

fn start_animation_loop<S: Scene>(window: &Window, runner: Rc<RefCell<Runner<S>>>) -> Result<(), JsValue> {
    let slot: FrameSlot = Rc::new(RefCell::new(None));
    let next = slot.clone();
    let window = window.clone();
    let raf_window = window.clone();

    *slot.borrow_mut() = Some(Closure::<dyn FnMut(f64)>::new(move |now: f64| {
        runner.borrow_mut().frame(now);

        if let Some(callback) = next.borrow().as_ref() {
            let _ = raf_window.request_animation_frame(callback.as_ref().unchecked_ref());
        }
    }));

    if let Some(callback) = slot.borrow().as_ref() {
        window.request_animation_frame(callback.as_ref().unchecked_ref())?;
    }

    Ok(())
}

pub fn show_fallback(container_id: &str, html: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(container) = document.get_element_by_id(container_id) {
                container.set_inner_html(html);
            }
        }
    }
}
