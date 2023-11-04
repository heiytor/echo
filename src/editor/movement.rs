use crate::util::usize_sub;

use super::Editor;

impl Editor {
    /// if n = 0, move to the beginning of the file.
    pub fn move_up(&mut self, n: usize) {
        if n == 0 {
            self.cy = 0;
            return;
        }

        if self.cy != 0 {
            self.cy = usize_sub(self.cy, n);

            let line_width = self.get_line_width(0);
            if self.cx > line_width {
                self.cx = line_width;
            }
        }
    }

    /// if n = 0, move to the end of the file.
    pub fn move_down(&mut self, n: usize) {
        if n == 0 {
            self.cy = self.content_lines();

            return;
        }

        if self.cy + n > self.content_lines() {
            self.cy = self.content_lines();
        } else if self.cy < self.content_lines() {
            self.cy += n;

            if self.cx > self.get_line_width(0) {
                self.cx = self.get_line_width(0);
            }
        }
    }

    /// Moves the cursor `n` digits to right.
    /// If n = 0, move to the end of the line.
    pub fn move_right(&mut self, n: usize) {
        if n == 0 {
            self.cx = self.get_line_width(0);
            return;
        }

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
    }

    /// Moves the cursor `n` digits to left.
    /// If n = 0, move to the beginning of the line.
    pub fn move_left(&mut self, n: usize) {
        if n == 0 {
            self.cx = 0;
            return;
        }

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

