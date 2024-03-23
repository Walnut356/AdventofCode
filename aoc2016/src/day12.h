#include "utils.h"
#include <minwindef.h>
#include <string.h>

typedef enum OpCode {
    // Copy Register to Register
    CRR,
    // Copy Immedaite to Register
    CIR,
    INC,
    DEC,
    // Jump Immediate Immediate
    JII,
    // Jump Immediate Register
    JIR,
    // Jump Register Immediate
    JRI,
} OpCode;

typedef struct Instr {
    OpCode op;
    i32 lhs;
    i32 rhs;
} Instr;

i64 p1(FILE* file) {
    Vec instrs = get_vec(sizeof(Instr));
    i32 reg[4] = { 0 };
    Vec buf = get_vec(1);
    while (get_line(file, &buf)) {
        as_cstr(&buf);
        char* line = (char*) buf.data;

        Instr instr = { 0 };

        switch (line[0]) {
        case 'c': {
            bool neg = false;
            u8 i = 4;
            if (line[i] == '-') {
                neg = true;
                i++;
            }

            if (isdigit(line[i])) {
                instr.op = CIR;
                while (isdigit(line[i])) {
                    instr.lhs = (instr.lhs * 10) + (line[i] - '0');
                    i++;
                }
                if (neg) {
                    instr.lhs *= -1;
                }
            } else {
                instr.op = CRR;
                instr.lhs = line[i] - 'a';
                i++;
            }

            i++;

            instr.rhs = line[i] - 'a';
            break;
        }
        case 'i': {
            instr.op = INC;
            instr.lhs = line[4] - 'a';
            break;
        }
        case 'd': {
            instr.op = DEC;
            instr.lhs = line[4] - 'a';
            break;
        }
        case 'j': {
            bool neg = false;
            u8 i = 4;
            if (line[i] == '-') {
                neg = true;
                i++;
            }

            if (isdigit(line[i])) {
                instr.op = JII;
                while (isdigit(line[i])) {
                    instr.lhs = (instr.lhs * 10) + (line[i] - '0');
                    i++;
                }
                if (neg) {
                    instr.lhs *= -1;
                }
            } else {
                instr.op = JRI;
                instr.lhs = line[i] - 'a';
                i++;
            }

            i++;

            if (line[i] == '-') {
                neg = true;
                i++;
            }

            if (isdigit(line[i])) {
                while (isdigit(line[i])) {
                    instr.rhs = (instr.rhs * 10) + (line[i] - '0');
                    i++;
                }
                if (neg) {
                    instr.rhs *= -1;
                }
            } else {
                if (instr.op == JII) {
                    instr.op = JIR;
                }
                instr.rhs = line[i] - 'a';
            }

            break;
        }
        }

        push(&instrs, &instr, sizeof(Instr));
    }

    u32 ip = 0;

    while (ip < len(&instrs, sizeof(Instr))) {
        Instr* instr = get_elmt(&instrs, ip, sizeof(Instr));

        switch (instr->op) {
        case CRR: {
            reg[instr->rhs] = reg[instr->lhs];
            break;
        }
        case CIR: {
            reg[instr->rhs] = instr->lhs;
            break;
        }
        case INC: {
            reg[instr->lhs] += 1;
            break;
        }
        case DEC: {
            reg[instr->lhs] -= 1;
            break;
        }
        case JII: {
            if (instr->lhs != 0) {
                ip += instr->rhs;
                continue;
            }
            break;
        }
        case JIR: {
            if (instr->lhs != 0) {
                ip += reg[instr->rhs];
                continue;
            }
            break;
        }
        case JRI: {
            if (reg[instr->lhs] != 0) {
                ip += instr->rhs;
                continue;
            }
        } break;
        }
        ip++;
    }

    return reg[0];
}

i64 p2(FILE* file) {
    Vec instrs = get_vec(sizeof(Instr));
    i32 reg[4] = { 0, 0, 1, 0 };
    Vec buf = get_vec(1);
    while (get_line(file, &buf)) {
        as_cstr(&buf);
        char* line = (char*) buf.data;

        Instr instr = { 0 };

        switch (line[0]) {
        case 'c': {
            bool neg = false;
            u8 i = 4;
            if (line[i] == '-') {
                neg = true;
                i++;
            }

            if (isdigit(line[i])) {
                instr.op = CIR;
                while (isdigit(line[i])) {
                    instr.lhs = (instr.lhs * 10) + (line[i] - '0');
                    i++;
                }
                if (neg) {
                    instr.lhs *= -1;
                }
            } else {
                instr.op = CRR;
                instr.lhs = line[i] - 'a';
                i++;
            }

            i++;

            instr.rhs = line[i] - 'a';
            break;
        }
        case 'i': {
            instr.op = INC;
            instr.lhs = line[4] - 'a';
            break;
        }
        case 'd': {
            instr.op = DEC;
            instr.lhs = line[4] - 'a';
            break;
        }
        case 'j': {
            bool neg = false;
            u8 i = 4;
            if (line[i] == '-') {
                neg = true;
                i++;
            }

            if (isdigit(line[i])) {
                instr.op = JII;
                while (isdigit(line[i])) {
                    instr.lhs = (instr.lhs * 10) + (line[i] - '0');
                    i++;
                }
                if (neg) {
                    instr.lhs *= -1;
                }
            } else {
                instr.op = JRI;
                instr.lhs = line[i] - 'a';
                i++;
            }

            i++;

            if (line[i] == '-') {
                neg = true;
                i++;
            }

            if (isdigit(line[i])) {
                while (isdigit(line[i])) {
                    instr.rhs = (instr.rhs * 10) + (line[i] - '0');
                    i++;
                }
                if (neg) {
                    instr.rhs *= -1;
                }
            } else {
                if (instr.op == JII) {
                    instr.op = JIR;
                }
                instr.rhs = line[i] - 'a';
            }

            break;
        }
        }

        push(&instrs, &instr, sizeof(Instr));
    }

    u32 ip = 0;

    while (ip < len(&instrs, sizeof(Instr))) {
        Instr* instr = get_elmt(&instrs, ip, sizeof(Instr));

        switch (instr->op) {
        case CRR: {
            reg[instr->rhs] = reg[instr->lhs];
            break;
        }
        case CIR: {
            reg[instr->rhs] = instr->lhs;
            break;
        }
        case INC: {
            reg[instr->lhs] += 1;
            break;
        }
        case DEC: {
            reg[instr->lhs] -= 1;
            break;
        }
        case JII: {
            if (instr->lhs != 0) {
                ip += instr->rhs;
                continue;
            }
            break;
        }
        case JIR: {
            if (instr->lhs != 0) {
                ip += reg[instr->rhs];
                continue;
            }
            break;
        }
        case JRI: {
            if (reg[instr->lhs] != 0) {
                ip += instr->rhs;
                continue;
            }
        } break;
        }
        ip++;
    }

    return reg[0];
}