use opengl_graphics::{GlGraphics, OpenGL};
use glutin_window::GlutinWindow as Window;
use piston::window::{WindowSettings, Size};
use piston::input::*;
use graphics::{clear};

use crate::colors::{Colors};
use crate::schemas::player::{Player};
use crate::schemas::enemy::{Monster};
use crate::geom::{Direction};
use crate::schemas::{GameObject};
use crate::textures::{load_cache, TextDraw};

#[derive(PartialEq)]
enum GameStatus {
    Fight, // fighting mode
    Win, // all monster have been killed
    Lose, // could be dead
}

pub struct GunScoreApp<'a> {
    pub gl: GlGraphics,
    pub window: Window,
    pub player: Player,
    pub monsters: Vec<Monster>,
    status: GameStatus,
    pub text_draw: TextDraw<'a>,
}

impl GunScoreApp<'_> {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let window_settings = WindowSettings::new("gun-score", [500, 500])
            .graphics_api(opengl);
        let glyph = load_cache(include_bytes!("../assets/SF_Atarian_System.ttf"));
        
        // glyph access on 'TextDraw.cache'

        GunScoreApp {
            window: window_settings.build().unwrap(),
            gl: GlGraphics::new(opengl),
            player: Player::new(&"Tono".to_string(), &0.0, &0.0),
            monsters: Vec::new(),
            status: GameStatus::Fight,
            text_draw: TextDraw::new(glyph),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let colors = Colors::init();
        let size = &self.get_size();
        
        // player
        let (x, y) = (self.player.pos.x, self.player.pos.y);
        let player = &self.player;

        // monster/enemy
        let enemies = &self.monsters;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(colors.white, gl);

            // check the game status.
            match self.status {
                GameStatus::Lose => {
                    self.text_draw.draw_center(&String::from("YOU LOSE!"), &colors.red, &32, &[
                                               f64::from(size.width),
                                               f64::from(size.height),
                    ], &c,gl);
                },
                _ => (),
            }

            // render player
            player.render(&c, gl);

            // render monster/enemy
            for enemy in enemies.iter() {
                enemy.render(&c, gl);
            }
        });
        drop(size);
        drop(colors);
        drop(x);
        drop(y);
    }

    fn get_size(&self) -> Size {
        let size = self.window.ctx.window().inner_size()
            .to_logical::<u32>(self.window.ctx.window().scale_factor());
        (size.width, size.height).into()
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let size = &self.get_size();
        // player update
        self.player.update(args.dt, *size);
        
        // enemies/monsters update
        if self.monsters.is_empty() {
            for _ in 1..10 {
                self.monsters.push(Monster::new_rand(f64::from(size.width), f64::from(size.height)));
            }
        }

        for monster in &mut self.monsters {
            monster.update(args.dt, *size);
            if monster.tabrakan(&self.player) {
                // TODO: game over
                if self.player.life <= 1 {
                    self.status = GameStatus::Lose;
                } else {
                    self.player.life -= 1;
                }
            }
        }
        drop(size);
    }

    pub fn input(&mut self, button: &Button, is_press: bool) {
        match button {
            Button::Keyboard(key) => {
                match key {
                    Key::W => {
                        if is_press {
                            self.player.start_move(Direction::NORTH)
                        } else {
                            self.player.stop_move(Direction::NORTH)
                        }
                    },
                    Key::S => {
                        if is_press {
                            self.player.start_move(Direction::SOUTH)
                        } else {
                            self.player.stop_move(Direction::SOUTH)
                        }
                    },
                    Key::A => {
                        if is_press {
                            self.player.start_move(Direction::WEST)
                        } else {
                            self.player.stop_move(Direction::WEST)
                        }
                    },
                    Key::D => {
                        if is_press {
                            self.player.start_move(Direction::EAST)
                        } else {
                            self.player.stop_move(Direction::EAST)
                        }
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
