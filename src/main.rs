use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};
use regex::Regex;

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

fn generate_primes(bound: u64) -> Vec<u64> {
    let mut primes: Vec<bool> = (0..bound + 1).map(|num| num == 2 || num & 1 != 0).collect();
    let mut num = 3u64;
    while num * num <= bound {
        let mut j: u64 = num * num;
        while j <= bound {
            primes[j as usize] = false;
            j += num;
        }
        num += 2;
    }
    primes.into_iter().enumerate().skip(2).filter_map(|(i, p)| if p {Some(i as u64)} else {None}).collect::<Vec<u64>>()
}

fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut b_prime_numbers: Vec<u64> = Vec::with_capacity((stop - start / 3) as usize);
    for n in start..stop + 1 {
        if is_prime(n) {
            let rn = reverse(n);
            if rn != n && is_prime(rn) {
                b_prime_numbers.push(n);
            }
        }
    }
    b_prime_numbers
}

fn is_prime(n: u64)  -> bool {
    match PRIME_NUMBERS.binary_search(&n) {
        Ok(_) => return true,
        Err(i) if i < PRIME_NUMBERS.len() => return false,
        _ => {
            for &pn in PRIME_NUMBERS.iter() {
                if n % pn == 0 { return false; }
            }
        }
    }
    true
}

fn reverse(n: u64) -> u64 {
    let mut nn = n.clone();
    let mut rn: u64 = 0;
    while nn > 0 {
        rn = rn * 10 + nn % 10;
        nn = nn / 10;
    }
    rn
}

static PRIME_NUMBERS: [u64; 551] = [
    2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,
    103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,
    199,211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,307,311,
    313,317,331,337,347,349,353,359,367,373,379,383,389,397,401,409,419,421,431,
    433,439,443,449,457,461,463,467,479,487,491,499,503,509,521,523,541,547,557,
    563,569,571,577,587,593,599,601,607,613,617,619,631,641,643,647,653,659,661,
    673,677,683,691,701,709,719,727,733,739,743,751,757,761,769,773,787,797,809,
    811,821,823,827,829,839,853,857,859,863,877,881,883,887,907,911,919,929,937,
    941,947,953,967,971,977,983,991,997,1009,1013,1019,1021,1031,1033,1039,1049,
    1051,1061,1063,1069,1087,1091,1093,1097,1103,1109,1117,1123,1129,1151,1153,
    1163,1171,1181,1187,1193,1201,1213,1217,1223,1229,1231,1237,1249,1259,1277,
    1279,1283,1289,1291,1297,1301,1303,1307,1319,1321,1327,1361,1367,1373,1381,
    1399,1409,1423,1427,1429,1433,1439,1447,1451,1453,1459,1471,1481,1483,1487,
    1489,1493,1499,1511,1523,1531,1543,1549,1553,1559,1567,1571,1579,1583,1597,
    1601,1607,1609,1613,1619,1621,1627,1637,1657,1663,1667,1669,1693,1697,1699,
    1709,1721,1723,1733,1741,1747,1753,1759,1777,1783,1787,1789,1801,1811,1823,
    1831,1847,1861,1867,1871,1873,1877,1879,1889,1901,1907,1913,1931,1933,1949,
    1951,1973,1979,1987,1993,1997,1999,2003,2011,2017,2027,2029,2039,2053,2063,
    2069,2081,2083,2087,2089,2099,2111,2113,2129,2131,2137,2141,2143,2153,2161,
    2179,2203,2207,2213,2221,2237,2239,2243,2251,2267,2269,2273,2281,2287,2293,
    2297,2309,2311,2333,2339,2341,2347,2351,2357,2371,2377,2381,2383,2389,2393,
    2399,2411,2417,2423,2437,2441,2447,2459,2467,2473,2477,2503,2521,2531,2539,
    2543,2549,2551,2557,2579,2591,2593,2609,2617,2621,2633,2647,2657,2659,2663,
    2671,2677,2683,2687,2689,2693,2699,2707,2711,2713,2719,2729,2731,2741,2749,
    2753,2767,2777,2789,2791,2797,2801,2803,2819,2833,2837,2843,2851,2857,2861,
    2879,2887,2897,2903,2909,2917,2927,2939,2953,2957,2963,2969,2971,2999,3001,
    3011,3019,3023,3037,3041,3049,3061,3067,3079,3083,3089,3109,3119,3121,3137,
    3163,3167,3169,3181,3187,3191,3203,3209,3217,3221,3229,3251,3253,3257,3259,
    3271,3299,3301,3307,3313,3319,3323,3329,3331,3343,3347,3359,3361,3371,3373,
    3389,3391,3407,3413,3433,3449,3457,3461,3463,3467,3469,3491,3499,3511,3517,
    3527,3529,3533,3539,3541,3547,3557,3559,3571,3581,3583,3593,3607,3613,3617,
    3623,3631,3637,3643,3659,3671,3673,3677,3691,3697,3701,3709,3719,3727,3733,
    3739,3761,3767,3769,3779,3793,3797,3803,3821,3823,3833,3847,3851,3853,3863,
    3877,3881,3889,3907,3911,3917,3919,3923,3929,3931,3943,3947,3967,3989,4001
];

