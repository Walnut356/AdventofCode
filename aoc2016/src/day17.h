#include "md5.h"
#include "utils.h"

typedef struct Pos {
    u8 x;
    u8 y;
} Pos;

u64 traverse_map(Pos pos, Vec* buf, u64 steps, u64* min_so_far) {
    if (steps > 1000000 || steps > *min_so_far) {
        return u64_MAX;
    }
    if (pos.x == 3 && pos.y == 3) {
        return steps;
    }

    u8 hash[17] = { 0 };
    md5String(buf->data, hash);
    u8 dirs[5] = { 0 };
    sprintf(dirs, "%02x%02x", hash[0], hash[1]);
    // same order as problem, U -> D -> L -> R
    bool doors[4] = {
        pos.y > 0 && dirs[0] >= 'b' && dirs[0] <= 'f',
        pos.y < 3 && dirs[1] >= 'b' && dirs[1] <= 'f',
        pos.x > 0 && dirs[2] >= 'b' && dirs[2] <= 'f',
        pos.x < 3 && dirs[3] >= 'b' && dirs[3] <= 'f',
    };

    for (int i = 0; i < 4; ++i) {
        if (!doors[i]) {
            continue;
        }
        Pos new_pos = { pos.x, pos.y };
        u8 letter = ' ';
        switch (i) {
        case 0:
            new_pos.y--;
            letter = 'U';
            break;
        case 1:
            new_pos.y++;
            letter = 'D';
            break;
        case 2:
            new_pos.x--;
            letter = 'L';
            break;
        case 3:
            new_pos.x++;
            letter = 'R';
            break;
        }
        push_byte(buf, letter);
        as_cstr(buf);
        u64 t = traverse_map(new_pos, buf, steps + 1, min_so_far);

        if (t < *min_so_far) {
            printf("%llu: %s\n", *min_so_far, &buf->data[8]);
            *min_so_far = t;
        }

        pop_delete(buf, 1);
        as_cstr(buf);
    }

    return *min_so_far;
}

u64 p1(FILE* file) {
    const u8 orig_line_len = 0;
    Vec buf = get_vec(1);
    get_line(file, &buf);
    as_cstr(&buf);

    Pos pos = { 0, 0 };

    u64 min_so_far = u64_MAX;

    traverse_map(pos, &buf, 0, &min_so_far);

    return 0;
}

u64 traverse_p2(Pos pos, Vec* buf, u64 steps) {
    if (pos.x == 3 && pos.y == 3) {
        return steps;
    }

    u8 hash[17] = { 0 };
    md5String(buf->data, hash);
    u8 dirs[5] = { 0 };
    sprintf(dirs, "%02x%02x", hash[0], hash[1]);
    // same order as problem, U -> D -> L -> R
    bool doors[4] = {
        pos.y > 0 && dirs[0] >= 'b' && dirs[0] <= 'f',
        pos.y < 3 && dirs[1] >= 'b' && dirs[1] <= 'f',
        pos.x > 0 && dirs[2] >= 'b' && dirs[2] <= 'f',
        pos.x < 3 && dirs[3] >= 'b' && dirs[3] <= 'f',
    };

    u64 result = 0;

    for (int i = 0; i < 4; ++i) {
        if (!doors[i]) {
            continue;
        }
        Pos new_pos = { pos.x, pos.y };
        u8 letter = ' ';
        switch (i) {
        case 0:
            new_pos.y--;
            letter = 'U';
            break;
        case 1:
            new_pos.y++;
            letter = 'D';
            break;
        case 2:
            new_pos.x--;
            letter = 'L';
            break;
        case 3:
            new_pos.x++;
            letter = 'R';
            break;
        }
        push_byte(buf, letter);
        as_cstr(buf);
        u64 t = traverse_p2(new_pos, buf, steps + 1);

        result = max(result, t);

        pop_delete(buf, 1);
        as_cstr(buf);
    }

    return result;
}

u64 p2(FILE* file) {
    const u8 orig_line_len = 0;
    Vec buf = get_vec(1);
    get_line(file, &buf);
    as_cstr(&buf);

    Pos pos = { 0, 0 };

    return traverse_p2(pos, &buf, 0);
}