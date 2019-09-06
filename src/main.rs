use std::cmp;
use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};
use std::iter;
use regex::Regex;

#[cfg(test)]
mod tests;

fn main() {
}

const PRIME_NUMBERS: [u64; 551] = [
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

// https://www.codewars.com/kata/5ac6932b2f317b96980000ca
fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();
    digits.iter().fold(0i32, | acc, &item| acc * 10  + item)
}

// https://www.codewars.com/kata/5648b12ce68d9daa6b000099
fn passengers_in_bus(bus_stops:&[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |sum, &(ppl_in, ppl_out)| sum + ppl_in - ppl_out)
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
    for n in start..stop + 1 {
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

fn is_prime(n: &u64) -> bool {
    match PRIME_NUMBERS.binary_search(n) {
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

// https://www.codewars.com/kata/54d81488b981293527000c8f
fn sum_pairs_naive(numbers: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut left: usize = numbers.len() - 1;
    let mut right: usize = numbers.len() - 1;
    let mut i: usize = 0;
    while i < right {
        for j in (i + 1)..right + 1 {
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

// https://www.codewars.com/kata/561e9c843a2ef5a40c0000a4
fn prime_gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    'outer: for l in m..(n - g as u64 + 1) {
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
        } else if self.n == 1 {
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
        self.last_not_one_index = self.partition.len() - (self.partition.last().cloned().unwrap() == 1) as usize - 1;
        Some(self.partition.clone())
    }
}

fn int_part(n: u32) -> String {
    let mut products: Vec<u32> = PartitionIter::new(n).map(|x| x.iter().product()).collect();
    products.sort();
    products.dedup();
    let range = products.last().unwrap() - products.first().unwrap();
    let sum: u32 = products.iter().sum();
    let average: f32 = sum as f32 / products.len() as f32;
    let middle = products.len() / 2;
    let median = match products.len() % 2 == 0 {
        true => (products[middle - 1] + products[middle]) as f32 / 2f32,
        false => products[middle] as f32
    };
    format!("Range: {} Average: {:.2} Median: {:.2}", range, average, median)
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

fn longest_consec_test(strarr: Vec<&str>, k: usize, exp: &str) {
    assert_eq!(&longest_consec(strarr, k), exp)
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

// https://www.codewars.com/kata/5629db57620258aa9d000014
fn count_and_filter_chars(s: &str) -> HashMap<char, usize> {
    let mut map = s.chars()
        .filter(|c| c.is_ascii_lowercase())
        .fold(HashMap::new(), |mut acc, item| {
            acc.entry(item).and_modify(|c| *c += 1).or_insert(1);
            acc
        });
    map.retain(|_, c| *c > 1);
    map
}

fn mix(s1: &str, s2: &str) -> String {
    let map1 = count_and_filter_chars(s1);
    let map2 = count_and_filter_chars(s2);
    let mut all_letters: HashSet<char> = HashSet::new();
    all_letters.extend(map1.keys());
    all_letters.extend(map2.keys());
    let mut list: Vec<(u8, usize, char)> = Vec::with_capacity(all_letters.len());
    for ch in all_letters {
        let c1 = map1.get(&ch).unwrap_or(&0usize).clone();
        let c2 = map2.get(&ch).unwrap_or(&0usize).clone();
        list.push((
            // Selected string number: 1, or 2, or 3 (i.e. '=').
            match c1.cmp(&c2) {
                Greater => 1,
                Less => 2,
                Equal => 3
            },
            // Count.
            cmp::max(c1, c2),
            // Char.
            ch
        ));
    }
    // Sorting by 1) Count (desc), 2) String number (asc), 3) Char (asc).
    list.sort_unstable_by(|a, b|
        match b.1.cmp(&a.1) {
            Equal => match a.0.cmp(&b.0) {
                Equal => a.2.cmp(&b.2),
                a => a
            },
            b => b
        });
    let result = list.iter()
        .map(|x| format!("{}:{}",
                         match x.0 { 1 => "1", 2 => "2", _ => "=" },
                         iter::repeat(x.2).take(x.1).collect::<String>(),
        ))
        .collect::<Vec<String>>()
        .join("/");
    result
}

// https://www.codewars.com/kata/5726f813c8dcebf5ed000a6b
fn get_prime_factors(n: &u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    if let Ok(i) = PRIME_NUMBERS.binary_search(n) {
        res.push(PRIME_NUMBERS[i]);
        return res;
    }

    for &pn in PRIME_NUMBERS.iter() {
        if n % pn != 0 { continue; }
        res.push(pn);
        res.extend(get_prime_factors(&(n / pn)).iter());
        break;
    }
    res
}

fn get_primes(start: u64, end: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    for n in start..(end + 1) {
        if is_prime(&n) {
            res.push(n);
        }
    }
    res
}

fn get_kprimes(k: usize, start: u64, end: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    for n in start..(end + 1) {
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
const LAST_DIGITS: [[u8; 4]; 10] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [6, 2, 4, 8],
    [1, 3, 9, 7],
    [6, 4, 6, 4],
    [5, 5, 5, 5],
    [6, 6, 6, 6],
    [1, 7, 9, 3],
    [6, 8, 4, 2],
    [1, 9, 1, 9],
];

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

// https://www.codewars.com/kata/529a92d9aba78c356b000353
#[derive(Debug, PartialEq, Eq)]
enum Cons<T: Clone> {
    Cons(T, Box<Cons<T>>),
    Null
}

impl<T: Clone> Cons<T> {
    pub fn new(head: T, tail: Self) -> Self {
        Cons::Cons(head, Box::new(tail))
    }
}

impl<T: Clone> Cons<T> {
    pub fn to_vec(&self) -> Vec<T> {
        match self {
            &Cons::Null => vec![],
            &Cons::Cons(ref head, ref tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());
                head
            }
        }
    }
}

impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(it: I) -> Self
        where I: IntoIterator<Item=T> {

        let mut it = it.into_iter();
        match it.next() {
            Some(v) => Cons::Cons(v, Box::new(Self::from_iter(it))),
            None => Cons::Null
        }
    }

    pub fn filter<F>(&self, fun: F) -> Self
        where F: Fn(&T) -> bool {
        match self {
            &Cons::Null => Cons::Null,
            &Cons::Cons(ref head, ref tail) => if fun(head) {
                Cons::new(head.clone(), tail.filter(fun))
            } else {
                tail.filter(fun)
            }
        }
    }

    pub fn map<F,S>(&self, fun: F) -> Cons<S>
        where F: Fn(T) -> S, S: Clone {
        match self {
            &Cons::Null => Cons::Null,
            &Cons::Cons(ref head, ref tail) =>
                Cons::new(fun(head.clone()), tail.map(fun))
        }
    }
}

// https://www.codewars.com/kata/5ae62fcf252e66d44d00008e
fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    let nums = [a * b * c, a + b + c, (a + b) * c, a * (b + c)];
    *nums.iter().max().unwrap()
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

// https://www.codewars.com/kata/5518a860a73e708c0a000027
fn modpow(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut base = base.clone();
    let mut exp = exp.clone();
    let mut res = 1;

    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % modulus;
        }
        base = (base * base) % modulus;
        exp = exp >> 1;
    }
    res
}

// Builds a new list while power > 1.
fn try_to_reduce_list(list: &[u64]) -> Vec<u64> {
    let mut reduced_list: Vec<u64> = Vec::new();
    for i in 0..list.len() {
        if i == list.len() - 1 {
            reduced_list.push(list[i]);
            break;
        }
        match list[i + 1] {
            0 => {
                // Counting consecutive zeros.
                let mut zero_cnt = 1u8;
                for j in (i + 2)..list.len() {
                    if list[j] == 0 {
                        zero_cnt += 1;
                    } else {
                        break;
                    }
                }
                if zero_cnt % 2 == 0 {
                    reduced_list.push(list[i]);
                } else if i == 0 {
                    reduced_list.push(1);
                }
                break;
            },
            1 => {
                reduced_list.push(list[i]);
                break;
            },
            _ => reduced_list.push(list[i])
        }
    }
    reduced_list
}

fn last_digit2(list: &[u64]) -> u64 {
    if list.is_empty() {
        return 1;
    }
    if list.len() == 1 {
        return list[0] % 10;
    }
    let reduced_list = try_to_reduce_list(list);
    if reduced_list.len() == 1 {
        return reduced_list[0] % 10;
    }
    let a = reduced_list[0];
    let b = reduced_list[1];
    if reduced_list.len() == 2 {
        return LAST_DIGITS[(a % 10) as usize][(b % 4) as usize] as u64;
    }
    let c = reduced_list[2];
    match a % 10 {
        x @ 4 | x @ 8 => match b % 2 {
            0 => 6,
            _ => x
        },
        2 => match b % 4 {
                1 => 2,
                3 => match c % 2 {
                    0 => 2,
                    _ => 8
                },
                _ => 6,
        },
        1 | 3 | 7 | 9 => match b % 4 {
            1 => a % 10,
            3 => match c % 2 {
                0 => a % 10,
                _ => modpow(a, 3, 10)
            },
            _ => 1
        },
        x => x
    }
}


// https://www.codewars.com/kata/51ba717bb08c1cd60f00002f
fn acc_to_string(acc: &[i32]) -> String {
    match acc.len() {
        0 => String::new(),
        1 => format!("{}", acc[0]),
        2 => format!("{},{}", acc[0], acc[1]),
        _ => format!("{}-{}", acc.first().unwrap(), acc.last().unwrap())
    }
}
fn range_extraction(a: &[i32]) -> String {
    let mut res: Vec<String> = Vec::new();
    let mut acc: Vec<i32> = Vec::new();
    for &n in a {
        if acc.is_empty() {
            acc.push(n);
            continue;
        }

        if acc.last().unwrap() + 1 == n {
            acc.push(n);
        } else {
            res.push(acc_to_string(&acc));
            acc.clear();
            acc.push(n);
        }
    }
    if !acc.is_empty() {
        res.push(acc_to_string(&acc));
    }
    res.join(",")
}