#include "md5.h"
#include "utils.h"

typedef struct Key {
    u64 idx;
    u8 val;
} Key;

bool contains_triple(u8 hash[16], u8* out) {
    // essentially `windows(2)`
    for (int i = 0; i < 15; ++i) {
        u8 v1 = (hash[i] & 0xF0) >> 4;
        u8 v2 = hash[i] & 0x0F;
        u8 v3 = (hash[i + 1] & 0xF0) >> 4;
        u8 v4 = hash[i + 1] & 0x0F;
        if ((v1 == v2 && v2 == v3) || (v2 == v3 && v3 == v4)) {
            *out = v2;
            return true;
        }
    }

    return false;
}

bool contains_five(u8 hash[16], u8* out) {
    // essentially `windows(3)`
    for (int i = 0; i < 14; ++i) {
        u8 v1 = (hash[i] & 0xF0) >> 4;
        u8 v2 = hash[i] & 0x0F;
        u8 v3 = (hash[i + 1] & 0xF0) >> 4;
        u8 v4 = hash[i + 1] & 0x0F;
        u8 v5 = (hash[i + 2] & 0xF0) >> 4;
        u8 v6 = hash[i + 2] & 0x0F;
        if ((v1 == v2 && v2 == v3 && v3 == v4 && v4 == v5) ||
            (v2 == v3 && v3 == v4 && v4 == v5 && v5 == v6))
        {
            *out = v2;
            return true;
        }
    }

    return false;
}

void print_hex(u8* hash) {
    for (int i = 0; i < 16; ++i) {
        printf("%02x", hash[i]);
    }
    // printf("\n");
}

u64 p1(FILE* file) {
    Vec buf = get_vec(1);
    get_line(file, &buf);
    as_cstr(&buf);

    Vec trip_q = get_vec(sizeof(Key));
    // Vec out_v = get_vec(1);

    u8 count = 0;
    bool add_more = true;
    u64 idx = 0;
    u64 final = 0;
    while (count < 64 || len(&trip_q, sizeof(Key))) {
        char val[256];
        u8 hash[17] = { 0 };
        sprintf(val, "%s%llu", buf.data, idx);
        md5String(val, hash);


        // printf("%llu\n", len(&trip_q, sizeof(Key)));

        // quick cleanup for "expired" triples
        loop_1:
            for (int i = 0; i < len(&trip_q, sizeof(Key)); ++i) {
                Key* elmt = (Key*) get_elmt(&trip_q, i, sizeof(Key));
                if (((idx - (elmt->idx + 1)) > 1000)) {
                    swap_remove(&trip_q, i, sizeof(Key));
                    goto loop_1;
                }
            }

        // the character of the triple/five
        u8 out;

        if (contains_five(hash, &out)) {
            // printf("%llu: ", idx);
            // print_hex(hash);
            // printf("\n");
            loop_2:
            for (int i = 0; i < len(&trip_q, sizeof(Key)); ++i) {
                Key* elmt = (Key*) get_elmt(&trip_q, i, sizeof(Key));
                if (elmt->val == out) {
                    count += 1;
                    // printf("[%hhu] %llu: ", count, elmt->idx);
                    // sprintf(val, "%s%llu", buf.data, elmt->idx);
                    // md5String(val, hash);
                    // print_hex(hash);
                    // printf(" -> %llu: ", idx);
                    // sprintf(val, "%s%llu", buf.data, idx);
                    // md5String(val, hash);
                    // print_hex(hash);
                    // printf("(%llu)\n", final);
                    final = max(final, elmt->idx);
                    if (count >= 64) {
                        // return elmt->idx;
                        add_more = false;

                    }
                    swap_remove(&trip_q, i, sizeof(Key));
                    goto loop_2;

                }
            }
        }

    label_1:
        if (contains_triple(hash, &out) && add_more) {
            // print_hex(hash);
            Key k = { idx, out };
            push(&trip_q, &k, sizeof(Key));
        }

        idx += 1;
    }

    return final;
}

void as_hex(u8 hash[16], u8 dest[33]) {
    // absolute abomination, i know. I just don't want to deal with this dynamically.
    sprintf(dest, "%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x", hash[0],
            hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7], hash[8], hash[9],
            hash[10], hash[11], hash[12], hash[13], hash[14], hash[15]);
}

bool hex_triple(u8 hex[33], u8* out) {
    // 33 - 1 (null byte), - 2 for the window
    for (int i = 0; i < 30; ++i) {
        if ((hex[i] == hex[i + 1]) && (hex[i + 1] == hex[i + 2])) {
            *out = hex[i];
            return true;
        }
    }

    return false;
}

