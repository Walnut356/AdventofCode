#include "utils.h"

typedef struct Disc {
    u8 start;
    u8 positions;
} Disc;

u64 p1(FILE* file) {
    Disc discs[6];

    for (int i = 0; i < 6; ++i) {
        u8 num;
        u8 pos;
        u8 start;
        fscanf(file, "Disc #%hhu has %hhu positions; at time=0, it is at position %hhu. ", &num,
               &pos, &start);
        discs[i] = (Disc){ start, pos };
    }

    u64 time = (discs[0].positions - 1) - discs[0].start;
    while (true) {
        bool win = true;
        for (int i = 1; i < 6; ++i) {
            if ((discs[i].start + time + i + 1) % discs[i].positions != 0) {
                goto cont;
            }
        }
        return time;
    cont:
        time += discs[0].positions;
    }
}

u64 p2(FILE* file) {
    Disc discs[7];

    for (int i = 0; i < 6; ++i) {
        u8 num;
        u8 pos;
        u8 start;
        fscanf(file, "Disc #%hhu has %hhu positions; at time=0, it is at position %hhu. ", &num,
               &pos, &start);
        discs[i] = (Disc){ start, pos };
    }

    discs[6] = (Disc) {0, 11};

    u64 time = (discs[0].positions - 1) - discs[0].start;
    while (true) {
        bool win = true;
        for (int i = 1; i < 7; ++i) {
            if ((discs[i].start + time + i + 1) % discs[i].positions != 0) {
                goto cont;
            }
        }
        return time;
    cont:
        time += discs[0].positions;
    }
}