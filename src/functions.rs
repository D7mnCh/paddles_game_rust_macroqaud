use crate::config::*;
use crate::objects::ball::*;
use crate::objects::paddel::*;
use macroquad::{miniquad::conf::Platform, prelude::*, rand::rand};

pub(crate) fn window_conf() -> Conf {
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

pub fn collistion(ball: &mut Ball, paddel: &mut Paddel, is_running: &mut bool) {
    // check for the height : if the edges (top/bottom) of the ball is
    // (bigger/smaller) then the (top/bottom) of the paddel
    // and also for (left/right)
    let dir: f32 = {
        let dir = (rand() % 2) as f32;
        if dir == 0. { 1. } else { -1. }
    };
    if ball.width <= paddel.width + PADDEL_WIDTH
        && ball.width + BALL_DIM >= paddel.width
        && paddel.height <= ball.height + BALL_DIM
        && paddel.height + PADDEL_HEIGHT >= ball.height
    {
        // alwasy when you have tolloning, just teleport it
        let ball_teleport_by = 7.;
        if ball.width < WIDTH as f32 / 2. {
            ball.width += ball_teleport_by;
        } else {
            ball.width -= ball_teleport_by;
        }
        ball.velocity.x *= -1.;
    }
    // after ball collide with the wall check witch paddel to add score with
    if ball.width <= 0. && paddel.width > (WIDTH / 2) as f32 {
        *is_running = false;

        ball.velocity.x *= dir;

        ball.width = WIDTH as f32 / 2.;
        ball.height = HEIGHT as f32 / 2.;
        paddel.score += 1;
    } else if ball.width + BALL_DIM >= WIDTH as f32 && paddel.width < (WIDTH / 2) as f32 {
        *is_running = false;

        ball.velocity.x *= dir;

        ball.width = WIDTH as f32 / 2.;
        ball.height = HEIGHT as f32 / 2.;

        paddel.score += 1;
    }
}
// draw text
pub fn game_stop() {
    draw_text(
        "Game stops!",
        WIDTH as f32 / 2.4,
        HEIGHT as f32 / 3.,
        50.,
        GRAY,
    );
    draw_text(
        "Press space",
        WIDTH as f32 / 2.4,
        HEIGHT as f32 / 1.3,
        50.,
        GRAY,
    );
}
// draw text
pub fn display_score(paddel: &Paddel) {
    let paddel_score = format!("score: {}", paddel.score);
    // checks for wich paddel to display above them the score
    if paddel.width < (WIDTH / 2) as f32 {
        draw_text(paddel_score.as_str(), 10., 40., 50., GRAY);
    } else {
        draw_text(paddel_score.as_str(), WIDTH as f32 - 200., 40., 50., GRAY);
    }
}
