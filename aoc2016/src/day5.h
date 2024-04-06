#include "utils.h"
#include "md5.h"

u64 p1(FILE* file) {
    Vec buf = get_vec(1);
    get_line(file, &buf);
    as_cstr(&buf);

    u8 count = 0;
    u64 i = 0;
    while (count < 8) {
        char val[256];
        u8 hash[17] = {0};
        do {

            sprintf(val, "%s%llu", buf.data, i);
            md5String(val, hash);
            i += 1;
        } while (!(hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0));
        count += 1;

        printf("%x", hash[2]);
    }

    printf("\n");
    return 0;
}

u64 p2(FILE* file) {
    Vec buf = get_vec(1);
    get_line(file, &buf);
    as_cstr(&buf);

    u8 result[8] = { 255, 255, 255, 255, 255, 255, 255, 255 };
    u8 count = 0;
    u64 i = 0;
    while (count < 8) {
        char val[256];
        u8 hash[17] = { 0 };
        do {
            sprintf(val, "%s%llu", buf.data, i);
            md5String(val, hash);
            i += 1;
        } while (!(hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xF0) == 0 && (hash[2] & 0x0F) < 8));
        if (result[hash[2] & 0x0F] == 255) {
            result[hash[2] & 0x0F] = (hash[3] & 0xF0) >> 4;
            count += 1;
        }
    }

    for (int i = 0; i < 8; ++i) {
        printf("%x", result[i]);
    }
    printf("\n");
    return 0;
}