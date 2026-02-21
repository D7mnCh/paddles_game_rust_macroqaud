use crate::config::Config;
use macroquad::prelude::Texture2D;
pub trait Renderable<'a> {
    fn draw(&mut self, texture: Option<&'a Texture2D>);
}
pub trait Updatable {
    fn update(&mut self, config: &Config);
}
