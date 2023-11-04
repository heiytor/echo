use std::ffi::CString;

use super::Window;

// Util methods for render
impl Window {
    #[inline]
    fn padding_x(&self) -> f32 {
        self.atlas.max_w * self.scale
    }

    #[inline]
    fn padding_y(&self) -> f32 {
        (self.atlas.max_h * 1.5) * self.scale
    }

    /// ....
    unsafe fn draw_glyphs(&self, content: &str, mut x: f32, mut y: f32, colors: [f32; 4]) {
        let start_x = x;

        self.editor.t_program.bind();

        let color = gl::GetUniformLocation(self.editor.t_program.id, CString::new("textColor").unwrap().as_ptr());
        gl::Uniform3fv(color, 1, colors.as_ptr());
        gl::BindVertexArray(self.editor.t_program.vao);

        for c in content.chars() {
            if c == '\n' {
                x = start_x; 
                y -= self.padding_y();

                continue;
            }

            let ch = &self.atlas.characters[c as usize];

            let w = ch.char_w as f32 * self.scale;
            let h = ch.char_h as f32 * self.scale;
            let xpos = x + ch.char_l as f32 * self.scale;
            let ypos = y - (ch.char_h as f32 - ch.char_t as f32) * self.scale;

            let vertices: [f32; 24] = [
                xpos,     ypos,     0.0, 1.0,
                xpos,     ypos + h, 0.0, 0.0,
                xpos + w, ypos + h, 1.0, 0.0,

                xpos,     ypos,     0.0, 1.0,
                xpos + w, ypos + h, 1.0, 0.0,
                xpos + w, ypos,     1.0, 1.0,
            ];

            gl::BindTexture(gl::TEXTURE_2D, ch.tex_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.editor.t_program.vbo);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const _);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            // TODO:
            // Currently, we render the text assuming a monospace font by always advancing 
            // by `self.atlas.max_w`. Ideally, we should use `x += ch.advance_x` for 
            // proportional fonts. However, an issue arises when dealing with a variable-width 
            // cursor, such as the cursor potentially exceeding the current character's bounds.
            x += self.padding_x();
        }

        gl::BindVertexArray(0);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
}

impl Window {
    pub fn next_frame(&self) {
        println!("{} | {} | {}", self.width, self.padding_y(), self.height/ self.padding_y());

        unsafe {
            // self.render_bar();
            self.render_text();
            self.render_cursor();
        }
    }

    unsafe fn render_text(&self) {
        self.draw_glyphs(
            self.editor.content.as_str(),
            self.atlas.max_w,
            self.height as f32 - self.atlas.max_h,
            self.theme.fg()
        );
    }

    unsafe fn render_cursor(&self) {
        self.editor.c_program.bind();

        let w: f32;
        let h: f32;
        let x: f32;
        let y: f32;

        match self.editor.mode {
            crate::editor::Mode::Insert => {
                w = (self.atlas.max_w * 0.1) * self.scale;
                h = self.padding_y();

                x = (self.editor.cx as f32 * self.atlas.max_w) + self.atlas.max_w;
                y = (self.height as f32 - self.atlas.max_h * 1.5) - (self.editor.cy as f32 * self.atlas.max_h * 1.5);
            },
            _ => {
                w = self.atlas.max_w * self.scale;
                h = self.padding_y();

                x = self.editor.cx as f32 * self.atlas.max_w;
                y = (self.height as f32 - self.atlas.max_h * 1.5) - (self.editor.cy as f32 * self.atlas.max_h * 1.5);
            },
        };

        let vertices: [f32; 24] = [
            x,     y,     0.0, 0.0,
            x + w, y,     1.0, 0.0,
            x + w, y + h, 1.0, 1.0,

            x,     y,     0.0, 0.0,
            x + w, y + h, 1.0, 1.0,
            x,     y + h, 0.0, 1.0,
        ];

        let color = gl::GetUniformLocation(self.editor.c_program.id, CString::new("cursorColor").unwrap().as_ptr());
        gl::Uniform4fv(color, 1, self.theme.cs().as_ptr());

        gl::BindVertexArray(self.editor.c_program.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.editor.c_program.vbo);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const _);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        gl::BindVertexArray(0);
    }

    unsafe fn render_bar(&self) {
        self.editor.c_program.bind();

        // Draw bar
        let w = self.width;
        let h = self.atlas.max_h * 3.0;

        let x = 0.0;
        let y = 0.0;

        let vertices: [f32; 24] = [
            x,     y,     0.0, 0.0,
            x + w, y,     1.0, 0.0,
            x + w, y + h, 1.0, 1.0,

            x,     y,     0.0, 0.0,
            x + w, y + h, 1.0, 1.0,
            x,     y + h, 0.0, 1.0,
        ];

        let color = gl::GetUniformLocation(self.editor.c_program.id, CString::new("cursorColor").unwrap().as_ptr());
        gl::Uniform4fv(color, 1, vec![0.0, 0.0, 0.0, 1.0].as_ptr());

        gl::BindVertexArray(self.editor.c_program.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.editor.c_program.vbo);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, vertices.as_ptr() as *const _);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        gl::BindVertexArray(0);

        self.left_bar();
        self.right_bar();
    }

    unsafe fn left_bar(&self) {
        let l1 = format!("{}", self.editor.mode);
        self.draw_glyphs(
            l1.as_str(),
            self.atlas.max_w, self.atlas.max_h,
            self.theme.fg()
        );

        let l2 = format!("{}", "~/something/foo.rs");
        self.draw_glyphs(
            l2.as_str(),
            self.atlas.max_w, self.atlas.max_h * 2.0,
            self.theme.fg()
        );
    }

    /// In the end, the screen should render something like:
    /// ```txt
    /// ~ 1,1       0%
    /// ~          fps
    /// ```
    unsafe fn right_bar(&self) {
        let l1 = format!("{:.1$}", self.fps, 1);
        self.draw_glyphs(
            l1.as_str(),
            self.width - (l1.len() as f32 * self.atlas.max_w) - self.atlas.max_w, self.atlas.max_h,
            self.theme.fg()
        );
        
        // - Since cx and cy represent the (x,y) coordinates starting from (0,0), we add 1 to both 
        //   to obtain a more intuitive number.
        let l2 = format!("{},{}\t\t\t\t\t{}", self.editor.cx + 1, self.editor.cy + 1, "TOP");
        self.draw_glyphs(
            l2.as_str(),
            self.width - (l2.len() as f32 * self.atlas.max_w) - self.atlas.max_w, self.atlas.max_h * 2.0,
            self.theme.fg()
        );
    }
}
