
#include "utils.hpp"
#include <cstring>
#include <iostream>
#include <map>
#include <string>
#include <tuple>
#include <vector>

u64 p1(std::string data) {
    u64 twos = 0;
    u64 threes = 0;

    // all lines are exactly the same length
    u8 buf[26]{};

    for (auto c : data) {
        switch (c) {
        case '\r':
            continue;
        case '\n': {
            auto found2 = false;
            auto found3 = false;
            for (auto l : buf) {
                if (l == 2 && !found2) {
                    twos += 1;
                    found2 = true;
                } else if (l == 3 && !found3) {
                    threes += 1;
                    found3 = true;
                }
            }
        }
            std::memset(buf, 0, 26);
            break;
        default:
            if (c < 'a' || c > 'z') {
                std::cout << c;
            }
            buf[c - 'a'] += 1;
        }
    }

    return twos * threes;
}

std::string p2(std::string data) {
    // my data ends in \n, not \r\n. that means all lines are exactly 27 characters long. There are
    // also exactly 250 lines

    for (int i = 0; i < 249; i++) {
        auto offset_1 = &data[i * 27];
        for (int j = i + 1; j < 250; ++j) {
            auto offset_2 = &data[j * 27];
            auto different = 0;

            for (int a = 0; a < 27; ++a) {
                if (offset_1[a] != offset_2[a]) {
                    different += 1;
                }
            }

            if (different == 1) {
                std::string result{};
                for (int a = 0; a < 27; ++a) {
                    if (offset_1[a] == offset_2[a]) {
                        result.push_back(offset_1[a]);
                    }
                }
                return result;
            }
        }
    }

    return std::string {};
}
