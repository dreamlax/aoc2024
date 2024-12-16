#include "utils/utils.h"
#include <cstdint>
#include <fstream>
#include <iostream>
#include <limits>
#include <sstream>
#include <string>
#include <string_view>
#include <unordered_set>
#include <vector>

#ifdef AOC_PART2
size_t count_all_routes(std::string_view mountain, size_t width, size_t current) {
    char current_value = mountain[current];
    if (current_value == '9') {
        return 1;
    }

    size_t routes = 0;

    // left
    if (current > 0 && mountain[current - 1] == current_value + 1) {
        routes += count_all_routes(mountain, width, current - 1);
    }

    // up
    if (current >= width && mountain[current - width] == current_value + 1) {
        routes += count_all_routes(mountain, width, current - width);
    }

    // right
    if (current < mountain.size() && mountain[current + 1] == current_value + 1) {
        routes += count_all_routes(mountain, width, current + 1);
    }

    // down
    if (current + width < mountain.size() && mountain[current + width] == current_value + 1) {
        routes += count_all_routes(mountain, width, current + width);
    }

    return routes;
}
#else
void find_summits(std::string_view mountain, size_t width, size_t current, std::unordered_set<size_t> &summits) {
    char current_value = mountain[current];
    if (current_value == '9') {
        summits.insert(current);
        return;
    }

    // left
    if (current > 0 && mountain[current - 1] == current_value + 1) {
        find_summits(mountain, width, current - 1, summits);
    }

    // up
    if (current >= width && mountain[current - width] == current_value + 1) {
        find_summits(mountain, width, current - width, summits);
    }

    // right
    if (current < mountain.size() && mountain[current + 1] == current_value + 1) {
        find_summits(mountain, width, current + 1, summits);
    }

    // down
    if (current + width < mountain.size() && mountain[current + width] == current_value + 1) {
        find_summits(mountain, width, current + width, summits);
    }
}
#endif

int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
    std::stringstream buffer;
    buffer << input.rdbuf();

    const std::string &mountain = buffer.str();
    size_t width = mountain.find('\n');
    if (width == std::string::npos) {
        width = mountain.size();
    }
    else {
        width += 1; // include the '\n'
    }

    size_t answer = 0;
    for (size_t i = 0; i < mountain.size(); i++) {
        if (mountain[i] == '0') {
#ifdef AOC_PART2
            answer += count_all_routes(mountain, width, i);
#else
            std::unordered_set<size_t> summits;
            find_summits(mountain, width, i, summits);
            answer += summits.size();
#endif
        }
    }

    std::cout << "Answer: " << answer << std::endl;
}
