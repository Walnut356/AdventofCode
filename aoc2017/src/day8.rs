#![allow(unused_imports)]

use crate::get_data;
use crate::utils::Map;

#[derive(Debug)]
enum Op {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl Op {
    fn from_str(s: &str) -> Self {
        match s {
            "==" => Self::Eq,
            "!=" => Self::Neq,
            "<" => Self::Lt,
            ">" => Self::Gt,
            "<=" => Self::Lte,
            ">=" => Self::Gte,
            _ => panic!("eef"),
        }
    }
}

#[derive(Debug)]
struct Instr<'a> {
    reg: &'a str,
    val: isize,
    cond_reg: &'a str,
    cond_val: isize,
    op: Op,
}

impl<'a> Instr<'a> {
    fn exe(&self, regs: &mut Map<&str, isize>) {
        let cond_reg = regs[self.cond_reg];

        if match self.op {
            Op::Eq => cond_reg == self.cond_val,
            Op::Neq => cond_reg != self.cond_val,
            Op::Lt => cond_reg < self.cond_val,
            Op::Gt => cond_reg > self.cond_val,
            Op::Lte => cond_reg <= self.cond_val,
            Op::Gte => cond_reg >= self.cond_val,
        } {
            *regs.get_mut(self.reg).unwrap() += self.val;
        }
    }
}

pub fn p1(data: String) -> isize {
    let mut regs: Map<&str, isize> = Map::default();
    let mut inst = Vec::new();

    data.lines().for_each(|line| {
        let mut tokens = line.split_ascii_whitespace();

        let reg = tokens.next().unwrap();
        regs.insert(reg, 0);
        let neg = tokens.next().unwrap().starts_with('d');
        let val = if neg {
            -tokens.next().unwrap().parse::<isize>().unwrap()
        } else {
            tokens.next().unwrap().parse::<isize>().unwrap()
        };

        // consume "if"
        tokens.next().unwrap();

        let cond_reg = tokens.next().unwrap();
        let op = Op::from_str(tokens.next().unwrap());
        let cond_val = tokens.next().unwrap().parse::<isize>().unwrap();
        inst.push(Instr {
            reg,
            val,
            cond_reg,
            cond_val,
            op,
        })
    });

    // dbg!(inst.len());
    inst.into_iter().for_each(|x| x.exe(&mut regs));

    // dbg!(regs.len());
    regs.into_iter().map(|x| x.1).max().unwrap()
}

pub fn p2(data: String) -> isize {
    let mut regs: Map<&str, isize> = Map::default();
    let mut inst = Vec::new();

    data.lines().for_each(|line| {
        let mut tokens = line.split_ascii_whitespace();

        let reg = tokens.next().unwrap();
        regs.insert(reg, 0);
        let neg = tokens.next().unwrap().starts_with('d');
        let val = if neg {
            -tokens.next().unwrap().parse::<isize>().unwrap()
        } else {
            tokens.next().unwrap().parse::<isize>().unwrap()
        };

        // consume "if"
        tokens.next().unwrap();

        let cond_reg = tokens.next().unwrap();
        let op = Op::from_str(tokens.next().unwrap());
        let cond_val = tokens.next().unwrap().parse::<isize>().unwrap();
        inst.push(Instr {
            reg,
            val,
            cond_reg,
            cond_val,
            op,
        })
    });

    let mut max = 0;
    // dbg!(inst.len());
    inst.into_iter().for_each(|x| {
        x.exe(&mut regs);
        max = regs[x.reg].max(max);
    });

    max
}

#[test]
fn test_d8() {
    let data = get_data(8);
    assert_eq!(p1(data.clone()), 3745);
    assert_eq!(p2(data), 4644);
}
