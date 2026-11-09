use mini3d::app::Frame;
use mini3d::geo::lat_lon_to_3d;
use mini3d::gfx::Gl;
use mini3d::material::{PointMaterial, PulseMaterial, SolidMaterial};
use mini3d::math::Vec3;
use mini3d::render::{Mesh, Points, PulseLine, Renderer};
use mini3d::scene::Scene;
use wasm_bindgen::prelude::*;

use crate::land_data::LAND_POSITIONS;

const GLOBE_RADIUS: f32 = 1.0;
const BODY_RADIUS: f32 = GLOBE_RADIUS * 0.995;
const LAND_RADIUS: f32 = GLOBE_RADIUS * 1.002;

const DOT_DIAMETER: f32 = 0.012;
const HIGHLIGHT_DIAMETER: f32 = 0.04;
const ARC_DIAMETER: f32 = 0.008;

const ARC_SEGMENTS: usize = 48;
const ARC_HEIGHT: f32 = 0.15;
const ARC_TAIL: f32 = 0.35;

const CITIES: [(f32, f32); 3] = [
    (48.8566, 2.3522),   // Paris
    (40.7128, -74.0060), // New York
    (35.6762, 139.6503), // Tokyo
];

const ROUTES: [(usize, usize); 3] = [(0, 1), (1, 2), (2, 0)];

pub struct Globe {
    sphere: Mesh,
    land: Points,
    highlights: Points,
    arcs: PulseLine,
    arc_progress: [f32; 3],
}

impl Globe {
    pub fn new(gl: &Gl) -> Result<Self, JsValue> {
        let sphere = Mesh::sphere(
            gl,
            BODY_RADIUS,
            64,
            32,
            SolidMaterial {
                base: [0.050, 0.057, 0.190],
                key: [0.15, 0.145, 0.125],
                rim: [0.010, 0.015, 0.040],
                light: [-0.55, 0.10, 0.83],
            },
        )?;

        let land = Points::from_positions(
            gl,
            &LAND_POSITIONS,
            LAND_RADIUS,
            PointMaterial {
                color: [0.80, 0.80, 0.80],
                diameter: DOT_DIAMETER,
                soft: false,
                fade_by_normal: true,
                ..PointMaterial::default()
            },
        )?;

        let city_points: Vec<Vec3> = CITIES
            .iter()
            .map(|&(lat, lon)| lat_lon_to_3d(lat, lon, LAND_RADIUS))
            .collect();

        let highlights = Points::from_vec3(
            gl,
            &city_points,
            1.0,
            PointMaterial {
                color: [0.98, 0.84, 0.60],
                diameter: HIGHLIGHT_DIAMETER,
                soft: true,
                fade_by_normal: false,
                ..PointMaterial::default()
            },
        )?;

        let route_pairs: Vec<(Vec3, Vec3)> = ROUTES
            .iter()
            .map(|&(from_idx, to_idx)| {
                (
                    lat_lon_to_3d(CITIES[from_idx].0, CITIES[from_idx].1, GLOBE_RADIUS),
                    lat_lon_to_3d(CITIES[to_idx].0, CITIES[to_idx].1, GLOBE_RADIUS),
                )
            })
            .collect();

        let arcs = PulseLine::from_great_circles(
            gl,
            &route_pairs,
            ARC_SEGMENTS,
            ARC_HEIGHT,
            PulseMaterial {
                diameter: ARC_DIAMETER,
                tail: ARC_TAIL,
                ..PulseMaterial::default()
            },
        )?;

        Ok(Self {
            sphere,
            land,
            highlights,
            arcs,
            arc_progress: [0.0, -0.6, -1.2],
        })
    }
}

impl Scene for Globe {
    fn update(&mut self, _gl: &Gl, frame: &Frame) {
        for progress in &mut self.arc_progress {
            *progress += frame.dt;
            if *progress > 1.0 + ARC_TAIL {
                *progress = -0.1;
            }
        }
    }

    fn render(&mut self, renderer: &mut Renderer, frame: &Frame) {
        renderer
            .clear()
            .mesh(&self.sphere, frame)
            .points(&self.land, frame)
            .points(&self.highlights, frame)
            .pulse(&self.arcs, &self.arc_progress, frame);
    }
}
