use objects::features::{balls::ChargedBall, walls::RigidWall};
use rand::Rng;
use vector::Vector;

pub fn get_walls<const DIM: usize>(length_info: &Vec<f64>) -> Vec<RigidWall<DIM>> {
    let basis_set = get_basis_set::<DIM>();

    let mut walls = vec![];
    for (e, len) in basis_set.iter().zip(length_info.iter()) {
        walls.push(RigidWall::new(e.clone(), 0.0));
        walls.push(RigidWall::new(e.clone() * -1.0, -len));
    }
    walls
}

pub fn get_balls<const DIM: usize>(
    n: usize,
    length_info: &Vec<f64>,
    mass: &f64,
    charge: &f64,
) -> Vec<ChargedBall<DIM>> {
    let zero_vec = Vector::<DIM>::new([0.0; DIM]);
    let mut rng = rand::thread_rng();
    // let basis_set = get_basis_set();

    let mut balls = vec![];
    for _ in 0..n {
        let mut coords: [f64; DIM] = [0.0; DIM];
        for (i, len) in length_info.iter().enumerate() {
            coords[i] = rng.gen::<f64>() * len;
        }

        balls.push(ChargedBall::new(
            Vector::<DIM>::new(coords),
            charge.clone(),
            mass.clone(),
            zero_vec.clone(),
        ))
    }
    balls
}

fn get_basis_set<const DIM: usize>() -> Vec<Vector<DIM>> {
    let mut basis_set = vec![];
    let zero_vec = Vector::<DIM>::new([0.0; DIM]);

    for i in 0..DIM {
        basis_set.push({
            let mut normal_vector = zero_vec.clone();
            normal_vector.coordinates[i] = 1.0;
            normal_vector
        })
    }
    basis_set
}
