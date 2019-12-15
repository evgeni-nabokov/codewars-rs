use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};
use std::char::from_digit;
use std::fmt;
use regex::{Regex, Match};
use super::common::*;

#[cfg(test)]
mod tests;
// https://www.codewars.com/kata/54d81488b981293527000c8f
fn sum_pairs(numbers: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut cache: HashSet<i8> = HashSet::new();
    cache.insert(numbers[0]);
    for i in 1..numbers.len() {
        let a = s - numbers[i];
        if cache.contains(&a) {
            return Some((a, numbers[i]))
        } else {
            cache.insert(numbers[i]);
        }
    }
    None
}

fn sum_pairs_naive(numbers: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut left: usize = numbers.len() - 1;
    let mut right: usize = numbers.len() - 1;
    let mut i: usize = 0;
    while i < right {
        for j in (i + 1)..=right {
            if numbers[i] + numbers[j] == s {
                if j < right || (j == right && i < left) {
                    left = i;
                    right = j;
                }
                break;
            }
        }
        i += 1;
    }
    if left != right {
        Some((numbers[left], numbers[right]))
    } else {
        return None;
    }
}

fn sum_pairs_slow(numbers: &[i8], s: i8) -> Option<(i8, i8)> {
    for right in 1..numbers.len() {
        for left in 0..right {
            if numbers[left] + numbers[right] == s {
                return Some((numbers[left], numbers[right]));
            }
        }
    }
    None
}

// https://www.codewars.com/kata/561e9c843a2ef5a40c0000a4
fn prime_gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    'outer: for l in m..=(n - g as u64) {
        if !is_prime(&l) { continue; }
        let r = l + g as u64;
        if !is_prime(&r) { continue; }
        for k in (l + 1)..(r) {
            if is_prime(&k) { continue 'outer; }
        }
        return Some((l, r));
    }
    None
}

// https://www.codewars.com/kata/5541f58a944b85ce6d00006a
fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut f0 = 0u64;
    let mut f1 = 1u64;
    while f0 * f1 < prod {
        let tmp = f1;
        f1 += f0;
        f0 = tmp;
    }
    (f0, f1, f0 * f1 == prod)
}

// https://www.codewars.com/kata/5726f813c8dcebf5ed000a6b
// Finds prime factorization. If a number is prime, returns an empty Vec.
// I made it as fast as I could.
fn get_prime_factors(n: &u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    // Fast lookup.
    if PRIME_NUMBERS.binary_search(n).is_ok() {
        res.push(*n);
        return res;
    }
    let mut cur_factor = n.to_owned();
    let mut cur_pn = *PRIME_NUMBERS.first().unwrap();
    let last_pn = *PRIME_NUMBERS.last().unwrap();
    let mut pn_index = 0;
    loop {
        if cur_factor % cur_pn != 0 {
            if cur_pn == last_pn {
                panic!("Reached the PRIME_NUMBERS' limit.");
            }
            pn_index += 1;
            cur_pn = PRIME_NUMBERS[pn_index];
            if cur_pn * cur_pn > cur_factor {
                res.push(cur_factor);
                break;
            }
            continue;
        }
        res.push(cur_pn);
        if cur_pn == cur_factor {
            break;
        }
        cur_factor /= cur_pn;
        // Fast lookup.
        if PRIME_NUMBERS.binary_search(&cur_factor).is_ok() {
            res.push(cur_factor);
            return res;
        }
    }
    res
}

fn get_primes(start: u64, end: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    for n in start..=end {
        if is_prime(&n) {
            res.push(n);
        }
    }
    res
}

fn get_kprimes(k: usize, start: u64, end: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    for n in start..=end {
        let prime_factors = get_prime_factors(&n);
        if prime_factors.len() == k {
            res.push(n);
        }
    }
    res
}

fn puzzle(s: u64) -> u8 {
    let mut counter = 0u8;
    for &c in get_kprimes(7, 2, s - 2).iter() {
        for &b in get_kprimes(3, 2, s - 2).iter() {
            for &a in get_primes(2, s - 2).iter() {
                if a + b + c == s {
                    counter += 1;
                }
            }
        }
    }
    counter
}

// https://www.codewars.com/kata/5511b2f550906349a70004e1
fn last_digit(str1: &str, str2: &str) -> u8 {
    if str2 == "0" {
        return 1;
    }

    let last_digit1 = str1.chars().last().unwrap().to_digit(10).unwrap() as usize;
    let last_two_digits2 = if str2.len() <= 2 {
        str2.parse::<u32>().unwrap()
    } else {
        str2.chars()
            .skip(str2.len() - 2)
            .take(2)
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    } as usize;

    LAST_DIGITS[last_digit1][last_two_digits2 % 4]
}

