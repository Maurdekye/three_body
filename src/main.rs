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

const NUM_BODIES: usize = 600;
const SUBSTEPS: usize = 5;
const TIME_SCALE: f32 = 100.0;
const RENDER_RADIUS_SCALE: f32 = 1.0;
const DEFAULT_RADIUS: f32 = 0.0005;

struct State {
    environment: Environment,
}

impl State {

    pub fn init_env() -> Environment {
        // Environment {
        //     bodies: vec![
        //         // Body::new(vec2(1., 0.9)),
        //         Body::new(vec2(0.4, 0.5)),
        //         Body::new(vec2(0.6, 0.5)),
        //         Body::new(vec2(0.5, 0.6)),
        //     ],
        // }
        Environment {
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
    }

    pub fn new(_ctx: &mut Context) -> GameResult<State> {
        Ok(State {
            environment: State::init_env(),
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::Space) {
            self.environment = State::init_env();
        }
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
                    body.radius * scale_factor * RENDER_RADIUS_SCALE,
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
