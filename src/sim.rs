use std::{fmt::Display, ptr};

use crate::vector::Vector;

const COLLISION_RADIUS: f32 = 0.0;

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

        // gravitation
        let accelerations: Vec<_> = self
            .bodies
            .iter()
            .map(|body| body.total_acceleration(&self))
            .collect();

        for (body, accel) in self.bodies.iter_mut().zip(accelerations) {
            body.velocity += accel * delta_time;
            body.position += body.velocity * delta_time;
        }

        // collision (thanks gpt)
        let collisions: Vec<_> = (0..self.bodies.len()).flat_map(|i| {
            ((i+1)..self.bodies.len()).filter_map(|j| {
                let (a, b) = (&self.bodies[i], &self.bodies[j]);
                ((a.position - b.position).len() < COLLISION_RADIUS).then_some((i, j))
            }).collect::<Vec<_>>()
        }).collect();

        for (i, j) in collisions {
            // println!("{i}, {j}");
            let (a, b) = self.bodies.split_at_mut(j);
            let (a, b) = (&mut a[i], &mut b[0]);

            let normal = (a.position - b.position).normalized();

            let relative_velocity = a.velocity - b.velocity;
            let velocity_along_normal = relative_velocity.dot(normal);

            if velocity_along_normal <= 0. {
                let restitution = 1.0;
                let impulse_magnitude = -(1.0 + restitution) * velocity_along_normal / (1.0 / a.mass + 1.0 / b.mass);
    
                let impulse = normal * impulse_magnitude;
                a.velocity -= impulse / a.mass;
                b.velocity += impulse / b.mass;
            }
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