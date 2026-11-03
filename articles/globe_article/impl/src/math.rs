use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 1e-8 {
            self * (1.0 / len)
        } else {
            self
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

/// Lat/lon in degrees -> unit sphere. Matches the Bevy version's axis
/// convention: lon = 0 is +Z, lon = 90 is +X, lat = 0 is the equator.
pub fn lat_lon_to_3d(lat_deg: f32, lon_deg: f32, radius: f32) -> Vec3 {
    let lat = lat_deg.to_radians();
    let lon = lon_deg.to_radians();
    Vec3::new(
        radius * lat.cos() * lon.sin(),
        radius * lat.sin(),
        radius * lat.cos() * lon.cos(),
    )
}

fn slerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    let dot = a.dot(b).clamp(-1.0, 1.0);
    let theta = dot.acos();
    if theta.abs() < 1e-6 {
        return a;
    }
    let sin_theta = theta.sin();
    let wa = ((1.0 - t) * theta).sin() / sin_theta;
    let wb = (t * theta).sin() / sin_theta;
    a * wa + b * wb
}

/// Great-circle path with a parabolic height bump, same shape as the
/// reference implementation (`4 * height * t * (1 - t)`).
pub fn great_circle_arc(from: Vec3, to: Vec3, segments: usize, height: f32) -> Vec<Vec3> {
    let a = from.normalize();
    let b = to.normalize();
    let mut points = Vec::with_capacity(segments + 1);

    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let p = slerp(a, b, t);
        let h = 4.0 * height * t * (1.0 - t);
        points.push(p * (1.0 + h));
    }

    points
}

/// Column-major 4x4 matrix, ready to upload to WebGL with transpose=false.
pub type Mat4 = [f32; 16];

pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_y * 0.5).tan();
    let mut m = [0.0; 16];
    m[0] = f / aspect.max(1e-6);
    m[5] = f;
    m[10] = (far + near) / (near - far);
    m[11] = -1.0;
    m[14] = (2.0 * far * near) / (near - far);
    m
}

pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let forward = (center - eye).normalize();
    let side = forward.cross(up).normalize();
    let up = side.cross(forward);

    let mut m = [0.0; 16];
    m[0] = side.x;
    m[4] = side.y;
    m[8] = side.z;
    m[12] = -side.dot(eye);

    m[1] = up.x;
    m[5] = up.y;
    m[9] = up.z;
    m[13] = -up.dot(eye);

    m[2] = -forward.x;
    m[6] = -forward.y;
    m[10] = -forward.z;
    m[14] = forward.dot(eye);

    m[15] = 1.0;
    m
}

/// Column-major matrix product: `a * b`.
pub fn multiply(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[k * 4 + row] * b[col * 4 + k];
            }
            out[col * 4 + row] = sum;
        }
    }
    out
}
