#include "utils/utils.h"
#include <algorithm>
#include <cassert>
#include <cstdio>
#include <format>
#include <fstream>
#include <iostream>
#include <numeric>
#include <span>

const size_t kBoardWidth = 101;
const size_t kBoardHeight = 103;

struct Robot {
    uint64_t position[2] {};
    int64_t vector[2] {};

    std::ostream &operator<<(std::ostream &out) const {
        return out << std::format("[{}, {}] -- [{}, {}]", position[0], position[1], vector[0], vector[1]) << std::endl;
    }

    void move(size_t seconds) {
        position[0] += seconds * kBoardWidth + (static_cast<int64_t>(seconds) * vector[0]);
        position[1] += seconds * kBoardHeight + (static_cast<int64_t>(seconds) * vector[1]);
        position[0] %= kBoardWidth;
        position[1] %= kBoardHeight;
    }

    size_t quadrant() const {
        static const size_t kHalfWidth = kBoardWidth / 2;
        static const size_t kHalfHeight = kBoardHeight / 2;

        if (position[0] == kHalfWidth || position[1] == kHalfHeight) {
            return 4;
        }

        size_t qx = position[0] / (kHalfWidth + 1);
        size_t qy = position[1] / (kHalfHeight + 1);

        return qy * 2 + qx;
    }
};

size_t safety_factor(const std::span<Robot> robots) {
    size_t quadrants[5] {};
    for (const auto r : robots) {
        quadrants[r.quadrant()]++;
    }

    return quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
}

std::istream &operator>>(std::istream &in, Robot &r) {
    std::string line;
    if (!std::getline(in, line) || line.empty()) {
        return in;
    }

    if (sscanf(line.c_str(), "p=%llu,%llu v=%lld,%lld", &r.position[0], &r.position[1], &r.vector[0], &r.vector[1]) != 4) {
        in.setf(std::ios_base::failbit);
        return in;
    }

    return in;
}

int main(int argc, char *argv[]) {
    Timer t;

	if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
    std::vector<Robot> robots;
    std::istream_iterator<Robot> start(input);
    std::istream_iterator<Robot> end;
    std::copy(start, end, std::back_inserter(robots));

#ifdef AOC_PART2
    size_t min_safety = std::numeric_limits<size_t>::max();
    size_t min_seconds = 0;
    for (size_t i = 1; i <= kBoardWidth * kBoardHeight; i++) {
        for (auto &r : robots) {
            r.move(1);
        }

        size_t sf = safety_factor(robots);
        if (sf < min_safety) {
            min_safety = sf;
            min_seconds = i;
        }
    }

    assert(min_safety != std::numeric_limits<size_t>::max());
    size_t answer = min_seconds;
#else
    for (auto &r : robots) {
        r.move(100);
    }

    size_t answer = safety_factor(robots);
#endif

    std::cout << "Answer: " << answer << std::endl;
}
