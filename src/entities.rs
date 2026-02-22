use crate::ball::Ball;
use crate::config::Config;
use crate::paddles::Paddle;
pub struct Entities {
    pub paddles: [Paddle; 2],
    pub ball: Ball,
}
impl Entities {
    pub fn new(config: &Config) -> Self {
        Self {
            paddles: Paddle::new(&config),
            ball: Ball::new(&config),
        }
    }
}
