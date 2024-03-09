#include "utils.h"
#include <stdio.h>

u64 p1(FILE* file) {
    char result[9] = { 0 };
    u32 counts[8][26] = { 0 };

    do {
        for (int i = 0; i < 8; ++i) {
            char val = fgetc(file);
            counts[i][val - 'a'] += 1;
        }

    } while (fgetc(file) != EOF);

    for (int i = 0; i < 8; ++i) {
        result[i] = array_max_idx(counts[i], 26) + 'a';
    }

    // printf("%s", result);
    return *(u64*) result;
}

u64 p2(FILE* file) {
    char result[9] = { 0 };
    u32 counts[8][26] = { 0 };

    do {
        for (int i = 0; i < 8; ++i) {
            char val = fgetc(file);
            counts[i][val - 'a'] += 1;
        }

    } while (fgetc(file) != EOF);

    for (int i = 0; i < 8; ++i) {
        result[i] = array_min_idx(counts[i], 26) + 'a';
    }

    printf("%s", result);
    return *(u64*) result;
}