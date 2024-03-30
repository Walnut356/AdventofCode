#include "utils.h"
#include <minwindef.h>

typedef enum Tile {
    Floor = 0,
    Wall = 1,
    Unresolved = 2,
    Walked = 3,
} Tile;

typedef struct Pos {
    u64 x;
    u64 y;
} Pos;

// 100x100 just to be safe, since we'll probably need to approach from behind
static Tile map[100][100] = { Unresolved };

void print_map(Pos curr) {
    for (int i = 0; i < 50; ++i) {
        for (int j = 0; j < 50; ++j) {
            if (i == 1 && j == 1) {
                printf("S");
                continue;
            }
            if (i == 39 && j == 31) {
                printf("G");
                continue;
            }
            if (i == curr.y && j == curr.x) {
                printf("C");
                continue;
            }
            switch (map[i][j]) {
            case Floor:
                printf(".");
                break;
            case Wall:
                printf("#");
                break;
            case Walked:
                printf("O");
                break;
            case Unresolved:
                printf("?");
                break;
            }
        }
        printf("\n");
    }
    printf("---\n");
}

static u8 dsitances[100][100] = { 0 };

static u64 fav = 0;

bool can_walk(Pos val) {
    if (map[val.y][val.x] != Unresolved) {
        return map[val.y][val.x] == Floor;
    }

    u64 temp =
        fav + ((val.x * val.x) + (3 * val.x) + (2 * val.x * val.y) + val.y + (val.y * val.y));

    bool result = __builtin_parityll(temp);
    map[val.y][val.x] = result;

    return result == Floor;
}

u64 traverse(Pos destination, Pos curr, u8 dist, u64* min_so_far) {
    if (curr.x == destination.x && curr.y == destination.y) {
        return dist;
    }
    if (dist > *min_so_far || curr.x > 49 || curr.y > 49) {
        return 1000;
    }

    u64 result = 1000;

    // easier to just try all 4 directions than to set up a vec
    Pos dirs[4] = {
        { curr.x + 1, curr.y },
        { curr.x, min(curr.y - 1, u64_MAX) },
        { min(curr.x - 1, u64_MAX), curr.y },
        { curr.x, curr.y + 1 },
    };

    for (int i = 0; i < 4; ++i) {
        if (dirs[i].x > 50 || dirs[i].y > 50 || !can_walk(dirs[i])) {
            continue;
        }
        // caching so we don't get caught in loops
        map[dirs[i].y][dirs[i].x] = Walked;
        u64 v = traverse(destination, dirs[i], dist + 1, min_so_far);
        // undo the cache as other (faster) paths may cross that tile
        map[dirs[i].y][dirs[i].x] = Floor;

        result = min(result, v);
    }

    *min_so_far = min(*min_so_far, result);

    // print_map(curr);
    return result;
}

u64 p1(FILE* file) {
    fscanf(file, "%llu", &fav);

    for (int i = 0; i < 50; ++i) {
        for (int j = 0; j < 50; ++j) {
            map[i][j] = Unresolved;
        }
    }

    // printf("can_walk((Pos){ 31, 39}) = %i\n", can_walk((Pos){ 31, 39 }));

    // print_map()

    u64 result = 0;
    u64 min_so_far = 1000;

    result = traverse((Pos){ 31, 39 }, (Pos){ 1, 1 }, 0, &min_so_far);

    return result;
}

bool can_walk_p2(Pos val) {
    if (map[val.y][val.x] != Unresolved) {
        return map[val.y][val.x] != Wall;
    }

    u64 temp =
        fav + ((val.x * val.x) + (3 * val.x) + (2 * val.x * val.y) + val.y + (val.y * val.y));

    bool result = __builtin_parityll(temp);
    map[val.y][val.x] = result;

    return result == Floor;
}

void traverse_p2(Pos curr, u8 dist, Vec* visited) {
    if (dist + 1 > 50 || curr.x > 99 || curr.y > 99) {
        return;
    }

    // easier to just try all 4 directions than to set up a vec
    Pos dirs[4] = {
        { curr.x + 1, curr.y },
        { curr.x, min(curr.y - 1, u64_MAX) },
        { min(curr.x - 1, u64_MAX), curr.y },
        { curr.x, curr.y + 1 },
    };

    for (int i = 0; i < 4; ++i) {
        if (dirs[i].x > 99 || dirs[i].y > 99 || !can_walk(dirs[i])) {
            continue;
        }
        // caching so we don't get caught in loops
        map[dirs[i].y][dirs[i].x] = Walked;
        if (!contains(visited, &dirs[i], sizeof(Pos))) {
            push(visited, &dirs[i], sizeof(Pos));
        }
        traverse_p2(dirs[i], dist + 1, visited);
        // undo the cache as other (faster) paths may cross that tile
        map[dirs[i].y][dirs[i].x] = Floor;
    }
}

u64 p2(FILE* file) {
    fscanf(file, "%llu", &fav);

    for (int i = 0; i < 100; ++i) {
        for (int j = 0; j < 100; ++j) {
            map[i][j] = Unresolved;
        }
    }

    // printf("can_walk((Pos){ 31, 39}) = %i\n", can_walk((Pos){ 31, 39 }));

    // print_map()

    Vec visited = get_vec(sizeof(Pos));

    traverse_p2((Pos){ 1, 1 }, 0, &visited);

    return len(&visited, sizeof(Pos));
}
