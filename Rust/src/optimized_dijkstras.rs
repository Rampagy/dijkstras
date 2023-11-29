use super::position::{Position};
use std::collections::{HashSet, HashMap};

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
    let mut oheap: Vec<(f32, Position)> = Vec::with_capacity(mapWidth + mapHeight);
    let mut oheap_copy: HashMap<Position, f32> = HashMap::with_capacity(mapHeight * mapWidth);

    let mut current: Position = Position::new(0, 0);
    let mut neighbors: [Position; 4];

    /* Add initial position to the search list */
    gscore.insert(start, 0.0);
    oheap.push((*gscore.get(&start).unwrap(), start));


    return path;
}