#![allow(unused_imports)]

use itertools::Itertools;

use crate::get_data;

// huh, i should make this a crate. It's pretty convenient
macro_rules! range_array {
    ($x:ty;$start:literal..$end:literal) => {{
        const END_BIGGER: bool = $end > $start;
        const MAX: isize = if END_BIGGER { $end } else { $start };
        const MIN: isize = if END_BIGGER { $start } else { $end };

        const fn _range_array() -> [$x; (MAX - MIN).unsigned_abs()] {
            let mut temp = [0; (MAX - MIN).unsigned_abs()];
            let mut i = 0;
            let mut m = MIN;
            let step = if END_BIGGER { 1 } else { -1 };

            while m < MAX {
                temp[i] = i as $x;
                i += 1;
                m += step;
            }
            temp
        }
        _range_array()
    }};
}

// this is so fuckin dumb. ExactSizeIterators, Range objects, and For loops aren't allowed in const
// functions yet, so there's not really a better way to set up this array at compile time afaik.
const fn init_array() -> [u8; 256] {
    let mut temp = [0; 256];
    let mut i = 0;
    while i < 256 {
        temp[i] = i as u8;
        i += 1;
    }

    temp
}

pub fn p1(data: String) -> usize {
    let mut nums = range_array![u8; 0..256];
    // used to temporarily store the reversed info
    let mut temp = [0; 256];

    let mut pos = 0;
    let mut skip_size = 0;
    data.split(',').for_each(|x| {
        let length = x.parse::<usize>().unwrap();
        assert!(length < 256);

        if length > 1 {
            if length + pos < nums.len() {
                let view = &mut nums[pos..pos + length];
                for i in 0..view.len() / 2 {
                    view.swap(i, (view.len() - 1) - i);
                }
            } else {
                // i could simplify this with indexing math but fuck it.
                for (i, val) in nums.into_iter().cycle().skip(pos).take(length).enumerate() {
                    temp[(length - 1) - i] = val;
                }
                for i in 0..length {
                    nums[(pos + i) % nums.len()] = temp[i];
                }
            }
        }

        pos = (pos + length + skip_size) % nums.len();
        skip_size += 1;
        // println!("{:?}", nums);
    });

    nums[0] as usize * nums[1] as usize
}

const BONUS: [u8; 5] = [17, 31, 73, 47, 23];

pub fn p2(data: String) -> String {
    let input = data.lines().next().unwrap().as_bytes();
    let mut nums = range_array![u8; 0..256];
    // used to temporarily store the reversed info
    let mut temp = [0; 256];

    let mut pos = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for &l in input.iter().chain(&BONUS) {
            let length = l as usize;
            if length > 1 {
                if length + pos < nums.len() {
                    let view = &mut nums[pos..pos + length];
                    for i in 0..view.len() / 2 {
                        view.swap(i, (view.len() - 1) - i);
                    }
                } else {
                    // i could simplify this with indexing math but fuck it.
                    for (i, val) in nums.into_iter().cycle().skip(pos).take(length).enumerate() {
                        temp[(length - 1) - i] = val;
                    }
                    for i in 0..length {
                        nums[(pos + i) % nums.len()] = temp[i];
                    }
                }
            }

            pos = (pos + length + skip_size) % nums.len();
            skip_size += 1;
        }
    }

    let mut result = String::with_capacity(32);
    for chunk in &nums.iter().chunks(16) {
        result.push_str(&format!(
            "{:02x}",
            chunk.cloned().reduce(|a, b| a ^ b).unwrap()
        ));
    }

    result
}

#[test]
fn test_d10() {
    let data = get_data(10);
    assert_eq!(p1(data.clone()), 1935);
    assert_eq!(p2(data), "dc7e7dee710d4c7201ce42713e6b8359");
}
