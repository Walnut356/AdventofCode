#include "utils.h"

void print_screen(u8* screen) {
    for (int r = 0; r < 6; r++) {
        for (int c = 0; c < 50; c++) {
            if (screen[(r * 50) + c] == 1) {
                printf("@");
            } else {
                printf(" ");
            }
        }
        printf("\n");
    }
}

u64 p1(FILE* file) {
    u64 result = 0;

    u8 screen[6][50] = { 0 };
    Vec buf = get_vec(1);

    while (get_line(file, &buf)) {
        // print_screen(&screen);

        as_cstr(&buf);
        char* line = (char*) buf.data;

        // rect 00x0
        if (buf.len < 11) {
            u8 cols = line[5] - '0';
            if (line[6] != 'x') {
                cols = (cols * 10) + (line[6] - '0');
            }

            u8 rows = line[buf.len - 1] - '0';

            for (int row = 0; row < rows; ++row) {
                for (int col = 0; col < cols; ++col) {
                    screen[row][col] = 1;
                }
            }
        }

        u8 row;
        u8 col;
        u8 amnt;

        // rotate [r/c]
        switch (line[7]) {
        case 'c':
            sscanf(line, "rotate column x=%hhd by %hhd", &col, &amnt);
            for (int iter = 0; iter < amnt; ++iter) {
                u8 temp = screen[5][col];
                for (int i = 5; i > 0; --i) {
                    screen[i][col] = screen[i - 1][col];
                }
                screen[0][col] = temp;
            }
            break;
        case 'r':
            sscanf(line, "rotate row y=%hhd by %hhd", &row, &amnt);

            for (int iter = 0; iter < amnt; ++iter) {
                u8 temp = screen[row][49];
                for (int i = 49; i > 0; --i) {
                    screen[row][i] = screen[row][i - 1];
                }
                screen[row][0] = temp;
            }
            break;
        }
    }

    for (int i = 0; i < 6 * 50; i++) {
        result += ((u8*) screen)[i];
    }

    return result;
}

u64 p2(FILE* file) {
    u64 result = 0;

    u8 screen[6][50] = { 0 };
    Vec buf = get_vec(1);

    while (get_line(file, &buf)) {
        // print_screen(&screen);

        as_cstr(&buf);
        char* line = (char*) buf.data;

        // rect 00x0
        if (buf.len < 11) {
            u8 cols = line[5] - '0';
            if (line[6] != 'x') {
                cols = (cols * 10) + (line[6] - '0');
            }

            u8 rows = line[buf.len - 1] - '0';

            for (int row = 0; row < rows; ++row) {
                for (int col = 0; col < cols; ++col) {
                    screen[row][col] = 1;
                }
            }
        }

        u8 row;
        u8 col;
        u8 amnt;

        // rotate [r/c]
        switch (line[7]) {
        case 'c':
            sscanf(line, "rotate column x=%hhd by %hhd", &col, &amnt);
            for (int iter = 0; iter < amnt; ++iter) {
                u8 temp = screen[5][col];
                for (int i = 5; i > 0; --i) {
                    screen[i][col] = screen[i - 1][col];
                }
                screen[0][col] = temp;
            }
            break;
        case 'r':
            sscanf(line, "rotate row y=%hhd by %hhd", &row, &amnt);

            for (int iter = 0; iter < amnt; ++iter) {
                u8 temp = screen[row][49];
                for (int i = 49; i > 0; --i) {
                    screen[row][i] = screen[row][i - 1];
                }
                screen[row][0] = temp;
            }
            break;
        }
    }

    print_screen(&screen);

    return result;
}