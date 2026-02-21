use crate::config::Config;
use crate::traits::*;
use macroquad::prelude::{
    GRAY, KeyCode, Texture2D, Vec2, WHITE, draw_text, draw_texture, is_key_down,
};

#[derive(Debug)]
pub enum Paddles<'a> {
    Right(Paddle<'a>),
    Left(Paddle<'a>),
}

#[derive(Debug)]
pub struct Paddle<'a> {
    pub texture: Option<&'a Texture2D>,
    pub pos: Vec2,
    pub size: Vec2,
    pub vel: Vec2,
    pub score: i32,
}

impl<'a> Renderable<'a> for Paddles<'a> {
    fn draw(&mut self, texture: Option<&'a Texture2D>) {
        match self {
            Paddles::Right(paddle) => {
                paddle.texture = texture;
                draw_texture(
                    paddle
                        .texture
                        .expect("could not find right-paddle's texture"),
                    paddle.pos.x,
                    paddle.pos.y,
                    WHITE,
                );
            }
            Paddles::Left(paddle) => {
                paddle.texture = texture;
                draw_texture(
                    paddle
                        .texture
                        .expect("could not find right-paddle's texture"),
                    paddle.pos.x,
                    paddle.pos.y,
                    WHITE,
                );
            }
        }
    }
}
impl<'a> Paddles<'a> {
    pub fn new(config: &Config) -> [Self; 2] {
        let wcfg = &config.window_config;
        let size: Vec2 = Vec2::new(20., 60.);
        let paddles = [
            Paddles::Right(Paddle {
                texture: None,
                pos: Vec2 {
                    x: wcfg.screen_width as f32 - size.x - 20.,
                    y: wcfg.screen_height as f32 / 2.,
                },
                vel: Vec2 { x: 10., y: 10. },
                size: Vec2 { x: 20., y: 60. },
                score: 0,
            }),
            Paddles::Left(Paddle {
                texture: None,
                pos: Vec2 {
                    x: size.x,
                    y: wcfg.screen_height as f32 / 2.,
                },
                vel: Vec2 { x: 10., y: 10. },
                size: Vec2 { x: 20., y: 60. },
                score: 0,
            }),
        ];
        paddles
    }
    pub fn draw_scores(&self) {
        let config = Config::new();
        let wcfg = config.window_config;

        match self {
            Paddles::Right(paddle) => {
                let paddel_score = format!("score: {}", paddle.score);
                draw_text(
                    paddel_score.as_str(),
                    wcfg.screen_width as f32 - 200.,
                    40.,
                    50.,
                    GRAY,
                );
            }
            Paddles::Left(paddle) => {
                let paddel_score = format!("score: {}", paddle.score);
                draw_text(paddel_score.as_str(), 10., 40., 50., GRAY);
            }
        }
    }

    pub fn reset_paddles(&mut self, config: &Config) {
        let wcfg = &config.window_config;

        match self {
            Paddles::Right(paddle) => {
                paddle.pos.y = wcfg.screen_height as f32 / 2.;
            }
            Paddles::Left(paddle) => {
                paddle.pos.y = wcfg.screen_height as f32 / 2.;
            }
        }
    }
}

impl<'a> Updatable for Paddles<'a> {
    fn update(&mut self, config: &Config) {
        let wcfg = &config.window_config;
        match self {
            Paddles::Right(paddle) => {
                if is_key_down(KeyCode::Down) {
                    if paddle.pos.y <= wcfg.screen_height as f32
                        && paddle.pos.y != wcfg.screen_height as f32 - paddle.size.y
                    {
                        paddle.pos.y += paddle.vel.y;
                    } else {
                        paddle.pos.y = wcfg.screen_height as f32 - paddle.size.y;
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
                        && paddle.pos.y != wcfg.screen_height as f32 - paddle.size.y
                    {
                        paddle.pos.y += paddle.vel.y;
                    } else {
                        paddle.pos.y = wcfg.screen_height as f32 - paddle.size.y;
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
