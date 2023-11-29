#include "optimized_djikstras.hpp"
#include "main.hpp"
#include "math.h"

/** References:
 *      https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-greedy-algo-7/
 *      https://www.geeksforgeeks.org/printing-paths-dijkstras-shortest-path-algorithm/
 * 
 */

inline float optimized_heuristic(Position a, Position b)
{
    return (float)abs( (a.x - b.x) + (a.y - b.y) );
}

inline bool greater_comp(const score_T a, const score_T b)
{
    return a.first > b.first;
}

inline void szudzikUnpair(unsigned long z, int* x, int* y)
{
    unsigned long sqrtz = floor(sqrt(z));
    unsigned long sqz = sqrtz * sqrtz;

    if ((z - sqz) >= sqrtz)
    {
        // possible x & y are backwards
        *x = sqrtz;
        *y = z - sqz - sqrtz;
    }
    else
    {
        // possible x & y are backwards
        *x = z - sqz;
        *y = sqrtz;
    }

    return;
}

vector<Position> optimized_djikstras_search(  const vector<vector<int>> &weighted_map,
                                              const Position start, const Position goal )
{
    int mapWidth = weighted_map.front().size();
    int mapHeight = weighted_map.size();
    const unsigned int mapSize = mapWidth*mapHeight;

    vector<Position> path = {};
    if (start.x < 0 || start.y < 0 || goal.x >= mapWidth || goal.y >= mapHeight ||
        start == goal || mapWidth < 2 || mapHeight < 2 )
    {
        return path;
    }

    unordered_set<Position> close_set;
    unordered_map<Position, Position> came_from;
    unordered_map<Position, float> gscore;
    vector<score_T> oheap;
    unordered_map<Position, float> oheap_copy;

    Position current;
    array<Position, 4> neighbors;
    std::unordered_map<Position, float>::iterator open_iter;

    // Memory preallocation
    oheap.reserve(mapWidth + mapHeight);
    oheap_copy.reserve(mapWidth * mapHeight);
    close_set.reserve(mapWidth * mapHeight);
    came_from.reserve(mapWidth * mapHeight);
    gscore.reserve(mapWidth * mapHeight);
    path.reserve(mapWidth + mapHeight);

    // add initial position to the search list
    gscore[start] = 0;
    oheap.emplace_back(gscore[start], start);
    push_heap(oheap.begin(), oheap.end(), greater_comp);
    oheap_copy.emplace(start, gscore[start]);

    int count = 0;
    while ( !oheap.empty() )
    {
        count++;
        pop_heap(oheap.begin(), oheap.end(), greater_comp);
        oheap.pop_back();
        current = oheap.front().second;
        oheap_copy.erase(current);
        close_set.insert(current);

        neighbors = current.get_surrounding_positions();

        // search surrounding neighbors
        for (Position neighbor : neighbors)
        {
            // if the neighbor is a valid position
            if (neighbor.x >= 0 && neighbor.y >= 0 &&
                neighbor.y < mapHeight && neighbor.x < mapWidth &&
                weighted_map[neighbor.y][neighbor.x] < 255)
            {
                float neighbor_gscore = gscore[current] + (float)weighted_map[neighbor.y][neighbor.x] +
                                optimized_heuristic(neighbor, current);

                // if the neighbor is already on the open list check to see if the neighbor is better before updating it
                open_iter = oheap_copy.find(neighbor);
                if (open_iter != oheap_copy.end() && neighbor_gscore < gscore[neighbor])
                {
                    // track the node's parent
                    came_from[neighbor] = current;

                    // gscore = cost to get from start to the current position
                    gscore[neighbor] = neighbor_gscore;

                    // update the neighbors values
                    oheap_copy[neighbor] = neighbor_gscore;


                    // remove the old gscore
                    for (int i = 0; i < oheap.size(); i++)
                    {
                        if (oheap[i].second == neighbor)
                        {
                            oheap.erase(oheap.begin() + i);
                            break;
                        }
                    }

                    // add the new fscore and sort
                    oheap.emplace_back(neighbor_gscore, neighbor);
                    make_heap(oheap.begin(), oheap.end(), greater_comp);
                    continue;
                }

                // check if it is on the closed list
                if (close_set.find(neighbor) != close_set.end() && neighbor_gscore < gscore[neighbor])
                {
                    // remove neighbor from closed list
                    close_set.erase(neighbor);
                }

                // Add to the open list
                if (close_set.find(neighbor) == close_set.end() && open_iter == oheap_copy.end())
                {
                    // track the node's parent
                    came_from[neighbor] = current;

                    // gscore = cost to get from start to the current position
                    gscore[neighbor] = neighbor_gscore;

                    // Add to the open list
                    oheap_copy.emplace(neighbor, neighbor_gscore);
                    oheap.emplace_back(neighbor_gscore, neighbor);
                    push_heap(oheap.begin(), oheap.end(), greater_comp);
                }
            }
        }
    }

    // trace path back from the goal
    current = goal;
    while (current != start)
    {
        path.push_back(current);
        current = came_from[current];
    }

    reverse(path.begin(), path.end());

    return path;
}
