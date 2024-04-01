#include "utils.h"

u64 p1(FILE* file) {
    const u32 MAX_LEN = 272;

    Vec a = get_vec(1);
    Vec b = get_vec(1);
    get_line(file, &a);

    // print_vec(&a);

    for (int i = 0; i < a.len; ++i) {
        a.data[i] -= '0';
    }
    // print_vec(&a);

    while (a.len < MAX_LEN) {
        b.len = 0;
        for (int i = 1; i <= a.len; ++i) {
            push_byte(&b, a.data[a.len - i] ^ 1);
        }
        push_byte(&a, 0);
        append_vec(&a, &b);
        // print_vec(&a);
    }
    // truncate unnecessary values
    a.len = MAX_LEN;

    do {
        // b will now house the checksum
        b.len = 0;
        for (int i = 0; i < a.len; i += 2) {
            push_byte(&b, (a.data[i] ^ a.data[i + 1]) == 0);
        }
        a.len = 0;
        append_vec(&a, &b);
    } while ((b.len & 1) == 0);

    print_vec(&b);
    return 0;
}

u64 p2(FILE* file) {
    const u32 MAX_LEN = 35651584;

    Vec a = get_vec(1);
    Vec b = get_vec(1);
    get_line(file, &a);

    // print_vec(&a);

    for (int i = 0; i < a.len; ++i) {
        a.data[i] -= '0';
    }
    // print_vec(&a);

    while (a.len < MAX_LEN) {
        b.len = 0;
        for (int i = 1; i <= a.len; ++i) {
            push_byte(&b, a.data[a.len - i] ^ 1);
        }
        push_byte(&a, 0);
        append_vec(&a, &b);
        // print_vec(&a);
    }
    // truncate unnecessary values
    a.len = MAX_LEN;

    do {
        // b will now house the checksum
        b.len = 0;
        for (int i = 0; i < a.len; i += 2) {
            push_byte(&b, (a.data[i] ^ a.data[i + 1]) == 0);
        }
        a.len = 0;
        append_vec(&a, &b);
    } while ((b.len & 1) == 0);

    print_vec(&b);
    return 0;
}