use wasm_bindgen::prelude::*;
use web_sys::{
    WebGl2RenderingContext as Gl, WebGlBuffer, WebGlProgram, WebGlShader, WebGlUniformLocation,
    WebGlVertexArrayObject,
};

use crate::land_data::LAND_POSITIONS;
use crate::math::{great_circle_arc, lat_lon_to_3d, Mat4};

// ---------------------------------------------------------------------------
// Scene constants (same visual numbers as the Bevy reference, minus the
// per-dot sphere meshes that made startup expensive)
// ---------------------------------------------------------------------------
const GLOBE_RADIUS: f32 = 1.0;
const BODY_RADIUS: f32 = GLOBE_RADIUS * 0.995;
const LAND_RADIUS: f32 = GLOBE_RADIUS * 1.002;

const DOT_DIAMETER: f32 = 0.012; // 2 * 0.006
const HIGHLIGHT_DIAMETER: f32 = 0.04; // 2 * 0.02
const ARC_DIAMETER: f32 = 0.008; // 2 * 0.004

const ARC_SEGMENTS: usize = 48;
const ARC_HEIGHT: f32 = 0.15;
pub const ARC_TAIL: f32 = 0.35;

// Correct coordinates for the three highlighted cities. The Bevy version
// labelled (18.8, 20.3) as Paris, which is in Chad; keep the same three
// routes but use the real coordinates.
const CITIES: [(f32, f32); 3] = [
    (48.8566, 2.3522),   // Paris
    (40.7128, -74.0060), // New York
    (35.6762, 139.6503), // Tokyo
];

const ROUTES: [(usize, usize); 3] = [(0, 1), (1, 2), (2, 0)];

const SPHERE_VS: &str = r#"#version 300 es
layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec3 a_normal;

uniform mat4 u_view_proj;
uniform mat4 u_view;

out vec3 v_normal_v;
out vec3 v_pos_v;

void main() {
    vec4 view_pos = u_view * vec4(a_pos, 1.0);
    v_pos_v = view_pos.xyz;
    v_normal_v = mat3(u_view) * a_normal;
    gl_Position = u_view_proj * vec4(a_pos, 1.0);
}
"#;

const SPHERE_FS: &str = r#"#version 300 es
precision highp float;

in vec3 v_normal_v;
in vec3 v_pos_v;
out vec4 out_color;

void main() {
    vec3 n = normalize(v_normal_v);
    vec3 v = normalize(-v_pos_v);

    // Broad key light from the upper-left, in view space, so the navy body
    // keeps its form as the camera orbits. This is the same light used to
    // shade the land dots.
    float key = pow(clamp(dot(n, normalize(vec3(-0.55, 0.10, 0.83))), 0.0, 1.0), 2.0);
    vec3 base = vec3(0.050, 0.057, 0.190) + vec3(0.15, 0.145, 0.125) * key;

    // Very soft edge so the silhouette doesn't disappear on dark themes.
    float rim = pow(1.0 - clamp(dot(n, v), 0.0, 1.0), 3.0);
    base += vec3(0.010, 0.015, 0.040) * rim;

    out_color = vec4(base, 1.0);
}
"#;

const DOT_VS: &str = r#"#version 300 es
layout(location = 0) in vec3 a_pos;

uniform mat4 u_view_proj;
uniform mat4 u_view;
uniform float u_size_scale;
uniform float u_diameter;

out float v_fade;
out float v_shade;

void main() {
    vec4 view_pos = u_view * vec4(a_pos, 1.0);

    // Point sprites are always camera-facing, so without this the dots
    // pile up into bright bands at the limb. Fade the ones whose surface
    // normal is turning away from the camera.
    vec3 normal_v = normalize(mat3(u_view) * normalize(a_pos));
    vec3 view_dir = normalize(-view_pos.xyz);
    float facing = clamp(dot(normal_v, view_dir), 0.0, 1.0);
    v_fade = smoothstep(0.0, 0.18, facing);

    // Same broad key light as the globe body, so the dot field has a lit
    // side instead of reading as a flat white sticker.
    float key = clamp(dot(normal_v, normalize(vec3(-0.55, 0.10, 0.83))), 0.0, 1.0);
    v_shade = 0.85 + 0.15 * key;

    vec4 clip = u_view_proj * vec4(a_pos, 1.0);
    if (clip.w <= 0.0) {
        gl_Position = vec4(2.0, 2.0, 2.0, 1.0);
        gl_PointSize = 0.0;
    } else {
        gl_Position = clip;
        gl_PointSize = max(1.0, u_diameter * u_size_scale / clip.w);
    }
}
"#;

