use wasm_bindgen::prelude::*;
pub use web_sys::WebGl2RenderingContext as Gl;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlShader, WebGlUniformLocation, WebGlVertexArrayObject,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferUsage {
    Static,
    Dynamic,
}

impl BufferUsage {
    pub fn to_gl(self) -> u32 {
        match self {
            Self::Static => Gl::STATIC_DRAW,
            Self::Dynamic => Gl::DYNAMIC_DRAW,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendMode {
    Opaque,
    PremultipliedAlpha,
    Additive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CullMode {
    None,
    Back,
    Front,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderState {
    pub depth_test: bool,
    pub depth_write: bool,
    pub blend: BlendMode,
    pub cull: CullMode,
}

impl RenderState {
    pub const OPAQUE: Self = Self {
        depth_test: true,
        depth_write: true,
        blend: BlendMode::Opaque,
        cull: CullMode::Back,
    };

    pub const SPRITE: Self = Self {
        depth_test: true,
        depth_write: false,
        blend: BlendMode::PremultipliedAlpha,
        cull: CullMode::None,
    };

    pub fn apply(&self, gl: &Gl) {
        if self.depth_test {
            gl.enable(Gl::DEPTH_TEST);
            gl.depth_func(Gl::LEQUAL);
        } else {
            gl.disable(Gl::DEPTH_TEST);
        }

        gl.depth_mask(self.depth_write);

        match self.blend {
            BlendMode::Opaque => gl.disable(Gl::BLEND),
            BlendMode::PremultipliedAlpha => {
                gl.enable(Gl::BLEND);
                gl.blend_func(Gl::ONE, Gl::ONE_MINUS_SRC_ALPHA);
            }
            BlendMode::Additive => {
                gl.enable(Gl::BLEND);
                gl.blend_func(Gl::ONE, Gl::ONE);
            }
        }

        match self.cull {
            CullMode::None => gl.disable(Gl::CULL_FACE),
            CullMode::Back => {
                gl.enable(Gl::CULL_FACE);
                gl.cull_face(Gl::BACK);
            }
            CullMode::Front => {
                gl.enable(Gl::CULL_FACE);
                gl.cull_face(Gl::FRONT);
            }
        }
    }
}

pub struct Buffer {
    raw: WebGlBuffer,
    target: u32,
}

impl Buffer {
    pub fn new<T>(gl: &Gl, target: u32, data: &[T], usage: BufferUsage) -> Result<Self, JsValue> {
        let raw = upload(gl, target, data, usage.to_gl())?;
        Ok(Self { raw, target })
    }

    pub fn with_capacity<T>(
        gl: &Gl,
        target: u32,
        len: usize,
        usage: BufferUsage,
    ) -> Result<Self, JsValue> {
        let raw = gl
            .create_buffer()
            .ok_or_else(|| JsValue::from_str("gl.createBuffer returned null"))?;
        let byte_len = (len * std::mem::size_of::<T>()) as i32;
        gl.bind_buffer(target, Some(&raw));
        gl.buffer_data_with_i32(target, byte_len, usage.to_gl());
        Ok(Self { raw, target })
    }

    pub fn bind(&self, gl: &Gl) {
        gl.bind_buffer(self.target, Some(&self.raw));
    }

    pub fn update<T>(&self, gl: &Gl, data: &[T]) {
        gl.bind_buffer(self.target, Some(&self.raw));
        gl.buffer_sub_data_with_i32_and_u8_array(self.target, 0, as_bytes(data));
    }

    pub fn raw(&self) -> &WebGlBuffer {
        &self.raw
    }
}

pub struct Geometry {
    pub vao: WebGlVertexArrayObject,
    pub buffers: Vec<WebGlBuffer>,
    pub count: i32,
    pub mode: u32,
}

pub fn uv_sphere(radius: f32, slices: u32, stacks: u32) -> (Vec<f32>, Vec<f32>, Vec<u16>) {
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

/// Unit quad in the XY plane (`[-0.5, 0.5]^2`), interleaved `(x, y, u, v)` where `u, v in [-1, 1]`.
pub fn unit_quad() -> ([f32; 16], [u16; 6]) {
    let vertices = [
        -0.5, -0.5, -1.0, -1.0, // bottom-left
        0.5, -0.5, 1.0, -1.0, // bottom-right
        0.5, 0.5, 1.0, 1.0, // top-right
        -0.5, 0.5, -1.0, 1.0, // top-left
    ];
    let indices = [0, 1, 2, 0, 2, 3];
    (vertices, indices)
}

pub fn compile_shader(gl: &Gl, kind: u32, source: &str) -> Result<WebGlShader, JsValue> {
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

pub fn link_program(
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

pub fn uniform(gl: &Gl, program: &WebGlProgram, name: &str) -> Result<WebGlUniformLocation, JsValue> {
    gl.get_uniform_location(program, name)
        .ok_or_else(|| JsValue::from_str(&format!("uniform {name} not found")))
}

pub fn create_vao(gl: &Gl) -> Result<WebGlVertexArrayObject, JsValue> {
    gl.create_vertex_array()
        .ok_or_else(|| JsValue::from_str("gl.createVertexArray returned null"))
}

pub fn upload<T>(gl: &Gl, target: u32, data: &[T], usage: u32) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl
        .create_buffer()
        .ok_or_else(|| JsValue::from_str("gl.createBuffer returned null"))?;

    gl.bind_buffer(target, Some(&buffer));
    gl.buffer_data_with_u8_array(target, as_bytes(data), usage);
    Ok(buffer)
}

pub fn as_bytes<T>(data: &[T]) -> &[u8] {
    // WebGL copies the data synchronously during bufferData / bufferSubData,
    // so this temporary view into wasm linear memory is valid for the call.
    unsafe { std::slice::from_raw_parts(data.as_ptr().cast::<u8>(), std::mem::size_of_val(data)) }
}
