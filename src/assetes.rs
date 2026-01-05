use macroquad::prelude::*;

pub struct Assets {
    pub paddle_left: Texture2D,
    pub paddle_right: Texture2D,
    pub ball: Texture2D,
    pub background: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        Self {
            paddle_left: load_texture("assets/paddle_left.png").await.unwrap(),
            paddle_right: load_texture("assets/paddle_right.png").await.unwrap(),
            ball: load_texture("assets/ball.png").await.unwrap(),
            background: load_texture("assets/background.png").await.unwrap(),
        }
    }
}
