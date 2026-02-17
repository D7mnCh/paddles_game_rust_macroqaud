use macroquad::prelude::*;
pub struct WindowConfig {
    pub screen_width: i32,
    pub screen_height: i32,
    pub window_title: &'static str,
}
pub struct GameplayConfig {
    pub paddle_width: f32 ,
    pub paddle_height: f32 ,

    pub ball_dim: f32 ,
}
pub struct Config{
    pub window_config: WindowConfig,
    pub gameplay_config: GameplayConfig
}
impl Config {
    pub fn new () -> Self {
        Self {
            window_config: WindowConfig {
                screen_width: 1200,
                screen_height: 800,
                window_title: "ping_pong",
            },
            gameplay_config: GameplayConfig {
                paddle_width: 20.,
                paddle_height: 60.,

                ball_dim: 30.,
            }
        }
    }
}
