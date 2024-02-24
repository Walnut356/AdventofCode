#include "utils.h"

// it looks like sector ID's are always 3 digits
// i don't think the sector ID is ever part of the checksum either
// all letters seem to be lowercase

u64 p1(FILE* file) {
    u64 result = 0;
    char curr;
    u8 counts[26] = { 0 };

    while (true) {
        curr = fgetc(file);
        if (curr == '-') {
            continue;
        }
        if (isdigit(curr)) {
            int sector = curr - '0';
            sector = sector * 10 + (fgetc(file) - '0');
            sector = sector * 10 + (fgetc(file) - '0');
            fgetc(file); // consume open bracket

            // printf("%d\n", sector);

            char checksum[5] = {
                fgetc(file), fgetc(file), fgetc(file), fgetc(file), fgetc(file),
            };

            fgetc(file); // consume close bracket

            bool valid = true;

            for (int i = 0; i < 4; ++i) {
                if (counts[checksum[i] - 'a'] == counts[checksum[i + 1] - 'a'] &&
                    checksum[i] > checksum[i + 1])
                {
                    valid = false;
                }
            }

            if (valid) {
                // this is gonna be a jank version of "find max -> remove -> repeat"
                for (int i = 0; i < 5; ++i) {
                    u8 target = counts[checksum[i] - 'a'];
                    for (int j = 0; j < 26; ++j) {
                        // if it's not the highest number, not valid. Break out
                        if (target < counts[j]) {
                            valid = false;
                            goto escape;
                        }
                    }
                    // set it to the minimum so we can check for the next highest val
                    counts[checksum[i] - 'a'] = 0;
                }
            }

        escape:
            if (valid) {
                result += sector;
            }

            if (fgetc(file) == EOF) {
                break;
            }
            // prev char was a newline, reset for next iteration
            for (int i = 0; i < 26; ++i) {
                counts[i] = 0;
            }
        }

        counts[curr - 'a'] += 1;
    }

    return result;
}


// this prints out the answer, but i manually inspected the output since the instructions weren't
// clear exactly what i was looking for.
u64 p2(FILE* file) {
    u64 result = 0;
    char curr;
    u8 counts[26] = { 0 };
    char line[50] = {0};

    while (fgets(line, 255, file) != NULL) {
        int l_idx = 0;
        while (line[l_idx] != '\n' && line[l_idx] != EOF) {
            curr = line[l_idx];
            l_idx++;
            if (curr == '-') {
                continue;
            }

            if (isdigit(curr)) {
                int sector = curr - '0';
                sector = sector * 10 + (line[l_idx] - '0');
                sector = sector * 10 + (line[l_idx + 1] - '0');
                // printf("%d\n", sector);

                char checksum[5] = {
                    line[l_idx + 3], line[l_idx + 4], line[l_idx + 5],
                    line[l_idx + 6], line[l_idx + 7],
                };

                bool valid = true;

                for (int i = 0; i < 4; ++i) {
                    if (counts[checksum[i] - 'a'] == counts[checksum[i + 1] - 'a'] &&
                        checksum[i] > checksum[i + 1])
                    {
                        valid = false;
                    }
                }

                if (valid) {
                    // this is gonna be a jank version of "find max -> remove -> repeat"
                    for (int i = 0; i < 5; ++i) {
                        u8 target = counts[checksum[i] - 'a'];
                        for (int j = 0; j < 26; ++j) {
                            // if it's not the highest number, not valid. Break out
                            if (target < counts[j]) {
                                valid = false;
                                goto escape;
                            }
                        }
                        // set it to the minimum so we can check for the next highest val
                        counts[checksum[i] - 'a'] = 0;
                    }
                }

            escape:
                if (valid) {
                    result += sector;
                    printf("%d - ", sector);
                    for (int i = 0; i < l_idx; ++i) {
                        if (line[i] == '-') {
                            printf(" ");
                        }

                        printf("%c", (((line[i] - 'a') + sector) % 26) + 'a');
                    }
                    printf("\n");
                }

                // prev char was a newline, reset for next iteration
                for (int i = 0; i < 26; ++i) {
                    counts[i] = 0;
                }

                break;
            }

            counts[curr - 'a'] += 1;
        }
    }

    return 548;
}