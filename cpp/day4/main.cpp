#include "utils/utils.h"
#include <cassert>
#include <fstream>
#include <iostream>
#include <sstream>

#ifndef AOC_PART2
bool is_mas(char m, char a, char s) {
    return m == 'M' && a == 'A' && s == 'S';
}

size_t count_xmas(std::string_view board) {
    size_t board_width = board.find('\n') + 1;
    size_t board_height = board.length() / board_width;
    size_t count = 0;
    size_t pos = board.find('X');

    assert(board_width != std::string::npos);
    assert(board.length() % board_width == 0);

    while (pos != std::string::npos) {
        size_t x = pos % board_width;
        size_t y = pos / board_width;

        bool check_left = x > 2;
        bool check_right = x < board_width - 3;
        bool check_up = y > 2;
        bool check_down = y < board_height - 3;

        if (check_left && board.substr(pos - 3, 3) == "SAM") {
            count++;
        }

        if (check_right && board.substr(pos + 1, 3) == "MAS") {
            count++;
        }

        if (check_up && is_mas(board[pos-board_width], board[pos-2*board_width], board[pos-3*board_width])) {
            count++;
        }

        if (check_down && is_mas(board[pos+board_width], board[pos+2*board_width], board[pos+3*board_width])) {
            count++;
        }

        if (check_up && check_left && is_mas(board[pos-board_width-1], board[pos-2*board_width-2], board[pos-3*board_width-3])) {
            count++;
        }

        if (check_down && check_left && is_mas(board[pos+board_width-1], board[pos+2*board_width-2], board[pos+3*board_width-3])) {
            count++;
        }

        if (check_up && check_right && is_mas(board[pos-board_width+1], board[pos-2*board_width+2], board[pos-3*board_width+3])) {
            count++;
        }

        if (check_down && check_right && is_mas(board[pos+board_width+1], board[pos+2*board_width+2], board[pos+3*board_width+3])) {
            count++;
        }

        pos = board.find('X', pos + 1);
    }

    return count;
}
#else
size_t count_x_mas(std::string_view board) {
    size_t board_width = board.find('\n') + 1;
    size_t board_height = board.length() / board_width;
    size_t count = 0;
    size_t pos = board.find('A');

    assert(board_width != std::string::npos);
    assert(board.length() % board_width == 0);

    while (pos != std::string::npos) {
        size_t x = pos % board_width;
        size_t y = pos / board_width;

        // check we are not too close to the edge
        if (x > 0 && x < board_width - 1 && y > 0 && y < board_height - 1) {
            std::string cross {
                board[pos-board_width-1],
                board[pos+board_width+1],
                board[pos-board_width+1],
                board[pos+board_width-1],
            };

            if (cross == "SMSM" || cross == "MSMS" || cross == "SMMS" || cross == "MSSM") {
                count++;
            }
        }

        pos = board.find('A', pos + 1);
    }

    return count;
}
#endif

int main(int argc, char *argv[]) {
    Timer t;

    if (argc < 2) {
        std::cerr << "Not enough arguments" << std::endl;
        return 1;
    }

    std::ifstream input(argv[1]);
    std::stringstream data;
    data << input.rdbuf();

#ifndef AOC_PART2
    std::cout << "Answer: " << count_xmas(data.str()) << std::endl;
#else
    std::cout << "Answer: " << count_x_mas(data.str()) << std::endl;
#endif
}