const DOT_FS: &str = r#"#version 300 es
precision mediump float;

uniform vec3 u_color;
uniform float u_soft;
in float v_fade;
in float v_shade;
out vec4 out_color;

void main() {
    float d = length(gl_PointCoord - vec2(0.5)) * 2.0;
    float core = 1.0 - smoothstep(0.72, 1.0, d);
    float alpha = core;
    vec3 color = u_color * v_shade;

    if (u_soft > 0.5) {
        float halo = 0.30 * (1.0 - smoothstep(0.0, 1.0, d));
        alpha = max(core, halo);
        color = u_color * (0.75 + 0.5 * core) * v_shade;
    } else {
        alpha *= v_fade;
    }

    if (alpha <= 0.01) discard;
    out_color = vec4(color * alpha, alpha);
}
"#;

const ARC_VS: &str = r#"#version 300 es
layout(location = 0) in vec3 a_pos;
layout(location = 1) in float a_t;
layout(location = 2) in float a_arc;

uniform mat4 u_view_proj;
uniform vec3 u_progress;
uniform float u_tail;
uniform float u_size_scale;
uniform float u_diameter;

out float v_intensity;

void main() {
    float progress = a_arc < 0.5
        ? u_progress.x
        : (a_arc < 1.5 ? u_progress.y : u_progress.z);
    float behind = progress - a_t;
    float intensity = 0.0;
    if (behind >= 0.0 && behind <= u_tail) {
        intensity = 1.0 - behind / u_tail;
    }
    intensity *= intensity;
    v_intensity = intensity;

    vec4 clip = u_view_proj * vec4(a_pos, 1.0);
    if (intensity <= 0.001 || clip.w <= 0.0) {
        gl_Position = vec4(2.0, 2.0, 2.0, 1.0);
        gl_PointSize = 0.0;
    } else {
        gl_Position = clip;
        gl_PointSize = max(1.0, u_diameter * u_size_scale / clip.w);
    }
}
"#;

const ARC_FS: &str = r#"#version 300 es
precision mediump float;

in float v_intensity;
out vec4 out_color;

void main() {
    float d = length(gl_PointCoord - vec2(0.5)) * 2.0;
    float alpha = (1.0 - smoothstep(0.55, 1.0, d)) * v_intensity;
    if (alpha <= 0.004) discard;

    vec3 cool = vec3(0.85, 0.55, 0.22);
    vec3 hot = vec3(1.0, 0.88, 0.62);
    vec3 color = mix(cool, hot, v_intensity * v_intensity);
    out_color = vec4(color * alpha, alpha);
}
"#;

struct SphereProgram {
    program: WebGlProgram,
    u_view_proj: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
}

struct DotProgram {
    program: WebGlProgram,
    u_view_proj: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
    u_size_scale: WebGlUniformLocation,
    u_diameter: WebGlUniformLocation,
    u_color: WebGlUniformLocation,
    u_soft: WebGlUniformLocation,
}

struct ArcProgram {
    program: WebGlProgram,
    u_view_proj: WebGlUniformLocation,
    u_progress: WebGlUniformLocation,
    u_tail: WebGlUniformLocation,
    u_size_scale: WebGlUniformLocation,
    u_diameter: WebGlUniformLocation,
}

struct Geometry {
    vao: WebGlVertexArrayObject,
    _buffers: Vec<WebGlBuffer>,
    count: i32,
    mode: u32,
}

pub struct Renderer {
    gl: Gl,
    sphere_program: SphereProgram,
    dot_program: DotProgram,
    arc_program: ArcProgram,
    sphere: Geometry,
    land: Geometry,
    highlights: Geometry,
    arcs: Geometry,
}

