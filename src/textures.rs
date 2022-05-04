use graphics::{Text, Image, rectangle, Context, Transformed, DrawState};
use opengl_graphics::{GlGraphics, Texture, GlyphCache, TextureSettings};
use std::path::Path;

use rusttype::{Font};

use crate::geom::{Position};
use crate::colors::{ColorComps};

pub struct IconStruct {
    pub img: Image,
    pub texture: Texture,
}

pub fn get_icon(pos: &Position) -> IconStruct {
    let image = Image::new()
        .rect(rectangle::square(pos.x, pos.y, 50.0));
    let texture = Texture::from_path(
        Path::new("assets/gun.png"), 
        &TextureSettings::new()).unwrap();

    IconStruct {
        img: image,
        texture: texture,
    }
}

pub fn load_font(font_bytes: &'static [u8]) -> Font<'_> {
    Font::try_from_bytes(font_bytes).expect("Unable to get font from u8-bytes")
}

pub fn load_cache(font_bytes: &'static [u8]) -> GlyphCache<'_> {
    GlyphCache::from_font(load_font(font_bytes), (), TextureSettings::new())
}


pub struct TextDraw<'a> {
    pub cache: GlyphCache<'a>,
}

impl<'a> TextDraw<'a> {
    pub fn new(cache: GlyphCache<'a>) -> Self {
        TextDraw {
            cache: cache,
        }
    }

    // source: https://github.com/a5huynh/defender-game/blob/master/src/gfx/utils.rs
    
    // text = text want to draw
    // color: ([0] = f64, [1] = f64, [2] = f64, [3] = f64, [4] = f64)
    // pos = ([0] = x, [1] = y)
    // size = font size
    // ctx = window context
    // gl = GlGraphics
    pub fn draw(&mut self, text: &String, color: &ColorComps, pos: &[f64; 2], size: &u32, ctx: &Context, gl: &mut GlGraphics) {
        let transformer = ctx.transform
            .trans(pos[0], pos[1]); // set the text position.
        Text::new_color(*color, *size)
            .draw(&text.to_string(), &mut self.cache, &DrawState::default(), transformer, gl)
            .unwrap();

        drop(transformer);
    }

    pub fn draw_center(&mut self, text: &String, color: &ColorComps, size: &u32, bounds: &[f64; 2], ctx: &Context, gl: &mut GlGraphics) {
        let half_size = f64::from(*size) / 2.0;
        
        let x = (bounds[0] / 2.0) - ((text.len() as f64) * half_size) / 2.0;
        let y = (bounds[1] / 2.0) - half_size;

        self.draw(text, color, &[x, y], size, ctx, gl);

        drop(x);
        drop(y);
        drop(half_size);
    }
}

