use vector::Vector;

const COULOMB_CONST: f64 = 8.9875517923 * 1e9;
// const GRAVITY_CONST: f64 = 0.0;

pub struct PhysicsEngine<const D: usize> {
    pub position: Vector<D>,
    pub velocity: Vector<D>,
    acceleration: Vector<D>,
    pub mass: f64,
    charge: f64,
}

impl<const D: usize> PhysicsEngine<D> {
    pub fn new(
        position: Vector<D>,
        velocity: Vector<D>,
        mass: f64,
        charge: f64,
    ) -> PhysicsEngine<D> {
        PhysicsEngine {
            position,
            velocity,
            acceleration: Vector::new([0.0; D]),
            mass,
            charge,
        }
    }
    pub fn calculate_interaction(&mut self, other: &mut PhysicsEngine<D>) {
        let displacement = self.position.clone() - other.position.clone();
        let distance = displacement.abs().clone();
        let force = displacement * (self.charge * other.charge * COULOMB_CONST / distance.powi(3));
        self.acceleration += force.clone() / self.mass;
        other.acceleration += force / -other.mass;
    }

    pub fn time_propagate(&mut self, time: f64) {
        self.position += self.velocity * time;
        self.velocity += self.acceleration * time;
        self.acceleration.clear();
    }

    pub fn get_potential_energy(&self, other: &PhysicsEngine<D>) -> f64 {
        COULOMB_CONST * self.charge * other.charge / (self.position - other.position).abs()
    }
    pub fn get_kinetic_energy(&self) -> f64 {
        0.5 * self.mass * self.velocity.inner_product(&self.velocity)
    }
}

pub fn get_total_energy<const D: usize>(physics_engines: &Vec<&PhysicsEngine<D>>) -> f64 {
    let mut energy = 0.0;

    energy += get_total_kinetic_energy(physics_engines);
    energy += get_total_potential_energy(physics_engines);

    energy
}

fn get_total_kinetic_energy<const D: usize>(physics_engines: &Vec<&PhysicsEngine<D>>) -> f64 {
    let mut energy = 0.0;
    for p_1 in physics_engines.iter() {
        energy += p_1.get_kinetic_energy();
    }
    energy
}
fn get_total_potential_energy<const D: usize>(physics_engines: &Vec<&PhysicsEngine<D>>) -> f64 {
    let mut energy = 0.0;
    let n = physics_engines.len();
    for (i, p_1) in physics_engines.iter().enumerate() {
        energy += p_1.get_kinetic_energy();
        for j in i + 1..n {
            let p_2 = &physics_engines[j];
            energy += p_1.get_potential_energy(p_2);
        }
    }
    energy
}

pub fn normalize_velocity<const D: usize>(
    physics_engines: &mut Vec<&mut PhysicsEngine<D>>,
    wanted_energy: f64,
) {
    let immut_engines: Vec<&PhysicsEngine<D>> = physics_engines.iter_mut().map(|p| &**p).collect();
    let k = get_total_kinetic_energy(&immut_engines);
    let u = get_total_potential_energy(&immut_engines);

    let nor = 1e28;
    let coeff = ({
        let x = (wanted_energy - u) / k;
        println!("x : {:.4}", x);
        println!(
            "wanted energy : {:.4}, u : {:.4}, k : {:.4}, e : {:.4}",
            wanted_energy * nor,
            u * nor,
            k * nor,
            (k + u) * nor
        );
        if x > 0.0 {
            x
        } else {
            1.0
        }
    })
    .sqrt();
    for p in physics_engines.iter_mut() {
        p.velocity = p.velocity * coeff;
    }
}
