use crate::ball::Ball;
use crate::config::Config;
use crate::game_state::GameState;
use crate::paddles::Paddles;
pub fn ball_paddle_collision(ball: &mut Ball, paddles: &mut [Paddles; 2]) {
    for paddle in paddles.iter_mut() {
        match paddle {
            // can you pls just learn about how this logic work, cause you just waste a lot of time trying to figure it out
            Paddles::Left(paddle) => {
                if ball.pos.x <= paddle.pos.x + paddle.size.x
                        //Top
                        &&  ball.pos.y + ball.rad >= paddle.pos.y
                        // buttom
                        && ball.pos.y <= paddle.pos.y + paddle.size.y
                {
                    ball.vel.x = ball.vel.x.abs();
                }
            }
            Paddles::Right(paddle) => {
                if ball.pos.x + ball.rad >= paddle.pos.x
                    && ball.pos.y + ball.rad >= paddle.pos.y
                    && ball.pos.y <= paddle.pos.y + paddle.size.y
                {
                    ball.vel.x = -ball.vel.x.abs();
                }
            }
        }
    }
}
pub fn add_score_to_paddle(
    ball: &mut Ball,
    paddles: &mut [Paddles; 2],
    config: &Config,
    game_state: &mut GameState,
) {
    let wcfg = &config.window_config;
    for paddle in paddles.iter_mut() {
        match paddle {
            Paddles::Right(paddle) => {
                if ball.pos.x <= 0. {
                    *game_state = GameState::Pausing;
                    paddle.score += 1;
                }
            }
            Paddles::Left(paddle) => {
                if ball.pos.x + ball.rad >= wcfg.screen_width as f32 {
                    *game_state = GameState::Pausing;
                    paddle.score += 1;
                }
            }
        }
        if ball.pos.x + ball.rad >= wcfg.screen_width as f32 || ball.pos.x <= 0. {
            paddle.reset_paddles(&config);
        }
    }
}
