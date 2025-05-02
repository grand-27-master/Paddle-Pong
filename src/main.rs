use ggez::conf::{WindowMode, WindowSetup};
use ggez::{ContextBuilder, event};
use pong_game::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};
use pong_game::game::GameState;

fn main() -> ggez::GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("pong_game", "Vashishth")
        .window_setup(WindowSetup::default().title("Paddle Pong"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
