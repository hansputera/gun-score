use graphics::{color};

pub type ColorComps = [f32; 4];
pub struct Colors {
    pub white: ColorComps,
    pub green: ColorComps,
    pub red: ColorComps,
    pub black: ColorComps,
}

impl Colors {
    pub fn init() -> Self {
        Colors {
            white: color::hex("FFFFFF"),
            green: color::hex("549EA0"),
            red: color::hex("F33333"),
            black: color::hex("000000"),
        }
    }
}

