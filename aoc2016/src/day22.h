#include "utils.h"

typedef struct Node {
    u16 size;
    u16 used;
    u16 avail;
    u16 perc;
} Node;

u64 p1(FILE* file) {
    Vec buf = with_capacity(64, 1);
    // skip header
    get_line(file, &buf);
    get_line(file, &buf);

    Node nodes[35][27] = { 0 };

    for (int r = 0; r < 35; ++r) {
        for (int c = 0; c < 27; ++c) {
            get_line(file, &buf);
            as_cstr(&buf);
            char* line = (char*) buf.data;
            u8 x, y;
            Node* node = &nodes[r][c];
            sscanf(line, "/dev/grid/node-x%hhu-y%hhu %huT %huT %huT %hu%%", &x, &y, &node->size,
                   &node->used, &node->avail, &node->perc);
        }
    }

    u64 count = 0;

    for (int i = 0; i < 35 * 27; ++i) {
        for (int j = 0; j < 35 * 27; ++j) {
            Node* a = &((Node*) nodes)[i];
            Node* b = &((Node*) nodes)[j];
            if (a->used != 0 && i != j && a->used <= b->avail) {
                count++;
            }
        }
    }

    return count;
}

// just gonna solve this by hand
u64 p2(FILE* file) {
    Vec buf = with_capacity(64, 1);
    // skip header
    get_line(file, &buf);
    get_line(file, &buf);

    Node nodes[35][27] = { 0 };
    u8 empty_pos[2] = {0, 0};

    for (int r = 0; r < 35; ++r) {
        for (int c = 0; c < 27; ++c) {
            get_line(file, &buf);
            as_cstr(&buf);
            char* line = (char*) buf.data;
            u8 x, y;
            Node* node = &nodes[r][c];
            sscanf(line, "/dev/grid/node-x%hhu-y%hhu %huT %huT %huT %hu%%", &x, &y, &node->size,
                   &node->used, &node->avail, &node->perc);
            if (node->perc == 0) {
                empty_pos[0] = c;
                empty_pos[1] = r;
            }
        }
    }

    for (int r = 0; r < 35; ++r) {
        for (int c = 0; c < 27; ++c) {
            printf("|");
            if (r == 0 && c == 0) {
                printf("  H  ");
            } else if (r == 34 && c == 0) {
                printf("  G  ");
            } else if (r == empty_pos[1] && c == empty_pos[0]) {
                printf("(0/%hu)", nodes[r][c].size);
            } else {
                printf("%hu/%hu", nodes[r][c].used, nodes[r][c].size);
            }
        }
        printf("|\n");
    }


    return 0;
}