#![allow(unused_imports)]

use std::arch::asm;

use crate::get_data;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Reg(u8),
    Imm(isize),
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Set(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Jnz(Operand, Operand),
}

impl std::str::FromStr for Op {
    // close enough
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        let opcode = tokens.next().unwrap();
        let op_one = tokens.next().map(|x| {
            if x.as_bytes()[0].is_ascii_alphabetic() {
                Operand::Reg(x.as_bytes()[0] - b'a')
            } else {
                Operand::Imm(x.parse().unwrap())
            }
        });
        let op_two = tokens.next().map(|x| {
            if x.as_bytes()[0].is_ascii_alphabetic() {
                Operand::Reg(x.as_bytes()[0] - b'a')
            } else {
                Operand::Imm(x.parse().unwrap())
            }
        });

        match opcode {
            "set" => Ok(Self::Set(op_one.unwrap(), op_two.unwrap())),
            "sub" => Ok(Self::Sub(op_one.unwrap(), op_two.unwrap())),
            "mul" => Ok(Self::Mul(op_one.unwrap(), op_two.unwrap())),
            "jnz" => Ok(Self::Jnz(op_one.unwrap(), op_two.unwrap())),
            _ => Err("invalid operand"),
        }
    }
}

pub fn p1(data: String) -> usize {
    let mut registers = [0isize; 8];

    let ops = data
        .lines()
        .map(|x| x.parse::<Op>().unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut i = 0isize;
    loop {
        if i.is_negative() || i >= ops.len() as isize {
            break;
        }

        match ops[i as usize] {
            Op::Set(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    registers[x as usize] = match o2 {
                        Operand::Reg(y) => registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            }
            Op::Sub(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    registers[x as usize] -= match o2 {
                        Operand::Reg(y) => registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            }
            Op::Mul(o1, o2) => {
                result += 1;
                if let Operand::Reg(x) = o1 {
                    registers[x as usize] *= match o2 {
                        Operand::Reg(y) => registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            }
            Op::Jnz(o1, o2) => {
                let val = match o1 {
                    Operand::Reg(x) => registers[x as usize],
                    Operand::Imm(x) => x,
                };
                if val != 0 {
                    i += match o2 {
                        Operand::Reg(x) => registers[x as usize],
                        Operand::Imm(x) => x,
                    };
                    continue;
                }
            }
        }

        i += 1;
    }

    result
}


pub fn p2(_data: String) -> isize {
    let val = (79 * 100) + 100000;
    let mut result = 0;
    for i in (val..=(val + 17000)).step_by(17) {
        if !primes::is_prime(i) {
            result += 1;
        }
    }

    result
}

#[allow(asm_sub_register)]
fn _temp() -> isize {
    let mut registers = [1, 0, 0, 0, 0, 0, 0, 0];
    let mut result = 0;
    unsafe {
        asm! {
            "mov {a}, 1",
            "mov {b}, 79",     // set b 79
            "mov {c}, {b}",    // set c b
            "cmp {a}, 0",      // jnz a 2
            "jne 21f",         // jnz a 2
            "jmp 2f",          // jnz 1 5
            "21:",
            "imul {b}, 100",   // mul b 100
            "add {b}, 100000", // sub b -100000
            "mov {c}, {b}",    // set c b
            "add {c}, 17000",  // sub c -17000
            "2:",
            "mov {f}, 1",      // set f 1
            "mov {d}, 2",      // set d 2
            "5:",
            "mov {e}, 2",      // set e 2
            "4:",
            "mov {g}, {d}",    // set g d
            "imul {g}, {e}",   // mul g e
            "sub {g}, {b}",    // sub g b
            "jne 3f",          // jnz g 2
            "mov {f}, 0",      // set f 0
            "3:",
            "add {e}, 1",      // sub e -1
            "mov {g}, {e}",    // set g e
            "sub {g}, {b}",    // sub g b
            "jne 4b",          // jnz g -8
            "add {d}, 1",      // sub d -1
            "mov {g}, {d}",    // set g d
            "sub {g}, {b}",    // sub g b
            "jne 5b",          // jnz g -13
            "cmp {f}, 0",      // jnz f 2
            "jne 6f",          // jnz f 2
            "add rcx, 1",      // sub h -1
            "6:",
            "mov {g}, {b}",    // set g b
            "sub {g}, {c}",    // sub g c
            "jne 7f",          // jnz g 2
            "jmp 8f",          // jnz 1 3
            "7:",
            "add {b}, 17",     // sub b -17
            "jmp 2b",          // jnz 1 -23
            "8:",
            "mov {res}, rcx",
            a = inout(reg) registers[0],
            b = inout(reg) registers[1],
            c = inout(reg) registers[2],
            d = inout(reg) registers[3],
            e = inout(reg) registers[4],
            f = inout(reg) registers[5],
            g = inout(reg) registers[6],
            inout("rcx") registers[7],
            res = inout(reg) result,
        }
    }

    result
}

#[test]
fn test_d23() {
    let data = get_data(23);
    assert_eq!(p1(data.clone()), 5929);
    assert_eq!(p2(data), 907);
}
