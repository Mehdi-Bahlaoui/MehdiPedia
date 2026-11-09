#[allow(clippy::excessive_precision, clippy::large_const_arrays)]
mod land_data;
mod math;
mod renderer;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Element, EventTarget, HtmlCanvasElement, MouseEvent, Node, PointerEvent,
    WebGl2RenderingContext, Window,
};

use crate::math::{look_at, multiply, perspective, Vec3};
use crate::renderer::{Renderer, ARC_TAIL};

const FOV_Y: f32 = std::f32::consts::PI / 4.0;
const NEAR: f32 = 0.1;
const FAR: f32 = 100.0;
const AUTO_ROTATE_SPEED: f32 = 0.06; // radians per second
const DRAG_SENSITIVITY: f32 = 0.005; // radians per CSS pixel
const INITIAL_PITCH: f32 = 20.0 * std::f32::consts::PI / 180.0;
const INITIAL_RADIUS: f32 = 3.0;

type FrameClosure = Closure<dyn FnMut(f64)>;
type FrameSlot = Rc<RefCell<Option<FrameClosure>>>;

struct App {
    window: Window,
    canvas: HtmlCanvasElement,
    container: Element,
    layout: Option<Element>,
    renderer: Renderer,

    yaw: f32,
    pitch: f32,
    radius: f32,

    dragging: bool,
    active_pointer: Option<i32>,
    last_x: f64,
    last_y: f64,

    css_w: f64,
    css_h: f64,
    dpr: f64,

    last_time: f64,
    arc_progress: [f32; 3],

    first_frame: bool,
    first_frame_at: f64,
    settled: bool,
}

impl App {
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

    fn frame(&mut self, now: f64) {
        self.update_size();

        let dt = if self.last_time > 0.0 {
            ((now - self.last_time) / 1000.0).clamp(0.0, 0.05) as f32
        } else {
            0.0
        };
        self.last_time = now;

        self.yaw += AUTO_ROTATE_SPEED * dt;

        for progress in &mut self.arc_progress {
            *progress += dt;
            if *progress > 1.0 + ARC_TAIL {
                *progress = -0.1;
            }
        }

        let width = self.canvas.width().max(1) as f32;
        let height = self.canvas.height().max(1) as f32;
        let aspect = width / height;

        let pitch_cos = self.pitch.cos();
        let eye = Vec3::new(
            self.radius * pitch_cos * self.yaw.sin(),
            self.radius * self.pitch.sin(),
            self.radius * pitch_cos * self.yaw.cos(),
        );

        let view = look_at(eye, Vec3::ZERO, Vec3::Y);
        let projection = perspective(FOV_Y, aspect, NEAR, FAR);
        let view_projection = multiply(&projection, &view);
        let size_scale = height / (2.0 * (FOV_Y * 0.5).tan());

        self.renderer
            .render(&view_projection, &view, &self.arc_progress, size_scale);

        if !self.first_frame {
            self.first_frame = true;
            self.first_frame_at = now;

            // Hides the loading skeleton and starts the existing slide-in
            // animation contract from assets/css/globe.css.
            let _ = self.container.class_list().add_1("globe-live");
            if let Some(layout) = &self.layout {
                let _ = layout.class_list().add_1("globe-ready");
            }
        }

        if self.first_frame && !self.settled && now - self.first_frame_at >= 350.0 {
            if let Some(layout) = &self.layout {
                let _ = layout.class_list().add_1("globe-settled");
            }
            self.settled = true;
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    if let Err(error) = run() {
        web_sys::console::error_1(&error);
        show_fallback();
    }
}

fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("no document"))?;

    let container = document
        .get_element_by_id("globe-container")
        .ok_or_else(|| JsValue::from_str("#globe-container not found"))?;

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    let canvas_element: &Element = canvas.unchecked_ref();
    canvas_element.set_attribute(
        "aria-label",
        "Interactive globe: land shown as dots, with animated routes between Paris, New York and Tokyo",
    )?;
    canvas_element.set_attribute("role", "img")?;

    container.append_with_node_1(canvas.unchecked_ref::<Node>())?;

    let context = canvas
        .get_context("webgl2")?
        .ok_or_else(|| JsValue::from_str("WebGL2 is not available in this browser"))?
        .dyn_into::<WebGl2RenderingContext>()?;

    let renderer = Renderer::new(context)?;
    let layout = document.query_selector(".globe-layout")?;

