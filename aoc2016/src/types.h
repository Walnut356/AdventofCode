#pragma once

#include <limits.h>
#include <stdint.h>

// sorry c devs, rust names are just better
typedef unsigned char u8;
typedef signed char i8;
typedef uint16_t u16;
typedef int16_t i16;
typedef uint32_t u32;
typedef int32_t i32;
typedef uint64_t u64;
typedef int64_t i64;

const u8 u8_MAX = UCHAR_MAX;
const i8 i8_MAX = CHAR_MAX;
const i8 i8_MIN = CHAR_MIN;

const u16 u16_MAX = USHRT_MAX;
const i16 i16_MAX = SHRT_MAX;
const i16 i16_MIN = SHRT_MIN;

const u32 u32_MAX = UINT_MAX;
const i32 i32_MAX = INT_MAX;
const i32 i32_MIN = INT_MIN;

const u64 u64_MAX = ULONG_MAX;
const i64 i64_MAX = LONG_MAX;
const i64 i64_MIN = LONG_MIN;

typedef float f32;
typedef double f64;
