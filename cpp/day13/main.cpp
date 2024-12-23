#include "utils/utils.h"
#include <algorithm>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <optional>
#include <tuple>
#include <vector>

struct ClawMachine {
	int64_t m[2][2];
	int64_t c[2];

	std::optional<std::tuple<int64_t, int64_t>> get_integer_intersection() const {
		int64_t determinant = m[0][0] * m[1][1] - m[0][1] * m[1][0];
		int64_t inverse[2][2] = {
			{ m[1][1], -m[0][1] },
			{ -m[1][0], m[0][0] }
		};

		int64_t r[2] = {
			inverse[0][0] * c[0] + inverse[0][1] * c[1],
			inverse[1][0] * c[0] + inverse[1][1] * c[1]
		};

		int64_t x = r[0] / determinant;
		int64_t x_rem = r[0] % determinant;

		int64_t y = r[1] / determinant;
		int64_t y_rem = r[1] % determinant;

		if (x < 0 || x_rem != 0 || y < 0 || y_rem != 0) {
			return std::optional<std::tuple<int64_t, int64_t>>();
		}
		else {
			return std::make_tuple(x, y);
		}
	}

	size_t get_tokens() const {
		int64_t a_presses;
		int64_t b_presses;
		std::tie(a_presses, b_presses) = get_integer_intersection().value_or(std::make_tuple(0, 0));
		return a_presses * 3 + b_presses;
	}
};

std::ostream &operator<<(std::ostream &out, const ClawMachine &c) {
	out << c.m[0][0] << "x + " << c.m[0][1] << "y = " << c.c[0];
	out << "; ";
	out << c.m[1][0] << "x + " << c.m[1][1] << "y = " << c.c[1];
	out << "\n";
	return out;
}

/*
	Button A: X+94, Y+34
	Button B: X+22, Y+67
	Prize: X=8400, Y=5400
*/
std::istream &operator>>(std::istream &in, ClawMachine &c) {
	std::string line;
	while (std::getline(in, line) && line.empty());
	if (!in) {
		return in;
	}

	if (sscanf(line.c_str(), "Button A: X+%lld, Y+%lld", &c.m[0][0], &c.m[1][0]) != 2) {
		in.setf(std::ios_base::failbit);
		return in;
	}

	if (!std::getline(in, line)) {
		return in;
	}

	if (sscanf(line.c_str(), "Button B: X+%lld, Y+%lld", &c.m[0][1], &c.m[1][1]) != 2) {
		in.setf(std::ios_base::failbit);
		return in;
	}

	if (!std::getline(in, line)) {
		return in;
	}

	if (sscanf(line.c_str(), "Prize: X=%lld, Y=%lld", &c.c[0], &c.c[1]) != 2) {
		in.setf(std::ios_base::failbit);
		return in;
	}

#ifdef AOC_PART2
	c.c[0] += 10'000'000'000'000;
	c.c[1] += 10'000'000'000'000;
#endif

	return in;
}

int main(int argc, char * argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
	std::vector<ClawMachine> claw_machines;
	std::istream_iterator<ClawMachine> start(input);
	std::istream_iterator<ClawMachine> end;
	std::copy(start, end, std::back_inserter(claw_machines));

	size_t answer = 0;
	for (const auto &cm : claw_machines) {
		answer += cm.get_tokens();
	}

	std::cout << "Answer: " << answer << std::endl;
}
