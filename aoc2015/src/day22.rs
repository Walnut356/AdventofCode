#![allow(unused_imports)]
#![allow(dead_code)]

use crate::get_data;
use fxhash::FxHashMap as Map;

// TODO eventually I want to come back and try to solve this one with less brute force. This seems
// very domain-optimizable.

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(i32)]
enum Spell {
    Missile = 53,
    Drain = 73,
    Shield = 113,
    Poison = 173,
    Recharge = 229,
}

impl Spell {
    fn cost(&self) -> i32 {
        *self as i32
    }

    fn dmg(&self) -> i32 {
        match self {
            Spell::Missile => 4,
            Spell::Drain => 2,
            Spell::Poison => 3,
            _ => 0,
        }
    }

    /// calculates the mana efficiency of a spell on remaining hp.
    fn mana_eff(&self, hp: i32) -> f32 {
        match self {
            Spell::Missile | Spell::Drain | Spell::Poison => {
                let cost = *self as i32 as f32;
                let dmg = self.dmg() as f32;

                (hp as f32 / dmg).ceil() * cost
            }
            _ => panic!("can't calculate efficiency of spells that deal no damage"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum State {
    Win = -1,
    Loss = 0,
    Battle = 1,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash,)]
struct Boss {
    hp: i32,
    dmg: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash,)]
struct Player {
    hp: i32,
    armor: i32,
    mana: i32,
    shield: u8,
    poison: u8,
    recharge: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hp: 50,
            armor: 0,
            mana: 500,
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }
}

impl Player {
    /// Attemp to cast spell. If casting spell is not possible, returns false.
    fn try_cast(&mut self, boss: &mut Boss, spell: Spell) -> bool {
        if self.mana < spell as i32 {
            return false;
        }
        self.mana -= spell as i32;
        // self.mana_spent += spell as i32;
        match spell {
            Spell::Missile => {
                boss.hp -= 4;
            }
            Spell::Drain => {
                boss.hp -= 2;
                self.hp += 2;
            }
            Spell::Shield => {
                if self.shield > 0 {
                    return false;
                }
                self.shield = 6;
            }
            Spell::Poison => {
                if self.poison > 0 {
                    return false;
                }
                self.poison = 6;
            }
            Spell::Recharge => {
                if self.recharge > 0 {
                    return false;
                }
                self.recharge = 5;
            }
        }

        true
    }

    /// Returns true if the player can cast any spell
    fn can_cast(&self) -> bool {
        self.mana >= Spell::Missile as i32
    }

    fn apply_effects(&mut self, boss: &mut Boss) {
        if self.shield > 0 {
            self.armor = 7;
            self.shield -= 1;
        } else {
            self.armor = 0;
        }
        if self.poison > 0 {
            boss.hp -= 3;
            self.poison -= 1;
        }
        if self.recharge > 0 {
            self.mana += 101;
            self.recharge -= 1;
        }
    }

    fn take_dmg(&mut self, boss: &mut Boss) -> State {
        self.hp -= (boss.dmg - self.armor).max(1);
        if self.hp <= 0 {
            State::Loss
        } else {
            State::Battle
        }
    }

    fn effective_mana(&self) -> i32 {
        let recharge = self.recharge as i32 * 101 - ((self.recharge as i32 / 2) * Spell::Missile as i32);
        self.mana + recharge
    }

    /// returns the number of rounds the player will live with current effects if no other spells are
    /// cast
    fn rtd(&self, boss: &Boss) -> i32 {
        let mut t = 0;
        let mut hp = self.hp;
        let mut a = self.armor;

        let a_dmg = boss.dmg - (self.armor.min(1) * 7);

        while hp > 0 {
            t += 1;
            if a > 0 {
                hp -= a_dmg;
                a -= 1;
            } else {
                hp -= boss.dmg;
            }
        }

        t
    }

