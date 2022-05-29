#include "main.hpp"

using namespace std;

void generate_maze(vector<vector<int>> &maze)
{
    #if !defined(TEST_DJIKSTRAS)
    maze.reserve(MAP_HEIGHT);
    // height
    for (int i = 0; i < MAP_HEIGHT; i++)
    {
        vector<int> maze_row;
        maze_row.reserve(MAP_WIDTH);
        // width
        for (int j = 0; j < MAP_WIDTH; j++)
        {
            maze_row.push_back(rand() % 255);
        }

        maze.push_back(maze_row);
    }
    #else // defined(TEST_DJIKSTRAS)
    maze = {
        {  1,   1,   1,   1,   1},
        {  1,   0,   1,   1,   1},
        {  1,   0,   0, 255,   1},
        {  1, 255, 255, 255,   1},
        {  1,   1,   1,   1,   1},
    };

    #endif
}


int main ()
{
    vector<vector<int>> maze;
    generate_maze(maze);

    #if defined(TEST_DJIKSTRAS)
    Position start = Position(0, 0);
    #else // !defined(TEST_DJIKSTRAS)
    Position start = Position((maze.front().size()-1) / 2, (maze.size()-1) / 2);
    #endif
    Position goal = Position(maze.front().size()-1, maze.size()-1);

    // make sure start and end are walkable
    maze[start.y][start.x] = 0;
    maze[goal.y][goal.x] = 0;

    // list of function pointers
    vector<vector<Position> (*)(const vector<vector<int>> &, const Position, const Position)> astar_functions;
    astar_functions.push_back( optimized_djikstras_search );

    // for each astar algorithm multiple iterations and report total time
    for (const auto astar_algo : astar_functions)
    {
        double total_time = 0;
        vector<Position> path;

        for (int i = 0; i < SEARCH_ITERATIONS; i++)
        {
            #ifdef TEST_DJIKSTRAS
            cout << i << endl;
            #endif

            chrono::high_resolution_clock::time_point start_time, end_time;
            double duration;

            start_time = chrono::high_resolution_clock::now();

            path = (*astar_algo)(maze, start, goal);

            end_time = chrono::high_resolution_clock::now();
            duration = chrono::duration_cast<chrono::microseconds>(end_time - start_time).count();

            total_time += duration;

            #ifdef TEST_DJIKSTRAS
            for (Position p : path)
            {
                cout << p << " ";
            }
            cout << endl << endl;
            #endif

            // generate new maze every time
            maze.clear();
            generate_maze(maze);

        }

        cout << "C++ path found in " << total_time/1000/1000 << " seconds" << endl;
    }
    
    return 0;
}