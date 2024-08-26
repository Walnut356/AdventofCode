pub use fxhash::FxHashMap as Map;
pub use fxhash::FxHashSet as Set;
use itertools::Itertools;

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

pub fn knot_hash_raw(input: &[u8]) -> [u8; 256] {
    let mut nums = range_array![u8; 0..256];
    // used to temporarily store the reversed info
    let mut temp = [0; 256];

    let mut pos = 0;
    let mut skip_size = 0;

    const BONUS: [u8; 5] = [17, 31, 73, 47, 23];

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

    nums
}

pub fn knot_hash_full(input: &[u8]) -> String {
    let mut nums = range_array![u8; 0..256];
    // used to temporarily store the reversed info
    let mut temp = [0; 256];

    let mut pos = 0;
    let mut skip_size = 0;

    const BONUS: [u8; 5] = [17, 31, 73, 47, 23];

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