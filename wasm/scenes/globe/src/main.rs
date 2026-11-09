#[allow(clippy::excessive_precision, clippy::large_const_arrays)]
mod land_data;
mod globe;

use mini3d::app::{App, Lifecycle};
use mini3d::camera::OrbitConfig;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    App::new(
        "globe-container",
        "Interactive globe: land shown as dots, with animated routes between Paris, New York and Tokyo",
    )
    .orbit(OrbitConfig {
        pitch: 20.0_f32.to_radians(),
        radius: 3.0,
        auto_rotate: 0.06,
        drag_sensitivity: 0.005,
        ..OrbitConfig::default()
    })
    .lifecycle(Lifecycle::slide_in(".globe-layout", 350.0))
    .run(globe::Globe::new);
}

fn main() {}