// https://www.codewars.com/kata/59ccf051dcc4050f7800008f
fn buddy_numbers(start: u64, limit: u64) -> Option<(u64, u64)> {
    for n in start..=limit {
        if is_prime(&n) {
            continue;
        }
        let n_sum = get_sum_of_factors(&n);
        if n_sum <= n + 1 {
            continue;
        }
        let m = n_sum - 1;
        let m_sum = get_sum_of_factors(&m);
        if m_sum == n + 1 {
            return Some((n, m))
        }
    }
    None
}

// Calculates sum of factors of a number using prime factorization.
fn get_sum_of_factors(n: &u64) -> u64 {
    let mut prime_factors: Vec<u64> = get_prime_factors(&n);
    if prime_factors.len() == 1 {
        return 1;
    }
    prime_factors.push(0); // A stub for one more iteration.
    let mut res = 1u64;
    let mut sum_of_powers = 1;
    let mut prev_factor = prime_factors[0];
    let mut power = 1;
    for &factor in prime_factors.iter() {
        if prev_factor == factor {
            power *= factor;
            sum_of_powers += power;
        } else {
            res *= sum_of_powers;
            power = factor;
            sum_of_powers = 1 + factor;
        }
        prev_factor = factor;
    }
    res - *n  // Do not include the number itself.
}

fn get_sum_of_factors_slow(n: &u64) -> u64 {
    let mut res = 0u64;
    for f in 1..=(n / 2) {
        if n % f == 0 {
            res += f;
        }
    }
    res
}

fn get_factors_slow(n: &u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    for f in 1..=(n / 2) {
        if n % f == 0 {
            res.push(f);
        }
    }
    res
}

// https://www.codewars.com/kata/5b1cd19fcd206af728000056
// The sum of the power series n*x^n is equal to x / (1 - x)^2.
fn find_x_of_pseries(m: f64) -> f64 {
    (2f64 * m + 1f64 - (4f64 * m + 1f64).sqrt()) / 2f64 / m
}

// https://www.codewars.com/kata/55a29405bc7d2efaff00007c
fn sum_of_series(n: i32) -> f64 {
    let mut denominator = 1.0;
    let mut sum = 1.0;
    for i in 1..n {
        denominator *= (n - i + 1) as f64;
        sum += 1.0 / denominator;
    }
    // Truncating the sum.
    let formatted_sum = format!("{:.6}", sum);
    formatted_sum.parse().unwrap()
}

// https://www.codewars.com/kata/56baeae7022c16dd7400086e
fn phone_directory(dir: &str, num: &str) -> String {
    let phone_re = Regex::new(r"\d\d?-\d{3}-\d{3}-\d{4}").unwrap();
    let mut matched_line: Option<(Match, &str)> = None;
    for line in dir.lines() {
        // Extracting phone number.
        let phone_match = phone_re.captures(line).unwrap().get(0).unwrap();
        let phone = phone_match.as_str();
        if num.cmp(phone) != Equal {
            continue;
        }
        if matched_line.is_none() {
            matched_line = Some((phone_match, line));
        } else {
            return format!("Error => Too many people: {}", num);
        }
    }

    if let Some((phone_match, line)) = matched_line {
        // Extracting name.
        let name_re = Regex::new(r"<.+?>").unwrap();
        let name_cap = name_re.captures(line).unwrap().get(0).unwrap();
        let name_len = name_cap.end() - name_cap.start();
        let name = &name_cap.as_str()[1..name_len - 1];

        // Extracting address.
        let (first_cap, second_cap) = if phone_match.start() < name_cap.start() {
            (phone_match, name_cap)
        } else {
            (name_cap, phone_match)
        };
        let mut address = format!("{}{}{}",
                                  &line[0..first_cap.start()],
                                  &line[first_cap.end()..second_cap.start()],
                                  &line[second_cap.end()..line.len()]);

        // Clearing address.
        let garbage_re = Regex::new(r"[/!;*+$]").unwrap();
        let multiple_commas = Regex::new(r"(?:\s*,\s*)+").unwrap();
        let underscore_re = Regex::new(r"_").unwrap();
        let multiple_spaces_re = Regex::new(r"\s+").unwrap();
        address = garbage_re.replace_all(&address, "").into_owned();
        address = multiple_commas.replace_all(&address, ", ").into_owned();
        address = underscore_re.replace_all(&address, " ").into_owned();
        address = multiple_spaces_re.replace_all(&address, " ").into_owned();
        address = address.trim().to_string();

        format!("Phone => {}, Name => {}, Address => {}", phone_match.as_str(), name, address)
    }  else {
        return format!("Error => Not found: {}", num);
    }
}

