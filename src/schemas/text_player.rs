use graphics::{Context};
use opengl_graphics::{GlGraphics};

use crate::schemas::player::{Player};
use crate::schemas::enemy::{Enemy};
use crate::geom::{Position};
use crate::textures::{TextDraw};
use crate::colors::{Colors};

pub struct TextPlayer<'a> {
    pub player: Player,
    pub text_draw: TextDraw<'a>,
}

impl<'a> TextPlayer<'a> {
    pub fn new(player: &Player, text_draw: TextDraw<'a>) -> Self {
        TextPlayer {
            player,
            text_draw,
        }
    }

    pub fn position(&self) -> &Position {
        &Position::new(
            self.player.pos.x,
            self.player.pos.y - 20.0, // keatas
        );
    }

    pub fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
       let colors = Colors::init();
       let position = self.position();

       self.text_draw.draw(&self.player.name,  &colors.black, &[
                           position.x,
                           position.y,
       ], &15, &ctx, gl);

       drop(colors);
       drop(position);
    }
}

