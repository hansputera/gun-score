use crate::geom::Position;
use graphics::{Context};
use opengl_graphics::GlGraphics;
use piston::window::Size;

pub mod player;
pub mod enemy;

pub trait GameObject {
    fn tabrakan(&self, object: &dyn GameObject) -> bool {
        let x = self.position().x - object.position().x;
        let y = self.position().y - object.position().y;
        let sums = x.powf(2.0) + y.powf(2.0);

        let radius_start = self.radius() - object.radius();
        let radius_end = self.radius() + object.radius();
        radius_start.powf(2.0) <= sums && sums <= radius_end.powf(2.0)
    }

    fn position(&self) -> &Position;
    fn radius(&self) -> f64;

    fn render(&self, ctx: &Context, gl: &mut GlGraphics);
    fn update(&mut self, _: f64, _: Size) {}
}
