use mini3d::app::{App, Frame};
use mini3d::camera::OrbitConfig;
use mini3d::gfx::Gl;
use mini3d::material::QuadMaterial;
use mini3d::math::{Quat, Vec3};
use mini3d::render::{InstancedQuads, QuadInstance, Renderer};
use mini3d::scene::Scene;
use wasm_bindgen::prelude::*;

const PARTICLE_COUNT: usize = 1200;
const FLOOR_Y: f32 = -4.0;
const DEFAULT_SPREAD_X: f32 = 2.4;
const REFERENCE_SPREAD_X: f32 = 3.44;
const CAMERA_RADIUS: f32 = 5.4;
const CAMERA_FOV_Y: f32 = 50.0;
const GRAVITY: f32 = 1.0;
const BURST_SPEED_MIN: f32 = 3.5;
const BURST_SPEED_MAX: f32 = 6.5;
const BURST_LATERAL: f32 = 0.55;
const BURST_Z_SPREAD: f32 = 1.2;

const PALETTE: [[f32; 3]; 6] = [
    [0.98, 0.32, 0.38], // coral red
    [0.99, 0.78, 0.22], // warm gold
    [0.24, 0.82, 0.58], // mint green
    [0.26, 0.62, 0.98], // sky blue
    [0.72, 0.44, 0.98], // violet
    [0.99, 0.54, 0.24], // tangerine
];

struct Rng(u32);

impl Rng {
    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    fn f32(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 * (1.0 / 16_777_216.0)
    }

    fn range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.f32()
    }
}

struct Particle {
    position: Vec3,
    velocity: Vec3,
    rotation: Quat,
    spin: Vec3,
    scale: [f32; 2],
    color: [f32; 3],
    drag: f32,
    phase: f32,
    age: f32,
    active: bool,
}

pub struct Confetti {
    quads: InstancedQuads,
    particles: Vec<Particle>,
    instances: Vec<QuadInstance>,
    rng: Rng,
    done: bool,
    cleared: bool,
    first_frame: bool,
}

impl Confetti {
    pub fn new(gl: &Gl) -> Result<Self, JsValue> {
        let mut rng = Rng(0xC0FF_EE42);
        let mut particles = Vec::with_capacity(PARTICLE_COUNT);
        let mut instances = Vec::with_capacity(PARTICLE_COUNT);

        for _ in 0..PARTICLE_COUNT {
            let particle = Self::spawn(&mut rng, DEFAULT_SPREAD_X);
            instances.push(Self::to_instance(&particle));
            particles.push(particle);
        }

        let material = QuadMaterial {
            ambient: 0.72,
            emissive: 0.14,
            ..QuadMaterial::default()
        };
        let mut quads = InstancedQuads::new(gl, PARTICLE_COUNT, material)?;
        quads.update(gl, &instances);

        Ok(Self {
            quads,
            particles,
            instances,
            rng,
            done: false,
            cleared: false,
            first_frame: true,
        })
    }

    fn spawn(rng: &mut Rng, spread_x: f32) -> Particle {
        let speed = rng.range(BURST_SPEED_MIN, BURST_SPEED_MAX);
        let lateral = rng.range(-1.0, 1.0);
        let fan = (spread_x / REFERENCE_SPREAD_X).clamp(0.45, 1.05);

        // The launcher sits just below the bottom centre of the viewport.
        let position = Vec3::new(
            rng.range(-0.35, 0.35),
            rng.range(3.0, 3.4),
            rng.range(-0.6, 0.6),
        );

        // A upward burst with a wide lateral fan and a little depth spread.
        let velocity = Vec3::new(
            lateral * speed * BURST_LATERAL * fan,
            speed,
            rng.range(-BURST_Z_SPREAD, BURST_Z_SPREAD),
        );

        let axis = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let rotation = Quat::from_axis_angle(axis, rng.range(0.0, std::f32::consts::TAU));

        let spin = Vec3::new(
            rng.range(-5.0, 5.0),
            rng.range(-5.0, 5.0),
            rng.range(-5.0, 5.0),
        );

        // Twice the old base size; perspective then stretches the near pieces
        // up to roughly four times the old size.
        let width = rng.range(0.068, 0.120);
        let height = width * rng.range(1.3, 2.2);
        let color = PALETTE[(rng.next_u32() as usize) % PALETTE.len()];

        Particle {
            position,
            velocity,
            rotation,
            spin,
            scale: [width, height],
            color,
            drag: rng.range(0.12, 0.30),
            phase: rng.range(0.0, std::f32::consts::TAU),
            age: 0.0,
            active: true,
        }
    }

    fn to_instance(p: &Particle) -> QuadInstance {
        QuadInstance {
            position: p.position.to_array(),
            rotation: p.rotation.to_array(),
            scale: p.scale,
            color: p.color,
        }
    }
}

impl Scene for Confetti {
    fn update(&mut self, gl: &Gl, frame: &Frame) {
        if self.done {
            return;
        }

        if self.first_frame {
            let view_half_width = CAMERA_RADIUS * (CAMERA_FOV_Y.to_radians() * 0.5).tan();
            let spread_x = (view_half_width * frame.aspect() * 0.95).clamp(1.2, 8.0);
            let density = (frame.aspect() / 1.44).clamp(0.28, 1.0);
            let active_count = (PARTICLE_COUNT as f32 * density) as usize;

            for (i, p) in self.particles.iter_mut().enumerate() {
                *p = Self::spawn(&mut self.rng, spread_x);
                p.active = i < active_count;
            }

            self.first_frame = false;
        }

        self.instances.clear();
        let dt = frame.dt;

        for p in &mut self.particles {
            if !p.active {
                continue;
            }

            p.age += dt;
            p.velocity.y -= GRAVITY * dt;
            p.velocity *= (1.0 - p.drag * dt).max(0.0);

            // A little paper flutter on top of the ballistic arc.
            p.velocity.x += (p.phase + p.age * 4.0).sin() * 1.2 * dt;
            p.velocity.z += (p.phase * 1.7 + p.age * 3.0).cos() * 0.9 * dt;

            p.position += p.velocity * dt;
            p.rotation = (Quat::from_scaled_axis(p.spin * dt) * p.rotation).normalize();

            if p.position.y < FLOOR_Y {
                p.active = false;
                continue;
            }

            self.instances.push(Self::to_instance(p));
        }

        self.quads.update(gl, &self.instances);

        if self.instances.is_empty() {
            self.done = true;
        }
    }

    fn render(&mut self, renderer: &mut Renderer, frame: &Frame) {
        if self.done {
            if !self.cleared {
                renderer.clear();
                self.cleared = true;
            }
            return;
        }

        renderer.clear().quads(&self.quads, frame);
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::new("confetti-container", "Confetti launcher animation")
        .orbit(OrbitConfig {
            pitch: 6.0_f32.to_radians(),
            radius: CAMERA_RADIUS,
            fov_y: CAMERA_FOV_Y.to_radians(),
            auto_rotate: 0.0,
            interactive: false,
            ..OrbitConfig::default()
        })
        .run(Confetti::new);
}

fn main() {}
