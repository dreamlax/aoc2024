#include <iostream>
#include <stdexcept>
#include "timer.h"

Timer::Timer()
: m_start(std::chrono::steady_clock::now())
{
}

Timer::~Timer() {
    std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
    std::chrono::nanoseconds elapsed = end - m_start;
    std::cerr
        << "Time elapsed: "
        << std::chrono::duration_cast<std::chrono::microseconds>(elapsed).count()
        << "µs"
        << std::endl;
}
