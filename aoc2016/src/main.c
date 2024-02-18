#include "day1.h"
#include "utils.h"
#include <stdio.h>
#include <time.h>

void run_part(u64 (*fn)(FILE*)) {
    FILE* file;
    int code = fopen_s(&file, "./aoc2016/test_data/day1.txt",
            "r");

    if (code != 0) {
        return;
    }

    double now = time_now();

    u64 result = fn(file);

    double dur = time_now();
    print_result(result, dur - now);
}

int main() {
    run_part(p1);
    run_part(p2);
}
