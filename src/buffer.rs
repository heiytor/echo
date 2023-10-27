use std::ffi::CString;

use crate::{shader::{Program, new_shader}, util::usize_sub};

pub struct Buffer {
    pub program: Program,
    pub data: String,

    /// Stores the total number of characters per line in the buffer.
    lines: Vec<u32>,
    /// Represents the cursor's X position, which is actually the index of the text to modify.
    pub cx: usize,
    /// Represents the cursor's Y position, which is the current line of the cursor.
    pub cy: usize,
}

impl Buffer {
    /// TODO: maybe we need to handle errors here?
    /// Gets the line `n` based on the cursor position.
    ///
    /// ### Example
    /// ```
    /// self.cy = 10;
    ///
    /// let previous_line = self.get_line(-2);
    /// let same_line = self.get_line(0);
    /// let next_line = self.get_line(2);
    ///
    /// assert_eq!(previous_line, 8);
    /// assert_eq!(same_line, 10);
    /// assert_eq!(next_line, 12);
    /// ```
    #[inline]
    fn get_line(&self, n: i32) -> usize {
        if n < 0 && self.cy == 0 {
            0
        } else {
            (self.cy as i32 + (n)) as usize
        }
    }

    /// TODO: maybe we need to handle errors here?
    /// Gets the width of the specified line.
    ///
    /// ### Example
    /// ```
    /// self.lines = vec![5];
    /// self.cy = 0;
    ///
    /// assert_eq!(get_line_width(0), 5);
    /// ```
    #[inline]
    fn get_line_width(&self, n: i32) -> usize {
        self.lines[self.get_line(n)] as usize
    }

    fn line(&self) -> usize {
        if self.lines.len() == 1 {
            return 0;
        }
        
        self.lines[0..self.cy]
            .iter()
            .fold(0, |acc, line| acc + 1 + *line as usize)
    }

    ///  ...
    #[inline]
    pub fn cursor_pos(&self) -> usize {
        self.line() + self.cx
    }

}

impl Buffer {
    pub fn up(&mut self, n: usize) {
        if self.cy != 0 {
            self.cy = usize_sub(self.cy, n);

            let line_width = self.get_line_width(0);
            if self.cx > line_width {
                self.cx = line_width;
            }
        }
    }

    pub fn down(&mut self, n: usize) {
        if self.cy < self.lines.len() - 1 {
            self.cy += n;

            let line_width = self.get_line_width(0);
            if self.cx > line_width {
                self.cx = line_width;
            }
        }
    }

    /// Moves the cursor `n` digits to right.
    pub fn right(&mut self, n: usize) {
        // println!("r: line={} ; before: n={} cursor={}", self.cy, n, self.cx);

        let line_width = self.get_line_width(0);
        let new_cx = self.cx + n;

        if (n != 1 && new_cx >= line_width) || (new_cx > line_width && self.cy == self.lines.len() - 1) {
            self.cx = line_width;
        } else if new_cx > line_width {
            self.cx = 0;
            self.cy += 1;
        } else {
            self.cx = new_cx;
        }

        // println!("r: line={} ; after: n={} cursor={}", self.cy, n, self.cx);
    }

    /// Moves the cursor `n` digits to left.
    pub fn left(&mut self, n: usize) {
        let line = self.get_line(0);

        if (line == 0 && n > self.cx) || (n != 1 && n > self.cx ) {
            self.cx = 0;
        } else if n > self.cx {
            self.cx = self.get_line_width(-1);
            self.cy -= 1;
        } else {
            self.cx -= n;
        }
    }
}

impl Buffer {
    pub fn new(width: f32, height: f32, data: &str) -> Result<Self, String> {
        let program = Program::new(
            &[
                new_shader(&CString::new(include_str!("./shaders/char.v.glsl")).unwrap(), gl::VERTEX_SHADER).unwrap(),
                new_shader(&CString::new(include_str!("./shaders/char.f.glsl")).unwrap(), gl::FRAGMENT_SHADER).unwrap(),
            ],
            width, 
            height,
        )?;

        Ok(Self {
            program,
            data: data.to_string(),
            cx: 0,
            cy: 0,
            lines: vec![0],
        })
    }

    pub fn insert_line(&mut self) {
        let pos = self.cursor_pos();
        self.data.insert_str(pos, "\n");

        if self.cx < self.lines[self.cy] as usize {
            let chars_to_move = self.lines[self.cy] as usize - self.cx;
            self.lines.insert(self.cy + 1, chars_to_move as u32);
            self.lines[self.cy] -= chars_to_move as u32;
        } else {
            self.lines.insert(self.cy + 1, 0);
        }

        self.cy += 1;
        self.cx = 0;
    }

    pub fn insert(&mut self, text: &str) {
        let pos = self.cursor_pos();

        self.data.insert_str(pos, text);

        self.lines[self.cy] += text.len() as u32;

        self.cx += text.len();
    }

    pub fn delete(&mut self) {
        if self.cy == 0 && self.cx == 0 {
            return;
        }

        let removed_char = self.data.remove(self.cursor_pos() - 1);
        if removed_char == '\n' {
            self.cx = self.lines[self.cy - 1] as usize;
            self.lines[self.cy - 1] += self.lines[self.cy];

            self.lines.remove(self.cy);

            self.cy -= 1;

            return
        }

        self.cx -= 1;
        self.lines[self.cy] -= 1;
    }
}
