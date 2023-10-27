use std::ffi::CString;

use crate::shader::{Program, new_shader};

pub struct Cursor {
    pub program: Program,
    pub position: (f32, f32),
}

impl Cursor {
    pub fn new(app_w: f32, app_h: f32) -> Result<Self, String>  {
        let program = Program::new(
            &[
                new_shader(&CString::new(include_str!("./shaders/cursor.v.glsl")).unwrap(), gl::VERTEX_SHADER)?,
                new_shader(&CString::new(include_str!("./shaders/cursor.f.glsl")).unwrap(), gl::FRAGMENT_SHADER)?,
            ],
            app_w,
            app_h,
        )?;

        Ok(Self {
            program,
            position: (0.0, 0.0),
        })
    }

    pub fn set_x(&mut self, x: f32) {
        self.position.0 = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.position.1 = y;
    }
}

