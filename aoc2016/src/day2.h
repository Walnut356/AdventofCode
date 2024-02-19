#include "utils.h"

u64 p1(FILE* file) {
    int c;
    int idx = 5;
    u64 code = 0;
    while ((c = fgetc(file)) != EOF) {
        if (c == '\n') {
            // convert individual digits to decimal number
            code = code * 10 + idx;
            continue;
        }

        // ignore invalid instructions, 2d -> 1d array math
        switch (c) {
        case 'U':
            if (idx > 3) {
                idx -= 3;
            }
            break;
        case 'R':
            // R column is 3, 6, 9
            if (idx % 3 != 0) {
                idx += 1;
            }
            break;
        case 'D':
            if (idx < 7) {
                idx += 3;
            }
            break;
        case 'L':
            // kinda ugly but it works, left column will be 0, 3, 6
            if ((idx - 1) % 3 != 0) {
                idx -= 1;
            }
            break;
        }
    }

    return code * 10 + idx;
}

u64 p2(FILE* file) {
    int c;
    int idx = 10;
    u64 code = 0;

    // the math is annoying so i'm not gonna bother thsi time

    // clang-format off
    u8 pad[25] = {
        0, 0, 1, 0, 0,
        0, 2, 3, 4, 0,
        5, 6, 7, 8, 9,
        0, 10, 11, 12, 0,
        0, 0, 12, 0, 0
    };
    // clang-format on

    while ((c = fgetc(file)) != EOF) {
        if (c == '\n') {
            // convert individual digits to decimal number
            code = (code << 8) | pad[idx];
            printf("%d ", pad[idx]);
            continue;
        }

        switch (c) {
        case 'U':
            if (idx - 5 > 0 && pad[idx - 5] != 0) {
                idx -= 5;
            }
            break;
        case 'R':
            // this works because even button 9 (which goes to the "next line" ends up at a 0)
            if (pad[idx + 1] != 0) {
                idx += 1;
            }
            break;
        case 'D':
            if (idx + 5 < 25 && pad[idx + 5] != 0) {
                idx += 5;
            }
            break;
        case 'L':
            if (pad[idx - 1] != 0) {
                idx -= 1;
            }
            break;
        }
    }


    // I'll have to "manually" translate the number but oh well
    return (code << 8) | idx;
}