fn backwards_prime_test(start: u64, stop: u64, exp: Vec<u64>) -> () {
    assert_eq!(backwards_prime(start, stop), exp)
}

#[test]
fn backwards_prime_tests() {
    let a = vec![13, 17, 31, 37, 71, 73, 79, 97];
    backwards_prime_test(1, 100, a);
    let a = vec![13, 17, 31];
    backwards_prime_test(1, 31, a);
    let a = vec![701, 709, 733, 739, 743, 751, 761, 769];
    backwards_prime_test(701, 799, a);
    let a = vec![1095047, 1095209, 1095319, 1095403];
    backwards_prime_test(1095000, 1095403, a);
    let a = vec![700000031];
    backwards_prime_test(700000008, 700000050, a);
}

// https://www.codewars.com/kata/56af1a20509ce5b9b000001e
struct Address<'a> {
    house_number: &'a str,
    street_and_town: &'a str,
    zipcode: &'a str,
}

fn travel(text: &str, zipcode: &str) -> String {
    let re = Regex::new(r"(\d+)\s(.+?)\s(\w{2}\s\d{5})").unwrap();
    let parts = re.captures_iter(text)
        .map(|caps| Address {
            house_number: caps.get(1).unwrap().as_str(),
            street_and_town: caps.get(2).unwrap().as_str(),
            zipcode: caps.get(3).unwrap().as_str()
        })
        .filter(|a| a.zipcode == zipcode)
        .fold((String::new(), String::new()), |mut parts, address| {
            if parts.0.len() > 0 {
                parts.0.push(',');
                parts.1.push(',');
            }
            parts.0.push_str(address.street_and_town);
            parts.1.push_str(address.house_number);
            parts
        });
    format!("{}:{}/{}", zipcode, parts.0, parts.1)
}

