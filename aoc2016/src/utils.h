#pragma once

#define NTDDI_WIN10_19H1
#define _WIN32_WINNT_WIN10
#include <windows.h>
#include <winnt.h>

#include <math.h>
#include <stdint.h>
#include <stdio.h>

#include <limits.h>
#include <profileapi.h>
#include <stdbool.h>
#include <string.h>
#include <time.h>

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

const u64 u64_MAX = ULONG_MAX;
const i64 i64_MAX = LONG_MAX;
const i64 i64_MIN = LONG_MIN;

typedef float f32;
typedef double f64;

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

// warning, mega jank ahead
/*
in short, i don't really know how this is "supposed" to be done, i'm just yoloing implementations
based on what i've used before in rust and c++.
*/
typedef struct Vec {
    u8* data;
    u64 len;
    u64 capacity;
} Vec;

Vec get_vec(u64 element_size) {
    return (Vec){ .len = 0, .capacity = 8 * element_size, .data = (u8*) malloc(element_size * 8) };
}

void grow_vec(Vec* v) {
    u64 old_len = v->len;
    u64 old_capacity = v->capacity;
    u8* new_data = (u8*) malloc(old_capacity * 2);
    for (int i = 0; i < old_len; ++i) {
        new_data[i] = v->data[i];
    }
    u8* old_data = v->data;
    v->data = new_data;
    v->capacity *= 2;
    free(old_data);
}

void free_vec(Vec* v) {
    free(v->data);
    // v->data = NULL;
}

void push(Vec* v, void* item, u64 size) {
    if (v->len + size >= v->capacity) {
        grow_vec(v);
    }
    for (int i = 0; i < size; ++i) {
        v->data[v->len + i] = ((u8*) item)[i];
    }
    v->len += size;
}

void pop(Vec* v, void* dest, u64 size) {
    for (int i = 1; i < (size + 1); ++i) {
        // this copies the bytes backwards
        ((u8*) dest)[size - i] = v->data[v->len - i];
    }
    v->len -= size;
}

/// Sets the byte just after the end of v->data to null to allow for use in null-terminated string
/// functions. This implementation is a push followed by decrementing the length this means that
///
/// 1. the null byte does not affect Vec functions
/// 2. if v->len == v->capacity, the buffer will reallocate to prevent buffer overruns.
void as_cstr(Vec* v) {
    u8 n = 0;
    push(v, &n, 1);
    v->len -= 1;
}

void print_vec(Vec* v) {
    printf("[");
    for (int i = 0; i < v->len - 1; ++i) {
        printf("%d, ", v->data[i]);
    }
    printf("%d]\n", v->data[v->len - 1]);
}

/// Look by element
bool contains(Vec* v, void* n, u64 size) {
    for (int i = 0; i < v->len; i += size) {
        if (memcmp(&v->data[i], n, size) == 0) {
            return true;
        }
    }

    return false;
}

/// Look by windows
bool contains_bytes(Vec* v, void* n, u64 size) {
    for (int i = 0; i < v->len; i++) {
        if (memcmp(&v->data[i], n, size) == 0) {
            return true;
        }
    }

    return false;
}

bool starts_with_bytes(Vec* v, void* n, u64 size) {
    return memcmp(v->data, n, size) == 0;
}

/// Fills the given buffer with all the characters up to (but not including) '\n' or EOF. Also skips
/// empty lines. Returns false if buffer is not valid (EOF or read error)
bool get_line(FILE* f, Vec* buf) {
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