// https://www.codewars.com/kata/573182c405d14db0da00064e
fn consec_kprimes(k: usize, arr: Vec<u64>) -> u64 {
    let mut counter = 0;
    let mut prev_is_kprime = false;
    for n in arr.iter() {
        let prime_factors = get_prime_factors(n);
        if prime_factors.len() == k {
            if prev_is_kprime {
                counter += 1;
            } else {
                prev_is_kprime = true;
            }
        } else {
            prev_is_kprime = false;
        }
    }
    counter
}

// https://www.codewars.com/kata/5a045fee46d843effa000070
fn factorial_decomp(n: u64) -> String {
    // A trivial corner case.
    if n == 1 {
        return "1".to_string();
    }
    let mut all_prime_factors: Vec<u64> = Vec::new();
    for i in 2..=n {
        all_prime_factors.extend_from_slice(&get_prime_factors(&i));
    }
    all_prime_factors.sort_unstable();
    all_prime_factors.push(0);  // A stub for one more iteration.
    let mut prev_factor = all_prime_factors[0];
    let mut exp = 1u64;
    let mut res: Vec<String> = Vec::new();
    for &factor in all_prime_factors.iter().skip(1) {
        if factor == prev_factor {
            exp += 1;
        } else {
            if exp == 1 {
                res.push(format!("{}", prev_factor));
            } else {
                res.push(format!("{}^{}", prev_factor, exp));
            }
            prev_factor = factor;
            exp = 1;
        }
    }
    res.join(" * ")
}

// https://www.codewars.com/kata/54d512e62a5e54c96200019e
fn prime_factors(n: u64) -> String {
    // A trivial corner case.
    if n == 1 {
        return "(1)".to_string();
    }
    let mut prime_factors = get_prime_factors(&n);
    // n is a prime number.
    if prime_factors.len() == 1 {
        return format!("({})", n);
    }
    prime_factors.push(0);  // A stub for one more iteration.
    let mut prev_factor = prime_factors[0];
    let mut exp = 1u64;
    let mut res: Vec<String> = Vec::new();
    for &factor in prime_factors.iter().skip(1) {
        if factor == prev_factor {
            exp += 1;
        } else {
            if exp == 1 {
                res.push(format!("({})", prev_factor));
            } else {
                res.push(format!("({}**{})", prev_factor, exp));
            }
            prev_factor = factor;
            exp = 1;
        }
    }
    res.join("")
}

// https://www.codewars.com/kata/513e08acc600c94f01000001
fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}",
            convert_to_byte(r),
            convert_to_byte(g),
            convert_to_byte(b))
}

fn convert_to_byte(c: i32) -> u8 {
    if c < 0 { 0 } else { if c > 255 { 255 } else { c as u8 } }
}

// https://www.codewars.com/kata/559a28007caad2ac4e000083
fn perimeter(n: u64) -> u64 {
    let mut f0 = 0;
    let mut f1 = 1;
    let mut sum = f0 + f1;
    for _ in 2..=(n + 1) {
        let tmp = f1;
        f1 += f0;
        f0 = tmp;
        sum += f1;
    }
    4 * sum
}

// https://www.codewars.com/kata/57591ef494aba64d14000526
fn john(n: u32) -> Vec<u32> {
    john_ann(&n).0
}

fn ann(n: u32) -> Vec<u32> {
    john_ann(&n).1
}

fn sum_john(n: u32) -> u32 {
    john(n).iter().sum()
}

fn sum_ann(n: u32) -> u32 {
    ann(n).iter().sum()
}

fn john_ann(n: &u32) -> (Vec<u32>, Vec<u32>) {
    let mut john_series = vec![0];
    let mut ann_series = vec![1];
    for day in 1..*n {
        john_series.push(day - ann_series[john_series[day as usize - 1] as usize]);
        ann_series.push(day - john_series[ann_series[day as usize - 1] as usize]);
    }
    (john_series, ann_series)
}

