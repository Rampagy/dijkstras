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
    let mapSizeL: usize = mapWidth*mapHeight;

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

    let mut current: Position = Position::new(0, 0);
    let mut neighbors: [Position; 4];

    /* Add initial position to the search list */
    gscore.insert(start, 0.0);

    /* Note: gscore is multiplied by -1 before being entered into the oheap
     *  because of how big of a pain in the ass it is to switch it from a
     *  max heap to a min heap */
    oheap.push(start, OrderedFloat::from(-1.0*gscore.get(&start).unwrap()));
    oheap.push(Position::new(5, 6), OrderedFloat(-1.2));



    for (pos, val) in oheap.into_sorted_iter() {
        println!("{} {}", pos, val);
    }


    return path;
}