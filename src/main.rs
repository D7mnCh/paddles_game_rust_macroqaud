mod assets;
mod ball;
mod paddles;
mod state;

use crate::miniquad::conf::Platform;
use crate::state::*;
use macroquad::prelude::*;

pub const WIDTH: i32 = 1200;
pub const HEIGHT: i32 = 800;
pub const WINDOW_TITLE: &str = "ping_pong";

pub const PADDLE_WIDTH: f32 = 20.;
pub const PADDLE_HEIGHT: f32 = 60.;
pub const PADDLE_VEL: f32 = 10.;

pub static mut BALL_VEL: Vec2 = Vec2::new(10., 10.);
pub const BALL_DIM: f32 = 30.;

pub fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_height: HEIGHT,
        window_width: WIDTH,
        #[cfg(target_os = "linux")]
        platform: Platform {
            linux_wm_class: WINDOW_TITLE,
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
