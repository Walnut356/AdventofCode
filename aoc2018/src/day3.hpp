
#include "utils.hpp"
#include <iostream>
#include <map>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

u64 p1(std::string data) {
    u64 result = 0;

    u8 fabric[1000][1000]{ 0 };

    for (auto line : Lines(data)) {
        auto x_coord = 0;
        auto y_coord = 0;
        auto x_dimm = 0;
        auto y_dimm = 0;
        for (int i = 1; i < line.len; ++i) {
            if (line.data[i] == '@') {
                i += 2; // skip space
                auto temp = parse_int(&line.data[i]);

                i += temp.second + 1; // skip comma
                x_coord = temp.first;

                temp = parse_int(&line.data[i]);

                y_coord = temp.first;
                i += temp.second + 2; // skip `: `

                temp = parse_int(&line.data[i]);

                x_dimm = temp.first;
                i += temp.second + 1; // skip `x`

                temp = parse_int(&line.data[i]);

                y_dimm = temp.first;
            }
        }

        for (int j = y_coord; j < y_coord + y_dimm; j++) {
            for (int i = x_coord; i < x_coord + x_dimm; i++) {
                fabric[j][i] += 1;
            }
        }
    }

    for (int i = 0; i < 1000; i++) {
        for (int j = 0; j < 1000; j++) {
            if (fabric[i][j] > 1) {
                result += 1;
            }

            // std::cout << (u64)fabric[i][j];
        }

        // std::cout << '\n';
    }

    return result;
}

u64 p2(std::string data) {
    u64 result = 0;

    std::vector<std::pair<Pos<int>, Pos<int>>> ranges{};

    for (auto line : Lines(data)) {
        auto x_coord = 0;
        auto y_coord = 0;
        auto x_dimm = 0;
        auto y_dimm = 0;
        for (int i = 1; i < line.len; ++i) {
            if (line.data[i] == '@') {
                i += 2; // skip space
                auto temp = parse_int(&line.data[i]);

                i += temp.second + 1; // skip comma
                x_coord = temp.first;

                temp = parse_int(&line.data[i]);

                y_coord = temp.first;
                i += temp.second + 2; // skip `: `

                temp = parse_int(&line.data[i]);

                x_dimm = temp.first;
                i += temp.second + 1; // skip `x`

                temp = parse_int(&line.data[i]);

                y_dimm = temp.first;
                break;
            }
        }

        ranges.push_back(std::make_pair(Pos<int>{ x_coord, y_coord },
                                        Pos<int>{ x_coord + x_dimm, y_coord + y_dimm }));
    }

    for (int i = 0; i < ranges.size(); i++) {
        auto [l1, l2] = ranges[i];

        bool intersect = false;
        for (int j = 0; j < ranges.size(); j++) {
            if (i == j) {
                continue;
            }
            auto [r1, r2] = ranges[j];
            if (l1.x < r2.x &&
                l2.x > r1.x &&
                l1.y < r2.y &&
                l2.y > r1.y) {
                intersect = true;
                break;
            }
        }

        if (!intersect) {
            return i + 1;
        }
    }

    return 0;
}
