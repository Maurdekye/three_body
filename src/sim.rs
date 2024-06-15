use std::{fmt::Display, ptr};

use crate::vector::Vector;


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
    pub bodies: Vec<Body>,
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