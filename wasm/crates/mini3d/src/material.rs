use crate::gfx::{BlendMode, CullMode, RenderState};

/// Key-lit + rim-lit opaque surface material (used for the globe body and 3D meshes).
#[derive(Clone, Copy, Debug)]
pub struct SolidMaterial {
    pub base: [f32; 3],
    pub key: [f32; 3],
    pub rim: [f32; 3],
    pub light: [f32; 3],
}

impl Default for SolidMaterial {
    fn default() -> Self {
        Self {
            base: [0.050, 0.057, 0.190],
            key: [0.15, 0.145, 0.125],
            rim: [0.010, 0.015, 0.040],
            light: [-0.55, 0.10, 0.83],
        }
    }
}

/// Camera-facing point-sprite material for dot clouds and soft highlight markers.
#[derive(Clone, Copy, Debug)]
pub struct PointMaterial {
    pub color: [f32; 3],
    pub diameter: f32,
    pub soft: bool,
    pub fade_by_normal: bool,
    pub light: [f32; 3],
}

impl Default for PointMaterial {
    fn default() -> Self {
        Self {
            color: [0.80, 0.80, 0.80],
            diameter: 0.012,
            soft: false,
            fade_by_normal: true,
            light: [-0.55, 0.10, 0.83],
        }
    }
}

/// Animated pulse trail material for routes and polylines.
#[derive(Clone, Copy, Debug)]
pub struct PulseMaterial {
    pub diameter: f32,
    pub tail: f32,
    pub cool_color: [f32; 3],
    pub hot_color: [f32; 3],
}

impl Default for PulseMaterial {
    fn default() -> Self {
        Self {
            diameter: 0.008,
            tail: 0.35,
            cool_color: [0.85, 0.55, 0.22],
            hot_color: [1.0, 0.88, 0.62],
        }
    }
}

/// Material for instanced 3D quads (e.g., tumbling 3D confetti paper, cards, debris).
#[derive(Clone, Copy, Debug)]
pub struct QuadMaterial {
    /// Rounded-corner radius in normalized quad UV space (`0.0` = sharp rectangle, `1.0` = circle/pill).
    pub corner_radius: f32,
    /// View-space directional light vector for two-sided paper shading.
    pub light: [f32; 3],
    /// Minimum ambient brightness (`0.0..=1.0`).
    pub ambient: f32,
    /// Self-illumination added on top of the lit color (`0.0` = none, higher = brighter/glowier).
    pub emissive: f32,
    pub double_sided: bool,
    pub blend: BlendMode,
    pub depth_write: bool,
}

impl Default for QuadMaterial {
    fn default() -> Self {
        Self {
            corner_radius: 0.12,
            light: [-0.45, 0.75, 0.50],
            ambient: 0.58,
            emissive: 0.0,
            double_sided: true,
            blend: BlendMode::Opaque,
            depth_write: true,
        }
    }
}

impl QuadMaterial {
    pub fn render_state(&self) -> RenderState {
        RenderState {
            depth_test: true,
            depth_write: self.depth_write,
            blend: self.blend,
            cull: if self.double_sided {
                CullMode::None
            } else {
                CullMode::Back
            },
        }
    }
}
