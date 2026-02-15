use macroquad::prelude::*;
use crate::state::*;
use crate::*;

pub enum Paddles {
    Right(Paddle),
    Left(Paddle)
}

pub struct Paddle{
   pub texture: Texture2D,
   pub pos: Vec2,
   pub score: i32,
}

impl Renderable for Paddles {
    fn draw (&self) {
        match self {
            Paddles::Right(paddle) =>{
                draw_texture(&paddle.texture, paddle.pos.x, paddle.pos.y, WHITE);
            },
            Paddles::Left(paddle) =>{ 
                draw_texture(&paddle.texture, paddle.pos.x, paddle.pos.y, WHITE);
            },
        }
    }
}

impl Movable for Paddles {
    fn _move (&mut self) {
        match self {
            Paddles::Right(paddle) =>{
                if is_key_down(KeyCode::Down) {
                    if paddle.pos.y <= HEIGHT as f32
                        && paddle.pos.y != HEIGHT as f32 - PADDLE_HEIGHT
                    {
                        paddle.pos.y += PADDLE_VEL;
                    } else {
                        paddle.pos.y = HEIGHT as f32 - PADDLE_HEIGHT;
                    }
                }
                if is_key_down(KeyCode::Up) {
                    if paddle.pos.y >= 0. && paddle.pos.y != 0. {
                        paddle.pos.y -= PADDLE_VEL;
                    } else {
                        paddle.pos.y = 0.;
                    }
                }
            },
            Paddles::Left(paddle) =>{
                if is_key_down(KeyCode::S) {
                    if paddle.pos.y <= HEIGHT as f32
                        && paddle.pos.y != HEIGHT as f32 - PADDLE_HEIGHT
                    {
                        paddle.pos.y += PADDLE_VEL;
                    } else {
                        paddle.pos.y = HEIGHT as f32 - PADDLE_HEIGHT;
                    }
                }
                if is_key_down(KeyCode::W) {
                    if paddle.pos.y >= 0. && paddle.pos.y != 0. {
                        paddle.pos.y -= PADDLE_VEL;
                    } else {
                        paddle.pos.y = 0.;
                    }
                }
            },
        }
    }
}

