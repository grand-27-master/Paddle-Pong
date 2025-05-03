use ggez::{Context, GameResult, event, graphics::{self, Text, DrawParam, Color}};
use crate::model::paddle::Paddle;
use crate::model::ball::Ball;
use crate::model::constants::{WINDOW_WIDTH, WINDOW_HEIGHT, INITIAL_LIVES};
use crate::model::utils::rects_intersect;

pub struct GameState {
    player: Paddle,
    ball: Ball,
    score: u32,
    lives: i32,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let player = Paddle::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT - 30.0);
        let ball   = Ball::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        Ok(Self { player, ball, score: 0, lives: INITIAL_LIVES })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // **Model calls**: update internal state
        self.player.update(ctx, dt);
        self.ball.update(dt);

        // **Controller logic**: collision, scoring, lives
        if rects_intersect(self.ball.rect, self.player.rect) {
            self.ball.velocity.y = -self.ball.velocity.y;
            self.ball.rect.y = self.player.rect.y - crate::model::constants::BALL_SIZE;
            self.score += 1;
        }

        if self.ball.rect.y > WINDOW_HEIGHT {
            self.lives -= 1;
            if self.lives <= 0 {
                ggez::event::quit(ctx);
            } else {
                self.ball.reset(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // **View**: clear, draw model + UI
        graphics::clear(ctx, Color::BLACK);
        self.player.draw(ctx)?;
        self.ball.draw(ctx)?;
        let score_text = Text::new(format!("Score: {}", self.score));
        let lives_text = Text::new(format!("Lives: {}", self.lives));
        graphics::draw(ctx, &score_text, DrawParam::default().dest([10.0, 10.0]))?;
        graphics::draw(
            ctx,
            &lives_text,
            DrawParam::default().dest([WINDOW_WIDTH - 100.0, 10.0]),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}