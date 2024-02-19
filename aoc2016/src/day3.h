#include "utils.h"
#include <stdio.h>

u64 p1(FILE* file) {
    u32 side1;
    u32 side2;
    u32 side3;

    u32 result = 0;

    while (true) {
        fscanf_s(file, "%d", &side1);
        fscanf_s(file, "%d", &side2);
        fscanf_s(file, "%d", &side3);

        if (fgetc(file) == EOF) {
            break;
        }
        if ((side1 + side2 > side3) && (side2 + side3 > side1) && (side3 + side1 > side2)) {
            result += 1;
            continue;
        }
    }

    return result;
}

bool test(u32 t[3]) {
    if ((t[0] + t[1] > t[2]) && (t[1] + t[2] > t[0]) && (t[2] + t[0] > t[1])) {
        return true;
    }
    return false;
}

u64 p2(FILE* file) {
    u32 t1[3];
    u32 t2[3];
    u32 t3[3];

    u32 result = 0;

    do {

        fscanf_s(file, "%d", &t1[0]);
        fscanf_s(file, "%d", &t2[0]);
        fscanf_s(file, "%d", &t3[0]);
        fscanf_s(file, "%d", &t1[1]);
        fscanf_s(file, "%d", &t2[1]);
        fscanf_s(file, "%d", &t3[1]);
        fscanf_s(file, "%d", &t1[2]);
        fscanf_s(file, "%d", &t2[2]);
        fscanf_s(file, "%d", &t3[2]);

        if (test(t1)) {
            result += 1;
        }
        if (test(t2)) {
            result += 1;
        }
        if (test(t3)) {
            result += 1;
        }
    } while (fgetc(file) != EOF);

    return result;
}