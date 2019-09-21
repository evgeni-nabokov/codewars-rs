use std::cmp;
use std::cmp::Ordering::*;
use std::iter;
use std::collections::{HashMap, HashSet};
use regex::{Regex, Match};
use super::common::*;

#[cfg(test)]
mod tests;

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

// https://www.codewars.com/kata/59df2f8f08c6cec835000012
fn meeting(s: &str) -> String {
    let upper_s = s.to_uppercase();
    let mut parsed_s = upper_s.split(';').map(|x| x.split(':').collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
    parsed_s.sort_unstable_by(|x, y| match x[1].cmp(y[1]) {
        Equal => x[0].cmp(y[0]),
        not_equal => not_equal
    });
    parsed_s.iter().map(|x| format!("({}, {})", x[1], x[0])).collect::<String>()
}

// https://www.codewars.com/kata/56a5d994ac971f1ac500003e
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.is_empty() || k <= 0 || k > strarr.len() { return String::new() }

    // Initialization by first k elements.
    let mut sliding_sum: usize = strarr.iter().take(k).map(|s| s.len()).sum();
    let mut max_sum = sliding_sum;
    let mut max_sum_start_index = 0;
    sliding_sum -= strarr[0].len();

    for i in k..strarr.len() {
        sliding_sum += strarr[i].len();
        let start_index = i - k + 1;
        if max_sum < sliding_sum {
            max_sum = sliding_sum;
            max_sum_start_index = start_index;
        }
        sliding_sum -= strarr[start_index].len();
    }
    let result: Vec<&str> = strarr.iter().skip(max_sum_start_index).take(k).cloned().collect();
    result.concat()
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

// https://www.codewars.com/kata/5539fecef69c483c5a000015
fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut b_prime_numbers: Vec<u64> = Vec::with_capacity((stop - start / 3) as usize);
    for n in start..=stop {
        if is_prime(&n) {
            let rn = reverse(&n);
            if rn != n && is_prime(&rn) {
                b_prime_numbers.push(n);
            }
        }
    }
    b_prime_numbers
}

fn reverse(n: &u64) -> u64 {
    let mut nn = n.clone();
    let mut rn: u64 = 0;
    while nn > 0 {
        rn = rn * 10 + nn % 10;
        nn = nn / 10;
    }
    rn
}

// https://www.codewars.com/kata/56af1a20509ce5b9b000001e
fn travel(text: &str, zip_code: &str) -> String {
    let re = Regex::new(r"(\d+)\s(.+?)\s(\w{2}\s\d{5})").unwrap();
    let parts = re.captures_iter(text)
        .map(|caps| Address {
            house_number: caps.get(1).unwrap().as_str(),
            street_and_town: caps.get(2).unwrap().as_str(),
            zip_code: caps.get(3).unwrap().as_str()
        })
        .filter(|a| a.zip_code == zip_code)
        .fold((String::new(), String::new()), |mut parts, address| {
            if parts.0.len() > 0 {
                parts.0.push(',');
                parts.1.push(',');
            }
            parts.0.push_str(address.street_and_town);
            parts.1.push_str(address.house_number);
            parts
        });
    format!("{}:{}/{}", zip_code, parts.0, parts.1)
}

struct Address<'a> {
    house_number: &'a str,
    street_and_town: &'a str,
    zip_code: &'a str,
}

// https://www.codewars.com/kata/52761ee4cffbc69732000738
fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_races_worth: [u32; 6] = [1, 2, 3, 3, 4, 10];
    let evil_races_worth: [u32; 7] = [1, 2, 2, 2, 3, 5, 10];

    let good_worth_sum = calc_worth_sum(&good_races_worth, good);
    let evil_worth_sum = calc_worth_sum(&evil_races_worth, evil);
    String::from(match good_worth_sum.cmp(&evil_worth_sum) {
        Greater => "Battle Result: Good triumphs over Evil",
        Less => "Battle Result: Evil eradicates all trace of Good",
        Equal => "Battle Result: No victor on this battle field",
    })
}

fn calc_worth_sum(races_worth: &[u32], army: &str) -> u32 {
    let mut sum: u32 = 0;
    for (index, str_worth) in army.split_whitespace().enumerate() {
        let race_count = str_worth.parse::<u32>().unwrap();
        sum += race_count * races_worth[index];
    }
    sum
}

// https://www.codewars.com/kata/58184387d14fc32f2b0012b2
fn fp_approx(x: f64) -> f64 {
    // First 4 members of the Maclaurin series of the function sqrt(1 + x) - 1.
    x / 2.0 * (1.0 - x / 4.0 + x * x / 8.0)
}

// https://www.codewars.com/kata/54b724efac3d5402db00065e
 struct MorseDecoder {
     morse_code: HashMap<String, String>,
 }

impl MorseDecoder {
    pub fn new() -> MorseDecoder {
        MorseDecoder {
            morse_code: [
                ("-----", "0"), (".----", "1"), ("..---", "2"), ("...--", "3"), ("....-", "4"),
                (".....", "5"), ("-....", "6"), ("--...", "7"), ("---..", "8"), ("----.", "9"),
                (".-", "A"), ("-...", "B"), ("-.-.", "C"), ("-..", "D"), (".", "E"), ("..-.", "F"),
                ("--.", "G"), ("....", "H"), ("..", "I"), (".---", "J"), ("-.-", "K"), (".-..", "L"),
                ("--", "M"), ("-.", "N"), ("---", "O"), (".--.", "P"), ("--.-", "Q"), (".-.", "R"),
                ("...", "S"), ("-", "T"), ("..-", "U"), ("...-", "V"), (".--", "W"), ("-..-", "X"),
                ("-.--", "Y"), ("--..", "Z")
            ].iter().map(|(x, y)| (x.to_string(), y.to_string())).collect()
        }
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        encode.trim().split("   ").map(|w| w.split_ascii_whitespace()
            .map(|code| self.morse_code.get(code).unwrap().to_owned())
            .collect::<Vec<String>>()
            .join("")
        ).collect::<Vec<String>>()
        .join(" ")
    }
}