// https://www.codewars.com/kata/54d7660d2daf68c619000d95
fn convert_fracts(mut list: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut all_prime_factors: HashMap<u64, u32> = HashMap::new();
    for (n1, n2) in list.iter_mut() {
        let mut n1_prime_factors = get_prime_factors_with_power(n1);
        let mut n2_prime_factors = get_prime_factors_with_power(n2);
        for (pf, n2_pwr) in n2_prime_factors.iter_mut() {
            // Reducing a fraction.
            if let Some(n1_pwr) = n1_prime_factors.get_mut(pf) {
                if *n2_pwr > *n1_pwr {
                    *n2_pwr -= *n1_pwr;
                    *n1_pwr = 0;
                } else {
                    *n1_pwr -= *n2_pwr;
                    *n2_pwr = 0;
                }
            }
            // Updating an existing prime factor or inserting the new.
            if let Some(all_pwr) = all_prime_factors.get_mut(pf) {
                if *all_pwr < *n2_pwr {
                    *all_pwr = *n2_pwr;
                }
            } else if *n2_pwr > 0 {
                all_prime_factors.insert(pf.to_owned(), n2_pwr.to_owned());
            }
        }
    }
    let lcm = all_prime_factors.iter().fold(1u64, |acc, (&pf, &pwr)| acc * pf.pow(pwr));
    list.iter().map(|(n1, n2)| (*n1 * lcm / * n2, lcm)).collect::<Vec<_>>()
}

fn get_prime_factors_with_power(n: &u64) -> HashMap<u64, u32> {
    let mut res = HashMap::new();
    let mut prime_factors = get_prime_factors(n);
    if prime_factors.len() == 1 {
        res.insert(*n, 1);
        return res;
    }
    prime_factors.push(0); // A stub for one more iteration.
    let mut pwr = 0;
    let mut prev_factor = prime_factors[0];
    for &factor in prime_factors.iter() {
        if prev_factor == factor {
            pwr += 1;
        } else {
            res.insert(prev_factor, pwr);
            pwr = 1;
        }
        prev_factor = factor;
    }
    res
}

// https://www.codewars.com/kata/55c6126177c9441a570000cc
fn order_weight(s: &str) -> String {
    let mut weights = s.split_ascii_whitespace()
        .map(|x| (x.chars().map(|c| c.to_digit(10).unwrap()).sum(), x.to_string()))
        .collect::<Vec<(u32, String)>>();
    weights.sort_unstable_by(|x, y| match x.0.cmp(&y.0) {
        Equal => x.1.cmp(&y.1),
        z => z
    });
    weights.iter().map(|x| x.1.clone()).collect::<Vec<_>>().join(" ")
}

// https://www.codewars.com/kata/58ee4962dc4f81d6f400001c
#[derive(Copy, Clone, Debug)]
struct Vector {
    pub i: f64,
    pub j: f64,
    pub k: f64
}

impl Vector {
    const EPS: f64 = 0.001;

    pub fn get_i() -> Vector {
        Vector::new(1.0, 0.0, 0.0)
    }

    pub fn get_j() -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    pub fn get_k() -> Vector {
        Vector::new(0.0, 0.0, 1.0)
    }

    pub fn new(i: f64, j: f64, k: f64) -> Vector {
        Vector { i, j, k }
    }

    pub fn get_magnitude(&self) -> f64 {
        (self.i.powf(2.0) + self.j.powf(2.0) + self.k.powf(2.0)).sqrt()
    }

    pub fn add(&self, another: Vector) -> Vector {
        Vector::new(self.i + another.i, self.j + another.j, self.k + another.k)
    }

    pub fn multiply_by_scalar(&self, multiplier: f64) -> Vector {
        Vector::new(self.i * multiplier, self.j * multiplier, self.k * multiplier)
    }

    pub fn dot(&self, another: Vector) -> f64 {
        self.i * another.i + self.j * another.j + self.k * another.k
    }

    pub fn cross(&self, another: Vector) -> Vector {
        Vector::new(
            self.j * another.k - self.k * another.j,
            self.k * another.i - self.i * another.k,
            self.i * another.j - self.j * another.i
        )
    }

