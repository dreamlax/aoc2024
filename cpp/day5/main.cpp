#include "utils/utils.h"
#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_set>
#include <vector>

template<typename T>
struct std::hash<std::pair<T, T>>
{
    size_t operator()(const std::pair<T, T> &p) const noexcept {
        return std::hash<T>{}(p.first) ^ (std::hash<T>{}(p.second) << 1);
    }
};

int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
    std::string line;
    bool rules_done = false;;

    std::unordered_set<std::pair<int, int>> rules;
    std::vector<std::vector<int>> updates;

    while (std::getline(input, line)) {
        if (line.empty()) {
            rules_done = true;
        }
        else if (!rules_done) {
            size_t end;
            int left = std::stoi(line, &end);
            int right = std::stoi(line.substr(end + 1));
            rules.emplace(std::make_pair(left, right));
        }
        else {
            std::vector<int> update;
            size_t last = 0;
            size_t comma = line.find(',');
            while (comma != std::string::npos) {
                update.push_back(std::stoi(line.substr(last, comma - last)));
                last = comma + 1;
                comma = line.find(',', last);
            }
            update.push_back(std::stoi(line.substr(last)));
            updates.emplace_back(update);
        }
    }

    auto comparator = [&rules](int l, int r) {
        return rules.find(std::make_pair(l, r)) != rules.end();
    };

    int sum = 0;
    for (auto update : updates) {
#ifndef AOC_PART2
        if (std::is_sorted(update.cbegin(), update.cend(), comparator)) {
            sum += update[update.size()/2];
        }
#else
        if (!std::is_sorted(update.cbegin(), update.cend(), comparator)) {
            std::sort(update.begin(), update.end(), comparator);
            sum += update[update.size()/2];
        }
#endif
    }

    std::cout << sum << std::endl;
}
