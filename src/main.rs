mod assets;
mod ball;
mod config;
mod entities;
mod game_state;
mod paddles;
mod state;
mod systems;
mod traits;

use crate::config::Config;
use crate::state::State;
use macroquad::prelude::miniquad;
use miniquad::conf::{Conf, Platform};

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
