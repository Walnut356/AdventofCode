#include "utils.h"

typedef enum OpCode {
    // Copy Register to Register
    CRR,
    // Copy Immedaite to Register
    CIR,
    _CII,
    _CRI,
    INC,
    DEC,
    // Jump Immediate Immediate
    JII,
    // Jump Immediate Register
    JIR,
    // Jump Register Immediate
    JRI,
    JRR,
    TGL,
} OpCode;

typedef struct Instr {
    OpCode op;
    i32 lhs;
    i32 rhs;
} Instr;

u64 p1(FILE* file) {
    Vec instrs = with_capacity(27, sizeof(Instr));
    i32 reg[4] = { 7, 0, 0, 0 };
    Vec buf = with_capacity(10, 1);
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
        case 't':
            instr.op = TGL;
            instr.lhs = line[4] - 'a';
            break;
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
        case _CRI:
        case _CII: { // invalid instr
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
            break;
        }
        case JRR: {
            if (reg[instr->lhs] != 0) {
                ip += reg[instr->rhs];
                continue;
            }
            break;
        }
        case TGL: {
            if (instr->lhs + ip > len(&instrs, sizeof(Instr))) {
                break;
            }
            Instr* target = get_elmt(&instrs, reg[instr->lhs] + (i32)ip, sizeof(Instr));
            switch (target->op) {
            case INC:
                target->op = DEC;
                break;
            case TGL:
            case DEC:
                target->op = INC;
                break;
            case CRR:
                target->op = JRR;
                break;
            case CIR:
                target->op = JIR;
                break;
            case JII:
                target->op = _CII;
                break;
            case JIR:
                target->op = CIR;
                break;
            case JRI:
                target->op = _CRI;
                break;
            case _CII:
                target->op = JII;
                break;
            case _CRI:
                target->op = JRI;
                break;
            case JRR:
                target->op = CRR;
                break;
            }
        }
        }
        ip++;
    }

    return reg[0];
}

u64 p2(FILE* file) {
    Vec instrs = with_capacity(27, sizeof(Instr));
    i32 reg[4] = { 12, 0, 0, 0 };
    Vec buf = with_capacity(10, 1);
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
        case 't':
            instr.op = TGL;
            instr.lhs = line[4] - 'a';
            break;
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
        case _CRI:
        case _CII: { // invalid instr
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
            break;
        }
        case JRR: {
            if (reg[instr->lhs] != 0) {
                ip += reg[instr->rhs];
                continue;
            }
            break;
        }
        case TGL: {
            if (instr->lhs + ip > len(&instrs, sizeof(Instr))) {
                break;
            }
            Instr* target = get_elmt(&instrs, reg[instr->lhs] + (i32) ip, sizeof(Instr));
            switch (target->op) {
            case INC:
                target->op = DEC;
                break;
            case TGL:
            case DEC:
                target->op = INC;
                break;
            case CRR:
                target->op = JRR;
                break;
            case CIR:
                target->op = JIR;
                break;
            case JII:
                target->op = _CII;
                break;
            case JIR:
                target->op = CIR;
                break;
            case JRI:
                target->op = _CRI;
                break;
            case _CII:
                target->op = JII;
                break;
            case _CRI:
                target->op = JRI;
                break;
            case JRR:
                target->op = CRR;
                break;
            }
        }
        }
        ip++;
    }

    return reg[0];
}