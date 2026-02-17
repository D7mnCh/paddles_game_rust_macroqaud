mod assets;
mod ball;
mod paddles;
mod state;
mod config;
mod game_state;
mod traits;

use crate::miniquad::conf::Platform;
use crate::state::*;
use macroquad::prelude::*;
use crate::config::*;

pub fn window_conf() -> Conf {
    let config = Config::new();
    let wcfg = &config.window_config;
    Conf {
        window_title: wcfg.window_title.to_owned(),
        window_height: wcfg.screen_height,
        window_width: wcfg.screen_width,
        #[cfg(target_os = "linux")]
        platform: Platform {
            linux_wm_class: wcfg.window_title,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = State::new().await;
    game.run().await;
}
