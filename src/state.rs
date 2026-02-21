use crate::assets::Assets;
use crate::ball::Ball;
use crate::config::Config;
use crate::game_state::GameState;
use crate::paddles::Paddles;
use crate::systems::*;
use crate::traits::*;
use macroquad::prelude::{
    GRAY, WHITE, draw_text, draw_texture, next_frame
};

pub struct State<'a> {
    pub game_state: GameState,
    pub config: Config,
    pub assets: Assets,
    pub paddles: [Paddles<'a>; 2],
    pub ball: Ball<'a>,
    //pub entities: Entities,
}
impl<'a> State<'a> {
    pub async fn new() -> Self {
        let assets: Assets = Assets::load().await;
        let config = Config::new();
        let paddles = Paddles::new(&config);
        let ball = Ball::new(&config);

        Self {
            game_state: GameState::Pausing,
            config: config,
            assets,
            paddles,
            ball,
        }
    }

    fn pause_game(&self) {
        let wcfg = &self.config.window_config;

        draw_text(
            "Game paused!",
            wcfg.screen_width as f32 / 2.4,
            wcfg.screen_height as f32 / 3.,
            50.,
            GRAY,
        );
        draw_text(
            "Press space",
            wcfg.screen_width as f32 / 2.4,
            wcfg.screen_height as f32 / 1.3,
            50.,
            GRAY,
        );
    }

    pub async fn run(&'a mut self) {
        loop {
            self.game_state.config_input_handling();
            draw_texture(&self.assets.background, 0., 0., WHITE);
            match self.game_state {
                GameState::Running => {
                    for paddle in self.paddles.iter_mut() {
                        paddle.update(&self.config);
                    }

                    ball_paddle_collision(&mut self.ball, &mut self.paddles);
                    add_score_to_paddle(
                        &mut self.ball,
                        &mut self.paddles,
                        &self.config,
                        &mut self.game_state,
                    );
                    self.ball.collision_detection(&self.config);
                    self.ball.update(&self.config);
                }
                GameState::Pausing => {
                    self.pause_game();
                }
                GameState::GameOver => break,
            }
            for paddle in self.paddles.iter_mut() {
                match paddle {
                    Paddles::Right(_) => {
                        paddle.draw(Some(&self.assets.right_paddle));
                        paddle.draw_scores();
                    }
                    Paddles::Left(_) => {
                        paddle.draw(Some(&self.assets.left_paddle));
                        paddle.draw_scores();
                    }
                }
            }
            self.ball.draw(Some(&self.assets.ball));
            next_frame().await;
        }
    }
}
