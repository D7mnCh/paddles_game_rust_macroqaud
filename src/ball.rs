use crate::state::*;
use crate::*;
use macroquad::prelude::*;
#[derive(Debug)]
pub struct Ball {
    pub texture: Texture2D,
    pub pos: Vec2,
}
impl Renderable for Ball {
    fn draw(&self) {
        draw_texture(&self.texture, self.pos.x, self.pos.y, WHITE);
    }
}
impl Updatable for Ball {
    // update it also based on her collision between the ceilling and the floor
    fn update(&mut self) {
        self.pos.x += unsafe { BALL_VEL.x };
        self.pos.y += unsafe { BALL_VEL.y };

        if self.pos.y + BALL_DIM >= HEIGHT as f32 {
            self.pos.y = HEIGHT as f32 - BALL_DIM;
            unsafe { BALL_VEL.y *= -1. };
        } else if self.pos.y <= 0. {
            self.pos.y = 0.;
            unsafe { BALL_VEL.y *= -1. };
        }
    }
}
