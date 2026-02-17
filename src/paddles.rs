use macroquad::prelude::*;
use crate::traits::*;
use crate::config::*;

#[derive(Debug)]
pub enum Paddles {
    Right(Paddle),
    Left(Paddle),
}

#[derive(Debug)]
pub struct Paddle {
    pub texture: Texture2D,
    pub pos: Vec2,
    pub vel: Vec2,
    pub score: i32,
}
impl Paddle {
    pub fn reset_paddles (&mut self) {
        let config = Config::new();
        let wcfg = config.window_config;

        self.pos.y = wcfg.screen_height as f32 / 2.;
        println!("right paddle reset ");
        self.pos.y = wcfg.screen_height as f32 / 2.;
        println!("left paddle reset");
    }
}
impl Renderable for Paddles {
    fn draw(&self) {
        match self {
            Paddles::Right(paddle) => {
                draw_texture(&paddle.texture, paddle.pos.x, paddle.pos.y, WHITE);
            }
            Paddles::Left(paddle) => {
                draw_texture(&paddle.texture, paddle.pos.x, paddle.pos.y, WHITE);
            }
        }
    }
}
impl Paddles {
    pub fn draw_scores(&self) {
        let config = Config::new();
        let wcfg = config.window_config;

        match self {
            Paddles::Right(paddle) => {
                let paddel_score = format!("score: {}", paddle.score);
                draw_text(paddel_score.as_str(), wcfg.screen_width as f32 - 200., 40., 50., GRAY);
            }
            Paddles::Left(paddle) => {
                let paddel_score = format!("score: {}", paddle.score);
                draw_text(paddel_score.as_str(), 10., 40., 50., GRAY);
            }
        }
    }

    pub fn reset_paddles (&mut self, config: &Config) {
        let wcfg = &config.window_config;

        match self {
            Paddles::Right(paddle) => {
                paddle.pos.y = wcfg.screen_height as f32 / 2.;
                println!("right paddle reset ");
            }
            Paddles::Left(paddle) => {
                paddle.pos.y = wcfg.screen_height as f32 / 2.;
                println!("left paddle reset");
            }
        }
    }
}

impl Updatable for Paddles {
    fn update(&mut self, config: &Config) {
        let gcfg = &config.gameplay_config;
        let wcfg = &config.window_config;
        match self {
            Paddles::Right(paddle) => {
                if is_key_down(KeyCode::Down) {
                    if paddle.pos.y <= wcfg.screen_height as f32
                        && paddle.pos.y != wcfg.screen_height as f32 - gcfg.paddle_height
                    {
                        paddle.pos.y += paddle.vel.y;
                    } else {
                        paddle.pos.y = wcfg.screen_height as f32 - gcfg.paddle_height;
                    }
                }
                if is_key_down(KeyCode::Up) {
                    if paddle.pos.y >= 0. && paddle.pos.y != 0. {
                        paddle.pos.y -= paddle.vel.y;
                    } else {
                        paddle.pos.y = 0.;
                    }
                }
            }
            Paddles::Left(paddle) => {
                if is_key_down(KeyCode::S) {
                    if paddle.pos.y <= wcfg.screen_height as f32
                        && paddle.pos.y != wcfg.screen_height as f32 - gcfg.paddle_height
                    {
                        paddle.pos.y += paddle.vel.y;
                    } else {
                        paddle.pos.y = wcfg.screen_height as f32 - gcfg.paddle_height;
                    }
                }
                if is_key_down(KeyCode::W) {
                    if paddle.pos.y >= 0. && paddle.pos.y != 0. {
                        paddle.pos.y -= paddle.vel.y;
                    } else {
                        paddle.pos.y = 0.;
                    }
                }
            }
        }
    }
}
