#include "utils.h"

void swap_position(Vec* vec, u8 x, u8 y) {
    u8 temp = vec->data[x];
    vec->data[x] = vec->data[y];
    vec->data[y] = temp;
}

void swap_letter(Vec* vec, u8 x, u8 y) {
    for (int i = 0; i < vec->len; ++i) {
        if (vec->data[i] == x) {
            vec->data[i] = y;
        } else if (vec->data[i] == y) {
            vec->data[i] = x;
        }
    }
}

void rotate_left(Vec* vec, Vec* temp, u8 steps) {
    // put the data from vec directly into temp at the correct rotated index
    for (int i = 0; i < vec->len; ++i) {
        int pos = i - steps;
        if (pos < 0) {
            pos = vec->len + pos;
        }
        temp->data[pos] = vec->data[i];
    }

    // swap vec and temp's data buffers
    u8* t = temp->data;
    temp->data = vec->data;
    vec->data = t;
}

void rotate_right(Vec* vec, Vec* temp, u8 steps) {
    // put the data from vec directly into temp at the correct rotated index
    for (int i = 0; i < vec->len; ++i) {
        int pos = (i + steps) % vec->len;
        temp->data[pos] = vec->data[i];
    }

    // swap vec and temp's data buffers
    u8* t = temp->data;
    temp->data = vec->data;
    vec->data = t;
}

void rotate_idx(Vec* vec, Vec* temp, u8 x) {
    u8 idx = 0;
    for (int i = 0; i < vec->len; ++i) {
        if (vec->data[i] == x) {
            idx = i;
            break;
        }
    }
    if (idx >= 4) {
        idx++;
    }

    rotate_right(vec, temp, idx + 1);
}

void reverse(Vec* vec, u8 x, u8 y) {
    int mid = ((y - x) + 1) / 2;
    for (int i = 0; i < mid; ++i) {
        u8 t = vec->data[x + i];
        vec->data[x + i] = vec->data[y - i];
        vec->data[y - i] = t;
    }
}

void move(Vec* vec, Vec* temp, u8 x, u8 y) {
    if (x < y) {
        int i = 0;
        while (i < x) {
            temp->data[i] = vec->data[i];
            ++i;
        }
        while (i < y) {
            temp->data[i] = vec->data[i + 1];
            ++i;
        }
        temp->data[y] = vec->data[x];
        ++i;
        while (i < vec->len) {
            temp->data[i] = vec->data[i];
            ++i;
        }
    } else if (y < x) {
        int i = 0;
        while (i < y) {
            temp->data[i] = vec->data[i];
            ++i;
        }
        temp->data[y] = vec->data[x];
        while (i < x) {
            ++i;
            temp->data[i] = vec->data[i - 1];
        }
        i++;
        while (i < vec->len) {
            temp->data[i] = vec->data[i];
            ++i;
        }
    } else {
        return;
    }

    // swap vec and temp's data buffers
    u8* t = temp->data;
    temp->data = vec->data;
    vec->data = t;
}

void loop(FILE* file, Vec* input, Vec* temp) {
    Vec buf = with_capacity(36, 1);
    // u8 line_num = 1;
    while (get_line(file, &buf)) {
        // printf("%hhu: ", line_num);
        // line_num++;
        char* line = (char*) buf.data;

        if (line[0] == 's') { // swap
            u8 x, y;
            if (line[5] == 'p') {
                sscanf(line, "swap position %hhu with position %hhu", &x, &y);
                swap_position(input, x, y);
            } else {
                sscanf(line, "swap letter %1c with letter %1c", &x, &y);
                swap_letter(input, x, y);
            }
        } else if (line[0] == 'r' && line[1] == 'o') { // rotate
            u8 x;
            if (line[7] == 'l') {
                sscanf(line, "rotate left %hhu steps", &x);
                rotate_left(input, temp, x);
            } else if (line[7] == 'r') {
                sscanf(line, "rotate right %hhu steps", &x);
                rotate_right(input, temp, x);
            } else if (line[7] == 'b') {
                sscanf(line, "rotate based on position of letter %1c", &x);
                rotate_idx(input, temp, x);
            }
        } else if (line[1] == 'e') { // reverse
            u8 x, y;
            sscanf(line, "reverse positions %hhu through %hhu", &x, &y);
            reverse(input, x, y);
        } else { // move
            u8 x, y;
            sscanf(line, "move position %hhu to position %hhu", &x, &y);
            move(input, temp, x, y);
        }
        // printf("%s\n", input.data);
    }
}

u64 p1(FILE* file) {
    Vec input = with_capacity(9, 1);
    push(&input, "abcdefgh", 8);
    as_cstr(&input);
    // used to simplify some of the operations, as they're a bit more annoying to do in-place.
    // SAFETY: the vec is instantiated to the same len/capacity as `input`. It is never read before
    // being filled with data. The len and capacity are never changed by any instruction, so we can
    // freely swap the underlying `.data` elements between the two without worry.
    Vec temp = (Vec){ malloc(input.capacity), input.len, input.capacity };
    as_cstr(&temp);

    loop(file, &input, &temp);

    printf("%s", input.data);
    return 0;
}

void permute(Vec* input, int start, int end, FILE* file, Vec* temp, bool* done) {
    if (*done) {
        return;
    }
    if (start == end) {
        char t[9];
        memcpy(t, input->data, 9);
        loop(file, input, temp);

        rewind(file);
        if (memcmp("fbgdceah", input->data, 8) == 0) {
            printf("%s", t);
            *done = true;
        }
        memcpy(input->data, t, 8);
        return;
    }

    permute(input, start + 1, end, file, temp, done);
    for (int i = start + 1; i < end; i++) {
        if (input->data[start] == input->data[i]) {
            continue;
        }
        u8 s1 = input->data[start];
        u8 s2 = input->data[i];
        input->data[start] = s2;
        input->data[i] = s1;

        permute(input, start + 1, end, file, temp, done);

        input->data[start] = s1;
        input->data[i] = s2;
    }
}

u64 p2(FILE* file) {
    // brute force time!
    Vec input = with_capacity(9, 1);
    push(&input, "abcdefgh", 8);
    as_cstr(&input);
    // used to simplify some of the operations, as they're a bit more annoying to do in-place.
    // SAFETY: the vec is instantiated to the same len/capacity as `input`. It is never read before
    // being filled with data. The len and capacity are never changed by any instruction, so we can
    // freely swap the underlying `.data` elements between the two without worry.
    Vec temp = (Vec){ malloc(input.capacity), input.len, input.capacity };
    as_cstr(&temp);

    bool done = false;
    permute(&input, 0, 8, file, &temp, &done);

    return 0;
}
