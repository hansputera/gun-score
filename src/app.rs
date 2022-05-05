use opengl_graphics::{GlGraphics, OpenGL};
use glutin_window::GlutinWindow as Window;
use piston::window::{WindowSettings, Size};
use piston::input::*;
use graphics::{clear};

use crate::colors::{Colors};
use crate::schemas::player::{Player};
use crate::schemas::enemy::{Monster, EnemyType};
use crate::schemas::bullet::{Bullet, Attacker};
use crate::geom::{Direction, Position};
use crate::schemas::{GameObject};
use crate::textures::{load_cache, TextDraw};

#[derive(PartialEq)]
enum GameStatus {
    Fight, // fighting mode
    Win, // all monster have been killed
    Lose, // could be dead
}

// fire cooldown
const FIRE_COOLDOWN: f64 = 0.1; // 10 bulls/sec

pub struct GunScoreApp<'a> {
    pub gl: GlGraphics,
    pub window: Window,
    pub player: Player,
    pub monsters: Vec<Monster>,
    pub bullets: Vec<Bullet>,
    pub text_draw: TextDraw<'a>,

    // game state
    status: GameStatus,
    bullets_cooldown: f64,
}

impl GunScoreApp<'_> {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let window_settings = WindowSettings::new("gun-score", [500, 500])
            .graphics_api(opengl)
            .exit_on_esc(true);
        let glyph = load_cache(include_bytes!("../assets/SF_Atarian_System.ttf"));
        
        // glyph access on 'TextDraw.cache'

        GunScoreApp {
            window: window_settings.build().unwrap(),
            gl: GlGraphics::new(opengl),
            player: Player::new(&"Tono".to_string(), &0.0, &0.0),
            monsters: Vec::new(),
            status: GameStatus::Fight,
            text_draw: TextDraw::new(glyph),
            bullets: Vec::new(),
            bullets_cooldown: 0.0,
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
        // bullets
        let bullets = &self.bullets;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(colors.white, gl);

            // draw player's name
            self.text_draw.draw(&String::from(format_args!("{} - ({:.2})", player.name.to_string(), player.health).to_string()), &colors.black, &[
                                player.pos.x, (player.pos.y - 20.0),
            ], &15, &c, gl);
            // draw player stats
            self.text_draw.draw(&String::from(format_args!("Amunition: {}", player.amunition).to_string()), &colors.black, &[
                                20.0, 40.0
            ], &25, &c, gl);
            self.text_draw.draw(&String::from(format_args!("Score: {}", player.score.floor()).to_string()), &colors.black, &[
                                20.0, 70.0,
            ], &25, &c, gl);
            
            self.text_draw.draw(&String::from(format_args!("Life: {}", player.life).to_string()), &colors.black, &[
                                f64::from(size.width) / 2.0,
								(f64::from(size.height) / 2.0) + 20.0,
            ], &15, &c, gl);

            // draw "Need reload ..." text when the player amunition is 0
            if self.player.amunition <= 0 {
                self.text_draw.draw(&String::from("Need reload, press (R)"), &colors.red, &[
                                    (f64::from(size.width) / 1.5),
                                    (f64::from(size.height) / 1.5)
                ], &20, &c, gl);
            }

            // check the game status.
            match self.status {
                GameStatus::Lose => {
                    self.text_draw.draw_center(&String::from("YOU LOSE!"), &colors.red, &40, &[
                                               f64::from(size.width),
                                               f64::from(size.height),
                    ], &c,gl);

                    self.text_draw.draw(&String::from("Press (1 / One) to restart"), &colors.blue, &[
                                        (f64::from(size.width) / 1.5),
                                        (f64::from(size.height) / 1.5) + 20.0,
                    ], &20, &c, gl);
                },
                GameStatus::Win => {
                    self.text_draw.draw_center(&String::from("You are the Winner!"), &colors.green, &40, &[
                                                             f64::from(size.width),
                                                             f64::from(size.height),
                    ], &c, gl);

                    self.text_draw.draw(&String::from("Press (1) to restart"), &colors.blue, &[
                                        (f64::from(size.width) / 1.5),
                                        (f64::from(size.height) / 1.5) + 20.0,
                    ], &20, &c, gl);
                },
                GameStatus::Fight => {
                    self.text_draw.draw_center(&String::from("Keep fight!"), &colors.black, &32, &[
                                                             f64::from(size.width),
                                                             f64::from(size.height),
                    ], &c, gl);
                }
            }

            // render player
            player.render(&c, gl);

            // render monster/enemy
            for enemy in enemies.iter() {
                enemy.render(&c, gl);
            }

            // render bullets
            for bullet in bullets.iter() {
                bullet.render(&c, gl);
            }
        });

        drop(size);
        drop(colors);
        drop(x);
        drop(y);
    }

    fn reset(&mut self) {
        self.player = Player::new(&"You".to_string(), &0.0, &0.0);

        self.status = GameStatus::Fight;
        self.monsters.clear();
        self.bullets.clear();
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

        if self.bullets_cooldown > 0.0 {
            self.bullets_cooldown -= args.dt;
        }

        if self.player.shooting {
            self.player.shooting = false;
            if self.player.amunition >= 1 {
                self.bullets.push(
                    Bullet::new(self.player.pos.x, self.player.pos.y, self.player.direction, Attacker::Player),
                );
                self.player.amunition -= 1;
            }
        }

		// only works in fight mode.
        if self.status == GameStatus::Fight {	
			if self.player.health < f64::from(1.0) {
				self.player.life -= 1;
				self.player.pos = Position::new(0.0, 0.0);
			}
			
			if self.player.life < u8::from(1) {
				self.status = GameStatus::Lose;
			}
			
			for monster in &mut self.monsters {
				monster.update(args.dt, *size);
				if monster.tabrakan(&self.player) {
					if self.player.life < u8::from(1) {
						self.status = GameStatus::Lose;
					} else {
						self.player.life -= 1;
						self.player.pos = Position::new(0.0, 0.0); // reset the position when the player died.
					}
				}
			}
			
			let mut enem_bulls: Vec<Bullet> = Vec::new();
			
			for bullet in &mut self.bullets {
				bullet.update(args.dt, *size);
				for monster in &mut self.monsters {
					if bullet.tabrakan(monster) && bullet.attacker == Attacker::Player {
						// if the monster type is a fighter, it will shoot a bullet to player.
						if monster.enemy_type == EnemyType::Fighter {
							enem_bulls.push(Bullet::new(monster.pos.x, monster.pos.y, bullet.source_direction, Attacker::Monster));
						}

						bullet.ttl = 0.0;
						let health = monster.health;

						monster.health -= f64::from(bullet.damage_count);
						// if the monster health <= 0.0, and the bullet is coming from Player.
						// Add (health - (health / Bullet#damage_count)) as score to player.
						if monster.health <= 0.0 && bullet.attacker == Attacker::Player {
							self.player.score += health - (health - f64::from(bullet.damage_count));
						}

						drop(health);
					} else if bullet.tabrakan(&self.player) && bullet.attacker == Attacker::Monster {
						bullet.ttl = 0.0;
						self.player.health -= f64::from(bullet.damage_count);
					}
				}
			}

			self.bullets.append(&mut enem_bulls);
			drop(enem_bulls);

			self.bullets.retain(|bullet| bullet.ttl > 0.0);
			self.monsters.retain(|monster| monster.health > 0.0);
			
			if self.monsters.is_empty() {
				self.status = GameStatus::Win;
			}
        } else {
			// just update the position.
			for bullet in &mut self.bullets {
				bullet.update(args.dt, *size);
			}
			
			for monster in &mut self.monsters {
				monster.update(args.dt, *size);
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
                    Key::Space => {
                        if is_press && self.bullets_cooldown <= 0.0 {
                            self.bullets_cooldown = FIRE_COOLDOWN;
                            self.player.shooting = true;
                        }
                    },
                    Key::R => {
                        if is_press && self.player.amunition <= 0 {
                            self.player.shooting = false;
                            self.player.amunition = 100;
                        }
                    },
                    Key::D1 => {
                        self.reset();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
