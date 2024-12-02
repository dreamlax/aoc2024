lib_LIBRARIES = libutils.a
libutils_a_SOURCES = \
    utils/timer.cpp

bin_PROGRAMS = \
    day1-part1 \
    day1-part2

day1_part1_LDADD = libutils.a
day1_part1_SOURCES = \
    day1/main.cpp

day1_part2_LDADD = libutils.a
day1_part2_SOURCES = \
    day1/main.cpp
day1_part2_CPPFLAGS = -DAOC_PART2
