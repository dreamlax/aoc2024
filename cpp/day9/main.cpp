#include "utils/utils.h"
#include <cctype>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <limits>
#include <map>
#include <optional>
#include <set>
#include <sstream>
#include <string>
#include <vector>

const uint16_t EMPTY = std::numeric_limits<uint16_t>::max();

std::vector<uint16_t> expand_disk(std::string_view compacted) {
    std::vector<uint16_t> disk;
    for (size_t i = 0; i < compacted.length(); i++) {
        if (!isdigit(compacted[i])) {
            continue;
        }

        size_t len = compacted[i] - '0';
        if (i % 2 == 0) {
            disk.insert(disk.end(), len, i / 2);
        }
        else {
            disk.insert(disk.end(), len, EMPTY);
        }
    }

    return disk;
}

#ifndef AOC_PART2
void frag(std::vector<uint16_t> &disk) {
    size_t i = 0;
    size_t j = disk.size() - 1;

    while (true) {
        while (i < disk.size() && disk[i] != EMPTY) {
            i++;
        }

        while (i < j && disk[j] == EMPTY) {
            j--;
        }

        std::swap(disk[i], disk[j]);
        i++;

        if (j == 0 || i >= j) {
            break;
        }

        j--;
    }
}
#else
std::tuple<std::vector<size_t>, std::map<size_t,std::set<size_t>>> get_disk_map(std::string_view compacted) {
    std::vector<size_t> file_lengths;
    std::map<size_t, std::set<size_t>> free_space;
    size_t acc = 0;

    for (size_t i = 0; i < compacted.length(); i++) {
        if (!isdigit(compacted[i])) {
            continue;
        }

        size_t length = static_cast<size_t>(compacted[i] - '0');
        if (i % 2 == 0) {
            file_lengths.push_back(length);
        }
        else {
            free_space[length].insert(acc);
        }
        acc += length;
    }

    return std::make_tuple(file_lengths, free_space);
}

void defrag(std::vector<uint16_t> &disk, const std::vector<size_t> &file_lengths, std::map<size_t, std::set<size_t>> &free_space) {
    size_t j = disk.size() - 1;

    while (true) {
        while (disk[j] == EMPTY) {
            if (j == 0)
                return;
            j--;
        }

        size_t file_id = disk[j];
        size_t file_length = file_lengths[file_id];

        size_t space_index = j;
        size_t space_amount = 0;
        auto space = free_space.lower_bound(file_length);
        while (space != free_space.end()) {
            auto first_available = space->second.begin();
            if (first_available != space->second.end() && *first_available < space_index) {
                space_index = *first_available;
                space_amount = space->first;
            }
            space++;
        }

        if (space_index == j) {
            // couldn't find a suitable spot
            if (j <= file_length)
                return;
            j -= file_length;
            continue;
        }

        for (size_t i = space_index; i < space_index + file_length; i++, j--) {
            std::swap(disk[i], disk[j]);
        }

        // bookkeeping
        free_space[space_amount].erase(space_index);
        if (file_length < space_amount) {
            free_space[space_amount - file_length].insert(space_index + file_length);
        }
    }
}
#endif

uint64_t checksum(const std::vector<uint16_t> &disk) {
    uint64_t checksum = 0;
    size_t i = 0;

    while (i < disk.size()) {
        if (disk[i] != EMPTY) {
            checksum += static_cast<uint64_t>(disk[i]) * i;
        }
        i++;
    }

    return checksum;
}

int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
    std::stringstream buffer;
    buffer << input.rdbuf();  

    auto disk = expand_disk(buffer.str());
#ifndef AOC_PART2
    frag(disk);
#else
    std::vector<size_t> file_lengths;
    std::map<size_t, std::set<size_t>> free_space;
    std::tie(file_lengths, free_space) = get_disk_map(buffer.str());
    defrag(disk, file_lengths, free_space);
#endif
    auto answer = checksum(disk);

    std::cout << "Answer: " << answer << std::endl;
}
