#pragma once

#define NTDDI_WIN10_19H1
#define _WIN32_WINNT_WIN10
#include <windows.h>
#include <winnt.h>

#include <math.h>

#include <stdio.h>

#include "map.h"
#include "vec.h"
#include "iter.h"
#include <profileapi.h>
#include <stdbool.h>
#include <string.h>
#include <time.h>

double time_now() {
    u64 time;
    u64 freq;
    QueryPerformanceCounter((LARGE_INTEGER*) &time);
    QueryPerformanceFrequency((LARGE_INTEGER*) &freq);
    return ((double) time / (double) freq) * 1000000000;
}

void print_result(u64 result, double t, u8 day, u8 part) {
    char* unit;
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

    printf("| %d-%d | %llu | %s%s |\n", day, part, result, t_str, unit);
}

/// Fills the given buffer with all the characters up to (but not including) '\n' or EOF. Also skips
/// empty lines. Returns false if buffer is not valid (EOF or read error)
bool get_line(FILE* f, Vec* buf) {
    // long pos = ftell(f);
    // fseek(f, 0, SEEK_END);
    // long f_len = ftell(f);
    // fseek(f, pos, SEEK_SET);

    char val = fgetc(f);
    while (feof(f) == 0 && (val == '\n' || val == '\r')) {
        val = fgetc(f);
    }

    if (feof(f) != 0) {
        return false;
    }

    buf->len = 0;

    while (feof(f) == 0 && val != '\n') {
        if (val != '\r') {
            push(buf, &val, 1);
        }
        val = fgetc(f);
    }

    return true;
}

/* ---------------------------------------------------------------------------------------------- */
/*                                           Array Utils                                          */
/* ---------------------------------------------------------------------------------------------- */

u32 array_max(u32* array, u32 len) {
    u32 max = 0;
    for (int i = 0; i < len; ++i) {
        max = array[i] > max ? array[i] : max;
    }

    return max;
}

u32 array_max_idx(u32* array, u32 len) {
    u32 max = 0;
    u32 idx = 0;
    for (int i = 0; i < len; ++i) {
        if (array[i] > max) {
            max = array[i];
            idx = i;
        }
    }

    return idx;
}

u32 array_min(u32* array, u32 len) {
    u32 min = 0;
    for (int i = 0; i < len; ++i) {
        min = array[i] < min ? array[i] : min;
    }

    return min;
}

u32 array_min_idx(u32* array, u32 len) {
    u32 min = 4294967295;
    u32 idx = 0;
    for (int i = 0; i < len; ++i) {
        if (array[i] < min) {
            min = array[i];
            idx = i;
        }
    }

    return idx;
}
