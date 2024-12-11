#include <iostream>
#include <stdexcept>
#include "timer.h"

Timer::Timer()
: m_start(std::chrono::high_resolution_clock::now())
, m_reported(false)
{
}

Timer::~Timer() {
    std::chrono::high_resolution_clock::time_point end = std::chrono::high_resolution_clock::now();
    std::chrono::nanoseconds elapsed = end - m_start;
    std::cerr
        << "Time elapsed: "
        << std::chrono::duration_cast<std::chrono::microseconds>(elapsed).count()
        << "µs"
        << std::endl;
    m_reported = true;
}
