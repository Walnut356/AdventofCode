#include "utils.h"
#include <stdio.h>

typedef struct Bot {
    i8 hi;
    i8 lo;
} Bot;

Bot make_bot() {
    return (Bot){ -1, -1 };
}

typedef struct Instr {
    u8 hi_ind;
    bool hi_bin;
    u8 lo_ind;
    bool lo_bin;
} Instr;

void insert_chip(Bot* b, i8 chip) {
    if (b->hi > chip) {
        b->lo = chip;
    } else {
        b->lo = b->hi;
        b->hi = chip;
    }
}

bool has_two(Bot* b) {
    return b->hi >= 0 && b->lo >= 0;
}

u64 p1(FILE* file) {
    Bot needle = {61, 17};

    Bot bots[255];
    for (u32 i = 0; i < 255; ++i) {
        bots[i] = make_bot();
    }
    u32 bins[255] = { 0 };
    // indexed by the bot executing the instruction
    Instr instrs[255] = { 0 };
    Vec buf = get_vec(1);
    // stack queue of the bots we know have 2 chips
    Vec two_stack = get_vec(1);

    while (get_line(file, &buf)) {
        as_cstr(&buf);
        char* line = (char*) buf.data;

        if (line[0] == 'v') {
            i8 val;
            u8 dest;

            sscanf(line, "value %hhd goes to bot %hhu", &val, &dest);
            insert_chip(&bots[dest], val);
            if (has_two(&bots[dest])) {
                push(&two_stack, &dest, 1);
                if (*((u16*) &bots[dest]) == *((u16*) &needle)) {
                    return dest;
                }
            }
        } else {
            u8 bot_id;
            char lo_bin[7] = { 0 };
            char hi_bin[7] = { 0 };
            u8 out_lo;
            u8 out_hi;
            sscanf(line, "bot %hhu gives low to %s %hhd and high to %s %hhd", &bot_id, lo_bin,
                   &out_lo, hi_bin, &out_hi);

            instrs[bot_id] = (Instr){
                out_hi,
                hi_bin[0] == 'o',
                out_lo,
                lo_bin[0] == 'o',
            };
        }
    }

    for (;;) {
        u8 bot_id;
        if (two_stack.len != 0) {
            pop(&two_stack, &bot_id, 1);
        } else {
            for (u32 i = 0; i < 255; i++) {
                if (has_two(&bots[i])) {
                    bot_id = i;
                    break;
                }
            }
        }

        Bot* bot = &bots[bot_id];
        Instr* instr = &instrs[bot_id];

        if (instr->lo_bin) {
            bins[instr->lo_ind] = bot->lo;
        } else {
            insert_chip(&bots[instr->lo_ind], bot->lo);
            if (has_two(&bots[instr->lo_ind])) {
                push(&two_stack, &instr->lo_ind, 1);
                if (*((u16*) &bots[instr->lo_ind]) == *((u16*) &needle)) {
                    return instr->lo_ind;
                }
            }
        }

        bot->lo = -1;

        if (instr->hi_bin) {
            bins[instr->hi_ind] = bot->hi;
        } else {
            insert_chip(&bots[instr->hi_ind], bot->hi);
            if (has_two(&bots[instr->hi_ind])) {
                push(&two_stack, &instr->hi_ind, 1);
                if (*((u16*)&bots[instr->hi_ind]) == *((u16*)&needle)) {
                    return instr->hi_ind;
                }
            }
        }

        bot->hi = -1;
    }
}

u64 p2(FILE* file) {
    Bot bots[255];
    for (u32 i = 0; i < 255; ++i) {
        bots[i] = make_bot();
    }
    u32 bins[255] = { 0 };
    // indexed by the bot executing the instruction
    Instr instrs[255] = { 0 };
    Vec buf = get_vec(1);
    // stack queue of the bots we know have 2 chips
    Vec two_stack = get_vec(1);

    while (get_line(file, &buf)) {
        as_cstr(&buf);
        char* line = (char*) buf.data;

        if (line[0] == 'v') {
            i8 val;
            u8 dest;

            sscanf(line, "value %hhd goes to bot %hhu", &val, &dest);
            insert_chip(&bots[dest], val);
            if (has_two(&bots[dest])) {
                push(&two_stack, &dest, 1);
            }
        } else {
            u8 bot_id;
            char lo_bin[7] = { 0 };
            char hi_bin[7] = { 0 };
            u8 out_lo;
            u8 out_hi;
            sscanf(line, "bot %hhu gives low to %s %hhd and high to %s %hhd", &bot_id, lo_bin,
                   &out_lo, hi_bin, &out_hi);

            instrs[bot_id] = (Instr){
                out_hi,
                hi_bin[0] == 'o',
                out_lo,
                lo_bin[0] == 'o',
            };
        }
    }

    for (;;) {
        if (bins[0] != 0 && bins[1] != 0 && bins[2] != 0) {
            break;
        }
        u8 bot_id;
        if (two_stack.len != 0) {
            pop(&two_stack, &bot_id, 1);
        } else {
            for (u32 i = 0; i < 255; i++) {
                if (has_two(&bots[i])) {
                    bot_id = i;
                    break;
                }
            }
        }

        Bot* bot = &bots[bot_id];
        Instr* instr = &instrs[bot_id];

        if (instr->lo_bin) {
            bins[instr->lo_ind] = bot->lo;
        } else {
            insert_chip(&bots[instr->lo_ind], bot->lo);
            if (has_two(&bots[instr->lo_ind])) {
                push(&two_stack, &instr->lo_ind, 1);
            }
        }

        bot->lo = -1;

        if (instr->hi_bin) {
            bins[instr->hi_ind] = bot->hi;
        } else {
            insert_chip(&bots[instr->hi_ind], bot->hi);
            if (has_two(&bots[instr->hi_ind])) {
                push(&two_stack, &instr->hi_ind, 1);
            }
        }

        bot->hi = -1;
    }

    return (u64)bins[0] * (u64)bins[1] * (u64)bins[2];
}