#![allow(unused_imports)]

use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::get_data;

pub fn p1(data: String) -> isize {
    let mut result = 0;

    let mut start = 0;
    let mut num = false;
    let mut neg_idx = usize::MAX;

    for (i, c) in data.chars().enumerate() {
        if c == '-' {
            neg_idx = i;
            continue;
        }

        if !num && c.is_ascii_digit() {
            start = i;
            num = true;
            continue;
        }

        if num && !c.is_ascii_digit() {
            let mut val = data.get(start..i).unwrap().parse::<isize>().unwrap();
            if neg_idx == start - 1 {
                val *= -1;
            }
            result += val;
            num = false;
        }
    }

    result
}

fn handle_value(v: Value) -> i64 {
    let mut temp = 0;
    match v {
        Value::Number(x) => temp += x.as_i64().unwrap(),
        Value::String(x) => {
            if x == "red" {
                return 0;
            }
        }
        Value::Array(x) => {
            for nested_v in x {
                temp += handle_value(nested_v);
            }
        }
        Value::Object(x) => temp += handle_struct(x),
        _ => (),
    }

    temp
}

fn handle_struct(map: Map<String, Value>) -> i64 {
    let mut temp = 0;
    for (k, v) in map {
        match v {
            Value::Number(x) => temp += x.as_i64().unwrap(),
            Value::String(x) => {
                if x == "red" {
                    return 0;
                }
            }
            Value::Array(x) => {
                for nested_v in x {
                    temp += handle_value(nested_v);
                }
            }
            Value::Object(x) => temp += handle_struct(x),
            _ => (),
        }
    }

    temp
}

pub fn p2(data: String) -> isize {
    let mut result = 0;

    let eef: Map<String, Value> = serde_json::from_str(&data).unwrap();

    handle_struct(eef) as isize

    // let mut temp = 0;
    // for (_k, v) in eef {
    //     match v {
    //         Value::Null => todo!(),
    //         Value::Bool(_) => todo!(),
    //         Value::Number(x) => temp += x.as_i64().unwrap(),
    //         Value::String(x) => if x == "red" {
    //             result = 0;
    //             break;
    //         },
    //         Value::Array(x) => temp += handle_array(x),
    //         Value::Object(x) => temp += handle_struct(x),
    //     }

    //     result += temp;
    //     temp = 0;
    // }

    // result as isize
}

#[test]
fn test_d12() {
    let data = get_data(12);
    assert_eq!(p1(data.clone()), 0);
    assert_eq!(p2(data), 0);
}
