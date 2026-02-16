use crate::assets::*;
use crate::ball::*;
use crate::paddles::*;
use crate::*;
use macroquad::{prelude::*, rand::rand};

pub trait Renderable {
    fn draw(&self);
}
pub trait Updatable {
    fn update(&mut self);
}
pub struct State {
    is_running: bool,
    background: Texture2D,
    paddles: [Paddles; 2],
    ball: Ball,
}
impl State {
    pub async fn new() -> Self {
        let assets: Assets = Assets::load().await;

        let paddles = [
            Paddles::Right(Paddle {
                texture: assets.right_paddle,
                pos: Vec2 {
                    x: WIDTH as f32 - PADDLE_WIDTH - 20.,
                    y: HEIGHT as f32 / 2. ,
                },
                score: 0,
            }),
            Paddles::Left(Paddle {
                texture: assets.left_paddle,
                pos: Vec2 {
                    x: PADDLE_WIDTH,
                    y: HEIGHT as f32 / 2. ,
                },
                score: 0,
            }),
        ];
        // dealling with seeds for getting rand ro work well it term of randomness
        rand::srand(macroquad::miniquad::date::now() as _);
        let dir: f32 = {
            let dir = (rand() % 2) as f32;
            if dir == 0. { 1. } else { -1. }
        };
        unsafe {
            BALL_VEL.x *= dir;
        };
        let ball = Ball {
            texture: assets.ball,
            pos: Vec2 {
                x: WIDTH as f32 / 2.,
                y: HEIGHT as f32 / 2.,
            },
        };

        Self {
            is_running: false,
            background: assets.background,
            paddles,
            ball,
        }
    }

    fn config_input_handling(&mut self) {
        /*
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        */
        if is_key_pressed(KeyCode::Space) {
            self.is_running = !self.is_running;
        }
    }

    fn paddle_ball_collision(&mut self) {
        for paddle in self.paddles.iter_mut() {
            match paddle {
                Paddles::Left(paddle) => {
                    if self.ball.pos.x <= paddle.pos.x + PADDLE_WIDTH
                        && paddle.pos.y <= self.ball.pos.y + BALL_DIM
                        && paddle.pos.y + PADDLE_HEIGHT >= self.ball.pos.y
                    {
                        // alwasy when you have tunneling, just teleport it
                        let ball_teleport_by = 10.;
                        self.ball.pos.x += ball_teleport_by;
                        unsafe {
                            BALL_VEL.x *= -1.;
                        }
                    }
                }
                Paddles::Right(paddle) => {
                    if self.ball.pos.x + BALL_DIM >= paddle.pos.x
                        && paddle.pos.y <= self.ball.pos.y + BALL_DIM
                        && paddle.pos.y + PADDLE_HEIGHT >= self.ball.pos.y
                    {
                        // alwasy when you have tunneling, just teleport it
                        let ball_teleport_by = 10.;
                        self.ball.pos.x -= ball_teleport_by;
                        unsafe {
                            BALL_VEL.x *= -1.;
                        }
                    }
                }
            }
        }
    }

    fn ball_wall_collision(&mut self) {
        let dir: f32 = {
            let dir = (rand() % 2) as f32;
            if dir == 0. { 1. } else { -1. }
        };
        for paddle in self.paddles.iter_mut() {
            match paddle {
                Paddles::Right(paddle) => {
                    // updating score
                    if self.ball.pos.x + BALL_DIM >= WIDTH as f32 {
                        self.is_running = false;

                        unsafe {
                            BALL_VEL *= dir;
                        }
                        self.ball.pos.x = WIDTH as f32 / 2.;
                        self.ball.pos.y = HEIGHT as f32 / 2.;

                        paddle.score += 1;
                    }
                }
                Paddles::Left(paddle) => {
                    // updating score
                    if self.ball.pos.x <= 0. {
                        self.is_running = false;

                        unsafe {
                            BALL_VEL *= dir;
                        }

                        self.ball.pos.x = WIDTH as f32 / 2.;
                        self.ball.pos.y = HEIGHT as f32 / 2.;

                        paddle.score += 1;
                    }
                }
            }
            // why this doesn't work for both ? only worked on the right paddle, that's weird
            if self.ball.pos.x + BALL_DIM >= WIDTH as f32
                || self.ball.pos.x <= 0. {
                    paddle.reset_paddles();
            }
        }
    }
    fn stop_game(&self) {
        draw_text(
            "Game stops!",
            WIDTH as f32 / 2.4,
            HEIGHT as f32 / 3.,
            50.,
            GRAY,
        );
        draw_text(
            "Press space",
            WIDTH as f32 / 2.4,
            HEIGHT as f32 / 1.3,
            50.,
            GRAY,
        );
    }
    pub async fn run(&mut self) {
        loop {
            self.config_input_handling();
            draw_texture(&self.background, 0., 0., WHITE);
            if self.is_running {
                for paddle in self.paddles.iter_mut() {
                    paddle.update();
                }

                self.ball.update();
                self.paddle_ball_collision();
                self.ball_wall_collision();
            }
            for paddle in self.paddles.iter() {
                paddle.draw_scores();
                paddle.draw();
            }
            self.ball.draw();
            if self.is_running == false {
                self.stop_game();
            }
            next_frame().await;
        }
    }
}
