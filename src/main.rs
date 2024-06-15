use ggez::{
    event::{self, EventHandler},
    graphics::{self, Color, DrawParam, Quad},
    mint::Vector2,
    Context, ContextBuilder, GameResult,
};
use sim::{Body, Environment};
use vector::vec2;

mod sim;
mod vector;

struct State {
    environment: Environment,
}

impl State {
    pub fn new(_ctx: &mut Context) -> GameResult<State> {
        Ok(State {
            environment: Environment {
                bodies: vec![
                    Body::new(vec2(1.0, 0.0)),
                    Body::new(vec2(-1.0, 0.0)),
                    Body::new(vec2(0.0, 1.0)),
                ],
            },
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        self.environment
            .sim_step(ctx.time.delta().as_secs_f32() / 10.0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        for body in &self.environment.bodies {
            let pos: Vector2<f32> = body.position.into();
            canvas.draw(
                &Quad,
                DrawParam::default()
                    .scale([0.5, 0.5])
                    .color(Color::BLACK)
                    .dest(pos),
            )
        }
        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) =
        ContextBuilder::new("three body simulation", "maurdekye").build()?;
    let state = State::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
