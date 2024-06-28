use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use ggez::mint::Vector2;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Vector<const D: usize>([f32; D]);

impl<const D: usize> Vector<D> {
    pub fn dot(self, rhs: Vector<D>) -> f32 {
        self.0
            .into_iter()
            .zip(rhs.0)
            .map(|(left, right)| left * right)
            .sum()
    }

    pub fn len(&self) -> f32 {
        self.0.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    pub fn normalized(&self) -> Vector<D> {
        *self / self.len()
    }

    pub fn normalize(&mut self) {
        *self /= self.len();
    }
}

macro_rules! component {
    ($n:ident, $nm:ident, $i:literal) => {
        pub fn $n(&self) -> f32 {
            self.0[$i]
        }

        pub fn $nm(&mut self) -> &mut f32 {
            &mut self.0[$i]
        }
    };
}

impl Vector<2> {
    component!(x, x_mut, 0);
    component!(y, y_mut, 1);
}

impl Vector<3> {
    component!(x, x_mut, 0);
    component!(y, y_mut, 1);
    component!(z, z_mut, 2);

    pub fn cross(&self, rhs: Vector<3>) -> Vector<3> {
        vec3(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

pub fn vec2(x: f32, y: f32) -> Vector<2> {
    Vector([x, y])
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vector<3> {
    Vector([x, y, z])
}

impl<const D: usize> Add for Vector<D> {
    type Output = Vector<D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut components = [0.0; D];
        for i in 0..D {
            components[i] = self.0[i] + rhs.0[i];
        }
        Vector(components)
    }
}

impl<const D: usize> AddAssign<Vector<D>> for Vector<D> {
    fn add_assign(&mut self, rhs: Vector<D>) {
        for i in 0..D {
            self.0[i] += rhs.0[i];
        }
    }
}

impl<const D: usize> Sub for Vector<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut components = [0.0; D];
        for i in 0..D {
            components[i] = self.0[i] - rhs.0[i];
        }
        Vector(components)
    }
}

impl<const D: usize> SubAssign<Vector<D>> for Vector<D> {
    fn sub_assign(&mut self, rhs: Vector<D>) {
        for i in 0..D {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl<const D: usize> Mul<f32> for Vector<D> {
    type Output = Vector<D>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut components = [0.0; D];
        for i in 0..D {
            components[i] = self.0[i] * rhs
        }
        Vector(components)
    }
}

impl<const D: usize> MulAssign<f32> for Vector<D> {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..D {
            self.0[i] *= rhs;
        }
    }
}

impl<const D: usize> Div<f32> for Vector<D> {
    type Output = Vector<D>;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl<const D: usize> DivAssign<f32> for Vector<D> {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl<const D: usize> Display for Vector<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.0
                .iter()
                .map(|c| format!("{c:.2}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl<const D: usize> Default for Vector<D> {
    fn default() -> Self {
        Self([0.0; D])
    }
}

impl<const D: usize> Sum for Vector<D> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

impl Into<Vector2<f32>> for Vector<2> {
    fn into(self) -> Vector2<f32> {
        Vector2 {
            x: self.0[0],
            y: self.0[1],
        }
    }
}

impl From<Vector2<f32>> for Vector<2> {
    fn from(value: Vector2<f32>) -> Self {
        Vector([value.x, value.y])
    }
}
