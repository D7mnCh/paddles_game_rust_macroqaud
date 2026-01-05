mod assetes;
mod config;
mod functions;
mod objects;
use crate::assetes::*;
use crate::config::*;
use crate::functions::*;
use crate::objects::{ball::*, paddel::*};

use macroquad::{prelude::*, rand::rand};

#[macroquad::main(window_conf)]
async fn main() {
    let mut is_running: bool = false;

    // loading textures
    let assests: Assets = Assets::load().await;

    //objects settings
    let mut paddel_1 = Paddel::new(
        LEFT_PADDEL_WIDTH_GAP,
        HEIGHT as f32 / 2.,
        PADDEL_WIDTH,
        PADDEL_HEIGHT,
        0,
        &assests.paddle_right,
    );
    let mut paddel_2 = Paddel::new(
        WIDTH as f32 - RIGHT_PADDEL_WIDTH_GAP,
        HEIGHT as f32 / 2.,
        PADDEL_WIDTH,
        PADDEL_HEIGHT,
        0,
        &assests.paddle_left,
    );

    rand::srand(macroquad::miniquad::date::now() as _);
    let dir = {
        let dir = rand() % 2;
        if dir == 0 { 1 } else { -1 }
    };
    let velocity: Vec2 = Vec2::new(dir as f32 * 7., 7.);
    // let ball_dim: Vec2 = Vec2::new(BALL_DIM, BALL_DIM);
    let mut ball: Ball = Ball::new(
        screen_width() / 2.,
        screen_height() / 2.,
        BALL_DIM,
        BALL_DIM,
        velocity,
        &assests.ball,
    );
    println!("{}", velocity.x);

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
            // println!("{dir}");
            // println!("{is_running}");
            // println!("{delta}");
        }
        // Drawing
        draw_texture(&assests.background, 0., 0., WHITE);
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
