#ifndef AOC2024_CPP_UTILS_TIMER_H
#define AOC2024_CPP_UTILS_TIMER_H

#include <chrono>

class Timer {
public:
    Timer();
    ~Timer();

private:
    std::chrono::steady_clock::time_point m_start;
};

#endif
