#pragma once

#include <cstring>
#include <string>
#include <utility>
#define NTDDI_WIN10_19H1
#define _WIN32_WINNT_WIN10
#include <windows.h>
#include <winnt.h>

#include <math.h>

#include <stdio.h>

#include <profileapi.h>
#include <stdbool.h>

#include <iostream>
#include <limits.h>
#include <stdint.h>


// sorry c devs, rust names are just better
typedef uint8_t u8;
typedef int8_t i8;
typedef uint16_t u16;
typedef int16_t i16;
typedef uint32_t u32;
typedef int32_t i32;
typedef uint64_t u64;
typedef int64_t i64;

const u8 u8_MAX = UCHAR_MAX;
const i8 i8_MAX = CHAR_MAX;
const i8 i8_MIN = CHAR_MIN;

const u16 u16_MAX = USHRT_MAX;
const i16 i16_MAX = SHRT_MAX;
const i16 i16_MIN = SHRT_MIN;

const u32 u32_MAX = UINT_MAX;
const i32 i32_MAX = INT_MAX;
const i32 i32_MIN = INT_MIN;

const u64 u64_MAX = ULLONG_MAX;
const i64 i64_MAX = LLONG_MAX;
const i64 i64_MIN = LLONG_MIN;

typedef float f32;
typedef double f64;

// Gonna be real, chrono looks like a complete fucking mess. No idea what's going on there. As such,
// I'll just continue to use my C function but a little bit spruced up
double time_now() {
    u64 time;
    u64 freq;
    QueryPerformanceCounter((LARGE_INTEGER*) &time);
    QueryPerformanceFrequency((LARGE_INTEGER*) &freq);
    return ((double) time / (double) freq) * 1000000000;
}

void print_dur(double t) {
    std::string unit{};
    double time;
    char t_str[30];
    if (t > 1000000000) { // s
        time = t / (1000 * 1000 * 1000);
        sprintf_s(t_str, 30, "%.03f", time);
        unit = "s";
    } else if (t > 1000000) { // ms
        time = t / (1000 * 1000);
        sprintf_s(t_str, 30, "%.4lf", time);
        unit = "ms";
    } else if (t > 1000) {
        time = t / 1000;
        sprintf_s(t_str, 30, "%.1lf", time);
        unit = "us";
    } else {
        time = t;
        sprintf_s(t_str, 30, "%.0lf", time);
        unit = "ns";
    }

    std::cout << t_str << unit << " |\n";
}

struct Line {
    Line& operator++() {
        data += len + 1;
        if (data >= end) {
            len = 0;
            return *this;
        }

        char* loc = strchr(data, '\n');
        if (loc == 0) {
            len = (u64) (end - data);
        } else {
            len = loc - data;
        }

        return *this;
    }

    bool operator!=(Line rhs) {
        auto val = data != rhs.data || len != rhs.len;
        // std::cout << val;
        return val;
    }

    Line operator*() {
        return *this;
    }

    char* data;
    u64 len;
    char* end;
};

struct Lines {
    // essentially gets a slice view of the target string
    explicit Lines(std::string& val) {
        this->curr = (char*) &val[0];
        this->last = (&val[0]) + val.length();
    }

    Line begin() {
        char* loc = strchr(this->curr, '\n');
        if (loc == 0) {
            return Line{ curr, (u64) (last - curr), last };
        } else {
            auto result = Line{ curr, (u64) (loc - curr), last };
            return result;
        }
    }

    Line end() {
        return Line{ last, 0, last };
    }

private:
    char* curr{ 0 };
    char* last{ 0 };
};

struct LinesRange {};

/// Turns a string into an i64, so long as the number is less than 256 digits long.
///
/// Returns the a pair of (`number`, `characters_processed`). `characters_processed` maybe not be
/// equal to the **digit**-length of `number` due to `+` and `-`
std::pair<i64, u8> parse_int(char* str) {
    i8 mul = 1;
    i64 state = 0;
    u8 i = 0;
    while (str[i] != 0) {
        auto c = str[i];
        switch (c) {
        case '+':
            mul = 1;
            break;
        case '-':
            mul = -1;
            break;
        default:
            if (std::isdigit(c)) {
                state = (state * 10) + (c - '0');
            } else {
                goto end;
            }
        };
        i += 1;
    }

end:
    return std::make_pair(state, i);
}

template<typename T>
struct Pos {
    T x, y;
};