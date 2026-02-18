use crate::*;
use crate::traits::*;
use macroquad::rand::rand;

use macroquad::prelude::*;
#[derive(Debug)]
pub struct Ball {
    pub texture: Texture2D,
    pub pos: Vec2,
    pub vel: Vec2,
}
impl Renderable for Ball {
    fn draw(&self) {
        draw_texture(&self.texture, self.pos.x, self.pos.y, WHITE);
    }
}
impl Updatable for Ball {
    // this update is only based on cielling and floor collision
    // i wanna add here also for walls
    fn update(&mut self, config: &Config) {
        let gcfg = &config.gameplay_config;
        let wcfg = &config.window_config;

        // update ball based when she hits the walls
        if self.pos.x + gcfg.ball_dim >= wcfg.screen_width as f32 
            || self.pos.x <= 0.
        {
            let dir: f32 = {
                let dir = (rand() % 2) as f32;
                if dir == 0. { 1. } else { -1. }
            };
            self.vel.x *= dir;
            self.pos.x = wcfg.screen_width as f32 / 2.;
            self.pos.y = wcfg.screen_height as f32 / 2.;
        }

        // update ball based when she hits the cielling or the floor
        if self.pos.y + gcfg.ball_dim >= wcfg.screen_height as f32 {
            self.pos.y = wcfg.screen_height as f32 - gcfg.ball_dim;
            self.vel.y *= -1. ;
        } else if self.pos.y <= 0. {
            self.pos.y = 0.;
             self.vel.y *= -1.;
        }

        // update the ball at the end
        self.pos.x += self.vel.x ;
        self.pos.y += self.vel.y ;

    }
}
