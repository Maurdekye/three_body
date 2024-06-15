use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, Color, DrawParam, Quad, Rect},
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
                    Body::new(vec2(0.4, 0.5)),
                    Body::new(vec2(0.6, 0.49)),
                    Body::new(vec2(0.5, 0.55)),
                ],
            },
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        self.environment.sim_step(ctx.time.delta().as_secs_f32() * 0.05);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let (w, h) = ctx.gfx.drawable_size();
        for body in &self.environment.bodies {
            let mut pos = body.position;
            pos *= w.min(h);
            let pos: Vector2<f32> = pos.into();
            canvas.draw(
                &Quad,
                DrawParam::default()
                    .scale([30.0, 30.0])
                    .color(Color::BLACK)
                    .dest(pos),
            )
        }
        // print!("\r{}", self.environment);
        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("three body simulation", "maurdekye")
        .window_mode(WindowMode::default().maximized(true))
        .build()?;
    let state = State::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
