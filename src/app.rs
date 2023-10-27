use std::ffi::CString;

use crate::{atlas::Chars, theme::Theme, cursor::Cursor, buffer::Buffer};

#[allow(dead_code)]
pub struct App {
    /// Current width of the window.
    pub w_width: u32,

    /// Current height of the window.
    pub w_height: u32,

    pub w_theme: Theme,

    /// Font bitmap
    pub atlas: Chars,

    buffer: Buffer, // TODO: must store buffer inside editor
    cursor: Cursor,
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
    RenderBuffer,
}

impl App {
    pub fn new(w_width: u32, w_height: u32, font: &str, font_h: u32) -> Result<Self, String> {
        let atlas = Chars::new(font, font_h).unwrap();
        let buffer = Buffer::new(w_width as f32, w_height as f32, "").unwrap();

        let mut cursor = Cursor::new(w_width as f32, w_height as f32)?;
        cursor.set_x(0.0 + atlas.max_w as f32);
        cursor.set_y((w_height - atlas.max_h) as f32);

        Ok(App {
            atlas,

            w_width,
            w_height,
            w_theme: Theme::default(),

            buffer,
            cursor,
        })
    }

    pub fn handle_event(&mut self, event: sdl2::event::Event) -> WindowEvent {
        // println!("******************************************");
        // for i in 0..self.buffer.lines.len() {
        //     println!("lines[{}]: {}", i, self.buffer.lines[i]);
        // }

        match event {
            sdl2::event::Event::Quit { .. } => {
                WindowEvent::Quit
            },
            sdl2::event::Event::KeyDown { keycode, .. } => {
                match keycode {
                    Some(sdl2::keyboard::Keycode::Backspace) => {
                        self.buffer.delete();


                        // println!("lenght: {}", self.buffer.lenght);
                        // println!("cursor_pos: {}", self.buffer.cursor_pos);

                        WindowEvent::Nothing
                    },
                    Some(sdl2::keyboard::Keycode::Return) => {
                        // self.buffer.insert("\n");
                        self.buffer.insert_line();

                        WindowEvent::Nothing
                    },
                    Some(sdl2::keyboard::Keycode::Tab) => {
                        self.buffer.insert("  ");

                        WindowEvent::Nothing
                    },


                    Some(sdl2::keyboard::Keycode::Left) => {
                        self.buffer.left(1);

                        WindowEvent::Nothing
                    },
                    Some(sdl2::keyboard::Keycode::Right) => {
                        self.buffer.right(1);

                        WindowEvent::Nothing
                    },
                    Some(sdl2::keyboard::Keycode::Up) => {
                        self.buffer.up(1);

                        return WindowEvent::Nothing;
                    },
                    Some(sdl2::keyboard::Keycode::Down) => {
                        self.buffer.down(1);

                        WindowEvent::Nothing
                    },

                    _ => {
                        WindowEvent::Nothing
                    },
                }
            },
            sdl2::event::Event::MouseWheel { .. } => {
                // println!("scroll do mouse");

                WindowEvent::RenderBuffer
            },
            sdl2::event::Event::TextInput { text, .. } => {
                self.buffer.insert(&text);

                // println!("lenght: {}", self.buffer.lenght);
                // println!("cursor_pos: {}", self.buffer.cursor_pos);

                WindowEvent::Nothing
            },
            _ => {
                WindowEvent::RenderBuffer
            },
        }
    }

    pub fn render_frame(&self, scale: f32) {
        let mut x: f32 = 0.0 + self.atlas.max_w as f32;
        let mut y: f32 = (self.w_height - self.atlas.max_h) as f32;
        let tmp_x = x;

        // draw self.buffer.data
        self.buffer.program.bind();

        unsafe {
            let color_location = gl::GetUniformLocation(self.buffer.program.id, CString::new("textColor").unwrap().as_ptr());
            gl::Uniform3fv(color_location, 1, self.w_theme.fg().as_ptr());

            gl::BindVertexArray(self.buffer.program.vao);

            for (i, line) in self.buffer.data.lines().enumerate() {
                // render the line number
                let line_num_str = format!("{:4}", i + 1);
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
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer.program.vbo);
                    gl::BufferSubData(gl::ARRAY_BUFFER, 0, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const _);
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);

                    x += ch.advance_x * scale;
                    y += ch.advance_y * scale;
                }

                x += 25.0;

                // render the line self.buffer.data
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

                    // println!("text:");
                    // for i in 0..6 {
                    //     let start = i * 4;
                    //     println!("{}: {:.1}, {:.1}, {:.1}, {:.1}", c, vertices[start], vertices[start + 1], vertices[start + 2], vertices[start + 3]);
                    // }

                    gl::BindTexture(gl::TEXTURE_2D, ch.tex_id);
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer.program.vbo);
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

        // // draw cursor
        // self.cursor.program.bind();
        // unsafe {
        //     // x = self.cursor.position.0;
        //     // y = self.cursor.position.1;
        //
        //     x = self.buffer.cx as f32 * self.atlas.max_w as f32;
        //     y = self.w_height as f32 - (self.buffer.cy as f32 * self.atlas.max_h as f32);
        //     println!("{}", y);
        //
        //     // Definindo os vértices para o cursor.
        //     let vertices: [f32; 24] = [
        //         x,                            y,                            0.0, 0.0,
        //         x + self.atlas.max_w as f32,  y,                            1.0, 0.0,
        //         x + self.atlas.max_w as f32,  y + self.atlas.max_h as f32,  1.0, 1.0,
        //
        //         x,                            y,                            0.0, 0.0,
        //         x + self.atlas.max_w as f32,  y + self.atlas.max_h as f32,  1.0, 1.0,
        //         x,                            y + self.atlas.max_h as f32,  0.0, 1.0,
        //     ];
        //
        //     println!("cursor:");
        //     for i in 0..6 {
        //         let start = i * 4;
        //         println!("{:.1}, {:.1}, {:.1}, {:.1}", vertices[start], vertices[start + 1], vertices[start + 2], vertices[start + 3]);
        //     }
        //
        //     let color_location = gl::GetUniformLocation(self.cursor.program.id, CString::new("cursorColor").unwrap().as_ptr());
        //     gl::Uniform3fv(color_location, 1, self.w_theme.fg().as_ptr());
        //
        //     gl::BindVertexArray(self.cursor.program.vao);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, self.cursor.program.vbo);
        //     gl::BufferSubData(gl::ARRAY_BUFFER, 0, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const _);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        //     gl::DrawArrays(gl::TRIANGLES, 0, 6);
        //
        //     gl::BindVertexArray(0);
        // }
    }
}
