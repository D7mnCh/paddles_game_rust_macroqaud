use macroquad::prelude::*;
#[derive(Debug)]
pub struct Paddel<'a> {
    pub height: f32,
    pub width: f32,
    width_size: f32,
    pub height_size: f32,
    pub score: i8,
    texture: &'a Texture2D,
}
impl<'a> Paddel<'a> {
    pub fn new(
        width: f32,
        height: f32,
        width_size: f32,
        height_size: f32,
        score: i8,
        texture: &'a Texture2D,
    ) -> Self {
        Self {
            width,
            height,
            width_size,
            height_size,
            score,
            texture,
        }
    }
    pub fn draw(&self) {
        draw_texture(self.texture, self.width, self.height, WHITE);
    }
}
