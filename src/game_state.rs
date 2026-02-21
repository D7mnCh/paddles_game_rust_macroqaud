use macroquad::prelude::{KeyCode, is_key_pressed};
pub enum GameState {
    Running,
    Pausing,
    GameOver,
}
impl GameState {
    pub fn config_input_handling(self: &mut Self) {
        if is_key_pressed(KeyCode::Escape) {
            match self {
                _ => *self = GameState::GameOver,
            }
        }
        if is_key_pressed(KeyCode::Space) {
            match self {
                GameState::Running => *self = GameState::Pausing,
                GameState::Pausing => *self = GameState::Running,
                _ => (),
            }
        }
    }
}
