#include "utils.h"


/* Hand "disassembly"
    a = ?;
    d = a;
    c = 4;
label_2:
    b = 633;
label_1:
    d += 1;
    b -= 1;
    if (b != 0) goto label_1;
    c -= 1;
    if (c != 0) goto label_2;
// the prior block is essentially d = a + (633 * 4);
label_11:
    a = d;
label_10:
    noop;
    b = a;
    a = 0;
label_6:
    c = 2;
label_5:
    if (b != 0) goto label_3;
    goto label_4;
label_3:
    b -= 1;
    c -= 1;
    if (c != 0) goto label_5;
    a += 1;
    goto label_6;
// prior block looks like a = b / 2 or 3?
label_4:
    b = 2;
label_9:
    if (c != 0) goto label_7;
    goto label_8;
label_7:
    b -= 1;
    c -= 1;
    goto label_9;
label_8:
    noop;
    push(b);
    if (a != 0) goto label_10;
    goto label_11;



*/

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
    OT,
} OpCode;

typedef struct Instr {
    OpCode op;
    i32 lhs;
    i32 rhs;
} Instr;

// I'm glad i didn't bother to optimize this lmao. The answer is 198, it only takes a few ms to
// calculate even with non-native code
bool exe_native(i32 a, Vec* output) {
    i32 b = 0;
    i32 c = 0;
    i32 d = 0;
    u8 prev = 1;
    u8 val = 0;

    d = a;
    c = 4;
label_2:
    b = 633;
label_1:
    d += 1;
    b -= 1;
    if (b != 0) {
        goto label_1;
    }
    c -= 1;
    if (c != 0) {
        goto label_2;
    }
// the prior block is essentially d = a + (633 * 4);
label_11:
    a = d;
label_10:
    b = a;
    a = 0;
label_6:
    c = 2;
label_5:
    if (b != 0) {
        goto label_3;
    }
    goto label_4;
label_3:
    b -= 1;
    c -= 1;
    if (c != 0) {
        goto label_5;
    }
    a += 1;
    goto label_6;
// prior block looks like a = b / 2 or 3?
label_4:
    b = 2;
label_9:
    if (c != 0) {
        goto label_7;
    }
    goto label_8;
label_7:
    b -= 1;
    c -= 1;
    goto label_9;
label_8:
    val = (u8) b;
    if (val == 1 && prev == 0 || val == 0 && prev == 1) {
        push(output, &val, 1);
        if (output->len == 100) {
            return true;
        }
        prev = val;
    } else {
        return false;
    }
    if (a != 0) {
        goto label_10;
    }
    goto label_11;
}

bool execute(i32 a, Vec* instrs, Vec* output) {
    i32 reg[4] = {a, 0, 0, 0};
    u32 ip = 0;
    u8 prev = 1;

    while (ip < len(instrs, sizeof(Instr)) && output->len < 100) {
        Instr* instr = get_elmt(instrs, ip, sizeof(Instr));

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
            if (instr->lhs + ip > len(instrs, sizeof(Instr))) {
                break;
            }
            Instr* target = get_elmt(instrs, reg[instr->lhs] + (i32) ip, sizeof(Instr));
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
        case OT: {
            u8 val = (u8) reg[instr->lhs];
            if (val == 1 && prev == 0 || val == 0 && prev == 1) {
                push(output, &val, 1);
                if (output->len == 100) {
                    return true;
                }
                prev = val;
            } else {
                return false;
            }
        }
        }
        ip++;
    }
    return true;
}

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
        case 'o':
            instr.op = OT;
            instr.lhs = line[4] - 'a';
        }


        push(&instrs, &instr, sizeof(Instr));
    }

    Vec output = with_capacity(100, 1);
    u64 result = 0;
    while (!exe_native(result, &output) && result < i32_MAX) {
        result += 1;
        output.len = 0;
            printf("%llu\n", result);
    }

    return result;
}

u64 p2(FILE* file) {

}