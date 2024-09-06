#pragma once

#include <string>
#define NTDDI_WIN10_19H1
#define _WIN32_WINNT_WIN10
#include <windows.h>
#include <winnt.h>

#include <math.h>

#include <stdio.h>

#include <profileapi.h>
#include <stdbool.h>

#include <limits.h>
#include <stdint.h>
#include <iostream>

// sorry c devs, rust names are just better
typedef unsigned char u8;
typedef signed char i8;
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
    std::string unit {};
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