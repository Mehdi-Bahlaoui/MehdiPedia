//! `mini3d` — a tiny Rust + WebGL2 3D renderer for WebAssembly animations.
//!
//! Designed for small, fast-starting browser scenes without a heavy game engine.
//! It provides:
//! - [`math`]: `Vec3`, `Quat`, and column-major `Mat4` helpers
//! - [`geo`]: spherical coordinates, slerp, and great-circle arcs
//! - [`gfx`]: low-level WebGL2 shaders, buffers, VAOs, geometry builders, and render state
//! - [`material`]: built-in materials for meshes, point clouds, pulse lines, and instanced 3D quads
//! - [`render`]: high-level [`render::Mesh`], [`render::Points`], [`render::PulseLine`],
//!   [`render::InstancedQuads`], and [`render::Renderer`]
//! - [`camera`]: [`camera::OrbitCamera`] and pointer-drag controls
//! - [`app`] & [`scene`]: canvas bootstrap, DPR resize, `requestAnimationFrame` loop, and slide-in lifecycle

pub mod app;
pub mod camera;
pub mod geo;
pub mod gfx;
pub mod material;
pub mod math;
pub mod render;
pub mod scene;
