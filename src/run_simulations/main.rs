use objects::features::{balls::ChargedBall, walls::RigidWall};
use objects::traits::ObjectTrait;
use vector::Vector;

fn main() {
    println!("hello world!");
    const DIM: usize = 2;
    let l_x = 10.0;
    let l_y = 10.0;
    let e_x = Vector::<DIM>::new([1.0, 0.0]);
    let e_y = Vector::<DIM>::new([0.0, 1.0]);

    let mass = 9.11 * 1e-31;
    let charge = 1.6 * 1e-19;

    let time_tick = 1e-3;
    let total_time = 1000.0;

    let mut walls = vec![
        RigidWall::new(e_x.clone(), 0.0),
        RigidWall::new(e_x.clone() * -1.0, -l_x),
        RigidWall::new(e_y.clone(), 0.0),
        RigidWall::new(e_y.clone() * -1.0, -l_y),
    ];

    let mut balls = vec![
        ChargedBall::new(
            e_x * (l_x / 4.0) + e_y * (l_y / 2.0),
            charge.clone(),
            mass.clone(),
            e_x * 0.0,
        ),
        ChargedBall::new(
            e_x * (3.0 * l_x / 4.0) + e_y * (3.0 * l_y / 4.0),
            charge.clone(),
            mass.clone(),
            e_x * 0.0,
        ),
    ];

    let mut current_time = 0.0;
    while current_time < total_time {
        for i in 0..balls.len() {
            let (left, right) = balls.split_at_mut(i + 1);

            let ball_1 = &mut left[i];

            for ball_2 in right.iter_mut() {
                ball_1.calculate_interaction(ball_2);
            }

            ball_1.time_propagate(time_tick);

            for wall in walls.iter() {
                wall.collide(&mut ball_1.position, &mut ball_1.velocity);
            }
        }
        println!("in time {}", current_time);

        for ball in balls.iter() {
            println!("position : {}", ball.position);
        }

        current_time += time_tick;
    }
}
