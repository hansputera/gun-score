//source: https://github.com/a5huynh/defender-game/blob/master/src/models/bullet.rs

use graphics::{Context, Transformed, ellipse};
use opengl_graphics::{GlGraphics};

use piston::window::Size;

use crate::geom::{Position, Direction};
use crate::util::{get_random_number};
use crate::colors::{Colors};

use super::{GameObject};

const BULLET_SPEED: f64 = 2.0;
const BULLET_SIZE: f64 = 20.0;
const BULLET_LIFETIME: f64 = 2.0;

#[derive(PartialEq)]
pub enum Attacker {
    Monster,
    Player,
}

pub struct Bullet {
    pub pos: Position,
    pub direction: Direction,
    pub size: f64,
    pub ttl: f64,
    pub damage_count: f32,
    pub attacker: Attacker,
}

impl Bullet {
    pub fn new(x: f64, y: f64, direction: Direction, attacker: Attacker) -> Self {
        Bullet {
            pos: Position::new(x,y),
            direction,
            size: BULLET_SIZE,
            ttl: BULLET_LIFETIME,
            damage_count: get_random_number::<f32>(5.0, 20.0),
            attacker,
        }
    }

    pub fn radius(&self) -> f64 {
        self.size / 2.0
    }
}

impl GameObject for Bullet {
    fn position(&self) -> &Position {
        &self.pos
    }

    fn radius(&self) -> f64 {
        BULLET_SIZE
    }

    fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        let colors = Colors::init();
        let radius = self.radius();
        let transformer = ctx.transform
            .trans(self.pos.x, self.pos.y);
        ellipse(colors.black, [0.0, 0.0, radius, radius], transformer, gl);
        //clear
        drop(colors);
        drop(radius);
        drop(transformer);
    }

    fn update(&mut self, dt: f64, _: Size) {
        self.ttl -= dt;
        match self.direction {
            Direction::EAST => self.pos.x += BULLET_SPEED,
            Direction::NORTH => self.pos.y -= BULLET_SPEED,
            Direction::WEST => self.pos.x -= BULLET_SPEED,
            Direction::SOUTH => self.pos.y += BULLET_SPEED,
        }
    }
}
