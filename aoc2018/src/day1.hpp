
#include "utils.hpp"
#include <string>
#include <unordered_set>
#include <utility>

i64 p1(std::string data) {
    i64 result = 0;

    i8 mul = 1;
    i64 state = 0;
    for (auto c : data) {
        switch (c) {
        case '+':
            mul = 1;
            break;
        case '-':
            mul = -1;
            break;
        case '\n':
            result += (state * mul);
            state = 0;
        case '\r':
            continue;
        default:
            state = (state * 10) + (c - '0');
        };
    }

    return result;
}

u64 p2(std::string data) {
    i64 result = 0;
    std::unordered_set<i64> set{};

    while (true) {
        i8 mul = 1;
        i64 state = 0;
        for (auto c : data) {
            switch (c) {
            case '+':
                mul = 1;
                break;
            case '-':
                mul = -1;
                break;
            case '\n': {
                result += (state * mul);
                auto val = set.insert(result);
                if (!val.second) {
                    return result;
                }
                state = 0;
            } break;
            case '\r':
                continue;
            default:
                state = (state * 10) + (c - '0');
            };
        }
    }
}
