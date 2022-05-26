#ifndef POSITION_HPP
#define POSITION_HPP

#include <string>
#include <array>

struct Position {
    int x;
    int y;

    Position() : x(0), y(0) {}
    Position(int x, int y) : x(x), y(y) {}

    bool operator==(const Position& other) const { return x == other.x && y == other.y; }
    bool operator!=(const Position& other) const { return x != other.x || y != other.y; }

    // strict weak ordering, useful for non-hash-based maps
    bool operator<(const Position &other) const {
        if (y != other.y) {
            return y < other.y;
        }
        return x < other.x;
    }

    std::string to_string() const {
        return std::to_string(x) + ':' + std::to_string(y);
    }

    std::array<Position, 4> get_surrounding_positions() {
        return {{
            Position{x + 0, y - 1}, // north
            Position{x + 1, y + 0}, // east
            Position{x + 0, y + 1}, // south
            Position{x - 1, y + 0}  // west
        }};
    }
};

namespace std {
    template <>
    struct hash<Position> {
        std::size_t operator()(const Position& position) const {
            //return ((position.x+position.y) * (position.x+position.y+1) / 2) + position.y; // cantors pairing
            return (position.x >= position.y) ? position.x * position.x + position.x + position.y : position.x + position.y * position.y; // szudziks function
            //int max_val = max(position.x, position.y); // rosenberg-strong function
            //return max_val*max_val + max_val + position.x - position.y; // rosenberg-strong function
        }
    };
}

#endif