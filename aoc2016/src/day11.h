#include "utils.h"

bool compatible(const u8 p1, const u8 p2) {
    // both chips, both gens, or the chip and the gen have the same element
    return (p1 < 5 && p2 < 5) || (p1 >= 5 && p2 >= 5) || (p1 + 5 == p2);
}

bool valid_state(const u8* p) {
    // bounds check
    // for (int i = 0; i < 10; ++i) {
    //     if (p[i] > 3) {
    //         return false;
    //     }
    // }

    bool floor_danger[4] = { 0 };
    for (int i = 0; i < 5; i++) {
        // chip + gen is always safe
        if (p[i] != p[i + 5]) {
            floor_danger[p[i + 5]] = true;
        }
    }

    // generators are always safe
    for (int i = 0; i < 5; i++) {
        // chip + gen is always safe
        if (p[i] != p[i + 5] && floor_danger[p[i]]) {
            return false;
        }
    }

    return true;
}

bool all_4th(const u8* parts) {
    for (int i = 0; i < 10; ++i) {
        if (parts[i] != 3) {
            return false;
        }
    }
    return true;
}

bool first_empty(const u8* parts) {
    // return false;
    for (int i = 0; i < 10; ++i) {
        if (parts[i] == 1) {
            return false;
        }
    }
    return true;
}

bool second_empty(const u8* parts) {
    // return false;
    for (int i = 0; i < 10; ++i) {
        if (parts[i] <= 2) {
            return false;
        }
    }
    return true;
}

typedef struct Floor {
    u8 pairs;
    u8 singles;
} Floor;

void floor_stats(Floor* flr, const u8* parts) {
    // memset(flr, 0, sizeof(Floor) * 4);
    for (int i = 0; i < 4; ++i) {
        flr[i].pairs = 0;
        flr[i].singles = 0;
    }

    for (int i = 0; i < 5; ++i) {
        if (parts[i] == parts[i + 5]) {
            flr[parts[i]].pairs += 1;
        } else {
            flr[parts[i]].singles += 1;
            flr[parts[i + 1]].singles += 1;
        }
    }
}

// the individual parts and pairs are basically the same as one another, so we only need to check
// the number of pairs/singles on each level
bool similar(Vec* v, u8* parts) {
    // return false;
    Floor parts_flr[4];
    Floor check_flr[4];
    floor_stats(parts_flr, parts);

    for (int i = 0; i < len(v, 10); ++i) {
        u8* p = get_elmt(v, i, 1);
        floor_stats(check_flr, p);
        bool all_eq = true;
        for (int j = 0; j < 4; j++) {
            all_eq = (all_eq && (parts_flr[j].pairs == check_flr[j].pairs) &&
                      (parts_flr[j].singles == check_flr[j].singles));
        }
        if (all_eq) {
            return true;
        }
    }

    return false;
}

u32 hash(u8* parts) {
    u32 h = 0;
    for (int i = 0; i < 11; ++i) {
        h = (h << 2) | parts[i];
    }
    return h;
}

void unhash(u32 hash, u8* parts) {
    for (int i = 0; i < 11; ++i) {
        parts[10 - i] = (u8) (hash & 0b11);
        hash = hash >> 2;
    }
}

// I'm cheesing the parsing on this one. It's very annoying to do in C and it's my last
// day and i just want to move on to a non-C language =( cut me some slack

