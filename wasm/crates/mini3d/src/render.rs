use wasm_bindgen::prelude::*;
use web_sys::{WebGlProgram, WebGlUniformLocation, WebGlVertexArrayObject};

use crate::app::Frame;
use crate::geo::great_circle_arc;
use crate::gfx::{
    create_vao, link_program, uniform, unit_quad, upload, uv_sphere, Buffer, BufferUsage, Geometry,
    Gl, RenderState,
};
use crate::material::{PointMaterial, PulseMaterial, QuadMaterial, SolidMaterial};
use crate::math::Vec3;

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

uniform vec3 u_base;
uniform vec3 u_key;
uniform vec3 u_rim;
uniform vec3 u_light;

in vec3 v_normal_v;
in vec3 v_pos_v;
out vec4 out_color;

void main() {
    vec3 n = normalize(v_normal_v);
    vec3 v = normalize(-v_pos_v);

    float key = pow(clamp(dot(n, normalize(u_light)), 0.0, 1.0), 2.0);
    vec3 base = u_base + u_key * key;

    float rim = pow(1.0 - clamp(dot(n, v), 0.0, 1.0), 3.0);
    base += u_rim * rim;

    out_color = vec4(base, 1.0);
}
"#;

const DOT_VS: &str = r#"#version 300 es
layout(location = 0) in vec3 a_pos;

uniform mat4 u_view_proj;
uniform mat4 u_view;
uniform float u_size_scale;
uniform float u_diameter;
uniform vec3 u_light;

out float v_fade;
out float v_shade;

