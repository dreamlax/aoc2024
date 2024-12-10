#include "utils/utils.h"
#include <cassert>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <tuple>
#include <vector>

enum class Direction : uint8_t {
    Up = 0x01,
    Right = 0x02,
    Down = 0x04,
    Left = 0x08
};

const uint8_t VISITED = 0x80;

Direction rotate(Direction d) {
    switch (d) {
        case Direction::Up:
            return Direction::Right;
        case Direction::Right:
            return Direction::Down;
        case Direction::Down:
            return Direction::Left;
        case Direction::Left:
            return Direction::Up;
    }

    assert(false);
}

std::optional<std::tuple<size_t, Direction>> get_next_position(std::string_view board, size_t board_width, size_t current, Direction cur_direction) {
    while (1) {
        size_t next_attempt;
        switch (cur_direction) {
            case Direction::Up:
                if (current < board_width) {
                    return std::optional<std::tuple<size_t, Direction>>();
                }
                next_attempt = current - board_width;
                break;
            
            case Direction::Right:
                next_attempt = current + 1;
                break;

            case Direction::Down:
                next_attempt = current + board_width;
                break;
            
            case Direction::Left:
                if (current == 0) {
                    return std::optional<std::tuple<size_t, Direction>>();
                }
                next_attempt = current - 1;
                break;
        }

        if (next_attempt >= board.length() || board[next_attempt] == '\n') {
            return std::optional<std::tuple<size_t, Direction>>();
        }

        switch (board[next_attempt]) {
            case '.':
            case 'X':
                return std::make_tuple(next_attempt, cur_direction);
            
            case '#':
                cur_direction = rotate(cur_direction);
                break;
            
            default:
                assert(false);
        }
    }
}

size_t trace_guard(std::string board) {
    size_t board_width = 1 + board.find('\n');
    size_t current = board.find('^');
    size_t steps = 0;
    Direction cur_direction = Direction::Up;

    board[current] = '.';

    while (1) {
        if (board[current] == '.') {
            board[current] = 'X';
            steps++;
        }

        auto result = get_next_position(board, board_width, current, cur_direction);
        if (!result) {
            break;
        }

        std::tie(current, cur_direction) = result.value();
    }

    return steps;
}

#ifdef AOC_PART2
size_t count_loops(std::string board) {
    size_t board_width = 1 + board.find('\n');
    size_t starting_position = board.find('^');
    size_t loops = 0;
    Direction starting_direction = Direction::Up;

    board[starting_position] = 'X';

    while (1) {
        auto barrier = get_next_position(board, board_width, starting_position, starting_direction);
        if (!barrier)
            break;
        
        size_t barrier_position;
        Direction next_direction;
        std::vector<uint8_t> path(board.length());
        std::tie(barrier_position, next_direction) = barrier.value();

        if (board[barrier_position] == 'X') {
            // can't put a barrier in a location we've already been in
            starting_position = barrier_position;
            starting_direction = next_direction;
            continue;
        }

        board[barrier_position] = '#';

        size_t current = starting_position;
        Direction cur_direction = starting_direction;

        while (1) {
            if ((path[current] & VISITED) && (path[current] & static_cast<uint8_t>(cur_direction))) {
                loops++;
                break;
            }

            path[current] |= VISITED;
            path[current] |= static_cast<uint8_t>(cur_direction);

            auto next = get_next_position(board, board_width, current, cur_direction);
            if (!next) {
                break;
            }

            std::tie(current, cur_direction) = next.value();
        }

        board[barrier_position] = 'X';
        starting_position = barrier_position;
        starting_direction = next_direction;
    }

    return loops;
}
#endif

int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
    std::stringstream data;
    data << input.rdbuf();

#ifndef AOC_PART2
    std::cout << "Answer: " << trace_guard(data.str()) << std::endl;
#else
    std::cout << "Answer: " << count_loops(data.str()) << std::endl;
#endif
}
    