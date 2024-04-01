#include "utils.h"

// woo linked lists!
typedef struct Elf {
    u32 num;
    void* next;
} Elf;

u64 p1(FILE* file) {
    u32 elf_count;
    fscanf(file, "%u", &elf_count);

    Elf* head = malloc(sizeof(Elf));
    Elf* prev = head;
    for (int i = 1; i < elf_count; ++i) {
        Elf* curr = malloc(sizeof(Elf));
        prev->num = i;
        prev->next = curr;
        prev = curr;
    }
    // close the loop
    prev->next = head;

    Elf* curr = head;
    while (curr->next != curr) {
        Elf* temp = curr->next;
        curr->next = temp->next;
        free(temp);
        curr = curr->next;
    }

    return curr->num;
}

u64 p2(FILE* file) {
    u32 elf_count;
    fscanf(file, "%u", &elf_count);

    // hey wait a minute...
    u32 i = 1;
    while (i * 3 < elf_count) {
        i *= 3;
    }

    return elf_count - i;
}