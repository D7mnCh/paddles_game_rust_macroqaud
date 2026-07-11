// NOTE this module should get systems.rs (system between the entities)


use crate::ball::Ball;
use crate::config::Config;
use crate::paddles::Paddle;
pub struct Entities {
    pub paddles: [Paddle; 2],
    pub ball: Ball,
}
impl Entities {
    pub fn build(config: &Config) -> Self {
        Self {
            paddles: Paddle::build(&config),
            ball: Ball::build(&config),
        }
    }
}
