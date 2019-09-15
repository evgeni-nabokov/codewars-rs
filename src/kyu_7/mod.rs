use std::cmp;
use std::cmp::Ordering::*;
use std::iter;
use std::collections::{HashMap, HashSet};
use super::common::*;

#[cfg(test)]
mod tests;

// https://www.codewars.com/kata/5aba780a6a176b029800041c
fn max_multiple(divisor: u32, bound: u32) -> u32 {
    bound / divisor * divisor
}

// https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9
fn row_weights(array: Vec<u32>) -> (u32, u32) {
    let mut team1_sum = 0u32;
    let mut team2_sum = 0u32;
    for (i, &w) in array.iter().enumerate() {
        if i % 2 == 0 {
            team1_sum += w;
        } else {
            team2_sum += w;
        }
    }
    (team1_sum, team2_sum)
}

// https://www.codewars.com/kata/5ac6932b2f317b96980000ca
fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();
    digits.iter().fold(0i32, |acc, &item| acc * 10  + item)
}

// https://www.codewars.com/kata/5648b12ce68d9daa6b000099
fn passengers_in_bus(bus_stops:&[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |sum, &(ppl_in, ppl_out)| sum + ppl_in - ppl_out)
}