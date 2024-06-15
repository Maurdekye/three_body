use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, Color, DrawParam, Mesh, MeshBuilder},
    mint::Vector2,
    Context, ContextBuilder, GameResult,
};
use sim::{Body, Environment};
use vector::vec2;

mod sim;
mod vector;

const SUBSTEPS: usize = 2;
const TIME_SCALE: f32 = 70.0;
const NUM_BODIES: usize = 1000;

struct State {
    environment: Environment,
}

impl State {
    pub fn new(_ctx: &mut Context) -> GameResult<State> {
        Ok(State {
            // environment: Environment {
            //     bodies: vec![
            //         Body::new(vec2(0.4, 0.5)),
            //         Body::new(vec2(0.6, 0.5)),
            //         Body {
            //             position: vec2(0.5, 0.55),
            //             radius: 0.02,
            //             ..Default::default()
            //         },
            //     ],
            // },
            environment: Environment {
                bodies: (0..NUM_BODIES).map(|_| {
                let radius = rand::random::<f32>().powi(2) * 0.001 + 0.004;
                let mass = radius.powi(2);
                Body {
                    position: vec2(rand::random(), rand::random()),
                    radius,
                    mass,
                    ..Default::default()
                }}).collect()
            }
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let substep_timescale = TIME_SCALE / (SUBSTEPS as f32);
        for _ in 0..SUBSTEPS {
            self.environment
                .sim_step(ctx.time.delta().as_secs_f32() * substep_timescale);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let (w, h) = ctx.gfx.drawable_size();
        let scale_factor = w.min(h);
        for body in &self.environment.bodies {
            let mut pos = body.position;
            pos *= scale_factor;
            let pos: Vector2<f32> = pos.into();
            let mesh_builder = &mut MeshBuilder::new();
            let mesh_data = mesh_builder
                .circle(
                    graphics::DrawMode::fill(),
                    pos,
                    body.radius * scale_factor,
                    0.1,
                    Color::BLACK,
                )?
                .build();
            let mesh = Mesh::from_data(ctx, mesh_data);
            canvas.draw(&mesh, DrawParam::default())
        }
        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("three body simulation", "maurdekye")
        .window_mode(WindowMode::default().dimensions(1600., 1600.))
        .build()?;
    let state = State::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
