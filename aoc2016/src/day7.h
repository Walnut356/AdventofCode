#include "utils.h"

u64 p1(FILE* file) {
    u64 result = 0;
    Vec buf = get_vec(1);

    while (get_line(file, &buf)) {
        char* line = (char*) buf.data;
        bool inner = false;
        bool valid = false;
        // manual `windows()`
        for (int i = 0; i < buf.len - 3; ++i) {
            if (line[i] == '[') {
                inner = true;
                continue;
            } else if (line[i] == ']') {
                inner = false;
                continue;
            }

            if (line[i] == line[i + 3] && line[i + 1] == line[i + 2] && line[i] != line[i + 1]) {
                if (inner) {
                    valid = false;
                    break;
                }

                valid = true;
            }
        }

        result += valid ? 1 : 0;
    }

    return result;
}

u64 p2(FILE* file) {
    u64 result = 0;
    Vec buf = get_vec(1);

    while (get_line(file, &buf)) {
        char* line = (char*) buf.data;
        bool inner = false;
        bool valid = false;
        // manual `windows()`
        for (int i = 0; i < buf.len - 2; ++i) {
            if (line[i + 1] == '[' || line[i + 1] == ']' || line[i + 2] == '[' ||
                line[i + 2] == ']')
            {
                continue;
            }
            if (line[i] == '[') {
                inner = true;
                continue;
            } else if (line[i] == ']') {
                inner = false;
                continue;
            }

            if (inner) {
                if (line[i] == line[i + 2] && line[i] != line[i + 1]) {
                    u8 needle[3] = { line[i + 1], line[i], line[i + 1] };
                    bool in = false;
                    for (int j = 0; j < buf.len - 2; ++j) {
                        if (line[j] == '[') {
                            in = true;
                            continue;
                        } else if (line[j] == ']') {
                            in = false;
                            continue;
                        }

                        if (!in) {
                            if (memcmp(&line[j], needle, 3) == 0) {
                                valid = true;
                                break;
                            }
                        }
                    }
                    if (valid) {
                        break;
                    }
                }
            }
        }

        result += valid ? 1 : 0;
    }

    return result;
}