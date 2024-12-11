#ifndef AOC2024_CPP_UTILS_TIMER_H
#define AOC2024_CPP_UTILS_TIMER_H

#include <chrono>

class Timer {
public:
    Timer();
    ~Timer();

    void report();

private:
    std::chrono::high_resolution_clock::time_point m_start;
    bool m_reported;
};

#endif
