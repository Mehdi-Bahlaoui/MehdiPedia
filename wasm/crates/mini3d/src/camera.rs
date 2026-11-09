use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget, HtmlCanvasElement, MouseEvent, PointerEvent, Window};

use crate::math::{look_at, multiply, perspective, Mat4, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct OrbitConfig {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub target: Vec3,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
    pub auto_rotate: f32,
    pub drag_sensitivity: f32,
    pub pitch_limit: f32,
    pub interactive: bool,
}

impl Default for OrbitConfig {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 20.0_f32.to_radians(),
            radius: 3.0,
            target: Vec3::ZERO,
            fov_y: std::f32::consts::PI / 4.0,
            near: 0.1,
            far: 100.0,
            auto_rotate: 0.0,
            drag_sensitivity: 0.005,
            pitch_limit: 85.0_f32.to_radians(),
            interactive: true,
        }
    }
}

pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub target: Vec3,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
    pub auto_rotate: f32,
    pub drag_sensitivity: f32,
    pub pitch_limit: f32,
    dragging: bool,
    active_pointer: Option<i32>,
    last_x: f64,
    last_y: f64,
}

impl OrbitCamera {
    pub fn new(config: OrbitConfig) -> Self {
        let limit = config.pitch_limit.abs();
        Self {
            yaw: config.yaw,
            pitch: config.pitch.max(-limit).min(limit),
            radius: config.radius,
            target: config.target,
            fov_y: config.fov_y,
            near: config.near,
            far: config.far,
            auto_rotate: config.auto_rotate,
            drag_sensitivity: config.drag_sensitivity,
            pitch_limit: limit,
            dragging: false,
            active_pointer: None,
            last_x: 0.0,
            last_y: 0.0,
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.yaw += self.auto_rotate * dt;
    }

    pub fn orbit(&mut self, dx_px: f32, dy_px: f32) {
        let limit = self.pitch_limit.abs();
        self.yaw -= dx_px * self.drag_sensitivity;
        self.pitch = (self.pitch + dy_px * self.drag_sensitivity)
            .max(-limit)
            .min(limit);
    }

    pub fn eye(&self) -> Vec3 {
        let pitch_cos = self.pitch.cos();
        self.target
            + Vec3::new(
                self.radius * pitch_cos * self.yaw.sin(),
                self.radius * self.pitch.sin(),
                self.radius * pitch_cos * self.yaw.cos(),
            )
    }

    pub fn view(&self) -> Mat4 {
        look_at(self.eye(), self.target, Vec3::Y)
    }

    pub fn projection(&self, aspect: f32) -> Mat4 {
        perspective(self.fov_y, aspect, self.near, self.far)
    }

    pub fn view_projection(&self, aspect: f32) -> (Mat4, Mat4, Mat4) {
        let view = self.view();
        let proj = self.projection(aspect);
        let view_proj = multiply(&proj, &view);
        (view, proj, view_proj)
    }

    pub fn size_scale(&self, viewport_height: f32) -> f32 {
        viewport_height / (2.0 * (self.fov_y * 0.5).tan())
    }
}

fn prevent_default(event: &PointerEvent) {
    let event: &web_sys::Event = event.unchecked_ref();
    event.prevent_default();
}

pub fn install_orbit_controls(
    window: &Window,
    canvas: &HtmlCanvasElement,
    camera: Rc<RefCell<OrbitCamera>>,
) -> Result<(), JsValue> {
    let canvas_target: &EventTarget = canvas.unchecked_ref();
    let window_target: &EventTarget = window.unchecked_ref();

    let cam_down = camera.clone();
    let canvas_down = canvas.clone();
    let on_down = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
        let mouse: &MouseEvent = event.unchecked_ref();
        if mouse.button() != 0 {
            return;
        }

        prevent_default(&event);

        let mut cam = cam_down.borrow_mut();
        cam.dragging = true;
        cam.active_pointer = Some(event.pointer_id());
        cam.last_x = mouse.client_x() as f64;
        cam.last_y = mouse.client_y() as f64;

        let canvas_element: &Element = canvas_down.unchecked_ref();
        let _ = canvas_element.set_pointer_capture(event.pointer_id());
    });
    canvas_target
        .add_event_listener_with_callback("pointerdown", on_down.as_ref().unchecked_ref())?;
    on_down.forget();

    let cam_move = camera.clone();
    let on_move = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
        let mut cam = cam_move.borrow_mut();
        if !cam.dragging || cam.active_pointer != Some(event.pointer_id()) {
            return;
        }

        prevent_default(&event);

        let mouse: &MouseEvent = event.unchecked_ref();
        let x = mouse.client_x() as f64;
        let y = mouse.client_y() as f64;
        let dx = (x - cam.last_x) as f32;
        let dy = (y - cam.last_y) as f32;

        cam.last_x = x;
        cam.last_y = y;
        cam.orbit(dx, dy);
    });
    window_target
        .add_event_listener_with_callback("pointermove", on_move.as_ref().unchecked_ref())?;
    on_move.forget();

    let cam_up = camera;
    let canvas_up = canvas.clone();
    let on_up = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
        let mut cam = cam_up.borrow_mut();
        if cam.active_pointer != Some(event.pointer_id()) {
            return;
        }

        let canvas_element: &Element = canvas_up.unchecked_ref();
        let _ = canvas_element.release_pointer_capture(event.pointer_id());
        cam.dragging = false;
        cam.active_pointer = None;
    });

    window_target.add_event_listener_with_callback("pointerup", on_up.as_ref().unchecked_ref())?;
    window_target
        .add_event_listener_with_callback("pointercancel", on_up.as_ref().unchecked_ref())?;
    on_up.forget();

    Ok(())
}
