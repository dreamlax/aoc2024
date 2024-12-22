#include "utils/utils.h"
#include <array>
#include <fstream>
#include <iostream>
#include <string>
#include <sstream>
#include <string_view>
#include <tuple>
#include <vector>

const uint16_t kEmpty = 0;

const uint16_t kLeft = 0b00001;
const uint16_t kUp = 0b0010;
const uint16_t kRight = 0b0100;
const uint16_t kDown = 0b1000;

struct Cell {
	uint16_t id = kEmpty;
	uint16_t neighbours = 0;
};

struct Farm {
	Farm(std::string_view farm)
	: m_farm(farm)
	{
		m_width = farm.find("\n");
		if (m_width == std::string::npos) {
			m_width = farm.length();
		}
		else {
			// include '\n' character
			m_width++;
		}
	}

	void find_plot(size_t pos, uint16_t id, std::vector<Cell> &analysis) const {
		char plot_name = m_farm[pos];
		if (plot_name == '\n') {
			return;
		}

		analysis[pos].id = id;

		std::tuple<size_t, uint16_t> next_positions[] {
			{ pos - 1, kLeft },
			{ pos + 1, kRight },
			{ pos - m_width, kUp },
			{ pos + m_width, kDown }
		};

		for (const auto next : next_positions) {
			size_t next_pos;
			uint16_t neighbour_flag;
			std::tie(next_pos, neighbour_flag) = next;
			if (next_pos > m_farm.length())
				continue;

			if (m_farm[next_pos] == plot_name) {
				analysis[pos].neighbours |= neighbour_flag;
				if (analysis[next_pos].id == kEmpty) {
					find_plot(next_pos, id, analysis);
				}
			}
		}
	}

	std::tuple<std::vector<Cell>, uint16_t> analyse() const {
		std::vector<Cell> analysis(m_farm.length());
		uint16_t current_id = 1;

		for (size_t pos = 0; pos < m_farm.length(); pos++) {
			if (analysis[pos].id == kEmpty) {
				find_plot(pos, current_id, analysis);
				current_id++;
			}
		}

		return std::make_tuple(analysis, current_id);
	}

#ifndef AOC_PART2
	size_t count_areas_and_fences(const std::vector<Cell> &analysis, uint16_t max_id) const {
		static const std::array<uint16_t, 16> pop_count_map = {
			0, 1, 1, 2,
			1, 2, 2, 3,
			1, 2, 2, 3,
			2, 3, 3, 4
		};
		std::vector<size_t> area_tally(max_id);
		std::vector<size_t> fence_tally(max_id);

		for (const auto &cell : analysis) {
			area_tally[cell.id]++;
			fence_tally[cell.id] += (4 - pop_count_map[cell.neighbours]);
		}

		size_t result = 0;
		// skip ID 0 (empty cells)
		for (size_t i = 1; i < max_id; i++) {
			result += area_tally[i] * fence_tally[i];
		}

		return result;
	}
#else
	size_t count_areas_and_fence_runs(const std::vector<Cell> &analysis, uint16_t max_id) const {
		std::vector<size_t> area_tally(max_id);
		std::vector<size_t> fence_tally(max_id);

		for (size_t x = 0; x < analysis.size() - 1; x++) {
			area_tally[analysis[x].id]++;

			bool current_has_up_fence = !(analysis[x].neighbours & kUp);
			bool next_has_up_fence = !(analysis[x+1].neighbours & kUp);
			if (current_has_up_fence && (!next_has_up_fence || analysis[x].id != analysis[x+1].id)) {
				fence_tally[analysis[x].id]++;
			}

			bool current_has_down_fence = !(analysis[x].neighbours & kDown);
			bool next_has_down_fence = !(analysis[x+1].neighbours & kDown);
			if (current_has_down_fence && (!next_has_down_fence || analysis[x].id != analysis[x+1].id)) {
				fence_tally[analysis[x].id]++;
			}
		}

		for (size_t x = 0; x < m_width; x++) {
			for (size_t y = x; y < analysis.size(); y += m_width) {
				bool next_in_bounds = y + m_width < analysis.size();

				bool current_has_left_fence = !(analysis[y].neighbours & kLeft);
				bool next_has_left_fence = next_in_bounds ? !(analysis[y + m_width].neighbours & kLeft) : false;
				if (current_has_left_fence && (!next_has_left_fence || (next_in_bounds && analysis[y].id != analysis[y + m_width].id))) {
					fence_tally[analysis[y].id]++;
				}

				bool current_has_right_fence = !(analysis[y].neighbours & kRight);
				bool next_has_right_fence = next_in_bounds ? !(analysis[y + m_width].neighbours & kRight) : false;
				if (current_has_right_fence && (!next_has_right_fence || (next_in_bounds && analysis[y].id != analysis[y + m_width].id))) {
					fence_tally[analysis[y].id]++;
				}
			}
		}

		size_t result = 0;
		// skip ID 0 (empty cells)
		for (size_t i = 1; i < max_id; i++) {
			result += area_tally[i] * fence_tally[i];
		}

		return result;
	}
#endif

	std::string_view m_farm;
	size_t m_width;
};


int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
	std::stringstream farm_buffer;
	farm_buffer << input.rdbuf();

	const std::string &farm_str = farm_buffer.str();
	const Farm farm(farm_str);

	std::vector<Cell> analysis;
	uint16_t max_id;
	std::tie(analysis, max_id) = farm.analyse();

#ifndef AOC_PART2
	size_t answer = farm.count_areas_and_fences(analysis, max_id);
#else
	size_t answer = farm.count_areas_and_fence_runs(analysis, max_id);
#endif

	std::cout << "Answer: " << answer << std::endl;
}
