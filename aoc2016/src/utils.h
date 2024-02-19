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
    if (t > 1000000000) { // s
        time = t / (1000 * 1000 * 1000);
        unit = "s";
    } else if (t > 1000000) { // ms
        time = t / (1000 * 1000);
        unit = "ms";
    } else if (t > 1000) {
        time = t / 1000;
        unit = "us";
    } else {
        time = t;
        unit = "ns";
    }

    printf("| %d-%d | %llu | %lf%s |\n", day, part, result, time, unit);
}

// warning, mega jank ahead
/*
in short, i don't really know how this is "supposed" to be done, i'm just yoloing implementations
based on what i've used before in rust and c++.
*/
typedef struct Vec {
    u64 len;
    u64 capacity;
    u8* data;
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
    for (int i = 0; i < size; ++i) {
        // this copies the bytes backwards
        ((u8*) dest)[size - i] = v->data[v->len - i];
    }
    v->len -= size;
}

void print_vec(Vec* v) {
    printf("[");
    for (int i = 0; i < v->len - 1; ++i) {
        printf("%d, ", v->data[i]);
    }
    printf("%d]\n", v->data[v->len - 1]);
}

bool contains(Vec* v, void* n, u64 size) {
    for (int i = 0; i < v->len; i += size) {
        if (memcmp(&v->data[i], n, size) == 0) {
            return true;
        }
    }

    return false;
}