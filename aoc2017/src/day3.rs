#![allow(unused_imports)]

use crate::get_data;

pub fn p1(data: String) -> usize {
    // the bottom right value of the square is equal to the area of the square. Using the bottom
    // right value, you can find the distance of the value from the cardinal directions. Add the
    // ring number (traverse inward) to the distance from the cardinal (traverse the perimeter)
    // for the answer.
    let val = data.parse::<isize>().unwrap();
    let mut ring_num: isize = 1;
    while (1 + 2 * ring_num).pow(2) < val {
        ring_num += 1;
    }

    let normalized = (1 + 2 * ring_num).pow(2) - val;

    ((normalized % (2 * ring_num)) - ring_num).unsigned_abs() + ring_num as usize
}

pub fn p2(data: String) -> usize {
    let val = data.parse::<usize>().unwrap();
    // seeding it saves me some conditional logic later
    let mut states: Vec<usize> = vec![1, 1, 2, 4, 5, 10];
    let mut diag_pos: usize = 0;
    let mut ring_num: usize = 1;
    let mut prev_corner = true;
    let mut new_ring = false;

    while *states.last().unwrap() < val {
        let mut curr = *states.last().unwrap();
        let normalized = (1 + 2 * ring_num).pow(2) - (states.len() + 1);
        let corner = normalized / (2 * ring_num);
        let offset = normalized % (2 * ring_num);

        if new_ring {
            prev_corner = true;
            curr += states[diag_pos];
            new_ring = false;
        } else if offset == 0 {
            // corner
            prev_corner = true;

            curr += states[diag_pos];

            if corner == 0 {
                // bottom right
                ring_num += 1;
                new_ring = true;
                curr += *states.get(diag_pos + 1).unwrap_or(&0);
                diag_pos += 1;
            }
        } else if prev_corner {
            curr += states[diag_pos]
                + *states.get(states.len().saturating_sub(2)).unwrap_or(&0)
                + *states.get(diag_pos + 1).unwrap_or(&0);
            prev_corner = false
        } else {
            curr += states[diag_pos] + states[diag_pos + 1];

            let next = (1 + 2 * ring_num).pow(2) - (states.len() + 2);
            // if the next value isn't a corner, get the "in-front" diagonal
            if next % (2 * ring_num) != 0 || (offset == 1 && corner == 0) {
                curr += *states.get(diag_pos + 2).unwrap_or(&0);
            }
            // else {
            //     let next_next = (1 + 2 * (ring_num + 2)).pow(2) - (states.len() + 3);
            //     if next_next % (2 * ring_num) != 0 && next_next / (2 * ring_num) == 0 {
            //         curr += *states.get(diag_pos + 2).unwrap_or(&0);
            //     }
            // }
            diag_pos += 1;
        }

        states.push(curr);
    }

    *states.last().unwrap()
}

#[test]
fn test_d3() {
    let data = get_data(3);
    assert_eq!(p1(data.clone()), 438);
    assert_eq!(p2(data), 266330);
}
