use macroquad::prelude::{Texture2D, load_texture};
pub struct Assets {
    pub left_paddle: Texture2D,
    pub right_paddle: Texture2D,
    pub ball: Texture2D,
    pub background: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        let left_paddle_img_path = "assets/left_paddle.png";
        let right_paddle_im_path = "assets/right_paddle.png";
        let ball_img_path = "assets/ball.png";
        let background_img_path = "assets/background.png";
        
        Self {
            left_paddle: load_texture(left_paddle_img_path).await.expect(format!("couldn't find path : {left_paddle_img_path}")),
            right_paddle: load_texture(right_paddle_im_path).await.expect(format!("couldn't find path : {right_paddle_img_path}")),
            ball: load_texture(ball_img_path).await.expect(format!("couldn't find path : {ball_paddle_img_path}")),
            background: load_texture(background_img_path).await.expect(format!("couldn't find path : {backgound_paddle_img_path}")),
        }
    }
}
