use vector::Vector;

pub struct RigidWall<const D: usize> {
    normal_vector: Vector<D>,
    crits: f64,
}

impl<const D: usize> RigidWall<D> {
    pub fn new(normal_vector: Vector<D>, crits: f64) -> RigidWall<D> {
        RigidWall {
            normal_vector,
            crits,
        }
    }

    pub fn collide(&self, position: &mut Vector<D>, velocity: &mut Vector<D>) {
        let position_norm = self.normal_vector.inner_product(&position);
        let velocity_norm = self.normal_vector.inner_product(&velocity);
        if position_norm > self.crits {
        } else {
            *position += self.normal_vector * 2.0 * (self.crits - position_norm);
            *velocity += self.normal_vector * -2.0 * velocity_norm;
        }
    }
}
