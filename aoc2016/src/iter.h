#pragma once
#include "types.h"
#include "vec.h"

typedef struct Iter {
    Vec* vec;
    u64 pos;
    u64 elmt_size;
} Iter;

Iter get_iter(Vec* vec, u64 elmt_size) {
    return (Iter) {vec, 0, elmt_size};
}

void* next(Iter* iter) {
    if (iter->pos * iter->elmt_size > iter->vec->len) {
        return NULL;
    }
    iter->pos += 1;
    return (void*) &iter->vec->data[(iter->pos - 1) * iter->elmt_size];
}