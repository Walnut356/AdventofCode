#pragma once
#include "types.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


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

void pop_delete(Vec* v, u64 size) {
    v->len -= size;
}

void* get_elmt(Vec* v, u64 idx, u64 size) {
    return &v->data[idx * size];
}

void* last_elmt(Vec* v, u64 size) {
    return &v->data[v->len - size];
}

u64 len(Vec* v, u64 size) {
    return v->len / size;
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

bool vec_eq(Vec* v1, Vec* v2) {
    return v1->len == v2->len && memcmp(v1->data, v2->data, v1->len) == 0;
}