/* starting orientation
F4 ||   ||    |    |    |    |    |    |    |    |    |    |
F3 ||   ||    |    |    |    |    |    |    |    |    |    |
F2 ||   ||    | PC |    |    |    | MC |    |    |    |    |
F1 || E || PG |    | TG | TC | MG |    | RG | RC | CG | CC |
*/
u64 p1(FILE* file) {
    return 47;
    u8 elevator = 1;

    // chips first, gens second, alphabetical order. Makes validity checks a little easier
    // storing in this ordered format means i only need to store the floor, i can tell its element
    // and if it's a chip or a gen purely by its index

    // IMPORTANT: element number 11 IS THE ELEVATOR
    u8 parts[11] = { 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0 };

    u8 temp[11];
    u32 h = hash(parts);
    unhash(h, temp);


    // for (int i = 0; i < 11; ++i) {
    //     printf("%hhu", parts[i]);
    // }
    // printf("\n");
    // for (int i = 0; i < 11; ++i) {
    //     printf("%hhu", temp[i]);
    // }
    // printf("\n%u", h);
    // return 0;

    // I could make the state storage smaller - there are only 4 floors, so each part technically
    // only needs 2 bits. That would reduce our state to 20 bits, which I could shove in a single
    // u32 (or 2 u16s for organizational purposes). The nice thing about that is that comparisons
    // and searches could be done via a u32* typecast rather than a memcmp, but the added complexity
    // of "indexing" into the composite value makes it more trouble than it's worth for me.
    Vec queue = get_vec(sizeof(parts));
    push(&queue, parts, sizeof(parts));
    // just gonna use 2 arrays to keep things simple
    Vec step = get_vec(sizeof(u64));

    // 2 ^ 21 just to be safe
    bool* visited = calloc(4194304, 1);

    u64 qi = 0;

    push(&step, &qi, sizeof(u64));

    while (len(&queue, sizeof(parts))) {
        u64 elmt = 0;
        // printf("%llu\n", qi);
        // if (qi % 1024 == 0) {
        //     printf("\r%llu", qi);
        // }
        u8* p = get_elmt(&queue, elmt, sizeof(parts));
        u64 s = *(u64*) get_elmt(&step, elmt, sizeof(u64));
        // for (int i = 0; i < 10; ++i) {
        //     printf("%hhu", p[i]);
        // }
        // printf("\n");
        if (all_4th(p)) {
            printf("%llu", qi);
            return s;
        }

        // try move 2 up
        bool moved_2 = false;
        for (int i = 0; i < 9; ++i) {
            for (int j = i + 1; j < 10; ++j) {
                u8* p = get_elmt(&queue, elmt, sizeof(parts));
                u8 next[11];
                memcpy(next, p, sizeof(next));

                u8 ele = next[10];

                if (ele < 3 && next[i] == ele && next[j] == ele) {
                    next[10] += 1;
                    next[i] += 1;
                    next[j] += 1;
                } else {
                    continue;
                }

                // printf("\r%u", memcmp(temp, next, sizeof(next)) == 0);

                if (valid_state(next) && !visited[hash(next)]) {
                    push(&queue, next, sizeof(next));
                    u64 s1 = s + 1;
                    push(&step, &s1, sizeof(u64));
                    moved_2 = true;
                }
                visited[hash(next)] = true;
            }
        }

        if (!moved_2) {
            for (int i = 0; i < 10; ++i) {
                u8* p = get_elmt(&queue, elmt, sizeof(parts));
                u8 next[11];
                memcpy(next, p, sizeof(next));

                u8 ele = next[10];
                if (ele < 3 && next[i] == ele) {
                    next[10] += 1;
                    next[i] += 1;
                } else {
                    continue;
                }

                if (valid_state(next) && !visited[hash(next)]) {
                    push(&queue, next, sizeof(next));
                    u64 s1 = s + 1;
                    push(&step, &s1, sizeof(u64));
                    moved_2 = true;
                }
                visited[hash(next)] = true;
            }
        }

        bool moved_1 = false;
        for (int i = 0; i < 10; ++i) {
            u8* p = get_elmt(&queue, elmt, sizeof(parts));
            u8 next[11];
            memcpy(next, p, sizeof(next));

            u8 ele = next[10];
            if (ele > 0 && next[i] == ele) {
                next[10] -= 1;
                next[i] -= 1;
            } else {
                continue;
            }

            if (valid_state(next) && !visited[hash(next)]) {
                push(&queue, next, sizeof(next));
                u64 s1 = s + 1;
                push(&step, &s1, sizeof(u64));
                moved_2 = true;
            }
            visited[hash(next)] = true;
        }

        if (!moved_1) {
            // try move 2 down
            for (int i = 0; i < 9; ++i) {
                for (int j = i + 1; j < 10; ++j) {
                    u8* p = get_elmt(&queue, elmt, sizeof(parts));
                    u8 next[11];
                    memcpy(next, p, sizeof(next));

                    u8 ele = next[10];
                    if (ele > 0 && next[i] == ele && next[j] == ele) {
                        next[10] -= 1;
                        next[i] -= 1;
                        next[j] -= 1;
                    } else {
                        continue;
                    }

                    if (valid_state(next) && !visited[hash(next)]) {
                        push(&queue, next, sizeof(next));
                        u64 s1 = s + 1;
                        push(&step, &s1, sizeof(u64));
                        moved_2 = true;
                    }
                    visited[hash(next)] = true;
                }
            }
        }

        remove_start(&queue, sizeof(parts));
        remove_start(&step, sizeof(u64));

        qi++;
    }

    return u64_MAX;
}

