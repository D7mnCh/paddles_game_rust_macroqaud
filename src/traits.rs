use crate::assets::Assets;
use crate::config::Config;
pub trait Renderable {
    fn draw(&mut self, texture: &Assets);
}
pub trait Updatable {
    fn update(&mut self, config: &Config);
}
