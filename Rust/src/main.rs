extern crate rand;

use rand::Rng;
use std::process::ExitCode;

mod position;

const MAP_HEIGHT: u32 = 500;
const MAP_WIDTH: u32 = 500;


fn main() -> ExitCode {
    let mut maze: Vec<Vec<u8>> = Vec::with_capacity((MAP_HEIGHT*MAP_WIDTH) as usize);
    generate_maze(MAP_HEIGHT, MAP_WIDTH, &mut maze);

    let start = position::Position::new(((maze[0].len() / 2) - 1) as i32,
                                   ((maze.len() / 2) - 1) as i32);
    let goal = position::Position::new((maze[0].len() - 1) as i32, 
                                   (maze.len() - 1) as i32);

    println!("{}  {}", start, goal);


    ExitCode::SUCCESS
}

fn generate_maze(h: u32, w: u32, maze: &mut Vec<Vec<u8>>) {
    maze.reserve(h as usize);
    // height
    for _ in 0..h {
        let mut maze_row: Vec<u8> = Vec::new();
        maze_row.reserve(w as usize);
        // width
        for _ in 0..w {
            maze_row.push(rand::thread_rng().gen_range(0..2));
        }

        maze.push(maze_row);
    }
}