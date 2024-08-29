#![allow(unused_imports)]

use std::collections::VecDeque;

use crate::get_data;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Reg(u8),
    Imm(isize),
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Snd(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

impl std::str::FromStr for Op {
    // close enough
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        let opcode = tokens.next().unwrap();
        let op_one = tokens.next().map(|x| if x.as_bytes()[0].is_ascii_alphabetic() {
            Operand::Reg(x.as_bytes()[0] - b'a')
        } else {
            Operand::Imm(x.parse().unwrap())
        });
        let op_two = tokens.next().map(|x| if x.as_bytes()[0].is_ascii_alphabetic() {
            Operand::Reg(x.as_bytes()[0] - b'a')
        } else {
            Operand::Imm(x.parse().unwrap())
        });

        match opcode {
            "snd" => Ok(Self::Snd(op_one.unwrap())),
            "set" => Ok(Self::Set(op_one.unwrap(), op_two.unwrap())),
            "add" => Ok(Self::Add(op_one.unwrap(), op_two.unwrap())),
            "mul" => Ok(Self::Mul(op_one.unwrap(), op_two.unwrap())),
            "mod" => Ok(Self::Mod(op_one.unwrap(), op_two.unwrap())),
            "rcv" => Ok(Self::Rcv(op_one.unwrap())),
            "jgz" => Ok(Self::Jgz(op_one.unwrap(), op_two.unwrap())),
            _ => Err("invalid operand")
        }
    }
}

pub fn p1(data: String) -> isize {
    // registers are letters so this is the most we'll ever have
    let mut registers = [0isize; 26];

    let ops = data.lines().map(|x| x.parse::<Op>().unwrap()).collect::<Vec<_>>();

    let mut result = 0;
    let mut i = 0isize;
    loop {
        if i.is_negative() || i >= ops.len() as isize {
            break;
        }

        match ops[i as usize] {
            Op::Snd(o) => match o {
                Operand::Reg(x) => result = registers[x as usize],
                Operand::Imm(x) => result = x,
            },
            Op::Set(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    registers[x as usize] = match o2 {
                        Operand::Reg(y) => registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            },
            Op::Add(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    registers[x as usize] += match o2 {
                        Operand::Reg(y) => registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            },
            Op::Mul(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    registers[x as usize] *= match o2 {
                        Operand::Reg(y) => registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            },
            Op::Mod(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    registers[x as usize] %= match o2 {
                        Operand::Reg(y) => registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            },
            Op::Rcv(o1) => match o1 {
                    Operand::Reg(x) => {
                        if registers[x as usize] != 0 {
                            return result
                        }
                    },
                    Operand::Imm(_) => panic!("unreachable"),
                },
            Op::Jgz(o1, o2) => {
                let val = match o1 {
                Operand::Reg(x) => registers[x as usize],
                Operand::Imm(x) => x,
            };
                if val > 0 {
                    i += match o2 {
                        Operand::Reg(x) => registers[x as usize],
                        Operand::Imm(x) => x,
                    };
                    continue;
                }

            },
        }

        i += 1;
    }


    -1
}

struct Program {
    ip: isize,
    registers: [isize; 26],
    queue: VecDeque<isize>,
    count: usize,
}

impl Program {
    fn step(&mut self, ops: &[Op], send_queue: &mut VecDeque<isize>) -> bool {
        if self.ip.is_negative() || self.ip >= ops.len() as isize {
            return false;
        }

        match ops[self.ip as usize] {
            Op::Snd(o) => {
                self.count += 1;
                match o {
                Operand::Reg(x) => send_queue.push_front(self.registers[x as usize]),
                Operand::Imm(x) => send_queue.push_front(x),
            }},
            Op::Set(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    self.registers[x as usize] = match o2 {
                        Operand::Reg(y) => self.registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            },
            Op::Add(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    self.registers[x as usize] += match o2 {
                        Operand::Reg(y) => self.registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            },
            Op::Mul(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    self.registers[x as usize] *= match o2 {
                        Operand::Reg(y) => self.registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            },
            Op::Mod(o1, o2) => {
                if let Operand::Reg(x) = o1 {
                    self.registers[x as usize] %= match o2 {
                        Operand::Reg(y) => self.registers[y as usize],
                        Operand::Imm(y) => y,
                    };
                }
            },
            Op::Rcv(o1) => match o1 {
                    Operand::Reg(x) => {
                        if !self.queue.is_empty() {
                            self.registers[x as usize] = self.queue.pop_back().unwrap();
                        } else {
                            return false;
                        }
                    },
                    Operand::Imm(_) => panic!("unreachable"),
                },
            Op::Jgz(o1, o2) => {
                let val = match o1 {
                Operand::Reg(x) => self.registers[x as usize],
                Operand::Imm(x) => x,
            };
                if val > 0 {
                    self.ip += match o2 {
                        Operand::Reg(x) => self.registers[x as usize],
                        Operand::Imm(x) => x,
                    };
                    return true;
                }

            },
        }

        self.ip += 1;

        true
    }
}

pub fn p2(data: String) -> usize {
    let ops = data.lines().map(|x| x.parse::<Op>().unwrap()).collect::<Vec<_>>();

    let mut p0 = Program { ip: 0, registers: [0; 26], queue: VecDeque::new(), count: 0 };
    let mut p1 = Program { ip: 0, registers: [0; 26], queue: VecDeque::new(), count: 0 };
    p1.registers[(b'p' - b'a') as usize] = 1;

    loop {
        let r0 = p0.step(&ops, &mut p1.queue);
        let r1 = p1.step(&ops, &mut p0.queue);
        if !r0 && !r1 {
            return p1.count
        }
    }
}

#[test]
fn test_d18() {
    let data = get_data(18);
    assert_eq!(p1(data.clone()), 9423);
    assert_eq!(p2(data), 7620);
}