bool hex_five(u8 hex[33], u8* out) {
    // 33 - 1 (null byte), - 4 for the window
    for (int i = 0; i < 28; ++i) {
        if ((hex[i] == hex[i + 1]) && (hex[i + 1] == hex[i + 2]) && (hex[i + 2]) == (hex[i + 3]) &&
            (hex[i + 3]) == (hex[i + 4]))
        {
            *out = hex[i];
            return true;
        }
    }

    return false;
}

// unoptimized version used for debugging
u64 _p1(FILE* file) {
    Vec buf = get_vec(1);
    get_line(file, &buf);
    as_cstr(&buf);

    Vec trip_q = get_vec(sizeof(Key));
    // Vec out_v = get_vec(1);

    u8 count = 0;
    bool add_more = true;
    u64 idx = 0;
    u64 final = u64_MAX;
    while (count < 64 || len(&trip_q, sizeof(Key))) {
        char val[256];
        u8 hash[17] = { 0 };
        u8 hex[33];
        sprintf(val, "%s%llu", buf.data, idx);
        md5String(val, hash);
        as_hex(hash, hex);
        u8 out;

        if (hex_triple(hex, &out) && add_more) {
            // print_hex(hash);
            for (u64 i = idx + 1; i < idx + 1001; ++i) {
                sprintf(val, "%s%llu", buf.data, i);
                md5String(val, hash);
                as_hex(hash, hex);
                u8 temp;
                if (hex_five(hex, &temp) && out == temp) {
                    count += 1;
                    printf("[%hhu] %llu: ", count, idx);
                    sprintf(val, "%s%llu", buf.data, idx);
                    md5String(val, hash);
                    print_hex(hash);
                    printf(" -> %llu: ", i);
                    sprintf(val, "%s%llu", buf.data, i);
                    md5String(val, hash);
                    print_hex(hash);
                    printf("(%llu)\n", final);
                    if (count == 64) {

                        return idx;
                    }
                }
            }
        }

        idx += 1;
    }

    return final;
}

int cmp(const void* a, const void* b) {
    u64 l = *((u64*) a);
    u64 r = *((u64*) b);
    if (l < r) {
        return -1;
    }
    if (l > r) {
        return 1;
    }

    return 0;
}

u64 p2(FILE* file) {
    Vec buf = get_vec(1);
    get_line(file, &buf);
    as_cstr(&buf);

    Vec trip_q = get_vec(sizeof(Key));
    // Vec out_v = get_vec(1);
    Vec indexes = get_vec(sizeof(u64));

    u8 count = 0;
    bool add_more = true;
    u64 idx = 0;
    while (count < 64 || len(&trip_q, sizeof(Key))) {
        char val[256];
        u8 hash[17] = { 0 };
        u8 hex[33] = { 0 };
        u8 temp[33] = {0};
        sprintf(val, "%s%llu", buf.data, idx);
        md5String(val, hash);
        as_hex(hash, hex);
        u8* h = hex;
        u8* t = temp;
        for (int i = 0; i < 2016; ++i) {
            md5String(h, t);
            as_hex(t, h);
        }


    // printf("%llu\n", len(&trip_q, sizeof(Key)));

    // quick cleanup for "expired" triples
    loop_1:
        for (int i = 0; i < len(&trip_q, sizeof(Key)); ++i) {
            Key* elmt = (Key*) get_elmt(&trip_q, i, sizeof(Key));
            if (((idx - (elmt->idx + 1)) > 1000)) {
                swap_remove(&trip_q, i, sizeof(Key));
                goto loop_1;
            }
        }

        // the character of the triple/five
        u8 out;

        if (hex_five(hex, &out)) {
        // printf("%llu: ", idx);
        // print_hex(hash);
        // printf("\n");
        loop_2:
            for (int i = 0; i < len(&trip_q, sizeof(Key)); ++i) {
                Key* elmt = (Key*) get_elmt(&trip_q, i, sizeof(Key));
                if (elmt->val == out) {
                    count += 1;
                    printf("[%hhu] %llu", count, elmt->idx);
                    printf(" -> %llu\n", idx);

                    push(&indexes, &elmt->idx, sizeof(u64));
                    if (count >= 64) {
                        add_more = false;
                    }
                    swap_remove(&trip_q, i, sizeof(Key));
                    goto loop_2;
                }
            }
        }

    label_1:
        if (hex_triple(hex, &out) && add_more) {
            // print_hex(hash);
            Key k = { idx, out };
            push(&trip_q, &k, sizeof(Key));
        }

        idx += 1;
    }

    qsort(indexes.data, len(&indexes, sizeof(u64)), sizeof(u64), cmp);

    return *(u64*)get_elmt(&indexes, 63, sizeof(u64));
}