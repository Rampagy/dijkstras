use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut maze: Vec<Vec<u8>> = Vec::new();
    generate_maze(500, 500, &mut maze);

    for maze_row in maze {
        for val in maze_row {
            print!("{} ", val);
        }
        println!("");
    }

    Ok(())
}

fn generate_maze(h: u32, w: u32, maze: &mut Vec<Vec<u8>>) {
    maze.reserve(h as usize);
    // height
    for _ in 0..h {
        let mut maze_row: Vec<u8> = Vec::new();
        maze_row.reserve(w as usize);
        // width
        for _ in 0..w {
            maze_row.push(0);
        }

        maze.push(maze_row);
    }
}