#ifndef MAIN_HPP
#define MAIN_HPP

#include <iostream>
#include <chrono>
#include <stdlib.h>

#include "position.hpp"
#include "optimized_djikstras.hpp"

//#define TEST_DJIKSTRAS



#ifdef TEST_DJIKSTRAS
    // override the above search iterations if testing
    // the astar algorithm
    #define SEARCH_ITERATIONS 1
#else // !defined(TEST_DJIKSTRAS)
    #define MAP_WIDTH 500
    #define MAP_HEIGHT 500
    #define SEARCH_ITERATIONS 100
    #define MAX_MAP_HEIGHT 10
#endif // TEST_DJIKSTRAS


#endif