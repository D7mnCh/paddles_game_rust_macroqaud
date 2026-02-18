use crate::assets::*;
use crate::traits::*;
use crate::ball::*;
use crate::paddles::*;
use crate::config::*;
use crate::game_state::*;
use macroquad::{prelude::*, rand::rand};

// i think i can make a struct that connect Entities (ball and paddles)
// or have different structs for entities that interects with each other
pub struct State {
    game_state: GameState,
    config: Config,
    // this is a weird field
    background: Texture2D,
    paddles: [Paddles; 2],
    ball: Ball,
}
impl  State  {
    pub async fn new() -> Self {
        // make this in config.rs file
        let assets: Assets = Assets::load().await;

        let config = Config::new();
        let gcfg = &config.gameplay_config;
        let wcfg = &config.window_config;

        let paddles = [
            Paddles::Right(Paddle {
                texture: assets.right_paddle,
                pos: Vec2 {
                    x: wcfg.screen_width as f32 - gcfg.paddle_width - 20.,
                    y: wcfg.screen_height as f32 / 2. ,
                },
                vel: Vec2 {
                    x: 10.,
                    y: 10.,
                },
                score: 0,
            }),
            Paddles::Left(Paddle {
                texture: assets.left_paddle,
                pos: Vec2 {
                    x: gcfg.paddle_width,
                    y: wcfg.screen_height as f32 / 2. ,
                },
                vel: Vec2 {
                    x: 10.,
                    y: 10.,
                },
                score: 0,
            }),
            ];
            // dealling with seeds for getting rand ro work well it term of randomness
            rand::srand(macroquad::miniquad::date::now() as _);
            let mut ball = Ball {
                texture: assets.ball,
                pos: Vec2 {
                    x: wcfg.screen_width as f32 / 2.,
                    y: wcfg.screen_height as f32 / 2.,
                },
                vel: Vec2 {
                    x: 10.,
                    y: 10.
                }
            };
            let dir: f32 = {
                let dir = (rand() % 2) as f32;
                if dir == 0. { 1. } else { -1. }
            };
            ball.vel.x *= dir;

            Self {
                game_state: GameState::Pausing,
                config: config,
                background: assets.background,
                paddles,
                ball,
            }
    }
    fn config_input_handling(&mut self) {
           if is_key_pressed(KeyCode::Escape) {
               match self.game_state {
                   _ => self.game_state = GameState::GameOver,
               }
           }
        if is_key_pressed(KeyCode::Space) {
            match self.game_state {
                GameState::Running => self.game_state = GameState::Pausing,
                GameState::Pausing => self.game_state = GameState::Running,
                _ => (),
            }
        }
    }

    // those methods don't belong here
    fn paddle_ball_collision(&mut self) {
        let gcfg = &self.config.gameplay_config;
        for paddle in self.paddles.iter_mut() {
            match paddle {
                Paddles::Left(paddle) => {
                    if self.ball.pos.x <= paddle.pos.x + gcfg.paddle_width
                        && paddle.pos.y <= self.ball.pos.y + gcfg.ball_dim
                            && paddle.pos.y + gcfg.paddle_height >= self.ball.pos.y
                    {
                        // alwasy when you have tunneling, just teleport it
                        let ball_teleport_by = 10.;
                        self.ball.pos.x += ball_teleport_by;
                        self.ball.vel.x *= -1.;
                    }
                }
                Paddles::Right(paddle) => {
                    if self.ball.pos.x + gcfg.ball_dim >= paddle.pos.x
                        && paddle.pos.y <= self.ball.pos.y + gcfg.ball_dim
                            && paddle.pos.y + gcfg.paddle_height >= self.ball.pos.y
                    {
                        // alwasy when you have tunneling, just teleport it
                        let ball_teleport_by = 10.;
                        self.ball.pos.x -= ball_teleport_by;
                        self.ball.vel.x *= -1.;
                    }
                }
            }
        }
    }

    fn add_score_to_paddle(&mut self) {
        let gcfg = &self.config.gameplay_config;
        let wcfg = &self.config.window_config;
        for paddle in self.paddles.iter_mut() {
            match paddle {
                Paddles::Right(paddle) => {
                    // updating score
                    if self.ball.pos.x <= 0. {
                        self.game_state = GameState::Pausing;
                        paddle.score += 1;
                    }
                }

                Paddles::Left(paddle) => {
                    // updating score
                    if self.ball.pos.x + gcfg.ball_dim >= wcfg.screen_width as f32 {
                        self.game_state = GameState::Pausing;
                        paddle.score += 1;
                    }
                }
            }
            if self.ball.pos.x + gcfg.ball_dim >= wcfg.screen_width as f32
                || self.ball.pos.x <= 0. {
                    paddle.reset_paddles(&self.config);
            }
        }
    }
    fn pause_game(&self) {
        let wcfg = &self.config.window_config;

        draw_text(
            "Game paused!",
            wcfg.screen_width as f32 / 2.4,
            wcfg.screen_height as f32 / 3.,
            50.,
            GRAY,
        );
        draw_text(
            "Press space",
            wcfg.screen_width as f32 / 2.4,
            wcfg.screen_height as f32 / 1.3,
            50.,
            GRAY,
        );
    }
    pub async fn run(&mut self) {
        loop {
            self.config_input_handling();
            draw_texture(&self.background, 0., 0., WHITE);
            match self.game_state {
                GameState::Running => {
                    for paddle in self.paddles.iter_mut() {
                        paddle.update(&self.config);
                    }

                    self.paddle_ball_collision();
                    self.add_score_to_paddle();
                    self.ball.update(&self.config);
                },
                GameState::Pausing => {
                    self.pause_game();
                },
                GameState::GameOver => {
                    break
                }
            }
            for paddle in self.paddles.iter() {
                paddle.draw_scores();
                paddle.draw();
            }
            self.ball.draw();
            next_frame().await;
            }
        }
    }
