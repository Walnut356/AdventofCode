#include "utils.hpp"
#include <format>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <chrono>

#include "day1.hpp"
const int DAY = 1;

int main() {
    std::ifstream t(std::format("./aoc2018/test_data/day{}.txt", DAY));
    std::stringstream buffer {};
    buffer << t.rdbuf();

    std::string data1 = buffer.str();
    std::string data2 = buffer.str();

    auto start = time_now();

    auto res1 = p1(data1);

    auto end = time_now();
    auto dur = end - start;

    std::cout << std::format("| {}-1 | {} | ", DAY, res1);
    print_dur(dur);

    start = time_now();

    auto res2 = p2(data2);

    end = time_now();
    dur = end - start;

    std::cout << std::format("| {}-2 | {} | ", DAY, res2);
    print_dur(dur);

    return 0;
}