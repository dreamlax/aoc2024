#include "utils/utils.h"
#include <array>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <iterator>
#include <sstream>
#include <string_view>
#include <tuple>
#include <unordered_map>
#include <vector>

using StoneValue = uint64_t;

struct CacheEntry {
    StoneValue stone;
    size_t blinks;

    CacheEntry(StoneValue stone, size_t blinks)
    : stone(stone)
    , blinks(blinks)
    {
    }

    bool operator==(const CacheEntry &other) const {
        return stone == other.stone && blinks == other.blinks;
    }
};

template<>
struct std::hash<CacheEntry>
{
    size_t operator()(const CacheEntry &entry) const noexcept {
        return entry.blinks + (entry.stone << 5);
    }
};

using Cache = std::unordered_map<CacheEntry, size_t>;

std::tuple<size_t, std::array<StoneValue, 2>> blink(StoneValue stone) {
    if (stone == 0) {
        return std::make_tuple(1, std::array<StoneValue, 2> { 1 });
    }

    size_t digits = std::log10(stone) + 1;
    if (digits % 2 == 0) {
        StoneValue divisor = std::pow(static_cast<StoneValue>(10), digits / 2);
        return std::make_tuple(2, std::array<StoneValue, 2> {
            stone / divisor,
            stone % divisor
        });
    }

    return std::make_tuple(1, std::array<StoneValue, 2> { stone * 2024 });
}

size_t count_stones_for_blinks(StoneValue stone, size_t blinks, Cache &cache) {
    auto entry = cache.find(CacheEntry(stone, blinks));
    if (entry != cache.end()) {
        return entry->second;
    }

    size_t count;
    std::array<StoneValue, 2> new_stones;
    std::tie(count, new_stones) = blink(stone);

    if (blinks == 1) {
        cache.insert(std::make_pair(CacheEntry(stone, blinks), count));
        return count;
    }

    size_t result = 0;
    for (size_t i = 0; i < count; i++) {
        result += count_stones_for_blinks(new_stones[i], blinks - 1, cache);
    }

    cache.insert(std::make_pair(CacheEntry(stone, blinks), result));
    return result;
}

int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);

    std::vector<uint64_t> stones;
    std::istream_iterator<uint64_t> start(input);
    std::istream_iterator<uint64_t> end;
    std::copy(start, end, std::back_inserter(stones));

#ifdef AOC_PART2
    size_t blinks = 75;
#else
    size_t blinks = 25;
#endif

    Cache cache;
    size_t answer = 0;
    for (size_t i = 0; i < stones.size(); i++) {
        answer += count_stones_for_blinks(stones[i], blinks, cache);
    }

    std::cout << "Answer: " << answer << std::endl;
}
