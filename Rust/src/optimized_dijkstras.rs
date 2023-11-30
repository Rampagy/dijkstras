extern crate priority_queue;
extern crate ordered_float;

use super::position::{Position};
use std::collections::{HashSet, HashMap};
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;

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
    let mut close_set: HashSet<Position> = HashSet::with_capacity(mapHeight * mapWidth);
    let mut came_from: HashMap<Position, Position> = HashMap::with_capacity(mapHeight * mapWidth);
    let mut gscore: HashMap<Position, f32> = HashMap::with_capacity(mapHeight * mapWidth);
    let mut oheap: PriorityQueue<Position, OrderedFloat<f32>> = PriorityQueue::with_capacity(mapWidth + mapHeight);
    let mut oheap_copy: HashMap<Position, f32> = HashMap::with_capacity(mapHeight * mapWidth);

    let mut current: Position;
    let mut neighbors: [Position; 4];

    /* Add initial position to the search list */
    gscore.insert(start, 0.0);

    /* Note: gscore is multiplied by -1 before being entered into the oheap
     *  because of how big of a pain in the ass it is to switch it from a
     *  max heap to a min heap */
    oheap.push(start, OrderedFloat::from(-1.0*(*gscore.get(&start).unwrap())));
    oheap_copy.insert(start, *gscore.get(&start).unwrap());

    let mut _count: u32 = 0;
    while !oheap.is_empty() {
        _count += 1;
        (current, _) = oheap.pop().unwrap();
        oheap_copy.remove(&current);
        close_set.insert(current);

        neighbors = current.get_surrounding_positions();

        /* Search surrounding neighbors */
        for neighbor in neighbors {
            /* if the neighbor is a valid position */
            if neighbor.x >= 0 && neighbor.y >= 0 && 
                    neighbor.y < mapHeight as i32 && neighbor.x < mapWidth as i32 &&
                    weighted_map[neighbor.y as usize][neighbor.x as usize] < 255 {
                let neighbor_gscore: f32 = *gscore.get(&current).unwrap() + weighted_map[neighbor.y as usize][neighbor.x as usize] as f32 + 
                                            optimized_heuristic(neighbor, current);

                /* if the neighbor is already on the open list check to see if the neighbor is better before updating it*/
                let in_open_list: bool = oheap_copy.contains_key(&neighbor);
                if in_open_list && neighbor_gscore < *gscore.get(&neighbor).unwrap(){
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
                if close_set.contains(&neighbor) && neighbor_gscore < *gscore.get(&neighbor).unwrap() {
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
        current = *came_from.get(&current).unwrap();
    }

    path.reverse();

    return path;
}

#[inline]
fn optimized_heuristic(a: Position, b: Position) -> f32 {
    return (((a.x - b.x) + (a.y - b.y)) as f32).abs();
}