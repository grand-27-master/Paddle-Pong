use ggez::{Context, GameResult, graphics::{self, Rect, DrawMode, Color, DrawParam}};
use ggez::mint::{Vector2, Point2};
use crate::constants::*;

pub struct Ball {
    pub rect: Rect,
    pub velocity: Vector2<f32>,
    pub rotation: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        let rect = Rect::new(
            x - BALL_SIZE / 2.0,
            y - BALL_SIZE / 2.0,
            BALL_SIZE,
            BALL_SIZE,
        );
        let angle = std::f32::consts::PI / 4.0;
        let velocity = Vector2 {
            x: BALL_SPEED * angle.cos(),
            y: BALL_SPEED * angle.sin(),
        };
        Self { rect, velocity, rotation: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        // move
        self.rect.x += self.velocity.x * dt;
        self.rect.y += self.velocity.y * dt;

        // animate rotation (no visual effect on circle, but kept for consistency)
        self.rotation += BALL_ROTATION_SPEED * dt;

        // bounce off left wall
        if self.rect.x <= 0.0 {
            self.rect.x = 0.0;
            self.velocity.x = -self.velocity.x;
        }
        // bounce off right wall
        if self.rect.x + BALL_SIZE >= WINDOW_WIDTH {
            self.rect.x = WINDOW_WIDTH - BALL_SIZE;
            self.velocity.x = -self.velocity.x;
        }
        // bounce off top
        if self.rect.y <= 0.0 {
            self.rect.y = 0.0;
            self.velocity.y = -self.velocity.y;
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        // create a circle mesh centered at origin, radius = BALL_SIZE/2
        let mesh = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2 { x: 0.0, y: 0.0 },
            BALL_SIZE / 2.0,
            2.0,                 // circle tolerance (vertex count tweak)
            Color::WHITE,
        )?;

        // draw it at the ball's center
        graphics::draw(
            ctx,
            &mesh,
            DrawParam::default().dest([
                self.rect.x + BALL_SIZE / 2.0,
                self.rect.y + BALL_SIZE / 2.0,
            ]),
        )
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.rect.x = x - BALL_SIZE / 2.0;
        self.rect.y = y - BALL_SIZE / 2.0;
        let angle = std::f32::consts::PI / 4.0;
        self.velocity.x = BALL_SPEED * angle.cos();
        self.velocity.y = BALL_SPEED * angle.sin();
        self.rotation = 0.0;
    }
}
