use crate::math::Vec3;

/// Lat/lon in degrees -> sphere of radius `radius`.
/// Axis convention: `lon = 0` is `+Z`, `lon = 90` is `+X`, `lat = 0` is the equator.
pub fn lat_lon_to_3d(lat_deg: f32, lon_deg: f32, radius: f32) -> Vec3 {
    let lat = lat_deg.to_radians();
    let lon = lon_deg.to_radians();
    Vec3::new(
        radius * lat.cos() * lon.sin(),
        radius * lat.sin(),
        radius * lat.cos() * lon.cos(),
    )
}

/// Spherical linear interpolation between two unit vectors.
pub fn slerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
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

/// Great-circle path with a parabolic height bump (`4 * height * t * (1 - t)`).
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
