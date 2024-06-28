use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, Color, DrawParam, Mesh, MeshBuilder},
    input::keyboard::KeyCode,
    mint::Vector2,
    Context, ContextBuilder, GameResult,
};
use sim::{Body, Environment};
use vector::{vec2, Vector};

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
        // Environment::new(
        //     (0..NUM_BODIES)
        //         .map(|_| {
        //             let radius = rand::random::<f32>().powi(2) * 0.001 + 0.004;
        //             let mass = radius.powi(2);
        //             Body {
        //                 position: vec2(rand::random(), rand::random()),
        //                 radius,
        //                 mass,
        //                 ..Default::default()
        //             }
        //         })
        //         .collect(),
        // )
        Environment::new(Vec::new())
    }

    pub fn new(_ctx: &mut Context) -> GameResult<State> {
        Ok(State {
            environment: State::init_env(),
        })
    }

    fn get_scale_factor(&self, ctx: &Context) -> f32 {
        let (w, h) = ctx.gfx.drawable_size();
        w.min(h)
    }

    fn get_screen_point(&self, ctx: &Context, mut point: Vector<2>) -> Vector2<f32> {
        let scale_factor = self.get_scale_factor(ctx);
        point *= scale_factor;
        point.into()
    }

    fn get_sim_point(&self, ctx: &Context, point: Vector2<f32>) -> Vector<2> {
        let scale_factor = self.get_scale_factor(ctx);
        let point: Vector<2> = point.into();
        point / scale_factor
    }
}

const NUMBER_KEYS: [KeyCode; 10] = [
    KeyCode::Key0,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
];

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mouse_point = self.get_sim_point(ctx, ctx.mouse.position().into());

        // instantaneous events
        if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
            self.environment = State::init_env();
        }

        for (i, &key) in NUMBER_KEYS.iter().enumerate() {
            if ctx.keyboard.is_key_just_pressed(key) {
                let radius = 1.5f32.powf(i as f32) * 0.0025;
                let mass = radius.powi(2);
                let body = Body {
                    position: mouse_point,
                    radius,
                    mass,
                    ..Default::default()
                };
                self.environment.add(body);
                break; // cant add two bodies in the same frame or it bugs out :/
            }
        }

        let substep_timescale = TIME_SCALE / (SUBSTEPS as f32);
        for _ in 0..SUBSTEPS {
            let mut step = self
                .environment
                .sim_step_context(ctx.time.delta().as_secs_f32() * substep_timescale);

            step.sim_step();

            // continuous events
            if ctx.mouse.button_pressed(event::MouseButton::Left) {
                step.attract_to(0.0001, mouse_point);
            }

            if ctx.keyboard.is_key_pressed(KeyCode::D) {
                step.dampen(0.0001);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let scale_factor = self.get_scale_factor(ctx);
        for body in self.environment.bodies() {
            let pos = self.get_screen_point(ctx, body.position);
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
