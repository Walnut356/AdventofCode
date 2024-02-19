#include "utils.h"

typedef enum Dir {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
} Dir;

u64 p1(FILE* file) {
    char dir;
    u32 steps = 0;
    Dir orient = UP;
    i32 x = 0;
    i32 y = 0;
    while (true) {
        fscanf_s(file, "%c%d", &dir, 1, &steps);
        switch (dir) {
        case 'L':
            orient = (Dir) ((orient + 3) % 4);
            break;
        case 'R':
            orient = (Dir) ((orient + 1) % 4);
            break;
        }

        switch (orient) {
        case UP:
            y += steps;
            break;
        case RIGHT:
            x += steps;
            break;
        case DOWN:
            y -= steps;
            break;
        case LEFT:
            x -= steps;
            break;
        }

        if (feof(file)) {
            break;
        }
        fgetc(file);
        fgetc(file);
    }

    return abs(x) + abs(y);
}

u64 p2(FILE* file) {
    char dir;
    u32 steps = 0;
    Dir orient = UP;
    i32 x = 0;
    i32 y = 0;
    Vec points = get_vec(sizeof(i32) * 2);
    while (true) {
        fscanf_s(file, "%c%d", &dir, 1, &steps);
        switch (dir) {
        case 'L':
            orient = (Dir) ((orient + 3) % 4);
            break;
        case 'R':
            orient = (Dir) ((orient + 1) % 4);
            break;
        }

        i32* val;
        i8 incr;
        switch (orient) {
        case UP:
            val = &y;
            incr = 1;
            break;
        case RIGHT:
            val = &x;
            incr = 1;
            break;
        case DOWN:
            val = &y;
            incr = -1;
            break;
        case LEFT:
            val = &x;
            incr = -1;
            break;
        }

        for (int i = 1; i <= steps; ++i) {
            *val += incr;

            i32 point[2] = { x, y };

            // printf("%d, %d\n", x, y);


            if (contains(&points, &point, sizeof(i32) * 2)) {
                free_vec(&points);

                return abs(x) + abs(y);
            }

            push(&points, &point, sizeof(i32) * 2);
        }

        if (feof(file)) {
            break;
        }
        fgetc(file);
        fgetc(file);
    }
}