fn travel_test(r: &str, zipcode:&str, exp: &str) -> () {
    //println!("r:{:?}", r);
    println!(" zipcode:{:?}", zipcode);
    let ans = travel(r, zipcode);
    println!("actual: {:?}", ans);
    println!("expect: {:?}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
}

#[test]
fn travel_tests() {
    let r = &String::from("123 Main Street St. Louisville OH 43071, 432 Main Long Road St. Louisville OH 43071,786 High Street Pollocksville NY 56432,
      54 Holy Grail Street Niagara Town ZP 32908, 3200 Main Rd. Bern AE 56210,1 Gordon St. Atlanta RE 13000,
      10 Pussy Cat Rd. Chicago EX 34342, 10 Gordon St. Atlanta RE 13000, 58 Gordon Road Atlanta RE 13000,
      22 Tokyo Av. Tedmondville SW 43098, 674 Paris bd. Abbeville AA 45521, 10 Surta Alley Goodtown GG 30654,
      45 Holy Grail Al. Niagara Town ZP 32908, 320 Main Al. Bern AE 56210, 14 Gordon Park Atlanta RE 13000,
      100 Pussy Cat Rd. Chicago EX 34342, 2 Gordon St. Atlanta RE 13000, 5 Gordon Road Atlanta RE 13000,
      2200 Tokyo Av. Tedmondville SW 43098, 67 Paris St. Abbeville AA 45521, 11 Surta Avenue Goodtown GG 30654,
      45 Holy Grail Al. Niagara Town ZP 32918, 320 Main Al. Bern AE 56215, 14 Gordon Park Atlanta RE 13200,
      100 Pussy Cat Rd. Chicago EX 34345, 2 Gordon St. Atlanta RE 13222, 5 Gordon Road Atlanta RE 13001,
      2200 Tokyo Av. Tedmondville SW 43198, 67 Paris St. Abbeville AA 45522, 11 Surta Avenue Goodville GG 30655,
      2222 Tokyo Av. Tedmondville SW 43198, 670 Paris St. Abbeville AA 45522, 114 Surta Avenue Goodville GG 30655,
      2 Holy Grail Street Niagara Town ZP 32908, 3 Main Rd. Bern AE 56210, 77 Gordon St. Atlanta RE 13000,
      100 Pussy Cat Rd. Chicago OH 13201");
    travel_test(r, "AA 45522", "AA 45522:Paris St. Abbeville,Paris St. Abbeville/67,670");
    travel_test(r, "OH 430", "OH 430:/");
}

// https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b

#[derive(Clone)]
struct PartitionIter {
    pub n: u32,

    partition: Vec<u32>,
    last_not_one_index: usize,
    started: bool,
    finished: bool
}

impl PartitionIter {
    pub fn new(n: u32) -> PartitionIter {
        PartitionIter {
            n,
            partition: Vec::with_capacity(n as usize),
            last_not_one_index: 0,
            started: false,
            finished: false,
        }
    }
}

impl Iterator for PartitionIter {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None
        }

        if !self.started {
            self.partition.push(self.n);
            self.started = true;
            return Some(self.partition.clone());
        }

        if self.n == 1 {
            self.finished = true;
            return None;
        }

        if self.partition[self.last_not_one_index] == 2 {
            self.partition[self.last_not_one_index] = 1;
            self.partition.push(1);
            if self.last_not_one_index == 0 {
                self.finished = true;
            } else {
                self.last_not_one_index -= 1;
            }
            return Some(self.partition.clone())
        }
        let replacement = self.partition[self.last_not_one_index] - 1;
        let total_replaced = replacement + (self.partition.len() - self.last_not_one_index) as u32;
        let reps = total_replaced / replacement;
        let rest = total_replaced % replacement;
        self.partition.drain(self.last_not_one_index..);
        self.partition.extend_from_slice(&vec![replacement; reps as usize]);
        if rest > 0 {
            self.partition.push(rest);
        }
        self.last_not_one_index = self.partition.len() - (self.partition.last().cloned().unwrap() == 1) as usize  - 1;
        Some(self.partition.clone())
    }
}

fn int_part(n: u32) -> String {
    let mut products_acc: HashSet<u32> = HashSet::new();
    let partitions = PartitionIter::new(n);
    for p in partitions {
        products_acc.insert(p.iter().product());
    }
    let mut products: Vec<u32> = products_acc.iter().map(|&i| i).collect();
    products.sort();
    let range = products.last().unwrap() - products.first().unwrap();
    let sum: u32 = products.iter().sum();
    let average: f32 = sum as f32 / products.len() as f32;
    let m = products.len() / 2;
    let median = match products.len() % 2 == 0 {
        true => (products[m - 1] + products[m]) as f32 / 2f32,
        false => products[m] as f32
    };
    format!("Range: {} Average: {:.2} Median: {:.2}", range, average, median)
}

fn part_test(ans: &str, sol: &str) {
    assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
}

#[test]
fn part_tests() {
    part_test(&int_part(1), "Range: 0 Average: 1.00 Median: 1.00");
    part_test(&int_part(2), "Range: 1 Average: 1.50 Median: 1.50");
    part_test(&int_part(3), "Range: 2 Average: 2.00 Median: 2.00");
    part_test(&int_part(4), "Range: 3 Average: 2.50 Median: 2.50");
    part_test(&int_part(5), "Range: 5 Average: 3.50 Median: 3.50");
    part_test(&int_part(6), "Range: 8 Average: 4.75 Median: 4.50");
}