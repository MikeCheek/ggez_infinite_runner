use ggez::mint::Vector2;

pub const GRAVITY:f32 = 0.5;
pub const JUMP_FORCE:Vector2<f32> = Vector2{x:0.0, y:-14.0};
pub const INCREASE_VELOCITY_EVERY: f64 = 5.0;

pub const OBSTACLE_VELOCITY: f32 = -8.0;
pub const OBSTACLE_SPEED_INCREASE_RATE: f32 = -0.1;

pub enum GameState {
    Started,
    GameOver
}