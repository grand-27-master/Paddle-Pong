use ggez::{Context, GameResult, graphics::{self, Rect, DrawMode}};
use crate::constants::*;
use crate::utils::clamp;

pub struct Paddle {
    pub rect: Rect,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        let rect = Rect::new(
            x - PADDLE_WIDTH / 2.0,
            y - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );
        Self { rect }
    }

    pub fn update(&mut self, ctx: &Context, dt: f32) {
        use ggez::input::keyboard::KeyCode;
        let mut dir = 0.0;
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            dir -= 1.0;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            dir += 1.0;
        }
        self.rect.x += dir * PADDLE_SPEED * dt;
        // keep inside window
        self.rect.x = clamp(self.rect.x, 0.0, WINDOW_WIDTH - PADDLE_WIDTH);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.rect,
            graphics::Color::WHITE,
        )?;
        graphics::draw(ctx, &mesh, ([0.0, 0.0],))
    }
}