    /// number of rounds to kill with remaining poison damage + spamming missile every turn
    fn rtk(&self, boss: &Boss) -> i32 {
        let mut t = 0;
        let mut hp = boss.hp;
        let mut p = self.poison as i32;
        while hp > 0 {
            t += 1;
            if p > 0 {
                hp -= p * 6;
                p -= 1;
            }
            hp -= 4;
        }

        t
    }
}

/// executes the player turn and the boss turn. Returns a value indicating the outcome of the turn.
fn turn(player: &mut Player, boss: &mut Boss, turn: i32) -> State {
    use Spell::*;
    player.apply_effects(boss);

    if !player.can_cast() {
        return State::Loss;
    }
    'player_turn: {
        // can we kill right now?
        let lethal = boss.hp - (player.poison.min(1) as i32 * 3);

        if lethal <= 4 && player.try_cast(boss, Missile) {
            return State::Win;
        }

        // can we kill by dumping missiles?
        let rtk = player.rtk(boss);
        dbg!(rtk);
        dbg!(player.rtd(boss));
        if rtk < player.rtd(boss) && rtk * 53 <= player.mana {
            player.try_cast(boss, Missile);
            break 'player_turn;
        }

        match turn {
            0 => {
                player.try_cast(boss, Shield);
            }
            1 => {
                player.try_cast(boss, Recharge);
            }
            _ => {
                // try to cast recharge and shield whenever possible to draw out the fight
                if player.try_cast(boss, Recharge) {
                    break 'player_turn;
                }
                if player.effective_mana() - (Recharge as i32) > Shield as i32 + Missile as i32
                    && player.try_cast(boss, Shield)
                {
                    break 'player_turn;
                }
                // use the most mana efficient spell whenever possible
                // if player.effective_mana() - (Recharge as i32) > Poison as i32
                //     && player.try_cast(boss, Poison)
                // {
                //     break 'player_turn;
                // }

                player.try_cast(boss, Missile);
            }
        }
    }

    player.apply_effects(boss);

    if boss.hp <= 0 {
        return State::Win;
    }

    player.take_dmg(boss)
}

fn dfs(player: Player, boss: Boss, spent: i32, cache: &mut Map<(Player, Boss), i32>, best: &mut i32, spells: Vec<Spell>) -> i32 {
    // base cases
    if !player.can_cast() || player.hp <= 0 || spent > *best {
        return i32::MAX;
    }
    if boss.hp <= 0 {
        return spent;
    }
    if let Some(val) = cache.get(&(player.clone(), boss.clone())) {
        return *val;
    }

    let mut result = i32::MAX;

    for spell in [Spell::Recharge, Spell::Poison, Spell::Shield, Spell::Drain, Spell::Missile,] {
        let mut p = player.clone();
        let mut b = boss.clone();
        if p.try_cast(&mut b, spell) { // player turn
            p.apply_effects(&mut b); // boss turn start
            if b.hp <= 0 {
                result = result.min(spent + spell as i32);
                continue;
            }
            p.take_dmg(&mut b); // boss turn
            if p.hp <= 0 {
                continue;
            }

            p.apply_effects(&mut b); // player turn start
            if b.hp <= 0 {
                result = result.min(spent + spell as i32);
                continue;
            }
            let mut temp = spells.clone();
            temp.push(spell);
            result = result.min(dfs(p, b, spent + spell as i32, cache, best, temp));

        }
    }

    cache.insert((player, boss), result);
    if result < *best {
        *best = result;
    }
    result
}

