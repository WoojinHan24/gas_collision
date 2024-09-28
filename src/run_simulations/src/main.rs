pub mod get_square_box;

use dotenv::dotenv;
use get_square_box::{get_balls, get_walls};
use objects::traits::ObjectTrait;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("hello world!");
    dotenv().ok();

    let mut log_file = std::env::var("LOG_DIR").expect("LOG_DIR is not set in .env file");
    log_file.push_str("/output.log");

    let python_interpreter =
        std::env::var("PYTHON_INTERPRETER").expect("PYTHON_INTERPRETER is not set in .env file");

    let python_logger = std::env::var("PYTHON_LOG_PROCESSOR")
        .expect("PYTHON_LOG_PROCESSOR is not set in .env file");

    const DIM: usize = 2;
    let l_x = 10.0;
    let l_y = 10.0;
    let length_info = vec![l_x, l_y];

    let n: usize = 5;
    let mass = 9.11e-31;
    let charge = 1.6 * 1e-19 * 10.0;

    let time_tick = 1e-3;
    let total_time = 10.0;

    let walls = get_walls::<DIM>(length_info.as_ref());

    let mut balls = get_balls::<DIM>(n, length_info.as_ref(), &mass, &charge);

    let mut current_time = 0.0;
    let mut logs = vec![];
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

        let mut positions = vec![];
        for ball in balls.iter() {
            positions.push(ball.position.clone());
        }
        logs.push(Log {
            time: current_time,
            positions,
        });

        current_time += time_tick;
    }

    let file = File::create(log_file.clone()).expect("Unable to create file");
    serde_json::to_writer(file, &logs).expect("Unable to write JSON");

    let python_output = Command::new(python_interpreter)
        .arg(python_logger)
        .env("FILE_POSITION", log_file)
        .output()
        .expect("Failed to execute Python Script");

    if python_output.status.success() {
        println!("Python script executed successfully!");
        io::stdout().write_all(&python_output.stdout).unwrap();
    } else {
        eprintln!("Error executing Python script.");
        io::stderr().write_all(&python_output.stderr).unwrap();
    }
}

use serde::{Deserialize, Serialize};
use vector::Vector;

#[derive(Serialize, Deserialize)]
pub struct Log<const D: usize> {
    time: f64,
    positions: Vec<Vector<D>>, // List of positions of all balls
}
