use graphics::{Image, rectangle};
use opengl_graphics::{Texture, TextureSettings};
use std::path::Path;

use crate::geom::{Position};

pub struct IconStruct {
    pub img: Image,
    pub texture: Texture,
}

pub fn get_icon(pos: &Position) -> IconStruct {
    let image = Image::new()
        .rect(rectangle::square(pos.x, pos.y, 50.0));
    let texture = Texture::from_path(
        Path::new("gun.png"), 
        &TextureSettings::new()).unwrap();

    IconStruct {
        img: image,
        texture: texture,
    }
}