pub fn p1(data: String) -> i32 {
    let boss = {
        let mut tokens = data.split_ascii_whitespace();

        tokens.next(); // hit
        tokens.next(); // points:
        let hp = tokens.next().unwrap().parse::<i32>().unwrap();
        tokens.next(); // damage:
        let dmg = tokens.next().unwrap().parse::<i32>().unwrap();
        Boss { hp, dmg }
    };

    let player = Player::default();

    let mut best = i32::MAX;
    let mut cache = Map::default();

    dfs(player, boss, 0, &mut cache, &mut best, Vec::new())

    // player.hp = 10;
    // player.mana = 250;

    // boss.hp = 14;
    // boss.dmg = 8;
    // use Spell::*;
    // for s in [Poison, Recharge, Shield, Poison, Recharge, Shield, Poison, Missile, Shield, Poison] {
    //     println!("{:?}", s);
    //     player.try_cast(&mut boss, s);
    //     println!("Cast:\n{:?}\n{:?}", &player, &boss);
    //     player.apply_effects(&mut boss);
    //     println!("Effects (Boss):\n{:?}\n{:?}", &player, &boss);
    //     println!("Boss Attack:\n{:?}\n{:?}\n\n", &player, &boss);
    //     player.apply_effects(&mut boss);
    //     println!("Effects (Plyr):\n{:?}\n{:?}", &player, &boss);

    // }

    // 0


    // let mut state = State::Battle;
    // let mut turn_count = 0;
    // while state == State::Battle {
    //     state = turn(&mut player, &mut boss, turn_count);
    //     turn_count += 1;
    // }

    // dbg!(state);
    // dbg!(turn_count);
    // dbg!(&boss);
    // dbg!(&player);

    // let ttk =

    // let min_alive = player.hp / boss.dmg;
    // let max_turns = {
    //     let dmg = (boss.dmg.saturating_sub(7)).max(1);
    //     match player.hp % dmg {
    //         0 => (player.hp / dmg),
    //         1 => (player.hp / dmg) + 2,
    //         _ => (player.hp / dmg) + 1,
    //     }
    //     // if matches!(player.hp % dmg, 0 | 1) {
    //     //     (player.hp / dmg) +
    //     // }
    // }
    // let poison_turns = boss.hp / 6; // poison is most efficient and deals damage twice per boss attack
    // let rem_hp = boss.hp % 6;

    // let min_missiles = boss.hp / Spell::Missile.dmg();
    // let mut missile_cost = min_missiles * Spell::Missile as i32;

    // player.mana_spent
}

fn dfs_2(player: Player, boss: Boss, spent: i32, cache: &mut Map<(Player, Boss), i32>, best: &mut i32, spells: Vec<Spell>) -> i32 {
    // base cases
    if !player.can_cast() || player.hp <= 0 || spent > *best {
        return i32::MAX;
    }
    if boss.hp <= 0 {
        return spent;
    }
    if let Some(val) = cache.get(&(player.clone(), boss.clone())) {
        return *val;
    }

    let mut result = i32::MAX;

    for spell in [Spell::Recharge, Spell::Poison, Spell::Shield, Spell::Drain, Spell::Missile,] {
        let mut p = player.clone();
        let mut b = boss.clone();
        if p.try_cast(&mut b, spell) { // player turn
            p.apply_effects(&mut b); // boss turn start
            if b.hp <= 0 {
                result = result.min(spent + spell as i32);
                continue;
            }
            p.take_dmg(&mut b); // boss turn
            if p.hp <= 0 {
                continue;
            }

            p.apply_effects(&mut b); // player turn start
            if b.hp <= 0 {
                result = result.min(spent + spell as i32);
                continue;
            }
            let mut temp = spells.clone();
            temp.push(spell);
            result = result.min(dfs(p, b, spent + spell as i32, cache, best, temp));
        }
    }

    cache.insert((player, boss), result);
    if result < *best {
        *best = result;
    }
    result
}

pub fn p2(data: String) -> i32 {
    let mut boss = {
        let mut tokens = data.split_ascii_whitespace();

        tokens.next(); // hit
        tokens.next(); // points:
        let hp = tokens.next().unwrap().parse::<i32>().unwrap();
        tokens.next(); // damage:
        let dmg = tokens.next().unwrap().parse::<i32>().unwrap();
        Boss { hp, dmg }
    };

    let mut player = Player::default();

    let mut best = i32::MAX;
    let mut cache = Map::default();

    player.hp -= 1;
    boss.dmg += 1;
    dfs_2(player, boss, 0, &mut cache, &mut best, Vec::new())
}

#[test]
fn test_d22() {
    let data = get_data(22);
    assert_eq!(p1(data.clone()), 1824);
    assert_eq!(p2(data), 1937);
}
