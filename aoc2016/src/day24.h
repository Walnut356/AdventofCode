#include "utils.h"

typedef struct Pos {
    u8 x;
    u8 y;
} Pos;

const u8 WIDTH = 179;
const u8 HEIGHT = 37;
static u8 MAP[37][179];

typedef struct Loc {
    u16 step;
    Pos pos;
} Loc;

u64 traverse(Pos start, Pos target) {
    Vec queue = with_capacity(1000, sizeof(Pos));
    // wackest hashmap around
    bool visited[65535] = { 0 };
    Loc l = { 0, start };
    push(&queue, &l, sizeof(Loc));
    visited[*(u16*) &start] = true;

    u64 i = 0;
    while (true) {
        Loc* loc = get_elmt(&queue, i, sizeof(Loc));
        if (loc->pos.x == target.x && loc->pos.y == target.y) {
            return loc->step;
        }
        u8 x = loc->pos.x;
        u8 y = loc->pos.y;
        Pos dirs[4] = {
            { x, y - 1 },
            { x, y + 1 },
            { x - 1, y },
            { x + 1, y },
        };

        for (int j = 0; j < 4; ++j) {
            if (MAP[dirs[j].y][dirs[j].x] != '#' && !visited[*(u16*) &dirs[j]]) {
                l = (Loc){ loc->step + 1, dirs[j] };

                push(&queue, &l, sizeof(Loc));
                visited[*(u16*) &dirs[j]] = true;
            }
        }

        i++;
    }
}

u64 distance(Vec* path, u64 dists[8][8], u8 curr, u64 dist, u64* min_so_far) {
    if (dist > *min_so_far) {
        return u64_MAX;
    }
    if (path->len == 8) {
        return dist;
    }

    u64 result = u64_MAX;

    for (u8 i = 0; i < 8; ++i) {
        if (i == curr || contains(path, &i, 1)) {
            continue;
        }
        push(path, &i, 1);
        u64 t = distance(path, dists, i, dist + dists[curr][i], min_so_far);
        pop_delete(path, 1);
        result = min(result, t);
    }

    *min_so_far = min(*min_so_far, result);

    return result;
}

u64 p1(FILE* file) {
    Vec buf = with_capacity(WIDTH, 1);
    u8 row = 0;
    while (get_line(file, &buf)) {
        memcpy(MAP[row], buf.data, WIDTH);
        row++;
    }

    // checked beforehand to ensure 8 total points
    Pos targets[8] = { 0 };

    for (int r = 0; r < HEIGHT; ++r) {
        for (int c = 0; c < WIDTH; ++c) {
            if (isdigit(MAP[r][c])) {
                targets[MAP[r][c] - '0'] = (Pos){ c, r };
            }
        }
    }

    u64 dists[8][8] = { 0 };



    for (int i = 0; i < 7; ++i) {
        for (int j = i + 1; j < 8; ++j) {
            u64 t = traverse(targets[j], targets[i]);

            dists[i][j] = t;
            dists[j][i] = t;
        }
    }

    // for (int i = 0; i < 8; ++i) {
    //     printf("| ");
    //     for (int j = 0; j < 8; ++j) {
    //         printf("%4llu | ", dists[i][j]);
    //     }
    //     printf("\n");
    // }

    Vec path = get_vec(1);

    push_byte(&path, 0);
    u64 min_so_far = u64_MAX;
    u64 m = u64_MAX;
    return distance(&path, dists, 0, 0, &min_so_far);
}

u64 distance_2(Vec* path, u64 dists[8][8], u8 curr, u64 dist, u64* min_so_far) {
    if (dist > *min_so_far) {
        return u64_MAX;
    }
    if (path->len == 8) {
        return dist + dists[curr][0];
    }

    u64 result = u64_MAX;

    for (u8 i = 0; i < 8; ++i) {
        if (i == curr || contains(path, &i, 1)) {
            continue;
        }
        push(path, &i, 1);
        u64 t = distance_2(path, dists, i, dist + dists[curr][i], min_so_far);
        pop_delete(path, 1);
        result = min(result, t);
    }

    *min_so_far = min(*min_so_far, result);

    return result;
}

u64 p2(FILE* file) {
    Vec buf = with_capacity(WIDTH, 1);
    u8 row = 0;
    while (get_line(file, &buf)) {
        memcpy(MAP[row], buf.data, WIDTH);
        row++;
    }

    // checked beforehand to ensure 8 total points
    Pos targets[8] = { 0 };

    for (int r = 0; r < HEIGHT; ++r) {
        for (int c = 0; c < WIDTH; ++c) {
            if (isdigit(MAP[r][c])) {
                targets[MAP[r][c] - '0'] = (Pos){ c, r };
            }
        }
    }

    u64 dists[8][8] = { 0 };

    for (int i = 0; i < 7; ++i) {
        for (int j = i + 1; j < 8; ++j) {
            u64 t = traverse(targets[j], targets[i]);

            dists[i][j] = t;
            dists[j][i] = t;
        }
    }

    // for (int i = 0; i < 8; ++i) {
    //     printf("| ");
    //     for (int j = 0; j < 8; ++j) {
    //         printf("%4llu | ", dists[i][j]);
    //     }
    //     printf("\n");
    // }

    Vec path = get_vec(1);

    path.len = 0;
    push_byte(&path, 0);
    u64 min_so_far = u64_MAX;
    u64 m = u64_MAX;
    return distance_2(&path, dists, 0, 0, &min_so_far);
}