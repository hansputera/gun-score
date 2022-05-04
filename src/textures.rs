use graphics::{Text, Image, rectangle, Context, Transformed, DrawState};
use opengl_graphics::{GlGraphics, Texture, GlyphCache, TextureSettings};
use std::path::Path;

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

pub fn load_cache(font_name: String) -> &'static GlyphCache<'static> {
    let font = format_args!("assets/{}", font_name.to_string());
    let cache = GlyphCache::from_font(Path::new(font), (), TextureSettings::new())
        .expect(format_args!("Unable to load {} font!", font));

    drop(font);
    cache.as_ref()
}

pub struct TextDraw<'a> {
    pub cache: GlyphCache<'a>,
    pub gl: GlGraphics,
}

impl<'a> TextDraw<'a> {
    pub fn new(cache: &GlyphCache<'a>, gl: &mut GlGraphics) -> Self {
        TextDraw {
            cache: *cache,
            gl: *gl,
        }
    }

    // source: https://github.com/a5huynh/defender-game/blob/master/src/gfx/utils.rs
    
    // text = text want to draw
    // color: ([0] = f64, [1] = f64, [2] = f64, [3] = f64, [4] = f64)
    // pos = ([0] = x, [1] = y)
    // size = font size
    // ctx = window context
    pub fn draw(&self, text: &String, color: &ColorComps, pos: &[f64; 2], size: &u32, ctx: &Context) {
        let transformer = ctx.transform
            .trans(pos[0], pos[1]); // set the text position.
        Text::new_color(*color, *size)
            .draw(&text.to_string(), &mut self.cache, &DrawState::default(), transformer, &mut self.gl)
            .unwrap();

        drop(transformer);
    }

    pub fn draw_center(&self, text: &String, color: &ColorComps, size: &u32, bounds: &[f64; 2], ctx: &Context) {
        let half_size = f64::from(size) / 2.0;
        
        let x = (bounds[0] / 2.0) - ((text.len() as f64) * half_size) / 2.0;
        let y = (bounds[1] / 2.0) - half_size;

        self.draw(text, color, &[x, y], size, ctx);

        drop(x);
        drop(y);
        drop(half_size);
    }
}

