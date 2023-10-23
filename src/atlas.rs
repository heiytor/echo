#[derive(Clone)]
pub struct Character {
    pub tex_id: gl::types::GLuint, // ID da textura do GL
    pub char_h: i32,
    pub char_w: i32,
    pub char_l: i32,
    pub char_t: i32,
    pub advance_x: f32,                  // Quantidade para mover para o próximo caractere
    pub advance_y: f32,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            tex_id: 0,
            char_h: 0,
            char_w: 0,
            char_l: 0,
            char_t: 0,
            advance_x: 0.0,
            advance_y: 0.0,
        }
    }
}

pub struct Chars {
    pub characters: Vec<Character>,
    /// guarda a altura máxima em pixel necessária para o maior glifo
    pub max_h: u32,
    /// guarda a largura máxima em pixel necessária para o maior glifo
    pub max_w: u32,
}

impl Chars {
    ///
    ///
    ///
    pub fn new(font: &str, height: u32) -> Result<Self, String> {
        let ft = freetype::Library::init().unwrap();

        let face = ft.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, height).unwrap();

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        let glyph = face.glyph();

        // ...
        const MARGIN: u32 = 1;
        const MAX_WIDTH: u32 = 1024;

        let mut tex_h: u32 = 0;
        let mut tex_w: u32 = 0;
        let mut row_h: u32 = 0;
        let mut row_w: u32 = 0;
        let mut max_h: u32 = 0;
        let mut max_w: u32 = 0;

        // ...
        for char in 32..128 {
            face.load_char(char, freetype::face::LoadFlag::RENDER).unwrap();

            let char_h = glyph.bitmap().rows() as u32;
            let char_w = glyph.bitmap().width() as u32;

            if row_w + char_w + MARGIN >= MAX_WIDTH {
                tex_h = tex_h + char_h;
                tex_w = std::cmp::max(tex_w, char_w);

                row_h = 0;
                row_w = 0;
            }

            row_h = std::cmp::max(row_h, char_h);
            row_w = row_w + char_w + MARGIN;

            max_w = std::cmp::max(max_w, char_w);
            max_h = std::cmp::max(max_h, char_h);
        }

        let mut characters: Vec<Character> = vec![Default::default(); 128];

        for char in 32..128 {
            face.load_char(char, freetype::face::LoadFlag::RENDER).unwrap();

            let bitmap = glyph.bitmap();

            let mut tex_id = char as gl::types::GLuint;

            unsafe {
                gl::GenTextures(1, &mut tex_id);
                gl::BindTexture(gl::TEXTURE_2D, tex_id);

                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    bitmap.width() as i32,
                    bitmap.rows() as i32,
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    bitmap.buffer().as_ptr() as *const _,
                );

                gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as gl::types::GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as gl::types::GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as gl::types::GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);
            }

            characters[char] = Character {
                tex_id,
                char_w: bitmap.width(),
                char_h: bitmap.rows(),
                char_l: glyph.bitmap_left(),
                char_t: glyph.bitmap_top(),
                advance_x: (glyph.advance().x >> 6) as f32,
                advance_y: (glyph.advance().y >> 6) as f32,
            };
        }

        Ok(Chars {
            characters,
            max_h,
            max_w,
        })
    }
}
