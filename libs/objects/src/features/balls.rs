use crate::object_log::Log;
use crate::traits::ObjectTrait;
use vector::Vector;

const COULOMB_CONST: f64 = 8.99 * 1e9;

pub struct ChargedBall<const D: usize> {
    pub position: Vector<D>,
    mass: f64,
    charge: f64,
    pub velocity: Vector<D>,
    acceleration: Vector<D>,
}

impl<const D: usize> ChargedBall<D> {
    pub fn new(position: Vector<D>, charge: f64, mass: f64, velocity: Vector<D>) -> ChargedBall<D> {
        ChargedBall {
            position,
            mass,
            charge,
            velocity,
            acceleration: Vector::new([0.0; D]),
        }
    }
}

impl<const D: usize> ObjectTrait for ChargedBall<D> {
    fn calculate_interaction(&mut self, other: &mut Self) {
        let displacement = self.position.clone() - other.position.clone();
        let distance = displacement.abs().clone();
        let force = displacement * (self.charge * other.charge * COULOMB_CONST / distance.powi(2));

        self.acceleration += force.clone() / self.mass;
        other.acceleration += force / -other.mass;
    }

    fn time_propagate(&mut self, time: f64) {
        self.position += self.velocity * time;
        self.velocity += self.acceleration * time;
        self.acceleration.clear();
    }
    fn get_log(self) -> Log {
        todo!()
    }
}
