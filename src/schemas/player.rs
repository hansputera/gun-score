// source: https://github.com/a5huynh/defender-game/blob/master/src/models/player.rs

use opengl_graphics::{GlGraphics};
use graphics::{Context, Transformed};
use piston::window::Size;
use crate::geom::{Position, Direction, restrict_to_bounds};
use super::GameObject;
use crate::textures::{IconStruct, get_icon};

const PLAYER_DRIFT: f64 = 0.2;
const PLAYER_SPEED: f64 = 5.0;
const PLAYER_SIZE: f64 = 5.0;

pub struct Player {
    pub name: String,
    pub pos: Position,
    move_offset: Position,
    pub drift_ttl: f64,
    pub direction: Direction,
    pub size: f64,
    pub life: u8,
    pub icon: IconStruct,
    pub amunition: f64,
    pub shooting: bool,
    pub reloading: bool,
    pub score: f64,
}

impl Player {
    pub fn new(name: &String, x: &f64, y: &f64) -> Self {
        let position = Position::new(*x, *y);
        Player {
            name: name.to_string(),
            pos: position,
            move_offset: Position::new(0.0, 0.0),
            drift_ttl: 0.0,
            direction: Direction::EAST,
            size: PLAYER_SIZE,
            life: 3,
            icon: get_icon(&position),
            amunition: 500.0,
            shooting: false,
            reloading: false,
            score: 0.0,
        }
    }

    pub fn start_move(&mut self, dir: Direction) {
        self.direction = dir;
        match dir {
            Direction::WEST => self.move_offset.x = -PLAYER_SPEED,
            Direction::NORTH => self.move_offset.y = -PLAYER_SPEED,
            Direction::EAST => self.move_offset.x = PLAYER_SPEED,
            Direction::SOUTH => self.move_offset.y = PLAYER_SPEED,
        }
    }

    pub fn stop_move(&mut self, dir: Direction) {
        self.drift_ttl = PLAYER_DRIFT;
        match dir {
            Direction::WEST => self.move_offset.x = 0.0,
            Direction::NORTH => self.move_offset.y = 0.0,
            Direction::EAST => self.move_offset.x = 0.0,
            Direction::SOUTH => self.move_offset.y = 0.0,
        }
    }
}

impl GameObject for Player {
    fn position(&self) -> &Position { &self.pos }
    fn radius(&self) -> f64 { self.size / 2.0 }

    fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        let direction = match self.direction {
            Direction::WEST => 180.0,
            Direction::NORTH => 270.0,
            Direction::EAST => 0.0,
            Direction::SOUTH => 90.0,
        };

        let radius = self.radius();

        let mut transform = ctx.transform
            .trans(self.pos.x, self.pos.y)
            .rot_deg(direction)
            .trans(-radius, -radius);

        // flip the gun
        if self.direction == Direction::WEST {
            transform = transform.flip_v();
        }

        self.icon.img.draw(&self.icon.texture, &ctx.draw_state, transform, gl);
        drop(radius);
        drop(transform);
    }

    fn update(&mut self, dt: f64, size: Size) {
        let radius = self.radius();
        self.pos.x += self.move_offset.x;
        self.pos.y += self.move_offset.y;

        if self.drift_ttl > 0.0 {
            self.drift_ttl -= dt;
            let drift_speed = PLAYER_SPEED / 2.0;
            match self.direction {
                Direction::NORTH => self.pos.y -= drift_speed,
                Direction::EAST => self.pos.x += drift_speed,
                Direction::SOUTH => self.pos.y += drift_speed,
                Direction::WEST => self.pos.x -= drift_speed,
            }
        }

        restrict_to_bounds(&mut self.pos, [radius, radius, f64::from(size.width), f64::from(size.height)]);
        //drop(radius);
    }
}