    let app = Rc::new(RefCell::new(App {
        window: window.clone(),
        canvas,
        container,
        layout,
        renderer,
        yaw: 0.0,
        pitch: INITIAL_PITCH,
        radius: INITIAL_RADIUS,
        dragging: false,
        active_pointer: None,
        last_x: 0.0,
        last_y: 0.0,
        css_w: 0.0,
        css_h: 0.0,
        dpr: 0.0,
        last_time: 0.0,
        // Same stagger as the Bevy version: -(arc_id) * 0.6 seconds.
        arc_progress: [0.0, -0.6, -1.2],
        first_frame: false,
        first_frame_at: 0.0,
        settled: false,
    }));

    app.borrow_mut().update_size();
    install_events(&window, &app)?;
    start_animation_loop(&window, app)
}

fn prevent_default(event: &PointerEvent) {
    let event: &web_sys::Event = event.unchecked_ref();
    event.prevent_default();
}

fn install_events(window: &Window, app: &Rc<RefCell<App>>) -> Result<(), JsValue> {
    let canvas = app.borrow().canvas.clone();
    let canvas_target: &EventTarget = canvas.unchecked_ref();
    let window_target: &EventTarget = window.unchecked_ref();

    let app_down = app.clone();
    let on_down = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
        let mouse: &MouseEvent = event.unchecked_ref();
        if mouse.button() != 0 {
            return;
        }

        prevent_default(&event);

        let mut app = app_down.borrow_mut();
        app.dragging = true;
        app.active_pointer = Some(event.pointer_id());
        app.last_x = mouse.client_x() as f64;
        app.last_y = mouse.client_y() as f64;

        let canvas_element: &Element = app.canvas.unchecked_ref();
        let _ = canvas_element.set_pointer_capture(event.pointer_id());
    });
    canvas_target
        .add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref())?;
    on_down.forget();

    let app_move = app.clone();
    let on_move = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
        let mut app = app_move.borrow_mut();
        if !app.dragging || app.active_pointer != Some(event.pointer_id()) {
            return;
        }

        prevent_default(&event);

        let mouse: &MouseEvent = event.unchecked_ref();
        let x = mouse.client_x() as f64;
        let y = mouse.client_y() as f64;
        let dx = x - app.last_x;
        let dy = y - app.last_y;

        app.last_x = x;
        app.last_y = y;
        app.yaw -= dx as f32 * DRAG_SENSITIVITY;
        app.pitch += dy as f32 * DRAG_SENSITIVITY;
        app.pitch = app
            .pitch
            .clamp(-85.0_f32.to_radians(), 85.0_f32.to_radians());
    });
    window_target
        .add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref())?;
    on_move.forget();

    let app_up = app.clone();
    let on_up = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
        let mut app = app_up.borrow_mut();
        if app.active_pointer != Some(event.pointer_id()) {
            return;
        }

        let canvas_element: &Element = app.canvas.unchecked_ref();
        let _ = canvas_element.release_pointer_capture(event.pointer_id());
        app.dragging = false;
        app.active_pointer = None;
    });

    window_target.add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref())?;
    window_target
        .add_event_listener_with_callback("pointercancel", on_up.as_ref().unchecked_ref())?;
    on_up.forget();

    Ok(())
}

fn start_animation_loop(window: &Window, app: Rc<RefCell<App>>) -> Result<(), JsValue> {
    let slot: FrameSlot = Rc::new(RefCell::new(None));
    let next = slot.clone();
    let window = window.clone();
    let raf_window = window.clone();

    *slot.borrow_mut() = Some(Closure::<dyn FnMut(f64)>::new(move |now: f64| {
        app.borrow_mut().frame(now);

        if let Some(callback) = next.borrow().as_ref() {
            let _ = raf_window.request_animation_frame(callback.as_ref().unchecked_ref());
        }
    }));

    if let Some(callback) = slot.borrow().as_ref() {
        window.request_animation_frame(callback.as_ref().unchecked_ref())?;
    }

    Ok(())
}

fn show_fallback() {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(container) = document.get_element_by_id("globe-container") {
                container.set_inner_html(
                    "<p style=\"max-width:32rem;margin:2rem auto;font-family:sans-serif;\
                     color:#888;text-align:center;\">The interactive globe needs WebGL2. \
                     Try a current version of Chrome, Firefox, Safari or Edge.</p>",
                );
            }
        }
    }
}

fn main() {}
