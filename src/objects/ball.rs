use crate::config::*;

use macroquad::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Ball<'a> {
    pub width: f32,
    pub height: f32,
    width_size: f32,
    height_size: f32,
    pub velocity: Vec2,
    texture: &'a Texture2D,
}
impl<'a> Ball<'a> {
    pub fn new(
        width: f32,
        height: f32,
        width_size: f32,
        height_size: f32,
        velocity: Vec2,
        texture: &'a Texture2D,
    ) -> Self {
        Self {
            width,
            height,
            width_size,
            height_size,
            texture,
            velocity,
        }
    }
    pub fn draw(&self) {
        draw_texture(&self.texture, self.width, self.height, WHITE);
    }
    pub fn update(&mut self) {
        self.width += self.velocity.x;
        self.height += self.velocity.y;

        if self.height + self.height_size >= HEIGHT as f32 {
            self.height = HEIGHT as f32 - BALL_DIM;
            self.velocity.y *= -1.;
        } else if self.height <= 0. {
            self.height = 0.;
            self.velocity.y *= -1.;
        }
    }
}
