#include "utils/utils.h"
#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>
#include <unordered_map>

int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
    std::string line;

#ifndef AOC_PART2
    std::vector<int> left_list;
    std::vector<int> right_list;
    while (std::getline(input, line)) {
        int left, right;

        std::istringstream ss(line);
        if (!(ss >> left >> right)) {
            throw std::runtime_error("Unable to read from input");
        }

        left_list.push_back(left);
        right_list.push_back(right);
    }

    std::sort(left_list.begin(), left_list.end());
    std::sort(right_list.begin(), right_list.end());

    long difference = 0;
    for (std::vector<int>::size_type i = 0; i < left_list.size(); i++) {
        difference += std::abs(left_list[i] - right_list[i]);
    }

    std::cout << "Answer: " << difference << std::endl;
#else
    std::vector<int> left_list;
    std::unordered_map<int, int> right_list;

    while (std::getline(input, line)) {
        int left, right;

        std::istringstream ss(line);
        if (!(ss >> left >> right)) {
            throw std::runtime_error("Unable to read from input");
        }

        left_list.push_back(left);
        right_list[right] += 1;
    }

    long similarity_score = 0;
    for (const auto l : left_list) {
        similarity_score += l * right_list[l];
    }

    std::cout << "Answer: " << similarity_score << std::endl;
#endif
}
