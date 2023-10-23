use std::ffi::{CStr, CString};

pub fn new_shader(src: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut status = gl::FALSE as gl::types::GLint;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
    }

    if status != (gl::TRUE as gl::types::GLint) {
        // TODO: handle errors with GetShaderInfoLog
        return Err("error at Shader::new".to_string());
    }

    Ok(id)
}

pub struct Program {
    pub id: gl::types::GLuint,
    pub shaders_id: Vec<gl::types::GLuint>,
    pub vbo: gl::types::GLuint,
    pub vao: gl::types::GLuint,
}

impl Program {
    pub fn new(shaders: &[gl::types::GLuint], width: f32, height: f32) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        let mut shaders_id = vec![];
        for shader in shaders {
            unsafe {
                gl::AttachShader(id, *shader);
            }

            shaders_id.push(*shader);
        }

        unsafe { gl::LinkProgram(id) };

        let mut status = gl::FALSE as gl::types::GLint;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);
        }

        if status != (gl::TRUE as gl::types::GLint) {
            // TODO: handle errors with GetProgramInfoLog
            return Err("error at Program::new".to_string());
        }
        
        let projection: nalgebra::Matrix4<f32> = nalgebra::Orthographic3::new(0.0, width, 0.0, height, -1.0, 1.0).to_homogeneous();
        unsafe {
            gl::UseProgram(id);
            let loc = gl::GetUniformLocation(id, CString::new("projection").unwrap().as_ptr());
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, projection.as_ptr());
        }

        let mut vbo: gl::types::GLuint = 0;
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (6 * 4 * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr, std::ptr::null(), gl::DYNAMIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, (4 * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizei, std::ptr::null());
        }

        // Deactivate current buffer, vertex array and shader program
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }

        Ok(Self {
            id,
            shaders_id,
            vbo,
            vao,
        })
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) };
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            for s_id in &mut self.shaders_id {
                gl::DeleteShader(*s_id);
            }
            gl::DeleteProgram(self.id);
        }
    }
}