impl Renderer {
    pub fn new(gl: Gl) -> Result<Self, JsValue> {
        let sphere_program = SphereProgram::new(&gl)?;
        let dot_program = DotProgram::new(&gl)?;
        let arc_program = ArcProgram::new(&gl)?;

        let sphere = sphere_geometry(&gl)?;
        let land = land_geometry(&gl)?;
        let highlights = highlight_geometry(&gl)?;
        let arcs = arc_geometry(&gl)?;

        gl.enable(Gl::DEPTH_TEST);
        gl.depth_func(Gl::LEQUAL);
        gl.enable(Gl::BLEND);
        gl.blend_func(Gl::ONE, Gl::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear_depth(1.0);

        Ok(Self {
            gl,
            sphere_program,
            dot_program,
            arc_program,
            sphere,
            land,
            highlights,
            arcs,
        })
    }

    pub fn resize(&self, width: i32, height: i32) {
        self.gl.viewport(0, 0, width.max(1), height.max(1));
    }

    pub fn render(&self, view_proj: &Mat4, view: &Mat4, arc_progress: &[f32; 3], size_scale: f32) {
        let gl = &self.gl;

        gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);

        // Opaque globe body first so it writes depth; everything else is a
        // camera-facing point sprite that depth-tests against it.
        gl.use_program(Some(&self.sphere_program.program));
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.sphere_program.u_view_proj),
            false,
            view_proj,
        );
        gl.uniform_matrix4fv_with_f32_array(Some(&self.sphere_program.u_view), false, view);
        gl.depth_mask(true);
        gl.bind_vertex_array(Some(&self.sphere.vao));
        gl.draw_elements_with_i32(self.sphere.mode, self.sphere.count, Gl::UNSIGNED_SHORT, 0);

        gl.use_program(Some(&self.dot_program.program));
        gl.uniform_matrix4fv_with_f32_array(Some(&self.dot_program.u_view_proj), false, view_proj);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.dot_program.u_view), false, view);
        gl.uniform1f(Some(&self.dot_program.u_size_scale), size_scale);
        gl.depth_mask(false);

        // Land dots: one draw call, 9,251 point sprites.
        gl.uniform1f(Some(&self.dot_program.u_diameter), DOT_DIAMETER);
        gl.uniform3f(Some(&self.dot_program.u_color), 0.80, 0.80, 0.80);
        gl.uniform1f(Some(&self.dot_program.u_soft), 0.0);
        gl.bind_vertex_array(Some(&self.land.vao));
        gl.draw_arrays(self.land.mode, 0, self.land.count);

        // City markers: same program, bigger soft sprite.
        gl.uniform1f(Some(&self.dot_program.u_diameter), HIGHLIGHT_DIAMETER);
        gl.uniform3f(Some(&self.dot_program.u_color), 0.98, 0.84, 0.60);
        gl.uniform1f(Some(&self.dot_program.u_soft), 1.0);
        gl.bind_vertex_array(Some(&self.highlights.vao));
        gl.draw_arrays(self.highlights.mode, 0, self.highlights.count);

        // Animated route pulses.
        gl.use_program(Some(&self.arc_program.program));
        gl.uniform_matrix4fv_with_f32_array(Some(&self.arc_program.u_view_proj), false, view_proj);
        gl.uniform3f(
            Some(&self.arc_program.u_progress),
            arc_progress[0],
            arc_progress[1],
            arc_progress[2],
        );
        gl.uniform1f(Some(&self.arc_program.u_tail), ARC_TAIL);
        gl.uniform1f(Some(&self.arc_program.u_size_scale), size_scale);
        gl.uniform1f(Some(&self.arc_program.u_diameter), ARC_DIAMETER);
        gl.bind_vertex_array(Some(&self.arcs.vao));
        gl.draw_arrays(self.arcs.mode, 0, self.arcs.count);

        gl.bind_vertex_array(None);
        gl.depth_mask(true);
    }
}

impl SphereProgram {
    fn new(gl: &Gl) -> Result<Self, JsValue> {
        let program = link_program(gl, SPHERE_VS, SPHERE_FS)?;
        Ok(Self {
            u_view_proj: uniform(gl, &program, "u_view_proj")?,
            u_view: uniform(gl, &program, "u_view")?,
            program,
        })
    }
}

impl DotProgram {
    fn new(gl: &Gl) -> Result<Self, JsValue> {
        let program = link_program(gl, DOT_VS, DOT_FS)?;
        Ok(Self {
            u_view_proj: uniform(gl, &program, "u_view_proj")?,
            u_view: uniform(gl, &program, "u_view")?,
            u_size_scale: uniform(gl, &program, "u_size_scale")?,
            u_diameter: uniform(gl, &program, "u_diameter")?,
            u_color: uniform(gl, &program, "u_color")?,
            u_soft: uniform(gl, &program, "u_soft")?,
            program,
        })
    }
}

