// NOTE this module should named as game.rs

use crate::assets::Assets;
use crate::config::Config;
use crate::entities::Entities;
use crate::game_state::GameState;
use crate::systems::*;
use crate::traits::*;
use macroquad::prelude::{GRAY, WHITE, draw_text, draw_texture, next_frame};

pub struct State {
    pub game_state: GameState,
    pub config: Config,
    pub assets: Assets,
    pub entities: Entities,
}
impl State {
    pub async fn init() -> Self {
        let assets: Assets = Assets::load().await;
        let config = Config::init();
        let entities = Entities::build(&config);
        Self {
            game_state: GameState::Pausing,
            config: config,
            assets,
            entities,
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

    pub async fn run(&mut self) {
        loop {
            self.game_state.config_input_handling();
            draw_texture(&self.assets.background, 0., 0., WHITE);
            match self.game_state {
                GameState::Running => {
                    for paddle in self.entities.paddles.iter_mut() {
                        paddle.update(&self.config);
                    }

                    ball_paddle_collision(&mut self.entities.ball, &mut self.entities.paddles);
                    add_score_to_paddle(
                        &mut self.entities.ball,
                        &mut self.entities.paddles,
                        &self.config,
                        &mut self.game_state,
                    );
                    self.entities.ball.collision_detection(&self.config);
                    self.entities.ball.update(&self.config);
                }
                GameState::Pausing => {
                    self.pause_game();
                }
                GameState::GameOver => break,
            }
            for paddle in self.entities.paddles.iter_mut() {
                paddle.draw(&self.assets);
                paddle.draw_scores();
            }
            self.entities.ball.draw(&self.assets);
            next_frame().await;
        }
    }
}
