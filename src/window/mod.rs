mod theme;
use crate::{editor, atlas};


#[allow(dead_code)]
pub struct Window {
    pub theme: theme::Theme,
    width: f32,
    height: f32,

    fps: f64,

    scale: f32,
    atlas: atlas::Atlas,
    editor: editor::Editor,
}

pub enum WindowEvent {
    Quit,
    Nothing,
    RenderBuffer,
}

impl Window {
    pub fn new(w_width: f32, w_height: f32, font: &str, font_h: u32) -> Result<Self, String> {
        let w_theme = theme::Theme::default();
        let editor = editor::Editor::new(w_width as f32, w_height as f32, "")?;
        let atlas = atlas::Atlas::new(font, font_h)?;

        Ok(Window {
            atlas,

            width: w_width,
            height: w_height,
            theme: w_theme,

            fps: 0.0,

            scale: 1.0,
            editor,
        })
    }

    pub fn handle_event(&mut self, event: sdl2::event::Event) -> WindowEvent {
        match event {
            sdl2::event::Event::Quit { .. } => {
                WindowEvent::Quit
            },
            _ => {
                self.editor.event(event);

                WindowEvent::Nothing
            }
        }
    }

    pub fn set_fps(&mut self, fps: f64) {
        self.fps = fps;
    }
}

pub mod render;