    pub fn is_parallel_to(&self, another: Vector) -> bool {
        if self.is_zero() || another.is_zero() {
            return false;
        }

        let s = [self.i.abs(), self.j.abs(), self.k.abs()];
        let a = [another.i.abs(), another.j.abs(), another.k.abs()];
        let mut l: Vec<f64> = Vec::with_capacity(3);
        for (&x1, &x2) in s.iter().zip(a.iter()) {
            if (x1 < Vector::EPS && x2 > Vector::EPS) || (x1 > Vector::EPS && x2 < Vector::EPS) {
                return false;
            }
            if x2 < Vector::EPS {
                continue;
            }
            l.push(x1 / x2);
        }
        if l.len() == 1 {
            return true;
        }
        l.windows(2).all(|w| (w[0] - w[1]).abs() < Vector::EPS)
    }

    pub fn is_perpendicular_to(&self, another: Vector) -> bool {
        if self.is_zero() || another.is_zero() {
            return false;
        }
        self.dot(another).abs() < Vector::EPS
    }

    pub fn normalize(&self) -> Vector {
        let m = self.get_magnitude();
        Vector::new(self.i / m, self.j / m, self.k / m)
    }

    pub fn is_normalized(&self) -> bool {
        self.get_magnitude() - 1.0 < Vector::EPS
    }

    fn is_equal(&self, another: Vector) -> bool {
        self.add(another.multiply_by_scalar(-1.0)).is_zero()
    }

    fn is_zero(&self) -> bool {
        self.i.abs() < Vector::EPS.abs() && self.j.abs() < Vector::EPS && self.k.abs() < Vector::EPS
    }
}

// https://www.codewars.com/kata/54e320dcebe1e583250008fd
pub fn dec2_fact_string(mut n: u64) -> String {
    let mut digits = Vec::new();
    let mut radix = 1;
    while n != 0 {
        digits.push(n % radix);
        n /= radix;
        radix += 1;
    }
    digits
        .into_iter()
        .rev()
        .map(|d| from_digit(d as u32, 36).unwrap().to_ascii_uppercase())
        .collect()
}

fn fact_string_2dec(s: String) -> u64 {
    let mut radix = 1u64;
    let mut base = 1u64;
    let mut res = 0u64;
    for ch in s.chars().rev().skip(1) {
        base *= radix;
        res += base * ch.to_digit(36).unwrap() as u64;
        radix += 1;
    }
    res
}

// https://www.codewars.com/kata/59de1e2fe50813a046000124
struct ProgramInfo {
    pub program: String,
    pub author: String,
    pub phone: String,
    pub date: String,
    pub version: String
}

impl fmt::Display for ProgramInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("Program: ").ok();
        fmt.write_str(self.program.as_str()).ok();
        fmt.write_str(" Author: ").ok();
        fmt.write_str(self.author.as_str()).ok();
        fmt.write_str(" Phone: ").ok();
        fmt.write_str(self.phone.as_str()).ok();
        fmt.write_str(" Date: ").ok();
        fmt.write_str(self.date.as_str()).ok();
        fmt.write_str(" Version: ").ok();
        fmt.write_str(self.version.as_str()).ok();
        Ok(())
    }
}

fn match_and_substitute(s: &str, prog: &str, ver: &str) -> String {
    let error_message = "ERROR: VERSION or PHONE".to_string();
    let lines = s.lines().collect::<Vec<&str>>();
    let phone = lines[3].split_ascii_whitespace().collect::<Vec<&str>>().last().unwrap().to_owned();
    let phone_re = Regex::new(r"\+1-\d{3}-\d{3}-\d{4}").unwrap();
    if !phone_re.is_match(phone) {
        return error_message
    }
    let version = lines[5].split_ascii_whitespace().collect::<Vec<&str>>().last().unwrap().to_owned();
    let version_re = Regex::new(r"^\d+\.\d+$").unwrap();
    if !version_re.is_match(version) {
        return error_message;
    }

    let p_info = ProgramInfo {
        program: prog.to_string(),
        author: "g964".to_string(),
        date: "2019-01-01".to_string(),
        version: (if version == "2.0" { version } else { ver }).to_string(),
        phone: "+1-503-555-0090".to_string()
    };
    p_info.to_string()
}

// https://www.codewars.com/kata/550f22f4d758534c1100025a
#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    NORTH, SOUTH, EAST, WEST
}

fn dir_reduction(arr: &[Direction]) -> Vec<Direction> {
    use Direction::*;
    let mut res: Vec<Direction> = Vec::new();
    for &d in arr.iter() {
        if res.is_empty() {
            res.push(d);
            continue;
        }
        match (d, res.last().unwrap()) {
            (NORTH, SOUTH) | (SOUTH, NORTH) | (EAST, WEST) | (WEST, EAST) => { res.pop(); },
            _ => { res.push(d); }
        }
    }
    res
}