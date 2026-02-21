use crate::traits::*;
use crate::*;
use macroquad::rand::rand;

use macroquad::prelude::*;
#[derive(Debug)]
pub struct Ball<'a> {
    pub texture: Option<&'a Texture2D>,
    pub pos: Vec2,
    pub vel: Vec2,
    // i think i should rename rad to size ?, cause the ball is bisically a rectiangle
    pub rad: f32,
    collision: Option<Collision>,
}
#[derive(Debug)]
enum Collision {
    Ceilling,
    RightWall,
    LeftWall,
    Floor,
}
impl<'a> Ball<'a> {
    pub fn new(config: &Config) -> Self {
        let wcfg = &config.window_config;
        // dealling with seeds for getting rand ro work well it term of randomness
        rand::srand(macroquad::miniquad::date::now() as _);
        let mut ball = Ball {
            texture: None,
            pos: Vec2 {
                x: wcfg.screen_width as f32 / 2.,
                y: wcfg.screen_height as f32 / 2.,
            },
            vel: Vec2 { x: 10., y: 10. },
            rad: 30.,
            collision: None,
        };
        let dir: f32 = {
            let dir = (rand() % 2) as f32;
            if dir == 0. { 1. } else { -1. }
        };
        ball.vel.x *= dir;
        ball
    }
    pub fn collision_detection(&mut self, config: &Config) {
        let wcfg = &config.window_config;
        if self.pos.x + self.rad >= wcfg.screen_width as f32 {
            self.collision = Some(Collision::RightWall)
        } else if self.pos.x <= 0. {
            self.collision = Some(Collision::LeftWall)
        }
        if self.pos.y + self.rad >= wcfg.screen_height as f32 {
            self.collision = Some(Collision::Floor);
            //println!("[Info]: ball hits the floor")
        } else if self.pos.y <= 0. {
            self.collision = Some(Collision::Ceilling)
        }
    }
}
impl<'a> Renderable<'a> for Ball<'a> {
    fn draw(&mut self, texture: Option<&'a Texture2D>) {
        self.texture = texture;
        draw_texture(
            self.texture.expect("could not find ball's texture"),
            self.pos.x,
            self.pos.y,
            WHITE,
        );
    }
}
impl<'a> Updatable for Ball<'a> {
    fn update(&mut self, config: &Config) {
        let wcfg = &config.window_config;
        if let Some(ref collision) = self.collision.take() {
            match collision {
                Collision::RightWall | Collision::LeftWall => {
                    let dir: f32 = {
                        let dir = (rand() % 2) as f32;
                        if dir == 0. { 1. } else { -1. }
                    };
                    self.vel.x *= dir;

                    self.pos.x = wcfg.screen_width as f32 / 2.;
                    self.pos.y = wcfg.screen_height as f32 / 2.;
                }
                Collision::Floor => {
                    self.vel.y = -self.vel.y.abs();
                }
                Collision::Ceilling => {
                    self.vel.y = self.vel.y.abs();
                }
            }
        }
        if let None = self.collision {
            self.pos.x += self.vel.x;
            self.pos.y += self.vel.y;
        }
    }
}
