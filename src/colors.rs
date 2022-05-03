use graphics::{color};

type ColorComps = [f32; 4];
pub struct Colors {
    pub white: ColorComps,
    pub green: ColorComps,
    pub red: ColorComps,
}

impl Colors {
    pub fn init() -> Self {
        Colors {
            white: color::hex("FFFFFF"),
            green: color::hex("549EA0"),
            red: color::hex("F33333"),
        }
    }
}

