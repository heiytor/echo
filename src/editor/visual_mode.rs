use super::{Editor, Mode};

impl Editor {
    pub(super) fn visual_mode(&mut self, event: sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyDown { keycode, .. } => {
                match keycode {
                    // Quit visual mode
                    Some(sdl2::keyboard::Keycode::Escape) => {
                        self.mode = Mode::Normal
                    },
                    _ => { },
                }
            },
            _ => { },
        }
    }
}