void main() {
    vec4 view_pos = u_view * vec4(a_pos, 1.0);

    vec3 normal_v = normalize(mat3(u_view) * normalize(a_pos));
    vec3 view_dir = normalize(-view_pos.xyz);
    float facing = clamp(dot(normal_v, view_dir), 0.0, 1.0);
    v_fade = smoothstep(0.0, 0.18, facing);

    float key = clamp(dot(normal_v, normalize(u_light)), 0.0, 1.0);
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
uniform float u_fade;
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
    } else if (u_fade > 0.5) {
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

uniform vec3 u_cool;
uniform vec3 u_hot;

in float v_intensity;
out vec4 out_color;

void main() {
    float d = length(gl_PointCoord - vec2(0.5)) * 2.0;
    float alpha = (1.0 - smoothstep(0.55, 1.0, d)) * v_intensity;
    if (alpha <= 0.004) discard;

    vec3 color = mix(u_cool, u_hot, v_intensity * v_intensity);
    out_color = vec4(color * alpha, alpha);
}
"#;

const QUAD_VS: &str = r#"#version 300 es
layout(location = 0) in vec2 a_local;
layout(location = 1) in vec2 a_uv;
layout(location = 2) in vec3 a_pos;
layout(location = 3) in vec4 a_rot;
layout(location = 4) in vec2 a_scale;
layout(location = 5) in vec3 a_color;

uniform mat4 u_view_proj;
uniform mat4 u_view;

out vec2 v_uv;
out vec3 v_normal_v;
out vec3 v_color;

vec3 quat_rotate(vec4 q, vec3 v) {
    vec3 t = 2.0 * cross(q.xyz, v);
    return v + q.w * t + cross(q.xyz, t);
}

void main() {
    vec3 local_pos = vec3(a_local * a_scale, 0.0);
    vec3 world_pos = a_pos + quat_rotate(a_rot, local_pos);
    vec3 world_normal = quat_rotate(a_rot, vec3(0.0, 0.0, 1.0));

    v_uv = a_uv;
    v_normal_v = mat3(u_view) * world_normal;
    v_color = a_color;
    gl_Position = u_view_proj * vec4(world_pos, 1.0);
}
"#;

const QUAD_FS: &str = r#"#version 300 es
precision mediump float;

uniform vec3 u_light;
uniform float u_ambient;
uniform float u_emissive;
uniform float u_corner_radius;

in vec2 v_uv;
in vec3 v_normal_v;
in vec3 v_color;
out vec4 out_color;

void main() {
    if (u_corner_radius > 0.001) {
        vec2 q = abs(v_uv) - vec2(1.0 - u_corner_radius);
        float dist = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - u_corner_radius;
        if (dist > 0.0) discard;
    }

    vec3 n = normalize(v_normal_v);
    if (!gl_FrontFacing) {
        n = -n;
    }

    float diff = abs(dot(n, normalize(u_light)));
    float shade = u_ambient + (1.0 - u_ambient) * diff;
    float glint = pow(diff, 10.0) * 0.24;
    vec3 color = v_color * (shade + u_emissive) + vec3(glint);
    out_color = vec4(color, 1.0);
}
"#;

struct SolidProgram {
    program: WebGlProgram,
    u_view_proj: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
    u_base: WebGlUniformLocation,
    u_key: WebGlUniformLocation,
    u_rim: WebGlUniformLocation,
    u_light: WebGlUniformLocation,
}

impl SolidProgram {
    fn new(gl: &Gl) -> Result<Self, JsValue> {
        let program = link_program(gl, SPHERE_VS, SPHERE_FS)?;
        Ok(Self {
            u_view_proj: uniform(gl, &program, "u_view_proj")?,
            u_view: uniform(gl, &program, "u_view")?,
            u_base: uniform(gl, &program, "u_base")?,
            u_key: uniform(gl, &program, "u_key")?,
            u_rim: uniform(gl, &program, "u_rim")?,
            u_light: uniform(gl, &program, "u_light")?,
            program,
        })
    }
}

struct PointProgram {
    program: WebGlProgram,
    u_view_proj: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
    u_size_scale: WebGlUniformLocation,
    u_diameter: WebGlUniformLocation,
    u_color: WebGlUniformLocation,
    u_soft: WebGlUniformLocation,
    u_fade: WebGlUniformLocation,
    u_light: WebGlUniformLocation,
}

impl PointProgram {
    fn new(gl: &Gl) -> Result<Self, JsValue> {
        let program = link_program(gl, DOT_VS, DOT_FS)?;
        Ok(Self {
            u_view_proj: uniform(gl, &program, "u_view_proj")?,
            u_view: uniform(gl, &program, "u_view")?,
            u_size_scale: uniform(gl, &program, "u_size_scale")?,
            u_diameter: uniform(gl, &program, "u_diameter")?,
            u_color: uniform(gl, &program, "u_color")?,
            u_soft: uniform(gl, &program, "u_soft")?,
            u_fade: uniform(gl, &program, "u_fade")?,
            u_light: uniform(gl, &program, "u_light")?,
            program,
        })
    }
}

struct PulseProgram {
    program: WebGlProgram,
    u_view_proj: WebGlUniformLocation,
    u_progress: WebGlUniformLocation,
    u_tail: WebGlUniformLocation,
    u_size_scale: WebGlUniformLocation,
    u_diameter: WebGlUniformLocation,
    u_cool: WebGlUniformLocation,
    u_hot: WebGlUniformLocation,
}

impl PulseProgram {
    fn new(gl: &Gl) -> Result<Self, JsValue> {
        let program = link_program(gl, ARC_VS, ARC_FS)?;
        Ok(Self {
            u_view_proj: uniform(gl, &program, "u_view_proj")?,
            u_progress: uniform(gl, &program, "u_progress")?,
            u_tail: uniform(gl, &program, "u_tail")?,
            u_size_scale: uniform(gl, &program, "u_size_scale")?,
            u_diameter: uniform(gl, &program, "u_diameter")?,
            u_cool: uniform(gl, &program, "u_cool")?,
            u_hot: uniform(gl, &program, "u_hot")?,
            program,
        })
    }
}

struct QuadProgram {
    program: WebGlProgram,
    u_view_proj: WebGlUniformLocation,
    u_view: WebGlUniformLocation,
    u_light: WebGlUniformLocation,
    u_ambient: WebGlUniformLocation,
    u_emissive: WebGlUniformLocation,
    u_corner_radius: WebGlUniformLocation,
}

impl QuadProgram {
    fn new(gl: &Gl) -> Result<Self, JsValue> {
        let program = link_program(gl, QUAD_VS, QUAD_FS)?;
        Ok(Self {
            u_view_proj: uniform(gl, &program, "u_view_proj")?,
            u_view: uniform(gl, &program, "u_view")?,
            u_light: uniform(gl, &program, "u_light")?,
            u_ambient: uniform(gl, &program, "u_ambient")?,
            u_emissive: uniform(gl, &program, "u_emissive")?,
            u_corner_radius: uniform(gl, &program, "u_corner_radius")?,
            program,
        })
    }
}

/// Indexed 3D triangle mesh rendered with a [`SolidMaterial`].
pub struct Mesh {
    program: SolidProgram,
    geometry: Geometry,
    pub material: SolidMaterial,
}

impl Mesh {
    pub fn sphere(
        gl: &Gl,
        radius: f32,
        slices: u32,
        stacks: u32,
        material: SolidMaterial,
    ) -> Result<Self, JsValue> {
        let (positions, normals, indices) = uv_sphere(radius, slices, stacks);

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

        let geometry = Geometry {
            vao,
            buffers: vec![pos_buf, norm_buf, idx_buf],
            count: indices.len() as i32,
            mode: Gl::TRIANGLES,
        };

        Self::from_geometry(gl, geometry, material)
    }

    pub fn from_geometry(
        gl: &Gl,
        geometry: Geometry,
        material: SolidMaterial,
    ) -> Result<Self, JsValue> {
        Ok(Self {
            program: SolidProgram::new(gl)?,
            geometry,
            material,
        })
    }
}

/// Camera-facing point-sprite cloud rendered with a [`PointMaterial`].
pub struct Points {
    program: PointProgram,
    geometry: Geometry,
    pub material: PointMaterial,
}

impl Points {
    pub fn from_positions(
        gl: &Gl,
        positions: &[[f32; 3]],
        scale: f32,
        material: PointMaterial,
    ) -> Result<Self, JsValue> {
        let mut flat = Vec::with_capacity(positions.len() * 3);
        for p in positions {
            flat.push(p[0] * scale);
            flat.push(p[1] * scale);
            flat.push(p[2] * scale);
        }
        Self::from_flat(gl, &flat, material)
    }

    pub fn from_vec3(
        gl: &Gl,
        positions: &[Vec3],
        scale: f32,
        material: PointMaterial,
    ) -> Result<Self, JsValue> {
        let mut flat = Vec::with_capacity(positions.len() * 3);
        for p in positions {
            flat.push(p.x * scale);
            flat.push(p.y * scale);
            flat.push(p.z * scale);
        }
        Self::from_flat(gl, &flat, material)
    }

    pub fn from_flat(gl: &Gl, flat_xyz: &[f32], material: PointMaterial) -> Result<Self, JsValue> {
        let vao = create_vao(gl)?;
        gl.bind_vertex_array(Some(&vao));
        let buf = upload(gl, Gl::ARRAY_BUFFER, flat_xyz, Gl::STATIC_DRAW)?;
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, Gl::FLOAT, false, 0, 0);
        gl.bind_vertex_array(None);

        Ok(Self {
            program: PointProgram::new(gl)?,
            geometry: Geometry {
                vao,
                buffers: vec![buf],
                count: (flat_xyz.len() / 3) as i32,
                mode: Gl::POINTS,
            },
            material,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PulsePoint {
    pub pos: [f32; 3],
    pub t: f32,
    pub channel: f32,
}

/// Animated point-sprite trails along up to 3 channels.
pub struct PulseLine {
    program: PulseProgram,
    geometry: Geometry,
    pub material: PulseMaterial,
}

impl PulseLine {
    pub fn new(gl: &Gl, points: &[PulsePoint], material: PulseMaterial) -> Result<Self, JsValue> {
        let vao = create_vao(gl)?;
        gl.bind_vertex_array(Some(&vao));
        let buf = upload(gl, Gl::ARRAY_BUFFER, points, Gl::STATIC_DRAW)?;

        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, Gl::FLOAT, false, 20, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_with_i32(1, 1, Gl::FLOAT, false, 20, 12);
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_with_i32(2, 1, Gl::FLOAT, false, 20, 16);

        gl.bind_vertex_array(None);

        Ok(Self {
            program: PulseProgram::new(gl)?,
            geometry: Geometry {
                vao,
                buffers: vec![buf],
                count: points.len() as i32,
                mode: Gl::POINTS,
            },
            material,
        })
    }

    pub fn from_great_circles(
        gl: &Gl,
        routes: &[(Vec3, Vec3)],
        segments: usize,
        height: f32,
        material: PulseMaterial,
    ) -> Result<Self, JsValue> {
        let mut points = Vec::with_capacity(routes.len() * (segments + 1));

        for (channel, &(from, to)) in routes.iter().enumerate() {
            let arc = great_circle_arc(from, to, segments, height);
            for (i, p) in arc.iter().enumerate() {
                points.push(PulsePoint {
                    pos: p.to_array(),
                    t: i as f32 / segments as f32,
                    channel: channel as f32,
                });
            }
        }

        Self::new(gl, &points, material)
    }
}

/// Per-instance data for [`InstancedQuads`] (48 bytes per instance).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QuadInstance {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 2],
    pub color: [f32; 3],
}

/// Hardware-instanced 3D quads for particle systems, tumbling confetti, or cards.
pub struct InstancedQuads {
    program: QuadProgram,
    vao: WebGlVertexArrayObject,
    _quad_buffers: [Buffer; 2],
    instance_buffer: Buffer,
    instance_count: i32,
    capacity: usize,
    pub material: QuadMaterial,
}

impl InstancedQuads {
    pub fn new(gl: &Gl, max_instances: usize, material: QuadMaterial) -> Result<Self, JsValue> {
        let instance_buffer = Buffer::with_capacity::<QuadInstance>(
            gl,
            Gl::ARRAY_BUFFER,
            max_instances,
            BufferUsage::Dynamic,
        )?;
        Self::build(gl, instance_buffer, 0, max_instances, material)
    }

    pub fn with_instances(
        gl: &Gl,
        instances: &[QuadInstance],
        usage: BufferUsage,
        material: QuadMaterial,
    ) -> Result<Self, JsValue> {
        let instance_buffer = Buffer::new(gl, Gl::ARRAY_BUFFER, instances, usage)?;
        Self::build(
            gl,
            instance_buffer,
            instances.len() as i32,
            instances.len(),
            material,
        )
    }

    fn build(
        gl: &Gl,
        instance_buffer: Buffer,
        instance_count: i32,
        capacity: usize,
        material: QuadMaterial,
    ) -> Result<Self, JsValue> {
        let (quad_verts, quad_indices) = unit_quad();

        let vao = create_vao(gl)?;
        gl.bind_vertex_array(Some(&vao));

        let vert_buf = Buffer::new(gl, Gl::ARRAY_BUFFER, &quad_verts, BufferUsage::Static)?;
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 2, Gl::FLOAT, false, 16, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_with_i32(1, 2, Gl::FLOAT, false, 16, 8);

        let idx_buf = Buffer::new(
            gl,
            Gl::ELEMENT_ARRAY_BUFFER,
            &quad_indices,
            BufferUsage::Static,
        )?;

        instance_buffer.bind(gl);
        let stride = std::mem::size_of::<QuadInstance>() as i32; // 48 bytes
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_with_i32(2, 3, Gl::FLOAT, false, stride, 0);
        gl.vertex_attrib_divisor(2, 1);

        gl.enable_vertex_attrib_array(3);
        gl.vertex_attrib_pointer_with_i32(3, 4, Gl::FLOAT, false, stride, 12);
        gl.vertex_attrib_divisor(3, 1);

        gl.enable_vertex_attrib_array(4);
        gl.vertex_attrib_pointer_with_i32(4, 2, Gl::FLOAT, false, stride, 28);
        gl.vertex_attrib_divisor(4, 1);

        gl.enable_vertex_attrib_array(5);
        gl.vertex_attrib_pointer_with_i32(5, 3, Gl::FLOAT, false, stride, 36);
        gl.vertex_attrib_divisor(5, 1);

        gl.bind_vertex_array(None);

        Ok(Self {
            program: QuadProgram::new(gl)?,
            vao,
            _quad_buffers: [vert_buf, idx_buf],
            instance_buffer,
            instance_count,
            capacity,
            material,
        })
    }

    pub fn update(&mut self, gl: &Gl, instances: &[QuadInstance]) {
        let len = instances.len().min(self.capacity);
        self.instance_buffer.update(gl, &instances[..len]);
        self.instance_count = len as i32;
    }
}

pub struct Renderer {
    gl: Gl,
}

impl Renderer {
    pub fn new(gl: Gl) -> Self {
        gl.enable(Gl::DEPTH_TEST);
        gl.depth_func(Gl::LEQUAL);
        gl.enable(Gl::BLEND);
        gl.blend_func(Gl::ONE, Gl::ONE_MINUS_SRC_ALPHA);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear_depth(1.0);

        Self { gl }
    }

    pub fn gl(&self) -> &Gl {
        &self.gl
    }

    pub fn resize(&self, width: i32, height: i32) {
        self.gl.viewport(0, 0, width.max(1), height.max(1));
    }

    pub fn clear(&mut self) -> &mut Self {
        self.gl.depth_mask(true);
        self.gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
        self
    }

    pub fn mesh(&mut self, mesh: &Mesh, frame: &Frame) -> &mut Self {
        let gl = &self.gl;
        let prog = &mesh.program;
        let mat = &mesh.material;

        gl.use_program(Some(&prog.program));
        gl.uniform_matrix4fv_with_f32_array(Some(&prog.u_view_proj), false, &frame.view_proj);
        gl.uniform_matrix4fv_with_f32_array(Some(&prog.u_view), false, &frame.view);
        gl.uniform3fv_with_f32_array(Some(&prog.u_base), &mat.base);
        gl.uniform3fv_with_f32_array(Some(&prog.u_key), &mat.key);
        gl.uniform3fv_with_f32_array(Some(&prog.u_rim), &mat.rim);
        gl.uniform3fv_with_f32_array(Some(&prog.u_light), &mat.light);

        gl.depth_mask(true);
        gl.bind_vertex_array(Some(&mesh.geometry.vao));
        gl.draw_elements_with_i32(
            mesh.geometry.mode,
            mesh.geometry.count,
            Gl::UNSIGNED_SHORT,
            0,
        );
        gl.bind_vertex_array(None);
        self
    }

    pub fn points(&mut self, points: &Points, frame: &Frame) -> &mut Self {
        let gl = &self.gl;
        let prog = &points.program;
        let mat = &points.material;

        gl.use_program(Some(&prog.program));
        gl.uniform_matrix4fv_with_f32_array(Some(&prog.u_view_proj), false, &frame.view_proj);
        gl.uniform_matrix4fv_with_f32_array(Some(&prog.u_view), false, &frame.view);
        gl.uniform1f(Some(&prog.u_size_scale), frame.size_scale);
        gl.uniform1f(Some(&prog.u_diameter), mat.diameter);
        gl.uniform3fv_with_f32_array(Some(&prog.u_color), &mat.color);
        gl.uniform1f(Some(&prog.u_soft), if mat.soft { 1.0 } else { 0.0 });
        gl.uniform1f(
            Some(&prog.u_fade),
            if mat.fade_by_normal { 1.0 } else { 0.0 },
        );
        gl.uniform3fv_with_f32_array(Some(&prog.u_light), &mat.light);

        RenderState::SPRITE.apply(gl);
        gl.bind_vertex_array(Some(&points.geometry.vao));
        gl.draw_arrays(points.geometry.mode, 0, points.geometry.count);
        gl.bind_vertex_array(None);
        gl.depth_mask(true);
        self
    }

    pub fn pulse(&mut self, pulse: &PulseLine, progress: &[f32; 3], frame: &Frame) -> &mut Self {
        let gl = &self.gl;
        let prog = &pulse.program;
        let mat = &pulse.material;

        gl.use_program(Some(&prog.program));
        gl.uniform_matrix4fv_with_f32_array(Some(&prog.u_view_proj), false, &frame.view_proj);
        gl.uniform3f(
            Some(&prog.u_progress),
            progress[0],
            progress[1],
            progress[2],
        );
        gl.uniform1f(Some(&prog.u_tail), mat.tail);
        gl.uniform1f(Some(&prog.u_size_scale), frame.size_scale);
        gl.uniform1f(Some(&prog.u_diameter), mat.diameter);
        gl.uniform3fv_with_f32_array(Some(&prog.u_cool), &mat.cool_color);
        gl.uniform3fv_with_f32_array(Some(&prog.u_hot), &mat.hot_color);

        RenderState::SPRITE.apply(gl);
        gl.bind_vertex_array(Some(&pulse.geometry.vao));
        gl.draw_arrays(pulse.geometry.mode, 0, pulse.geometry.count);
        gl.bind_vertex_array(None);
        gl.depth_mask(true);
        self
    }

    pub fn quads(&mut self, quads: &InstancedQuads, frame: &Frame) -> &mut Self {
        if quads.instance_count <= 0 {
            return self;
        }

        let gl = &self.gl;
        let prog = &quads.program;
        let mat = &quads.material;

        mat.render_state().apply(gl);

        gl.use_program(Some(&prog.program));
        gl.uniform_matrix4fv_with_f32_array(Some(&prog.u_view_proj), false, &frame.view_proj);
        gl.uniform_matrix4fv_with_f32_array(Some(&prog.u_view), false, &frame.view);
        gl.uniform3fv_with_f32_array(Some(&prog.u_light), &mat.light);
        gl.uniform1f(Some(&prog.u_ambient), mat.ambient);
        gl.uniform1f(Some(&prog.u_emissive), mat.emissive);
        gl.uniform1f(Some(&prog.u_corner_radius), mat.corner_radius);

        gl.bind_vertex_array(Some(&quads.vao));
        gl.draw_elements_instanced_with_i32(
            Gl::TRIANGLES,
            6,
            Gl::UNSIGNED_SHORT,
            0,
            quads.instance_count,
        );
        gl.bind_vertex_array(None);
        gl.disable(Gl::CULL_FACE);
        gl.depth_mask(true);
        self
    }
}
