use std::ffi::CString;

use crate::{atlas::Chars, shader::{Program, new_shader}, theme::Theme};


#[allow(dead_code)]
pub struct Window {
    pub current_text: String, // TODO: put in editor

    /// Current width of the window.
    pub w_width: u32,

    /// Current height of the window.
    pub w_height: u32,

    pub w_theme: Theme,

    /// Font bitmap
    pub atlas: Chars,

    pub char_shader: Program,
    pub cursor_shader: Program,
}

//
//
//
//
//
// TODO: REMOVE SOME UNWRAPPERS
//
//
//
//
//

pub enum WindowEvent {
    Quit,
    Nothing,
}

impl Window {
    pub fn new(width: u32, height: u32, font: &str, font_h: u32) -> Self {
        let atlas = Chars::new(font, font_h).unwrap();

        let char_shader = Program::new(&[
           new_shader(&CString::new(include_str!("./shaders/char.v.glsl")).unwrap(), gl::VERTEX_SHADER).unwrap(),
           new_shader(&CString::new(include_str!("./shaders/char.f.glsl")).unwrap(), gl::FRAGMENT_SHADER).unwrap(),
        ], width as f32, height as f32).unwrap();

        let cursor_shader = Program::new(&[
           new_shader(&CString::new(include_str!("./shaders/cursor.v.glsl")).unwrap(), gl::VERTEX_SHADER).unwrap(),
           new_shader(&CString::new(include_str!("./shaders/cursor.f.glsl")).unwrap(), gl::FRAGMENT_SHADER).unwrap(),
        ], width as f32, height as f32).unwrap();

        Window {
            current_text: "".to_string(),
            atlas,
            w_width: width,
            w_height: height,
            w_theme: Theme::default(),
            char_shader,
            cursor_shader,
        }
    }

    pub fn handle_event(&mut self, event: sdl2::event::Event) -> WindowEvent {
        match event {
            sdl2::event::Event::Quit { .. } => {
                WindowEvent::Quit
            },
            sdl2::event::Event::KeyDown { keycode, .. } => {
                match keycode {
                    Some(sdl2::keyboard::Keycode::Backspace) => {
                        if self.current_text.len() == 0 {
                            return WindowEvent::Nothing;
                        }

                        self.current_text.pop();
                        return WindowEvent::Nothing;
                    },
                    Some(sdl2::keyboard::Keycode::Return) => {
                        self.current_text.push('\n');
                        return WindowEvent::Nothing;
                    },
                    Some(sdl2::keyboard::Keycode::Tab) => {
                        self.current_text.push(' ');
                        self.current_text.push(' ');
                        return WindowEvent::Nothing;
                    },
                    Some(sdl2::keyboard::Keycode::Up) => {

                        return WindowEvent::Nothing;
                    },
                    Some(sdl2::keyboard::Keycode::Down) => {

                        return WindowEvent::Nothing;
                    },
                    Some(sdl2::keyboard::Keycode::Left) => {

                        return WindowEvent::Nothing;
                    },
                    Some(sdl2::keyboard::Keycode::Right) => {

                        return WindowEvent::Nothing;
                    },
                    _ => {
                        return WindowEvent::Nothing;
                    },
                }
            }
            sdl2::event::Event::TextInput { text, .. } => {
                self.current_text.push(text.chars().next().unwrap());
                return WindowEvent::Nothing;
            },
            _ => {
                WindowEvent::Nothing
            },
        }
    }

    pub fn render_frame(&self, text: &str, mut x: f32, mut y: f32, scale: f32, color: [f32; 3]) {
        // draw text
        let tmp_x = x;
        let tmp_y = y;
        self.char_shader.bind();

        unsafe {
            let color_location = gl::GetUniformLocation(self.char_shader.id, CString::new("textColor").unwrap().as_ptr());
            gl::Uniform3fv(color_location, 1, color.as_ptr());

            gl::BindVertexArray(self.char_shader.vao);

            // render the line number
            for (i, line) in text.lines().enumerate() {
                let line_num_str = format!("{:3}", i + 1);
                for c in line_num_str.chars() {
                    let ch = &self.atlas.characters[c as usize];

                    let xpos = x + ch.char_l as f32 * scale;
                    let ypos = y - (ch.char_h as f32 - ch.char_t as f32) * scale;

                    let w = ch.char_w as f32 * scale;
                    let h = ch.char_h as f32 * scale;

                    let vertices: [f32; 24] = [
                        xpos,     ypos,       0.0, 1.0,
                        xpos,     ypos + h,   0.0, 0.0,
                        xpos + w, ypos + h,   1.0, 0.0,

                        xpos,     ypos,       0.0, 1.0,
                        xpos + w, ypos + h,   1.0, 0.0,
                        xpos + w, ypos,       1.0, 1.0,
                    ];

                    // Renderizar o quad para o caractere
                    gl::BindTexture(gl::TEXTURE_2D, ch.tex_id);
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.char_shader.vbo);
                    gl::BufferSubData(gl::ARRAY_BUFFER, 0, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const _);
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);

                    x += ch.advance_x * scale;
                    y += ch.advance_y * scale;
                }

                x += 25.0;

                // render the line text
                for c in line.chars() {
                    if c == '\n' {
                        x = tmp_x; 
                        y -= self.atlas.max_h as f32 * scale;
                        continue;
                    }

                    let ch = &self.atlas.characters[c as usize];

                    let xpos = x + ch.char_l as f32 * scale;
                    let ypos = y - (ch.char_h as f32 - ch.char_t as f32) * scale;

                    let w = ch.char_w as f32 * scale;
                    let h = ch.char_h as f32 * scale;

                    let vertices: [f32; 24] = [
                        xpos,     ypos,       0.0, 1.0,
                        xpos,     ypos + h,   0.0, 0.0,
                        xpos + w, ypos + h,   1.0, 0.0,

                        xpos,     ypos,       0.0, 1.0,
                        xpos + w, ypos + h,   1.0, 0.0,
                        xpos + w, ypos,       1.0, 1.0,
                    ];

                    // Renderizar o quad para o caractere
                    gl::BindTexture(gl::TEXTURE_2D, ch.tex_id);
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.char_shader.vbo);
                    gl::BufferSubData(gl::ARRAY_BUFFER, 0, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const _);
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);

                    x += ch.advance_x * scale;
                    y += ch.advance_y * scale;
                }

                x = tmp_x;
                y -= self.atlas.max_h as f32 * scale;
            }

            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        
        // draw cursor
        self.cursor_shader.bind();

        x = tmp_x;
        y = tmp_y;

        // Definindo os vértices para o cursor.
        let vertices: [f32; 24] = [
            x,                            y,                            0.0, 0.0,
            x + self.atlas.max_w as f32,  y,                            1.0, 0.0,
            x + self.atlas.max_w as f32,  y + self.atlas.max_h as f32,  1.0, 1.0,

            x,                            y,                            0.0, 0.0,
            x + self.atlas.max_w as f32,  y + self.atlas.max_h as f32,  1.0, 1.0,
            x,                            y + self.atlas.max_h as f32,  0.0, 1.0,
        ];

        unsafe {
            let color_location = gl::GetUniformLocation(self.cursor_shader.id, CString::new("cursorColor").unwrap().as_ptr());
            gl::Uniform3fv(color_location, 1, color.as_ptr());

            gl::BindVertexArray(self.cursor_shader.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.cursor_shader.vbo);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const _);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::BindVertexArray(0);
        }
    }
}

