extern crate fastrand;
extern crate priority_queue;
extern crate ordered_float;

mod position;

use std::process::ExitCode;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;
use position::{Position, PositionBuildHasher};

const MAP_HEIGHT: u32 = 500;
const MAP_WIDTH: u32 = 500;
const SEARCH_ITERATIONS: u32 = 100;

#[allow(non_snake_case)]
fn main() -> ExitCode {
    let mut maze: Vec<Vec<u8>> = Vec::with_capacity((MAP_HEIGHT*MAP_WIDTH) as usize);
    generate_maze(&mut maze);

    let start = Position::new(((maze[0].len() / 2) - 1) as i32,
                                        ((maze.len() / 2) - 1) as i32);
    let goal = Position::new((maze[0].len() - 1) as i32, 
                                       (maze.len() - 1) as i32);

    /* Make sure start and end are walkable */
    maze[start.y as usize][start.x as usize] = 0;
    maze[goal.y as usize][goal.x as usize] = 0;

    let mut total_time: f64 = 0.0;
    let mut _path: Vec<Position>;

    for _ in 0..SEARCH_ITERATIONS {
        let start_time = Instant::now();

        _path = optimized_dijkstras_search(&maze, start, goal);

        let duration = start_time.elapsed();
        total_time += duration.as_secs() as f64 + 1e-9f64*duration.subsec_nanos() as f64;

        /* generate new maze every time */
        maze.clear();
        generate_maze(&mut maze);
    }


    println!("Rust path found in {} seconds", total_time);

    ExitCode::SUCCESS
}


#[allow(non_snake_case)]
pub fn optimized_dijkstras_search(  weighted_map: &Vec<Vec<u8>>, start: Position, 
                                    goal: Position ) -> Vec<Position> {
    let mapWidth: usize = weighted_map[0].len();
    let mapHeight: usize = weighted_map.len();

    let mut path: Vec<Position> = Vec::with_capacity(1 as usize);
    if start.x < 0 || start.y < 0 || goal.x >= mapWidth as i32 || goal.y >= mapHeight as i32 ||
       start == goal || mapWidth < 2 || mapHeight < 2 {
        return path;
    }

    /* Memory allocation */
    let mut close_set: HashSet<Position, PositionBuildHasher> = HashSet::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut came_from: HashMap<Position, Position, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut gscore: HashMap<Position, f32, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);
    let mut oheap: PriorityQueue<Position, OrderedFloat<f32>, PositionBuildHasher> = PriorityQueue::with_capacity_and_hasher(mapWidth + mapHeight, PositionBuildHasher);
    let mut oheap_copy: HashMap<Position, f32, PositionBuildHasher> = HashMap::with_capacity_and_hasher(mapHeight * mapWidth, PositionBuildHasher);

    let mut current: Position;
    let mut neighbors: [Position; 4];

    /* Add initial position to the search list */
    gscore.insert(start, 0.0);

    /* Note: gscore is multiplied by -1 before being entered into the oheap
     *  because of how big of a pain in the ass it is to switch it from a
     *  max heap to a min heap */
    oheap.push(start, OrderedFloat::from(-1.0*(*gscore.get(&start).unwrap_or(&0.0))));
    oheap_copy.insert(start, *gscore.get(&start).unwrap_or(&0.0));

    let mut _count: u32 = 0;
    while !oheap.is_empty() {
        _count += 1;
        (current, _) = oheap.pop().unwrap_or((Position::new(0,0), OrderedFloat::from(0.0)));
        oheap_copy.remove(&current);
        close_set.insert(current);

        neighbors = current.get_surrounding_positions();

        /* Search surrounding neighbors */
        for neighbor in neighbors {
            /* if the neighbor is a valid position */
            if neighbor.x >= 0 && neighbor.y >= 0 && 
                    neighbor.y < mapHeight as i32 && neighbor.x < mapWidth as i32 &&
                    weighted_map[neighbor.y as usize][neighbor.x as usize] < 255 {
                let neighbor_gscore: f32 = *gscore.get(&current).unwrap_or(&0.0) + weighted_map[neighbor.y as usize][neighbor.x as usize] as f32 + 
                                            optimized_heuristic(neighbor, current);

                /* if the neighbor is already on the open list check to see if the neighbor is better before updating it*/
                let in_open_list: bool = oheap_copy.contains_key(&neighbor);
                if in_open_list && neighbor_gscore < *gscore.get(&neighbor).unwrap_or(&0.0){
                    /* track the node's parent */
                    came_from.insert(neighbor, current);

                    /* gscore = cost to get from the start to the current position */
                    gscore.entry(neighbor).and_modify(|val| *val = neighbor_gscore);

                    /* update the neighbors values */
                    oheap_copy.entry(neighbor).and_modify(|val| *val = neighbor_gscore);

                    /* remove the old gscore */
                    oheap.remove(&neighbor);

                    /* Add the new fscore and sort */
                    oheap.push(neighbor, OrderedFloat::from(-1.0*neighbor_gscore));
                    continue;
                }

                /* check if it is on the closed list */
                if close_set.contains(&neighbor) && neighbor_gscore < *gscore.get(&neighbor).unwrap_or(&0.0) {
                    /* remove neighbor from closed list */
                    close_set.remove(&neighbor);
                }

                /* Add to the open list */
                if !close_set.contains(&neighbor) && !in_open_list {
                    /* track the node's parent */
                    came_from.insert(neighbor, current);

                    /* gscore = cost to get rom the start to the current position */
                    gscore.insert(neighbor, neighbor_gscore);

                    /* add to the open list */
                    oheap_copy.insert(neighbor, neighbor_gscore);
                    oheap.push(neighbor, OrderedFloat::from(-1.0*neighbor_gscore));
                }
            }
        }
    }

    /* trace path back from the goal */
    current = goal;
    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap_or(&Position::new(0,0));
    }

    path.reverse();

    return path;
}

#[inline]
fn optimized_heuristic(a: Position, b: Position) -> f32 {
    return (((a.x - b.x) + (a.y - b.y)) as f32).abs();
}


fn generate_maze(maze: &mut Vec<Vec<u8>>) {
    maze.reserve(MAP_HEIGHT as usize);

    // height
    for _ in 0..MAP_HEIGHT {
        let mut maze_row: Vec<u8> = Vec::with_capacity(MAP_WIDTH as usize);
        // width
        for _ in 0..MAP_WIDTH {
            maze_row.push(fastrand::u8(0..2));
        }

        maze.push(maze_row);
    }
}