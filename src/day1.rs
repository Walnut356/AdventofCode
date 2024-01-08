#![allow(unused_imports)]


pub fn part_one(data: String) -> isize {
    let mut result = 0;

    for char in data.chars() {
        match char {
            '(' => result += 1,
            ')' => result -= 1,
            _ => panic!("malformed input")
        }
    }

    result
}

pub fn part_two(data: String) -> usize {
    let mut result = 0;

    for (i, char) in data.chars().enumerate() {
        match char {
            '(' => result += 1,
            ')' => result -= 1,
            _ => panic!("malformed input")
        }
        if result == -1 {
            return i + 1;
        }
    }

    0
}