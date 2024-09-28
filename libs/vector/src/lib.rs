use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vector<const D: usize> {
    coordinates: [f64; D],
}

impl<const D: usize> Vector<D> {
    pub fn new(coordinates: [f64; D]) -> Self {
        Vector { coordinates }
    }

    pub fn inner_product(&self, other: &Vector<D>) -> f64 {
        self.coordinates
            .iter()
            .zip(other.coordinates.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn abs(&self) -> f64 {
        self.inner_product(self).sqrt()
    }

    pub fn clear(&mut self) {
        self.coordinates = [0.0; D]
    }
}

impl<const D: usize> Sub for Vector<D> {
    type Output = Vector<D>;

    fn sub(self, other: Vector<D>) -> Self::Output {
        let coordinates = self
            .coordinates
            .iter()
            .zip(other.coordinates.iter())
            .map(|(a, b)| a - b)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        Vector { coordinates }
    }
}

impl<const D: usize> Add for Vector<D> {
    type Output = Vector<D>;

    // Add corresponding coordinates from two Vectors and return a new Vector.
    fn add(self, other: Vector<D>) -> Self::Output {
        let coordinates = self
            .coordinates
            .iter()
            .zip(other.coordinates.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap(); // Convert Vec<f64> back to [f64; D]
        Vector { coordinates }
    }
}
impl<const D: usize> AddAssign for Vector<D> {
    // Perform in-place addition of another Vector to self.
    fn add_assign(&mut self, other: Vector<D>) {
        for (a, b) in self.coordinates.iter_mut().zip(other.coordinates.iter()) {
            *a += b;
        }
    }
}

impl<const D: usize> Mul<f64> for Vector<D> {
    type Output = Vector<D>;

    fn mul(self, scalar: f64) -> Self::Output {
        let coordinates = self
            .coordinates
            .iter()
            .map(|&coord| coord * scalar)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        Vector { coordinates }
    }
}

impl<const D: usize> Div<f64> for Vector<D> {
    type Output = Vector<D>;

    // Divide each coordinate of the Vector by a scalar value and return a new Vector.
    fn div(self, scalar: f64) -> Self::Output {
        let coordinates = self
            .coordinates
            .iter()
            .map(|&coord| coord / scalar)
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap(); // Convert Vec<f64> back to [f64; D]
        Vector { coordinates }
    }
}

impl<const D: usize> fmt::Display for Vector<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_coords: Vec<String> = self
            .coordinates
            .iter()
            .map(|&x| format!("{:.2}", x))
            .collect();

        write!(f, "({})", formatted_coords.join(", "))
    }
}
