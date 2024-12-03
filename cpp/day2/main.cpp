#include "utils/utils.h"
#include <algorithm>
#include <bitset>
#include <cassert>
#include <fstream>
#include <functional>
#include <iostream>
#include <iterator>
#include <optional>
#include <span>
#include <sstream>
#include <vector>

bool is_up(int diff) {
    return diff < 0 && diff >= -3;
}

bool is_down(int diff) {
    return diff > 0 && diff <= 3;
}

bool is_safe_part_1(const std::vector<int> &report) {
    assert(report.size() > 0);
    std::vector<int> differences(report.size() - 1);
    for (size_t i = 0; i < differences.size(); i++) {
        differences[i] = report[i] - report[i + 1];
    }

    return std::all_of(differences.begin(), differences.end(), is_up) || std::all_of(differences.begin(), differences.end(), is_down);
}

#ifdef AOC_PART2
bool scan_report(const std::vector<int> &report, std::function<bool(int)> checker) {
    size_t scan[3] = { 0, 1, 2 };
    std::optional<size_t> outlier;

    while (1) {
        if (scan[1] == report.size()) {
            return true;
        }
        else if (scan[2] == report.size()) {
            return checker(report[scan[0]] - report[scan[1]]) || !outlier;
        }

        bool scan_results[3] = {
            checker(report[scan[0]] - report[scan[1]]),
            checker(report[scan[1]] - report[scan[2]]),
            checker(report[scan[0]] - report[scan[2]])
        };

        if (scan_results[0] && scan_results[1]) {
            // no outliers
            scan[0] = scan[2];
            scan[1] = scan[0] + 1;
            scan[2] = scan[0] + 2;
        }
        else if (!scan_results[0] && scan_results[1] && scan_results[2]) {
            // first number is outlier
            if (outlier) {
                return false;
            }
            outlier = scan[0];
            scan[0]++;
            scan[1]++;
            scan[2]++;
        }
        else if (!scan_results[1] && scan_results[2]) {
            // second number is outlier
            if (outlier) {
                return false;
            }
            outlier = scan[1];
            scan[1]++;
            scan[2]++;
        }
        else if (scan_results[0] && !scan_results[1] && !scan_results[2]) {
            // thid number is outlier
            if (outlier) {
                return false;
            }
            outlier = scan[2];
            scan[2]++;
        }
        else if (!scan_results[0] && !scan_results[2]) {
            // impossible to be safe, two outliers
            return false;
        }
    }

    assert(false);
}

bool is_safe_part_2(const std::vector<int> &report) {
    // filter out outlier cases on the start or end
    if (is_safe_part_1(std::vector<int> { report.begin(), report.end() - 1 }) || is_safe_part_1(std::vector<int> { report.begin() + 1, report.end() })) {
        return true;
    }

    std::function<bool(int)> checkers[] = { is_up, is_down };

    for (auto checker : checkers) {
        if (scan_report(report, checker)) {
            return true;
        }
    }

    return false;
}
#endif

int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
    std::string line;

    std::vector<std::vector<int>> reports;
    
    while (std::getline(input, line)) {
        std::istringstream report(line);
        std::istream_iterator<int> start(report);
        std::istream_iterator<int> end;
        reports.emplace_back(std::vector<int> { start, end });
    }

#ifndef AOC_PART2
    size_t safe = std::count_if(reports.begin(), reports.end(), is_safe_part_1);
    std::cout << "Answer: " << safe << std::endl;
#else
    size_t safe = std::count_if(reports.begin(), reports.end(), is_safe_part_2);
    std::cout << "Answer: " << safe << std::endl;
#endif
}
