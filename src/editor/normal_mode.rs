use super::{Editor, Mode};

impl Editor {
    pub(super) fn normal_mode(&mut self, event: sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyDown { keycode, .. } => {
                match keycode {
                    // Switch to normal mode
                    Some(sdl2::keyboard::Keycode::Escape) => {
                        self.set_mode(Mode::Normal);
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
                    _ => {},
                };
            },
            sdl2::event::Event::TextInput { text, .. } => {
                match text.as_str() {
                    // Switch to insert mode before the current character
                    "i" => {
                        self.move_left(1);
                        self.set_mode(Mode::Insert);
                    },
                    // Switch to insert mode at the beginning of the line
                    "I" => {
                        self.move_left(0);
                        self.set_mode(Mode::Insert);
                    },
                    // Switch to insert mode after the current character
                    "a" => {
                        self.set_mode(Mode::Insert);
                    },
                    // Switch to insert mode at the end of the line
                    "A" => {
                        self.move_right(0);
                        self.set_mode(Mode::Insert);
                    },
                    // Insert a new line below the current one and switch to insert mode
                    "o" => {
                        self.move_right(0);
                        self.insert_line();
                        self.set_mode(Mode::Insert);
                    },
                    // Insert a new line above the current one and switch to insert mode
                    "O" => {
                        self.move_left(0);
                        self.insert_line();
                        self.move_up(1);
                        self.set_mode(Mode::Insert);
                    },
                    // Switch to visual mode
                    "v" => {
                        self.set_mode(Mode::Visual);
                    },
                    // Switch to visual line mode
                    "V" => {
                        self.set_mode(Mode::Visual);
                    },
                    // Move cursor `n` positions to the left
                    "h" => {
                        let n = self.cmd_stack.parse::<usize>().unwrap_or(1);
                        self.move_left(n);
                        self.cmd_stack = "".to_string();
                    },
                    // Move cursor `n` positions downward
                    "j" => {
                        let n = self.cmd_stack.parse::<usize>().unwrap_or(1);
                        self.move_down(n);
                        self.cmd_stack = "".to_string();
                    },
                    // Move cursor `n` positions upward
                    "k" => {
                        let n = self.cmd_stack.parse::<usize>().unwrap_or(1);
                        self.move_up(n);
                        self.cmd_stack = "".to_string();
                    },
                    // Move cursor `n` positions to the right
                    "l" => {
                        let n = self.cmd_stack.parse::<usize>().unwrap_or(1);
                        self.move_right(n);
                        self.cmd_stack = "".to_string();
                    },
                    // Move cursor to the beginning of the file if "gg"
                    "g" => {
                        if self.cmd_stack == "g" {
                            self.move_up(0);
                            self.move_left(0);
                            self.cmd_stack = "".to_string();
                        } else {
                            self.stack_cmd(text.as_str());
                        }
                    },
                    // Move cursor to the end of the file
                    "G" => {
                        self.move_down(0);
                        self.move_right(0);
                    },
                    // Begin delete command or delete current line if "dd"
                    "d" => {
                        if self.cmd_stack == "d" {
                            self.delete_line();
                            self.cmd_stack = "".to_string();
                        } else {
                            self.stack_cmd(text.as_str());
                        }
                    },
                    // Add the command to the stack for further processing
                    _ => {
                        self.stack_cmd(text.as_str());
                    },
                };
            },
            _ => { },
        };
    }

    fn stack_cmd(&mut self, cmd: &str) {
        if let Some(first_char) = cmd.chars().next() {
            self.cmd_stack.push(first_char);
        }
    }

    fn delete_line(&mut self) {
        // Check if the line exists
        if self.cy >= self.lines.len() {
            return;
        }

        let start_pos = self.line();
        let end_pos = start_pos + self.get_line_width(0);

        self.content.drain(start_pos..=end_pos);

        // Remove the line from the lines vector
        self.lines.remove(self.cy);

        // If the deleted line was the last line, move the cursor up
        if self.cy >= self.lines.len() {
            self.cy -= 1;
            if self.cx > self.lines[self.cy] as usize {
                self.cx = self.lines[self.cy] as usize;
            }
        }
    }
}
