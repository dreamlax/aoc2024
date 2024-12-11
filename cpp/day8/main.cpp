#include "utils/utils.h"
#include <algorithm>
#include <cctype>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <span>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using CharMap = std::unordered_map<char, std::vector<size_t>>;

CharMap get_antennae(std::string_view board) {
    CharMap result;

    for (size_t i = 0; i < board.length(); i++) {
        if (std::isalnum(board[i])) {
            result[board[i]].push_back(i);
        }
    }

    return result;
}

#ifndef AOC_PART2
size_t count_antinodes(std::string_view board, const CharMap &antennae) {
    size_t board_width = board.find('\n');

    if (board_width == std::string::npos) {
        board_width = board.length();
    }

    std::unordered_set<size_t> antinode_positions;

    for (const auto &kv : antennae) {
        const auto &points = kv.second;
        for (size_t i = 0; i < points.size() - 1; i++) {
            for (size_t j = i + 1; j < points.size(); j++) {
                size_t diff = points[j] - points[i];
                ssize_t x1 = points[i] % (board_width + 1);
                ssize_t x2 = points[j] % (board_width + 1);
                ssize_t dx = x2 - x1;

                if (points[i] >= diff && x1 - dx >= 0 && x1 - dx < board_width) {
                    antinode_positions.insert(points[i] - diff);
                }

                if (points[j] + diff < board.length() && x2 + dx >= 0 && x2 + dx < board_width) {
                    antinode_positions.insert(points[j] + diff);
                }
            }
        }
    }

    return antinode_positions.size();
}
#else
size_t count_antinodes(std::string_view board, const CharMap &antennae) {
    size_t board_width = board.find('\n');

    if (board_width == std::string::npos) {
        board_width = board.length();
    }

    std::unordered_set<size_t> antinode_positions;

    for (const auto &kv : antennae) {
        const auto &points = kv.second;
        for (size_t i = 0; i < points.size() - 1; i++) {
            for (size_t j = i + 1; j < points.size(); j++) {
                antinode_positions.insert(points[i]);
                antinode_positions.insert(points[j]);

                size_t diff = points[j] - points[i];
                ssize_t x1 = points[i] % (board_width + 1);
                ssize_t x2 = points[j] % (board_width + 1);
                ssize_t dx = x2 - x1;

                ssize_t b = x1 - dx;
                size_t x = points[i];
                while (b >= 0 && b < static_cast<ssize_t>(board_width)) {
                    if (x < diff)
                        break;
                    b -= dx;
                    x -= diff;
                    antinode_positions.insert(x);
                }

                b = x2 + dx;
                x = points[j];
                while (b >= 0 && b < static_cast<ssize_t>(board_width))
                {
                    if (x + diff > board.length())
                        break;
                    b += dx;
                    x += diff;
                    antinode_positions.insert(x);
                }
            }
        }
    }

    return antinode_positions.size();
}
#endif

int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    int64_t grand_total = 0;
    std::ifstream input(argv[1]);
    std::stringstream buffer;
    buffer << input.rdbuf();

    auto antennae = get_antennae(buffer.str());
    auto answer = count_antinodes(buffer.str(), antennae);
    std::cout << "Answer: " << answer << std::endl;
}
