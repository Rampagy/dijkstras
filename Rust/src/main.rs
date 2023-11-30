extern crate rand;

use rand::Rng;
use std::process::ExitCode;
use std::time::{Instant};

mod position;
mod optimized_dijkstras;

const MAP_HEIGHT: u32 = 500;
const MAP_WIDTH: u32 = 500;
const SEARCH_ITERATIONS: u32 = 1;

#[allow(non_snake_case)]
fn main() -> ExitCode {
    let mut maze: Vec<Vec<u8>> = Vec::with_capacity((MAP_HEIGHT*MAP_WIDTH) as usize);
    generate_maze(&mut maze);

    let start = position::Position::new(((maze[0].len() / 2) - 1) as i32,
                                        ((maze.len() / 2) - 1) as i32);
    let goal = position::Position::new((maze[0].len() - 1) as i32, 
                                       (maze.len() - 1) as i32);

    /* Make sure start and end are walkable */
    maze[start.y as usize][start.x as usize] = 0;
    maze[goal.y as usize][goal.x as usize] = 0;

    let mut total_time: f64 = 0.0;
    let mut path: Vec<position::Position>;

    for _ in 0..SEARCH_ITERATIONS {
        let start_time = Instant::now();

        path = optimized_dijkstras::optimized_dijkstras_search(&maze, start, goal);

        let duration = start_time.elapsed();
        total_time += duration.as_secs() as f64 + 1e-9f64*duration.subsec_nanos() as f64;

        /* generate new maze every time */
        maze.clear();
        generate_maze(&mut maze);
    }


    println!("Rust path found in {} seconds", total_time);

    ExitCode::SUCCESS
}

fn generate_maze(maze: &mut Vec<Vec<u8>>) {
    maze.reserve(MAP_HEIGHT as usize);
    // height
    for _ in 0..MAP_HEIGHT {
        let mut maze_row: Vec<u8> = Vec::new();
        maze_row.reserve(MAP_WIDTH as usize);
        // width
        for _ in 0..MAP_WIDTH {
            maze_row.push(rand::thread_rng().gen_range(0..2));
        }

        maze.push(maze_row);
    }
}