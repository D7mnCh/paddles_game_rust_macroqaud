use macroquad::prelude::{Texture2D, load_texture};
pub struct Assets {
    pub left_paddle: Texture2D,
    pub right_paddle: Texture2D,
    pub ball: Texture2D,
    pub background: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        Self {
            // i think i get error becasuse i have both a file and directory have the same name
            left_paddle: load_texture("assets/left_paddle.png").await.unwrap(),
            right_paddle: load_texture("assets/right_paddle.png").await.unwrap(),
            ball: load_texture("assets/ball.png").await.unwrap(),
            background: load_texture("assets/background.png").await.unwrap(),
        }
    }
}
