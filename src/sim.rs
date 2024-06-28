use std::{fmt::Display, ptr};

use crate::{vector::Vector, DEFAULT_RADIUS};

#[derive(Debug, Clone)]
pub struct Body {
    pub position: Vector<2>,
    pub velocity: Vector<2>,
    pub mass: f32,
    pub radius: f32,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            position: Default::default(),
            velocity: Default::default(),
            mass: 1.0,
            radius: DEFAULT_RADIUS,
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
    pub fn new(bodies: Vec<Body>) -> Self {
        Self { bodies }
    }

    pub fn sim_step_context(&mut self, delta_time: f32) -> EnvironmentSimulator<'_> {
        EnvironmentSimulator {
            env: self,
            delta_time,
            collisions: Vec::new(),
        }
    }

    pub fn bodies(&self) -> &Vec<Body> {
        &self.bodies
    }

    pub fn add(&mut self, body: Body) {
        self.bodies.push(body);
    }
}

pub struct EnvironmentSimulator<'a> {
    env: &'a mut Environment,
    delta_time: f32,
    collisions: Vec<Vector<2>>,
}

impl EnvironmentSimulator<'_> {
    pub fn sim_step(&mut self) -> &mut Self {
        // gravitation
        let accelerations: Vec<_> = self
            .env
            .bodies
            .iter()
            .map(|body| body.total_acceleration(&self.env))
            .collect();

        for (body, accel) in self.env.bodies.iter_mut().zip(accelerations) {
            body.velocity += accel * self.delta_time;
            body.position += body.velocity * self.delta_time;
        }

        // collision (thanks gpt)
        let collisions: Vec<_> = (0..self.env.bodies.len())
            .flat_map(|i| {
                ((i + 1)..self.env.bodies.len())
                    .filter_map(|j| {
                        let a = &self.env.bodies[i];
                        let b = &self.env.bodies[j];
                        ((a.position - b.position).len() < a.radius + b.radius).then_some((i, j))
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for (i, j) in collisions {
            let (a, b) = self.env.bodies.split_at_mut(j);
            let (a, b) = (&mut a[i], &mut b[0]);

            let delta = a.position - b.position;
            let normal = delta.normalized();

            let relative_velocity = a.velocity - b.velocity;
            let velocity_along_normal = relative_velocity.dot(normal);

            if velocity_along_normal <= 0. {
                let restitution = 1.0;
                let impulse_magnitude =
                    -(1.0 + restitution) * velocity_along_normal / (1.0 / a.mass + 1.0 / b.mass);

                let impulse = normal * impulse_magnitude;
                a.velocity += impulse / a.mass;
                b.velocity -= impulse / b.mass;
            }

            let collision_point = delta * (b.radius / (a.radius + b.radius)) + b.position;
            a.position = collision_point + normal * a.radius;
            b.position = collision_point - normal * b.radius;
            self.collisions.push(collision_point);
        }

        self
    }

    pub fn attract_to(&mut self, force: f32, position: Vector<2>) {
        for body in self.env.bodies.iter_mut() {
            let delta = position - body.position;
            let delta_norm = delta.normalized();
            let strength = (force * self.delta_time);// / body.mass;
            body.velocity += delta_norm * strength;
        }
    }

    pub fn dampen(&mut self, force: f32) {
        for body in self.env.bodies.iter_mut() {
            let strength = (force * self.delta_time);// / body.mass;
            body.velocity -= body.velocity.normalized() * strength;
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
