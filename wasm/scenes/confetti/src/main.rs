use mini3d::app::{App, Frame};
use mini3d::camera::OrbitConfig;
use mini3d::gfx::Gl;
use mini3d::material::QuadMaterial;
use mini3d::math::{Quat, Vec3};
use mini3d::render::{InstancedQuads, QuadInstance, Renderer};
use mini3d::scene::Scene;
use wasm_bindgen::prelude::*;

const PARTICLE_COUNT: usize = 1200;
const GRAVITY: f32 = 1.35;
const DRAG: f32 = 0.85;
const FLUTTER_AMP: f32 = 1.6;
const FLUTTER_FREQ: f32 = 5.5;
const FLOOR_Y: f32 = -2.2;

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
    phase: f32,
    age: f32,
}

pub struct Confetti {
    quads: InstancedQuads,
    particles: Vec<Particle>,
    instances: Vec<QuadInstance>,
    rng: Rng,
}

impl Confetti {
    pub fn new(gl: &Gl) -> Result<Self, JsValue> {
        let mut rng = Rng(0xC0FF_EE42);
        let mut particles = Vec::with_capacity(PARTICLE_COUNT);
        let mut instances = Vec::with_capacity(PARTICLE_COUNT);

        for _ in 0..PARTICLE_COUNT {
            let mut p = Self::spawn_particle(&mut rng, true);
            p.age = rng.range(0.0, 4.0);
            instances.push(Self::to_instance(&p));
            particles.push(p);
        }

        let mut quads = InstancedQuads::new(gl, PARTICLE_COUNT, QuadMaterial::default())?;
        quads.update(gl, &instances);

        Ok(Self {
            quads,
            particles,
            instances,
            rng,
        })
    }

    fn spawn_particle(rng: &mut Rng, initial_spread: bool) -> Particle {
        let y = if initial_spread {
            rng.range(FLOOR_Y, 2.4)
        } else {
            rng.range(1.9, 2.6)
        };

        let axis = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let rotation = Quat::from_axis_angle(axis, rng.range(0.0, std::f32::consts::TAU));

        let spin = Vec3::new(
            rng.range(-6.0, 6.0),
            rng.range(-6.0, 6.0),
            rng.range(-6.0, 6.0),
        );

        let w = rng.range(0.032, 0.058);
        let h = w * rng.range(1.3, 2.2);
        let color = PALETTE[(rng.next_u32() as usize) % PALETTE.len()];

        Particle {
            position: Vec3::new(rng.range(-2.2, 2.2), y, rng.range(-1.6, 1.6)),
            velocity: Vec3::new(
                rng.range(-0.35, 0.35),
                rng.range(-0.9, -0.25),
                rng.range(-0.35, 0.35),
            ),
            rotation,
            spin,
            scale: [w, h],
            color,
            phase: rng.range(0.0, std::f32::consts::TAU),
            age: 0.0,
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
        let dt = frame.dt;
        for (i, p) in self.particles.iter_mut().enumerate() {
            p.age += dt;
            p.velocity.y -= GRAVITY * dt;
            p.velocity *= (1.0 - DRAG * dt).max(0.0);

            let flutter = (p.phase + p.age * FLUTTER_FREQ).sin() * FLUTTER_AMP;
            let sway = (p.phase * 1.7 + p.age * (FLUTTER_FREQ * 0.7)).cos() * (FLUTTER_AMP * 0.6);
            p.velocity.x += flutter * dt;
            p.velocity.z += sway * dt;

            p.position += p.velocity * dt;
            p.rotation = (Quat::from_scaled_axis(p.spin * dt) * p.rotation).normalize();

            if p.position.y < FLOOR_Y {
                *p = Self::spawn_particle(&mut self.rng, false);
            }

            self.instances[i] = Self::to_instance(p);
        }

        self.quads.update(gl, &self.instances);
    }

    fn render(&mut self, renderer: &mut Renderer, frame: &Frame) {
        renderer.clear().quads(&self.quads, frame);
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::new("confetti-container", "Interactive 3D confetti animation")
        .orbit(OrbitConfig {
            pitch: 10.0_f32.to_radians(),
            radius: 3.4,
            auto_rotate: 0.08,
            ..OrbitConfig::default()
        })
        .run(Confetti::new);
}

fn main() {}
