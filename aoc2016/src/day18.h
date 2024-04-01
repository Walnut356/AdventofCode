#include "utils.h"

u64 p1(FILE* file) {
    const u64 ROW_LENGTH = 100;
    u64 result = 0;

    // these could be exact-size, but technically inputs COULD be of different
    // row lengths, so we'll use resizable ones.
    Vec a = with_capacity(ROW_LENGTH, 1);
    Vec b = with_capacity(ROW_LENGTH, 1);
    get_line(file, &a);

    for (int i = 0; i < ROW_LENGTH; ++i) {
        if (a.data[i] == '.') {
            result += 1;
        }
    }

    b.len = 100;

    // reuse our buffers to save on allocations
    Vec* prev = &a;
    Vec* curr = &b;
    Vec* temp = NULL;

    for (int i = 0; i < 39; ++i) {
        for (int j = 0; j < ROW_LENGTH; ++j) {
            u8 l = j == 0 ? '.' : prev->data[j - 1];
            u8 c = prev->data[j];
            u8 r = j == ROW_LENGTH - 1 ? '.' : prev->data[j + 1];

            bool trap_l = l == '^';
            bool trap_c = c == '^';
            bool trap_r = r == '^';

            if ((trap_c && (trap_l ^ trap_r)) || (trap_l ^ trap_r)) {
                curr->data[j] = '^';
            } else {
                curr->data[j] = '.';
                result += 1;
            }
        }

        temp = prev;
        prev = curr;
        curr = temp;
    }

    return result;
}

u64 p2(FILE* file) {
    const u64 ROW_LENGTH = 100;
    u64 result = 0;

    // these could be exact-size, but technically inputs COULD be of different
    // row lengths, so we'll use resizable ones.
    Vec a = with_capacity(ROW_LENGTH, 1);
    Vec b = with_capacity(ROW_LENGTH, 1);
    get_line(file, &a);

    for (int i = 0; i < ROW_LENGTH; ++i) {
        if (a.data[i] == '.') {
            result += 1;
        }
    }

    b.len = 100;

    // reuse our buffers to save on allocations
    Vec* prev = &a;
    Vec* curr = &b;
    Vec* temp = NULL;

    for (int i = 0; i < (400000 - 1); ++i) {
        for (int j = 0; j < ROW_LENGTH; ++j) {
            u8 l = j == 0 ? '.' : prev->data[j - 1];
            u8 c = prev->data[j];
            u8 r = j == ROW_LENGTH - 1 ? '.' : prev->data[j + 1];

            bool trap_l = l == '^';
            bool trap_c = c == '^';
            bool trap_r = r == '^';

            if ((trap_c && (trap_l ^ trap_r)) || (trap_l ^ trap_r)) {
                curr->data[j] = '^';
            } else {
                curr->data[j] = '.';
                result += 1;
            }
        }

        temp = prev;
        prev = curr;
        curr = temp;
    }

    return result;
}