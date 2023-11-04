use super::{Editor, Mode};

impl Editor {
    pub(super) fn insert_mode(&mut self, event: sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyDown { keycode, .. } => {
                match keycode {
                    // Quit insert mode
                    Some(sdl2::keyboard::Keycode::Escape) => {
                        self.mode = Mode::Normal
                    },
                    // ...
                    Some(sdl2::keyboard::Keycode::Tab) => {
                        self.insert("  ");
                    },
                    // Move cursor 1 position to the left
                    Some(sdl2::keyboard::Keycode::Left) => {
                        self.move_left(1);
                    },
                    // Move cursor 1 position downward
                    Some(sdl2::keyboard::Keycode::Down) => {
                        self.move_down(1);
                    },
                    // Move cursor 1 position upward
                    Some(sdl2::keyboard::Keycode::Up) => {
                        self.move_up(1);
                    },
                    // Move cursor 1 position to the right
                    Some(sdl2::keyboard::Keycode::Right) => {
                        self.move_right(1);
                    },
                    // Insert a new line
                    Some(sdl2::keyboard::Keycode::Return) => {
                        self.insert_line();
                    },
                    // Delete character under cursor
                    Some(sdl2::keyboard::Keycode::Backspace) => {
                        self.delete();
                    },
                    _ => { },
                }
            },
            sdl2::event::Event::TextInput { text, .. } => {
                self.insert(&text);
            },
            _ => { },
        }
    }

    pub fn insert_line(&mut self) {
        let pos = self.cursor_pos();
        self.content.insert_str(pos, "\n");

        if self.cx < self.get_line_width(0) {
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

        self.content.insert_str(pos, text);

        self.lines[self.cy] += text.len() as u32;

        self.cx += text.len();
    }

    pub fn delete(&mut self) {
        if self.cy == 0 && self.cx == 0 {
            return;
        }

        let removed_char = self.content.remove(self.cursor_pos() - 1);
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

