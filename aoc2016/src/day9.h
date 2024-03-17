#include "utils.h"
#include <stdio.h>

u64 p1(FILE* file) {
    u64 result = 0;
    char curr = fgetc(file);

    while (feof(file) == 0) {
        if (curr != '(') {
            result += 1;
            curr = fgetc(file);
            continue;
        }

        u32 chars = fgetc(file) - '0';
        curr = fgetc(file);
        while (curr != 'x') {
            chars = (chars * 10) + (curr - '0');
            curr = fgetc(file);
        }

        u32 reps = fgetc(file) - '0';
        curr = fgetc(file);
        while (curr != ')') {
            reps = (reps * 10) + (curr - '0');
            curr = fgetc(file);
        }

        result += reps * chars;
        // printf("(%dx%d) start: %ld, end: ", chars, reps, ftell(file));

        fseek(file, chars, SEEK_CUR);
        // printf("%ld\n", ftell(file));

        curr = fgetc(file);
    }

    return result;
}

u64 calc_inner(FILE* file, char* curr, u32 given_chars, u32 given_reps) {
    u64 result = 0;
    *curr = fgetc(file);
    for (int i = 0; i < given_chars; ++i) {
        if (*curr != '(') {
            result += given_reps;
            *curr = fgetc(file);
            continue;
        }

        u32 chars = fgetc(file) - '0';
        *curr = fgetc(file);
        i += 2;
        while (*curr != 'x') {
            chars = (chars * 10) + (*curr - '0');
            *curr = fgetc(file);
            i += 1;
        }

        u32 reps = fgetc(file) - '0';
        *curr = fgetc(file);
        i += 2;
        while (*curr != ')') {
            reps = (reps * 10) + (*curr - '0');
            *curr = fgetc(file);
            i += 1;
        }

        result += calc_inner(file, curr, chars, reps * given_reps);
        i += chars;
    }

    return result;
}

u64 p2(FILE* file) {
    u64 result = 0;
    char curr = fgetc(file);

    while (feof(file) == 0) {
        if (curr != '(') {
            result += 1;
            curr = fgetc(file);
            continue;
        }

        u32 chars = fgetc(file) - '0';
        curr = fgetc(file);
        while (curr != 'x') {
            chars = (chars * 10) + (curr - '0');
            curr = fgetc(file);
        }

        u32 reps = fgetc(file) - '0';
        curr = fgetc(file);
        while (curr != ')') {
            reps = (reps * 10) + (curr - '0');
            curr = fgetc(file);
        }

        result += calc_inner(file, &curr, chars, reps);
    }

    // printf("%ld", ftell(file));

    return result;
}