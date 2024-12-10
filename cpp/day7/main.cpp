#include "utils/utils.h"
#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <iterator>
#include <sstream>
#include <span>
#include <tuple>
#include <vector>

#ifndef AOC_PART2
bool can_add_or_mul_to_total(int64_t total, std::span<int64_t> stack) {
    if (stack.size() == 0) {
        return total == 0;
    }

    int64_t last = stack.back();
    return 
        ((total >= last) && can_add_or_mul_to_total(total - last, stack.subspan(0, stack.size() - 1))) ||
        ((total % last == 0) && can_add_or_mul_to_total(total / last, stack.subspan(0, stack.size() - 1)));
}
#else
bool can_add_or_mul_to_total(int64_t total, std::span<int64_t> stack) {
    if (stack.size() == 0) {
        return total == 0;
    }

    int64_t last = stack.back();
    int64_t next_total = total - last;
    int64_t digits = std::pow(10, std::log10(next_total) + 1);

    return
        ((total >= last) && can_add_or_mul_to_total(next_total, stack.subspan(0, stack.size() - 1))) ||
        ((total % last == 0) && can_add_or_mul_to_total(total / last, stack.subspan(0, stack.size() - 1))) ||
        ((next_total % digits == 0) && can_add_or_mul_to_total(next_total / digits, stack.subspan(0, stack.size() - 1)));
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
    std::string line;

    while (std::getline(input, line)) {
        size_t colon = line.find(':');
        assert(colon != std::string::npos);

        int64_t total = std::stoll(line.substr(0, colon));
        std::istringstream operand_text(line.substr(colon + 1));
        std::istream_iterator<int64_t> start(operand_text);
        std::istream_iterator<int64_t> end;
        std::vector<int64_t> operands(start, end);
        
        if (can_add_or_mul_to_total(total, operands)) {
            grand_total += total;
        }
    }

    std::cout << "Answer: " << grand_total << std::endl;
}
