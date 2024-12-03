#include "utils/utils.h"
#include <algorithm>
#include <cctype>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

int64_t scan_line(std::string_view line) {
    size_t pos = 0;
    int64_t sum = 0;
    while (1) {
        pos = line.find("mul(", pos);
        if (pos == std::string::npos) {
            break;
        }

        pos += 4; // skip "mul("

        auto comma = std::find_if_not(line.begin() + pos, line.end(), [](auto i) { return std::isdigit(i); });
        if (comma == line.end() || *comma != ',') {
            pos = comma - line.begin();
            continue;
        }

        auto terminator = std::find_if_not(comma + 1, line.end(), [](auto i) { return std::isdigit(i); });
        if (terminator == line.end() || *terminator != ')') {
            pos = terminator - line.begin();
            continue;
        }

        auto operand1 = std::stol(std::string { line.begin() + pos, comma });
        auto operand2 = std::stol(std::string { comma + 1, terminator });
        
        sum += operand1 * operand2;
        pos = terminator - line.begin() + 1;
    }

    return sum;
}

#ifdef AOC_PART2
int64_t scan_line_limited(std::string_view line) {
    size_t pos = 0;
    int64_t sum = 0;

    while (1) {
        size_t dont_idx = line.find("don't()", pos);
        sum += scan_line(line.substr(pos, dont_idx - pos));

        size_t do_idx = line.find("do()", dont_idx);
        if (do_idx == std::string::npos) {
            return sum;
        }

        pos = do_idx;
    }

    return sum + scan_line(line.substr(pos));
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
    std::cout << "Answer: " << scan_line(data.str()) << std::endl;
#else
    std::cout << "Answer: " << scan_line_limited(data.str()) << std::endl;
#endif
}
