#include "utils.h"
#include <stdio.h>

typedef struct Range {
    u32 low;
    u32 high;
} Range;

int cmp_range(const void* a, const void* b) {
    u32 l = ((Range*) a)->low;
    u32 r = ((Range*) b)->low;
    if (l < r) {
        return -1;
    }
    if (l > r) {
        return 1;
    }

    return 0;
}

u64 p1(FILE* file) {
    Vec buf = get_vec(1);
    Vec ranges = get_vec(sizeof(Range));

    // int q = 0;

    while (get_line(file, &buf)) {
        as_cstr(&buf);
        Range r;
        sscanf(buf.data, "%u-%u", &r.low, &r.high);

        push(&ranges, &r, sizeof(Range));
        // q++;
        // printf("%u-%u\n", r.low, r.high);
    }
    // printf("%d", q);

    qsort(&ranges.data[0], len(&ranges, sizeof(Range)), sizeof(Range), &cmp_range);

    u32 min = 0;

    for (int i = 0; i < len(&ranges, sizeof(Range)); i++) {
        Range* r = (Range*) get_elmt(&ranges, i, sizeof(Range));
        if (r->low <= min && r->high >= min) {
            min = r->high + 1;
        }
    }

    return min;
}

u64 p2(FILE* file) {
    Vec buf = get_vec(1);
    Vec ranges = get_vec(sizeof(Range));

    while (get_line(file, &buf)) {
        as_cstr(&buf);
        Range r;
        sscanf(buf.data, "%u-%u", &r.low, &r.high);

        push(&ranges, &r, sizeof(Range));
    }

    push(&ranges, &(Range){u32_MAX, u32_MAX}, sizeof(Range));

    qsort(&ranges.data[0], len(&ranges, sizeof(Range)), sizeof(Range), &cmp_range);

    u32 result = u32_MAX;
    u32 start = 0;
    u32 end = 0;

    for (int i = 0; i < len(&ranges, sizeof(Range)); i++) {
        Range* r = (Range*) get_elmt(&ranges, i, sizeof(Range));

        // if the range "extends" the current range
        if (r->low <= end + 1 && r->high >= end) {
            end = r->high;
            // if there's a gap, reset and update result
        } else if (r->low > end) {
            result -= (end - start) + 1;
            start = r->low;
            end = r->high;
        }
        // otherwise, the range is completely contained by what we have and we don't do anything
    }
    result -= (end - start);

    return result;
}