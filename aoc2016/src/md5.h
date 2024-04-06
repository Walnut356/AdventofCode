// I'm not super keen on using 3rd party libraries, but having to make my own md5 implementation
// for the handful of problems that use it seems like a waste when that doesn't seem like the
// purpose of those problems in the first place. It's a concession, but a small one i think.

// credits to: https://github.com/Zunawe/md5-c/blob/main/md5.h

#ifndef MD5_H
#define MD5_H

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


typedef struct {
    uint64_t size;      // Size of input in bytes
    uint32_t buffer[4]; // Current accumulation of hash
    uint8_t input[64];  // Input to be used in the next step
    uint8_t digest[16]; // Result of algorithm
} MD5Context;

void md5Init(MD5Context* ctx);
void md5Update(MD5Context* ctx, uint8_t* input, size_t input_len);
void md5Finalize(MD5Context* ctx);
void md5Step(uint32_t* buffer, uint32_t* input);

void md5String(char* input, uint8_t* result);
void md5File(FILE* file, uint8_t* result);

#endif