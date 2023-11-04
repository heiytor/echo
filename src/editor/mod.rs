use std::{ffi::CString, fmt::{Display, Formatter}};

use crate::shader::{Program, new_shader};

pub enum Mode {
    Insert,
    Normal,
    Visual,
}


impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Mode::Insert => write!(f, "Insert"),
            Mode::Normal => write!(f, "Normal"),
            Mode::Visual => write!(f, "Visual"),
        }
    }
}

pub struct Editor {
    pub mode: Mode,
    cmd_stack: String,

    // ===============
    // Text properties
    // ===============

    /// The OpenGL program responsible for rendering the text in the editor.
    pub t_program: Program,
    /// The content (text/code) of the editor.
    pub content: String,
    /// Each element of the vector represents the total number of characters in the corresponding line of the content.
    lines: Vec<u32>,

    // =================
    // Cursor properties
    // =================
    
    /// OpenGL program of the cursor.
    pub c_program: Program,
    /// Represents the cursor's X position, which is actually the index of the text to modify.
    pub cx: usize,
    cx_keep: usize,
    /// Represents the cursor's Y position, which is the current line of the cursor.
    pub cy: usize,
}

impl Editor {
    pub fn new(app_w: f32, app_h: f32, data: &str) -> Result<Self, String> {
        let t_program = Program::new(
            &[
                new_shader(&CString::new(include_str!("../shader/char.v.glsl")).unwrap(), gl::VERTEX_SHADER).unwrap(),
                new_shader(&CString::new(include_str!("../shader/char.f.glsl")).unwrap(), gl::FRAGMENT_SHADER).unwrap(),
            ],
            app_w, 
            app_h,
        )?;

        let c_program = Program::new(
            &[
                new_shader(&CString::new(include_str!("../shader/cursor.v.glsl")).unwrap(), gl::VERTEX_SHADER)?,
                new_shader(&CString::new(include_str!("../shader/cursor.f.glsl")).unwrap(), gl::FRAGMENT_SHADER)?,
            ],
            app_w,
            app_h,
        )?;

        Ok(Self {
            mode: Mode::Normal,
            cmd_stack: "".to_string(),

            t_program,
            content: data.to_string(),
            lines: vec![0],

            c_program,

            cx: 0,
            cx_keep: 0,

            cy: 0,
        })
    }
}

impl Editor {
    pub fn event(&mut self, event: sdl2::event::Event) {
        match self.mode {
            Mode::Insert => {
                self.insert_mode(event)
            },
            Mode::Normal => {
                self.normal_mode(event)
            },
            Mode::Visual => {
                self.visual_mode(event)
            },
        }
    }
}

// util implementations
impl Editor {
    /// ...
    #[inline]
    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode
    }

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

    #[inline]
    fn line(&self) -> usize {
        if self.lines.len() == 1 {
            return 0;
        }
        
        self.lines[0..self.cy]
            .iter()
            .fold(0, |acc, line| acc + 1 + *line as usize)
    }
    
    /// Determines the absolute cursor position considering both its line and column positions.
    #[inline]
    fn cursor_pos(&self) -> usize {
        self.line() + self.cx
    }

    #[inline]
    fn content_lines(&self) -> usize {
        self.lines.len() - 1
    }
}

pub mod movement;
pub mod insert_mode;
pub mod normal_mode;
pub mod visual_mode;
