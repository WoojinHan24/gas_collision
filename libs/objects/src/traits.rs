use crate::add_on::physics_engine::PhysicsEngine;
use vector::Vector;

pub trait ObjectTrait<const D: usize> {
    fn calculate_interaction(&mut self, other: &mut PhysicsEngine<D>);
    fn time_propagate(&mut self, time: f64);
    fn get_position(&self) -> Vector<D>;
}
