#pragma once

#include "types.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct IntEntry {
    i64 key;
    void* value;
} IntEntry;


typedef struct IntMap {
    u32 len;
    u32 capacity;
    IntEntry* data;
} IntMap;