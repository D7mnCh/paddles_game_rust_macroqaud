use crate::assets::*;
use crate::paddles::*;
use crate::ball::*;
use crate::*;

pub trait Renderable {
    fn draw(&self);
}
pub trait Movable {
    fn _move(&mut self);
}
pub trait Collidable {}

struct State {
    is_running: bool,
    Assets: Assets,
    paddles: [Paddles;2],
    ball: Ball2
}
impl State {
    pub fn collistion(&mut self) {
        // check for the height : if the edges (top/bottom) of the ball is
        // (bigger/smaller) then the (top/bottom) of the paddel
        // and also for (left/right)
        let dir: f32 = {
            let dir = (rand() % 2) as f32;
            if dir == 0. { 1. } else { -1. }
        };
        for paddle in self.paddles.iter_mut() {
            match paddle {
                // don't forget to comment
                // the variant can't have the same inner value, you need a method to make the
                // the compiler trusts you !
                Paddles::Right(paddle) =>{
                    if self.ball.pos.x <= paddle.pos.x + PADDEL_WIDTH
                        && self.ball.pos.x + BALL_DIM >= paddle.pos.x
                            && paddle.pos.y <= self.ball.pos.y + BALL_DIM
                            && paddle.pos.y + PADDEL_HEIGHT >= self.ball.pos.y
                    {
                        // alwasy when you have tunneling, just teleport it
                        let ball_teleport_by = 7.;
                        if self.ball.pos.x < WIDTH as f32 / 2. {
                            self.ball.pos.x += ball_teleport_by;
                        } else {
                            self.ball.pos.x -= ball_teleport_by;
                        }
                        unsafe {BALL_VEL *= -1.;}
                    }
                    // update score for each paddle
                    // left
                    if self.ball.pos.x <= 0. && paddle.pos.x > (WIDTH / 2) as f32 {
                        self.is_running = false;

                        unsafe {BALL_VEL *= dir;}

                        self.ball.pos.x = WIDTH as f32 / 2.;
                        self.ball.pos.y = HEIGHT as f32 / 2.;
                        paddle.score += 1;
                        // right
                    } else if self.ball.pos.x + BALL_DIM >= WIDTH as f32 && paddle.pos.x < (WIDTH / 2) as f32 {
                        self.is_running = false;

                        unsafe {BALL_VEL *= -1.;}

                        self.ball.pos.x = WIDTH as f32 / 2.;
                        self.ball.pos.y = HEIGHT as f32 / 2.;

                        paddle.score += 1;
                    }

                },
                Paddles::Left(paddle) => {

                },
                _ => todo!(),
            }
        }
    }
}
