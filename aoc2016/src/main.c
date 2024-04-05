#include "day21.h"
#include "utils.h"
#include <stdio.h>

const u8 day = 21;

void run_part(u64 (*fn)(FILE*), u8 part) {
    FILE* file;
    // easiest way to get the correct size
    char path[37] = {0};
    sprintf(path, "../aoc2016/test_data/day%d.txt", day);
    int code = fopen_s(&file, path, "r");

    if (code != 0) {
        sprintf(path, "../../aoc2016/test_data/day%d.txt", day);
        int code = fopen_s(&file, path, "r");
        if (code != 0) {
            printf("Can't open file. Error code: %d\n", code);
            return;
        }
    }

    double now = time_now();

    i64 result = fn(file);

    double dur = time_now();
    print_result(result, dur - now, day, part);

    // I know this isn't strictly speaking necessary for something so small, but I figure I may as
    // well get in the habit
    fclose(file);
}

int main() {
    run_part(p1, 1);
    run_part(p2, 2);
}
