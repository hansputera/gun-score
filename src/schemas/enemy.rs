//source: https://github.com/a5huynh/defender-game/blob/master/src/models/enemy.rs

use graphics::{Context, rectangle, Transformed};
use opengl_graphics::{GlGraphics};

use crate::util::{get_random_number_float};
use crate::geom::{Position, restrict_to_bounds};
use crate::colors::Colors;

use piston::window::Size;

use super::GameObject;

pub struct Enemy {
    pub health: f64,
    pub pos: Position,
    pub size: f64, // enemy's body size
    move_ttl: f64,
}

const MOVE_RADIUS: f64 = 6.0;
const MOVE_TTL: f64 = 0.1; // 0.1 secs / 100 ms
const ENEMY_RADIUS: f64 = 10.0;

impl Enemy {
    pub fn new(x: f64, y: f64) -> Self {
        Enemy {
            health: get_random_number_float(10.0, 1000.0),
            move_ttl: MOVE_TTL,
            pos: Position::new(x,y),
            size: ENEMY_RADIUS * 2.0,
        }
    }

    // generate random monster/enemy.
    pub fn new_rand(max_x: f64, max_y: f64) -> Enemy {
        let randx = get_random_number_float(0.0, max_x);
        let randy = get_random_number_float(0.0, max_y);
        Enemy::new(randx, randy)
    }
}

impl GameObject for Enemy {
    fn position(&self) -> &Position {
        &self.pos
    }

    fn radius(&self) -> f64 {
        self.size / 2.0
    }

    fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        let colors = Colors::init();
        // TODO: change the enemy's body to image.
        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.radius();
        let transform = ctx.transform.trans(self.pos.x, self.pos.y)
            .trans(-radius, -radius);

        rectangle(colors.red, square, transform, gl);
        //get out
        drop(square);
        drop(colors);
        drop(transform);
    }

    fn update(&mut self, dt: f64, size: Size) {
        // Only move every <MOVE_TTL> seconds
        self.move_ttl -= dt;
        if self.move_ttl <= 0.0 {
            // Randomly move in a random direction.
            let radius = self.radius();
            self.pos.x += get_random_number_float(0.0, MOVE_RADIUS * 2.0)
                - MOVE_RADIUS;
            self.pos.y += get_random_number_float(0.0, MOVE_RADIUS * 2.0)
                - MOVE_RADIUS;

            restrict_to_bounds(
                &mut self.pos,
                [radius, radius, f64::from(size.width), f64::from(size.height)]
                );
                // Don't move outside the bounds of the window.
                self.move_ttl = MOVE_TTL;
        }
    }
}

pub type Monster = Enemy;
