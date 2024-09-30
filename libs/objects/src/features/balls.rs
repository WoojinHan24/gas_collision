use crate::add_on::physics_engine::PhysicsEngine;
use crate::traits::ObjectTrait;
use vector::Vector;

pub struct ChargedBall<const D: usize> {
    pub physics_engine: PhysicsEngine<D>,
}

impl<const D: usize> ChargedBall<D> {
    pub fn new(physics_engine: PhysicsEngine<D>) -> ChargedBall<D> {
        ChargedBall { physics_engine }
    }
}

impl<const D: usize> ObjectTrait<D> for ChargedBall<D> {
    fn calculate_interaction(&mut self, other: &mut PhysicsEngine<D>) {
        self.physics_engine.calculate_interaction(other);
    }

    fn time_propagate(&mut self, time: f64) {
        self.physics_engine.time_propagate(time);
    }

    fn get_position(&self) -> Vector<D> {
        self.physics_engine.position.clone()
    }
}
