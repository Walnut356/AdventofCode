#include "day1.h"
#include "utils.h"
#include <stdio.h>
#include <time.h>

void run_part(u64 (*fn)(FILE*)) {
    FILE* file;
    fopen_s(&file, "G:/coding/My Projects/Learning/AdventofCode/aoc/aoc2016/test_data/day1.txt",
            "r");

    double now = time_now();

    u64 result = fn(file);

    double dur = time_now();
    print_result(result, dur - now);
}

int main() {
    run_part(d1p1);
    run_part(d1p2);
}
