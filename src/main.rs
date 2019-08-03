use std::cmp::Ordering::*;
use std::collections::HashMap;

fn main() {
}

// https://www.codewars.com/kata/52761ee4cffbc69732000738
fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_races_worth: [u32; 6] = [1, 2, 3, 3, 4, 10];
    let evil_races_worth: [u32; 7] = [1, 2, 2, 2, 3, 5, 10];

    fn calc_worth_sum(races_worth: &[u32], army: &str) -> u32 {
        let mut sum: u32 = 0;
        for (index, str_worth) in army.split_whitespace().enumerate() {
            let race_count = str_worth.parse::<u32>().unwrap();
            sum += race_count * races_worth[index];
        }
        sum
    }

    let good_worth_sum = calc_worth_sum(&good_races_worth, good);
    let evil_worth_sum = calc_worth_sum(&evil_races_worth, evil);
    String::from(match good_worth_sum.cmp(&evil_worth_sum) {
        Greater => "Battle Result: Good triumphs over Evil",
        Less => "Battle Result: Evil eradicates all trace of Good",
        Equal => "Battle Result: No victor on this battle field",
    })
}

#[test]
fn good_vs_evil_tests() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}

// https://www.codewars.com/kata/5ac6932b2f317b96980000ca
fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();
    digits.iter().fold(0i32, | acc, &item| acc * 10  + item)
}

#[test]
fn min_value_tests() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}

// https://www.codewars.com/kata/5648b12ce68d9daa6b000099
fn passengers_in_bus(bus_stops:&[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |sum, &(ppl_in, ppl_out)| sum + ppl_in - ppl_out)
}

#[test]
fn passengers_in_bus_tests() {
    assert_eq!(passengers_in_bus(&[(10, 0),(3, 5),(5, 8)]), 5);
    assert_eq!(passengers_in_bus(&[(3, 0),(9, 1),(4, 10),(12, 2),(6, 1),(7, 10)]), 17);
    assert_eq!(passengers_in_bus(&[(3, 0),(9, 1),(4, 8),(12, 2),(6, 1),(7, 8)]), 21);
}

// https://www.codewars.com/kata/54b42f9314d9229fd6000d9c
fn duplicate_encode(word: &str)->String {
    let mut symbol_counter = HashMap::new();

    let word_uppercase = word.to_uppercase();
    for key in word_uppercase.chars() {
        *symbol_counter.entry(key).or_insert(0) += 1;
    }

    let mut result = String::new();
    for key in word_uppercase.chars() {
        if symbol_counter[&key] == 1 {
            result.push('(');
        } else {
            result.push(')');
        }
    }

    result
}

#[test]
fn duplicate_encode_tests() {
    assert_eq!(duplicate_encode("din"),"(((");
    assert_eq!(duplicate_encode("recede"),"()()()");
    assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
    assert_eq!(duplicate_encode("(( @"),"))((");
}

// https://www.codewars.com/kata/550498447451fbbd7600041c
fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    if a.is_empty() && b.is_empty() {
        return true;
    }

    let mut sa: Vec<i64> = a.iter().map(|x| x * x).collect();
    sa.sort();

    let mut sb = b.clone();
    sb.sort();

    sa == sb
}

fn comp_test(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

#[test]
fn comp_tests() {
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    comp_test(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    comp_test(a1, a2, false);
}