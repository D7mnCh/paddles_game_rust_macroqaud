use crate::assets::Assets;
use crate::config::Config;
use crate::traits::*;
use macroquad::prelude::{GRAY, KeyCode, Vec2, WHITE, draw_text, draw_texture, is_key_down};

#[derive(Debug)]
pub enum Side {
    Right,
    Left,
}

#[derive(Debug)]
pub struct Paddle {
    pub side: Side,
    pub pos: Vec2,
    pub size: Vec2,
    pub vel: Vec2,
    pub score: i32,
}

impl Renderable for Paddle {
    fn draw(&mut self, texture: &Assets) {
        match self.side {
            Side::Right => {
                draw_texture(&texture.right_paddle, self.pos.x, self.pos.y, WHITE);
            }
            Side::Left => {
                draw_texture(&texture.left_paddle, self.pos.x, self.pos.y, WHITE);
            }
        }
    }
}
impl Paddle {
    pub fn new(config: &Config) -> [Self; 2] {
        let wcfg = &config.window_config;
        let size: Vec2 = Vec2::new(20., 60.);
        let paddles = [
            Self {
                side: Side::Right,
                pos: Vec2 {
                    x: wcfg.screen_width as f32 - size.x - 20.,
                    y: wcfg.screen_height as f32 / 2.,
                },
                vel: Vec2 { x: 10., y: 10. },
                size: Vec2 { x: 20., y: 60. },
                score: 0,
            },
            Self {
                side: Side::Left,
                pos: Vec2 {
                    x: size.x,
                    y: wcfg.screen_height as f32 / 2.,
                },
                vel: Vec2 { x: 10., y: 10. },
                size: Vec2 { x: 20., y: 60. },
                score: 0,
            },
        ];
        paddles
    }
    pub fn draw_scores(&self) {
        let config = Config::new();
        let wcfg = config.window_config;

        match self.side {
            Side::Right => {
                let paddel_score = format!("score: {}", self.score);
                draw_text(
                    paddel_score.as_str(),
                    wcfg.screen_width as f32 - 200.,
                    40.,
                    50.,
                    GRAY,
                );
            }
            Side::Left => {
                let paddel_score = format!("score: {}", self.score);
                draw_text(paddel_score.as_str(), 10., 40., 50., GRAY);
            }
        }
    }

    pub fn reset_paddles(&mut self, config: &Config) {
        let wcfg = &config.window_config;

        match self.side {
            Side::Right => {
                self.pos.y = wcfg.screen_height as f32 / 2.;
            }
            Side::Left => {
                self.pos.y = wcfg.screen_height as f32 / 2.;
            }
        }
    }
}

impl Updatable for Paddle {
    fn update(&mut self, config: &Config) {
        let wcfg = &config.window_config;
        match self.side {
            Side::Right => {
                if is_key_down(KeyCode::Down) {
                    if self.pos.y <= wcfg.screen_height as f32
                        && self.pos.y != wcfg.screen_height as f32 - self.size.y
                    {
                        self.pos.y += self.vel.y;
                    } else {
                        self.pos.y = wcfg.screen_height as f32 - self.size.y;
                    }
                }
                if is_key_down(KeyCode::Up) {
                    if self.pos.y >= 0. && self.pos.y != 0. {
                        self.pos.y -= self.vel.y;
                    } else {
                        self.pos.y = 0.;
                    }
                }
            }
            Side::Left => {
                if is_key_down(KeyCode::S) {
                    if self.pos.y <= wcfg.screen_height as f32
                        && self.pos.y != wcfg.screen_height as f32 - self.size.y
                    {
                        self.pos.y += self.vel.y;
                    } else {
                        self.pos.y = wcfg.screen_height as f32 - self.size.y;
                    }
                }
                if is_key_down(KeyCode::W) {
                    if self.pos.y >= 0. && self.pos.y != 0. {
                        self.pos.y -= self.vel.y;
                    } else {
                        self.pos.y = 0.;
                    }
                }
            }
        }
    }
}