impl ArcProgram {
    fn new(gl: &Gl) -> Result<Self, JsValue> {
        let program = link_program(gl, ARC_VS, ARC_FS)?;
        Ok(Self {
            u_view_proj: uniform(gl, &program, "u_view_proj")?,
            u_progress: uniform(gl, &program, "u_progress")?,
            u_tail: uniform(gl, &program, "u_tail")?,
            u_size_scale: uniform(gl, &program, "u_size_scale")?,
            u_diameter: uniform(gl, &program, "u_diameter")?,
            program,
        })
    }
}

// ---------------------------------------------------------------------------
// Geometry
// ---------------------------------------------------------------------------
fn sphere_geometry(gl: &Gl) -> Result<Geometry, JsValue> {
    let (positions, normals, indices) = uv_sphere(BODY_RADIUS, 64, 32);

    let vao = create_vao(gl)?;
    gl.bind_vertex_array(Some(&vao));

    let pos_buf = upload(gl, Gl::ARRAY_BUFFER, &positions, Gl::STATIC_DRAW)?;
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, Gl::FLOAT, false, 0, 0);

    let norm_buf = upload(gl, Gl::ARRAY_BUFFER, &normals, Gl::STATIC_DRAW)?;
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_with_i32(1, 3, Gl::FLOAT, false, 0, 0);

    let idx_buf = upload(gl, Gl::ELEMENT_ARRAY_BUFFER, &indices, Gl::STATIC_DRAW)?;
    gl.bind_vertex_array(None);

    Ok(Geometry {
        vao,
        _buffers: vec![pos_buf, norm_buf, idx_buf],
        count: indices.len() as i32,
        mode: Gl::TRIANGLES,
    })
}

fn land_geometry(gl: &Gl) -> Result<Geometry, JsValue> {
    let mut positions = Vec::with_capacity(LAND_POSITIONS.len() * 3);
    for p in LAND_POSITIONS.iter() {
        positions.push(p[0] * LAND_RADIUS);
        positions.push(p[1] * LAND_RADIUS);
        positions.push(p[2] * LAND_RADIUS);
    }

    let vao = create_vao(gl)?;
    gl.bind_vertex_array(Some(&vao));
    let buf = upload(gl, Gl::ARRAY_BUFFER, &positions, Gl::STATIC_DRAW)?;
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, Gl::FLOAT, false, 0, 0);
    gl.bind_vertex_array(None);

    Ok(Geometry {
        vao,
        _buffers: vec![buf],
        count: LAND_POSITIONS.len() as i32,
        mode: Gl::POINTS,
    })
}

fn highlight_geometry(gl: &Gl) -> Result<Geometry, JsValue> {
    let mut positions = Vec::with_capacity(CITIES.len() * 3);
    for (lat, lon) in CITIES {
        let p = lat_lon_to_3d(lat, lon, LAND_RADIUS);
        positions.extend_from_slice(&[p.x, p.y, p.z]);
    }

    let vao = create_vao(gl)?;
    gl.bind_vertex_array(Some(&vao));
    let buf = upload(gl, Gl::ARRAY_BUFFER, &positions, Gl::STATIC_DRAW)?;
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, Gl::FLOAT, false, 0, 0);
    gl.bind_vertex_array(None);

    Ok(Geometry {
        vao,
        _buffers: vec![buf],
        count: CITIES.len() as i32,
        mode: Gl::POINTS,
    })
}

fn arc_geometry(gl: &Gl) -> Result<Geometry, JsValue> {
    // Interleaved: position (3), t (1), arc id (1) -> 20 bytes per point.
    let mut data = Vec::with_capacity(ROUTES.len() * (ARC_SEGMENTS + 1) * 5);

    for (arc_id, &(from_idx, to_idx)) in ROUTES.iter().enumerate() {
        let from = lat_lon_to_3d(CITIES[from_idx].0, CITIES[from_idx].1, GLOBE_RADIUS);
        let to = lat_lon_to_3d(CITIES[to_idx].0, CITIES[to_idx].1, GLOBE_RADIUS);
        let points = great_circle_arc(from, to, ARC_SEGMENTS, ARC_HEIGHT);

        for (i, p) in points.iter().enumerate() {
            let t = i as f32 / ARC_SEGMENTS as f32;
            data.extend_from_slice(&[p.x, p.y, p.z, t, arc_id as f32]);
        }
    }

    let vao = create_vao(gl)?;
    gl.bind_vertex_array(Some(&vao));
    let buf = upload(gl, Gl::ARRAY_BUFFER, &data, Gl::STATIC_DRAW)?;

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, Gl::FLOAT, false, 20, 0);
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_with_i32(1, 1, Gl::FLOAT, false, 20, 12);
    gl.enable_vertex_attrib_array(2);
    gl.vertex_attrib_pointer_with_i32(2, 1, Gl::FLOAT, false, 20, 16);

    gl.bind_vertex_array(None);

    Ok(Geometry {
        vao,
        _buffers: vec![buf],
        count: (data.len() / 5) as i32,
        mode: Gl::POINTS,
    })
}

