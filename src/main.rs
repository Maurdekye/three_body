use std::{fmt::Display, ptr};

use vector::{vec2, Vector};

mod vector;

#[derive(Debug, Clone)]
pub struct Body {
    pub position: Vector<2>,
    pub velocity: Vector<2>,
    pub mass: f32,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            position: Default::default(),
            velocity: Default::default(),
            mass: 1.0,
        }
    }
}

impl Body {
    pub fn new(position: Vector<2>) -> Body {
        Body {
            position,
            ..Default::default()
        }
    }

    pub fn acceleration(&self, other: &Body) -> Vector<2> {
        let delta = other.position - self.position;
        let dist = delta.len();
        let force = (self.mass * other.mass) / (dist * dist);
        delta.normalized() * force
    }

    pub fn total_acceleration(&self, env: &Environment) -> Vector<2> {
        env.bodies
            .iter()
            .flat_map(|body| (!ptr::eq(self, body)).then(|| self.acceleration(body)))
            .sum()
    }
}

pub struct Environment {
    bodies: Vec<Body>,
}

impl Environment {
    pub fn sim_step(&mut self, delta_time: f32) {
        let accelerations: Vec<_> = self
            .bodies
            .iter()
            .map(|body| body.total_acceleration(&self))
            .collect();
        for (body, accel) in self.bodies.iter_mut().zip(accelerations) {
            body.velocity += accel * delta_time;
            body.position += body.velocity * delta_time;
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.bodies
                .iter()
                .map(|body| format!("{}", body.position))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

fn main() {
    let mut environment = Environment {
        bodies: vec![Body::new(vec2(1.0, 0.0)), Body::new(vec2(-1.0, 0.0))],
    };

    for _ in 0..10 {
        println!("{environment}");
        environment.sim_step(0.1);
    }

    println!("{environment}");
}
