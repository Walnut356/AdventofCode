for i in range(1, 26):
    with open(f"./aoc2017/src/day{i}.rs", "w") as f:
        f.write(
            f"""#![allow(unused_imports)]

use crate::get_data;


pub fn p1(data: String) -> usize {{
    let result = 0;

    result
}}

pub fn p2(data: String) -> usize {{
    let result = 0;

    result
}}

#[test]
fn test_d{i}() {{
    let data = get_data({i});
    assert_eq!(p1(data.clone()), 0);
    assert_eq!(p2(data), 0);
}}"""
        )

    with open(f"./aoc2017/test_data/day{i}.txt", 'w') as f:
        pass