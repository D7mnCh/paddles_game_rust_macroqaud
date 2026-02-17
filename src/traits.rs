use crate::config::*;
pub trait Renderable {
    fn draw(&self);
}
pub trait Updatable {
    fn update(&mut self,config: &Config);
}
pub trait Controllable {}
