#![allow(unused_imports)]

use crate::get_data;

// (cost, dmg, armor)
const WEPN: [(usize, usize, usize); 5] = [
    // (0, 1, 0), this puzzle is slightly wrong/bugged. "All attacks deal a minimum of 1 damage", and
    // nowhere does it state that you *cannot attack*, rather that *your damage starts at 0*.
    (8, 4, 0),
    (10, 5, 0),
    (25, 6, 0),
    (40, 7, 0),
    (74, 8, 0),
];

const ARMR: [(usize, usize, usize); 6] = [
    (0, 0, 0),
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
];

const RING: [(usize, usize, usize); 6] = [
    (20, 0, 1),
    (25, 1, 0),
    (40, 0, 2),
    (50, 2, 0),
    (80, 0, 3),
    (100, 3, 0),
];

pub fn wins(me: &(usize, usize, usize), boss: &(usize, usize, usize)) -> bool {
    // my time to kill is less than their time to kill
    // can be equal because player goes first
    // damage must be at least 1
    (boss.0 as f32 / (me.1.saturating_sub(boss.2)).max(1) as f32).ceil()
        <= (me.0 as f32 / (boss.1.saturating_sub(me.2)).max(1) as f32).ceil()
}

pub fn p1(data: String) -> usize {
    let mut result = usize::MAX;
    // health, damage, armor
    let mut boss: (usize, usize, usize) = (0, 0, 0);

    let mut tokens = data.split_ascii_whitespace();

    tokens.next().unwrap(); // hit
    tokens.next().unwrap(); // points
    boss.0 = tokens.next().unwrap().parse::<usize>().unwrap();
    tokens.next().unwrap(); // damage
    boss.1 = tokens.next().unwrap().parse::<usize>().unwrap();
    tokens.next().unwrap(); // armor
    boss.2 = tokens.next().unwrap().parse::<usize>().unwrap();

    // i'd love to spend the time doing this smarter, but there's so few possibilities it hardly
    // seems worth it.
    for w in WEPN {
        let me = (100, w.1, 0);
        if wins(&me, &boss) {
            result = result.min(w.0);
        }
        for a in ARMR {
            let me = (100, w.1, a.2);
            if wins(&me, &boss) {
                result = result.min(w.0 + a.0);
            }
            for r1 in RING {
                let me = (100, w.1 + r1.1, a.2 + r1.2);
                if wins(&me, &boss) {
                    result = result.min(w.0 + a.0 + r1.0);
                }
                for r2 in RING {
                    let me = (100, w.1 + r1.1 + r2.1, a.2 + r1.2 + r2.2);
                    if wins(&me, &boss) {
                        result = result.min(w.0 + a.0 + r1.0 + r2.0);
                    }
                }
            }
        }
    }

    result
}

pub fn p2(data: String) -> usize {
    let mut result = 0;
    // health, damage, armor
    let mut boss: (usize, usize, usize) = (0, 0, 0);

    let mut tokens = data.split_ascii_whitespace();

    tokens.next().unwrap(); // hit
    tokens.next().unwrap(); // points
    boss.0 = tokens.next().unwrap().parse::<usize>().unwrap();
    tokens.next().unwrap(); // damage
    boss.1 = tokens.next().unwrap().parse::<usize>().unwrap();
    tokens.next().unwrap(); // armor
    boss.2 = tokens.next().unwrap().parse::<usize>().unwrap();

    for w in WEPN {
        let me = (100, w.1, 0);
        if !wins(&me, &boss) {
            result = result.max(w.0);
        }
        for a in ARMR {
            let me = (100, w.1, a.2);
            if !wins(&me, &boss) {
                result = result.max(w.0 + a.0);
            }
            for r1 in RING {
                let me = (100, w.1 + r1.1, a.2 + r1.2);
                if !wins(&me, &boss) {
                    result = result.max(w.0 + a.0 + r1.0);
                }
                for r2 in RING {
                    let me = (100, w.1 + r1.1 + r2.1, a.2 + r1.2 + r2.2);
                    if !wins(&me, &boss) {
                        result = result.max(w.0 + a.0 + r1.0 + r2.0);
                    }
                }
            }
        }
    }

    result
}

#[test]
fn test_d21() {
    let data = get_data(21);
    assert_eq!(p1(data.clone()), 78);
    assert_eq!(p2(data), 148);
}
