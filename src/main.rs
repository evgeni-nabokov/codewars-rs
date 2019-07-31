use std::cmp::Ordering::*;
use std::collections::HashMap;

fn main() {
}

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
fn good_vs_evil_test() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}

fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();
    digits.iter().fold(0i32, | acc, &item| acc * 10  + item)
}

#[test]
fn min_value_test() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}

fn passengers_in_bus(bus_stops:&[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |sum, &(ppl_in, ppl_out)| sum + ppl_in - ppl_out)
}

#[test]
fn number_in_bus_test() {
    assert_eq!(passengers_in_bus(&[(10, 0),(3, 5),(5, 8)]), 5);
    assert_eq!(passengers_in_bus(&[(3, 0),(9, 1),(4, 10),(12, 2),(6, 1),(7, 10)]), 17);
    assert_eq!(passengers_in_bus(&[(3, 0),(9, 1),(4, 8),(12, 2),(6, 1),(7, 8)]), 21);
}