fn uv_sphere(radius: f32, slices: u32, stacks: u32) -> (Vec<f32>, Vec<f32>, Vec<u16>) {
    let mut positions = Vec::with_capacity(((slices + 1) * (stacks + 1) * 3) as usize);
    let mut normals = Vec::with_capacity(((slices + 1) * (stacks + 1) * 3) as usize);
    let mut indices = Vec::with_capacity((slices * stacks * 6) as usize);

    for stack in 0..=stacks {
        let theta = std::f32::consts::PI * stack as f32 / stacks as f32;
        let y = theta.cos();
        let ring = theta.sin();

        for slice in 0..=slices {
            let phi = std::f32::consts::TAU * slice as f32 / slices as f32;
            let x = ring * phi.sin();
            let z = ring * phi.cos();

            normals.extend_from_slice(&[x, y, z]);
            positions.extend_from_slice(&[x * radius, y * radius, z * radius]);
        }
    }

    for stack in 0..stacks {
        for slice in 0..slices {
            let a = stack * (slices + 1) + slice;
            let b = a + slices + 1;

            indices.extend_from_slice(&[a as u16, b as u16, (a + 1) as u16]);
            indices.extend_from_slice(&[b as u16, (b + 1) as u16, (a + 1) as u16]);
        }
    }

    (positions, normals, indices)
}

// ---------------------------------------------------------------------------
// WebGL helpers
// ---------------------------------------------------------------------------
fn compile_shader(gl: &Gl, kind: u32, source: &str) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(kind)
        .ok_or_else(|| JsValue::from_str("gl.createShader returned null"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    let ok = gl
        .get_shader_parameter(&shader, Gl::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);

    if !ok {
        let log = gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "unknown shader error".to_string());
        gl.delete_shader(Some(&shader));
        return Err(JsValue::from_str(&format!(
            "shader compile failed: {log}\n--- source ---\n{source}"
        )));
    }

    Ok(shader)
}

fn link_program(
    gl: &Gl,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<WebGlProgram, JsValue> {
    let vertex = compile_shader(gl, Gl::VERTEX_SHADER, vertex_source)?;
    let fragment = compile_shader(gl, Gl::FRAGMENT_SHADER, fragment_source)?;

    let program = gl
        .create_program()
        .ok_or_else(|| JsValue::from_str("gl.createProgram returned null"))?;

    gl.attach_shader(&program, &vertex);
    gl.attach_shader(&program, &fragment);
    gl.link_program(&program);

    let ok = gl
        .get_program_parameter(&program, Gl::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);

    gl.delete_shader(Some(&vertex));
    gl.delete_shader(Some(&fragment));

    if !ok {
        let log = gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "unknown link error".to_string());
        gl.delete_program(Some(&program));
        return Err(JsValue::from_str(&format!("program link failed: {log}")));
    }

    Ok(program)
}

fn uniform(gl: &Gl, program: &WebGlProgram, name: &str) -> Result<WebGlUniformLocation, JsValue> {
    gl.get_uniform_location(program, name)
        .ok_or_else(|| JsValue::from_str(&format!("uniform {name} not found")))
}

fn create_vao(gl: &Gl) -> Result<WebGlVertexArrayObject, JsValue> {
    gl.create_vertex_array()
        .ok_or_else(|| JsValue::from_str("gl.createVertexArray returned null"))
}

fn upload<T>(gl: &Gl, target: u32, data: &[T], usage: u32) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl
        .create_buffer()
        .ok_or_else(|| JsValue::from_str("gl.createBuffer returned null"))?;

    gl.bind_buffer(target, Some(&buffer));
    gl.buffer_data_with_u8_array(target, as_bytes(data), usage);
    Ok(buffer)
}

fn as_bytes<T>(data: &[T]) -> &[u8] {
    // WebGL copies the data synchronously during bufferData, so this
    // temporary view into wasm linear memory is valid for the call.
    unsafe { std::slice::from_raw_parts(data.as_ptr().cast::<u8>(), std::mem::size_of_val(data)) }
}