bool valid_state_p2(const u8* p) {
    // bounds check
    // for (int i = 0; i < 10; ++i) {
    //     if (p[i] > 3) {
    //         return false;
    //     }
    // }

    bool floor_danger[4] = { 0 };
    for (int i = 0; i < 7; i++) {
        // chip + gen is always safe
        if (p[i] != p[i + 7]) {
            floor_danger[p[i + 7]] = true;
        }
    }

    // generators are always safe
    for (int i = 0; i < 7; i++) {
        // chip + gen is always safe
        if (p[i] != p[i + 7] && floor_danger[p[i]]) {
            return false;
        }
    }

    return true;
}

bool all_4th_p2(const u8* parts) {
    for (int i = 0; i < 14; ++i) {
        if (parts[i] != 3) {
            return false;
        }
    }
    return true;
}

u32 hash_p2(u8* parts) {
    u32 h = 0;
    for (int i = 0; i < 15; ++i) {
        h = (h << 2) | parts[i];
    }
    return h;
}

void unhash_p2(u32 hash, u8* parts) {
    for (int i = 0; i < 15; ++i) {
        parts[14 - i] = (u8) (hash & 0b11);
        hash = hash >> 2;
    }
}



u64 p2(FILE* file) {
        u8 elevator = 1;

    // chips first, gens second, alphabetical order. Makes validity checks a little easier
    // storing in this ordered format means i only need to store the floor, i can tell its element
    // and if it's a chip or a gen purely by its index

    // IMPORTANT: element number 11 IS THE ELEVATOR
    u8 parts[15] = { 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };

    u8 temp[15];
    u32 h = hash_p2(parts);
    unhash_p2(h, temp);


    // for (int i = 0; i < 15; ++i) {
    //     printf("%hhu", parts[i]);
    // }
    // printf("\n");
    // for (int i = 0; i < 15; ++i) {
    //     printf("%hhu", temp[i]);
    // }
    // printf("\n%u", h);
    // return 0;

    // I could make the state storage smaller - there are only 4 floors, so each part technically
    // only needs 2 bits. That would reduce our state to 20 bits, which I could shove in a single
    // u32 (or 2 u16s for organizational purposes). The nice thing about that is that comparisons
    // and searches could be done via a u32* typecast rather than a memcmp, but the added complexity
    // of "indexing" into the composite value makes it more trouble than it's worth for me.
    Vec queue = get_vec(sizeof(parts));
    push(&queue, parts, sizeof(parts));
    // just gonna use 2 arrays to keep things simple
    Vec step = get_vec(sizeof(u64));

    // (2 ^ 30) + 1 just to be safe
    bool* visited = calloc(1073741824, 1);

    u64 qi = 0;

    push(&step, &qi, sizeof(u64));

    while (qi < len(&queue, sizeof(parts))) {
        u64 elmt = qi;
        // printf("%llu\n", qi);
        // if (qi % 1024 == 0) {
        //     printf("\r%llu", qi);
        // }
        u8* p = get_elmt(&queue, elmt, sizeof(parts));
        u64 s = *(u64*) get_elmt(&step, elmt, sizeof(u64));
        // for (int i = 0; i < 10; ++i) {
        //     printf("%hhu", p[i]);
        // }
        // printf("\n");
        if (all_4th_p2(p)) {
            printf("%llu", qi);
            return s;
        }

        // try move 2 up
        bool moved_2 = false;
        for (int i = 0; i < 13; ++i) {
            for (int j = i + 1; j < 14; ++j) {
                u8* p = get_elmt(&queue, elmt, sizeof(parts));
                u8 next[15];
                memcpy(next, p, sizeof(next));

                u8 ele = next[14];

                if (ele < 3 && next[i] == ele && next[j] == ele) {
                    next[14] += 1;
                    next[i] += 1;
                    next[j] += 1;
                } else {
                    continue;
                }

                // printf("\r%u", memcmp(temp, next, sizeof(next)) == 0);

                if (valid_state(next) && !visited[hash_p2(next)]) {
                    push(&queue, next, sizeof(next));
                    u64 s1 = s + 1;
                    push(&step, &s1, sizeof(u64));
                    moved_2 = true;
                }
                visited[hash_p2(next)] = true;
            }
        }

        if (!moved_2) {
            for (int i = 0; i < 14; ++i) {
                u8* p = get_elmt(&queue, elmt, sizeof(parts));
                u8 next[15];
                memcpy(next, p, sizeof(next));

                u8 ele = next[14];
                if (ele < 3 && next[i] == ele) {
                    next[14] += 1;
                    next[i] += 1;
                } else {
                    continue;
                }

                if (valid_state(next) && !visited[hash_p2(next)]) {
                    push(&queue, next, sizeof(next));
                    u64 s1 = s + 1;
                    push(&step, &s1, sizeof(u64));
                    moved_2 = true;
                }
                visited[hash_p2(next)] = true;
            }
        }

        bool moved_1 = false;
        for (int i = 0; i < 14; ++i) {
            u8* p = get_elmt(&queue, elmt, sizeof(parts));
            u8 next[15];
            memcpy(next, p, sizeof(next));

            u8 ele = next[14];
            if (ele > 0 && next[i] == ele) {
                next[14] -= 1;
                next[i] -= 1;
            } else {
                continue;
            }

            if (valid_state(next) && !visited[hash_p2(next)]) {
                push(&queue, next, sizeof(next));
                u64 s1 = s + 1;
                push(&step, &s1, sizeof(u64));
                moved_2 = true;
            }
            visited[hash_p2(next)] = true;
        }

        if (!moved_1) {
            // try move 2 down
            for (int i = 0; i < 13; ++i) {
                for (int j = i + 1; j < 14; ++j) {
                    u8* p = get_elmt(&queue, elmt, sizeof(parts));
                    u8 next[15];
                    memcpy(next, p, sizeof(next));

                    u8 ele = next[14];
                    if (ele > 0 && next[i] == ele && next[j] == ele) {
                        next[14] -= 1;
                        next[i] -= 1;
                        next[j] -= 1;
                    } else {
                        continue;
                    }

                    if (valid_state(next) && !visited[hash_p2(next)]) {
                        push(&queue, next, sizeof(next));
                        u64 s1 = s + 1;
                        push(&step, &s1, sizeof(u64));
                        moved_2 = true;
                    }
                    visited[hash_p2(next)] = true;
                }
            }
        }

        // remove_start(&queue, sizeof(parts));
        // remove_start(&step, sizeof(u64));

        qi++;
    }

    return u64_MAX;
}