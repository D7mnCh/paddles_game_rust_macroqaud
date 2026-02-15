mod assets;
mod config;
mod functions;
mod objects;
mod state;
mod paddles;
mod ball;

use crate::assets::*;
use crate::config::*;
use crate::functions::*;
use crate::objects::{ball::*, paddel::*};
use crate::state::*;
use crate::paddles::*;
use crate::ball::*;

pub const WIDTH: i32 = 1200;
pub const HEIGHT: i32 = 800;
pub const WINDOW_TITLE: &str = "ping_pong";

pub const PADDLE_WIDTH: f32 = 20.;
pub const PADDLE_HEIGHT: f32 = 60.;
pub const PADDLE_VEL: f32 = 10.;

pub static mut BALL_VEL: f32 = 10.;

use macroquad::{prelude::*, rand::rand};

#[macroquad::main(window_conf)]
async fn main() {
    let mut is_running: bool = false;

    // loading textures
    let assets: Assets = Assets::load().await;

    //objects settings
    let mut paddel_1 = Paddel::new(
        LEFT_PADDEL_WIDTH_GAP,
        HEIGHT as f32 / 2.,
        PADDEL_WIDTH,
        PADDEL_HEIGHT,
        0,
        &assets.paddle_right,
    );
    let mut paddel_2 = Paddel::new(
        WIDTH as f32 - RIGHT_PADDEL_WIDTH_GAP,
        HEIGHT as f32 / 2.,
        PADDEL_WIDTH,
        PADDEL_HEIGHT,
        0,
        &assets.paddle_left,
    );

    rand::srand(macroquad::miniquad::date::now() as _);
    let direction = {
        let direction = rand() % 2;
        if direction == 0 { 1 } else { -1 }
    };
    let velocity: Vec2 = Vec2::new(direction as f32 * 7., 7.);
    let mut ball: Ball = Ball::new(
        screen_width() / 2.,
        screen_height() / 2.,
        BALL_DIM,
        BALL_DIM,
        velocity,
        &assets.ball,
    );
    ///////////////////////////

    loop {
        // Event handling
        if is_key_pressed(KeyCode::Escape) {
            break;
        } else if is_key_pressed(KeyCode::Space) {
            is_running = !is_running;
        }

        // Update logic
        if is_running {
            if is_key_down(KeyCode::S) {
                if paddel_1.height <= HEIGHT as f32
                    && paddel_1.height != HEIGHT as f32 - paddel_1.height_size
                {
                    paddel_1.height += PADDEL_VEL;
                } else {
                    paddel_1.height = HEIGHT as f32 - paddel_1.height_size;
                }
            }
            if is_key_down(KeyCode::W) {
                if paddel_1.height >= 0. && paddel_1.height != 0. {
                    paddel_1.height -= PADDEL_VEL;
                } else {
                    paddel_1.height = 0.;
                }
            }
            if is_key_down(KeyCode::Down) {
                if paddel_2.height <= HEIGHT as f32
                    && paddel_2.height != HEIGHT as f32 - paddel_2.height_size
                {
                    paddel_2.height += PADDEL_VEL;
                } else {
                    paddel_2.height = HEIGHT as f32 - paddel_2.height_size;
                }
            }
            if is_key_down(KeyCode::Up) {
                if paddel_2.height >= 0. && paddel_2.height != 0. {
                    paddel_2.height -= PADDEL_VEL;
                } else {
                    paddel_2.height = 0.;
                }
            }
            ball.update();
            collistion(&mut ball, &mut paddel_1, &mut is_running);
            collistion(&mut ball, &mut paddel_2, &mut is_running);

            //Debuging
            // println!("paddel left  score: {}", paddel_1.score);
            // println!("paddel right score: {}", paddel_2.score);
            // println!("{ball:?}");
            // println!("{direction}");
            // println!("{is_running}");
            // println!("{delta}");
        }
        // Drawing
        draw_texture(&assets.background, 0., 0., WHITE);
        if !is_running {
            game_stop();
            display_score(&paddel_1);
            display_score(&paddel_2);
        }

        draw_fps();
        paddel_1.draw();
        paddel_2.draw();
        ball.draw();

        next_frame().await;
    }
}
/*
 TODO:
    make a struct (state) that groups data (make main for now just for)
    inialization of the value
*/
