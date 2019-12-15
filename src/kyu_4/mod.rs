use std::cmp;
use std::cmp::Ordering::*;
use std::iter;
use std::collections::{HashMap, HashSet};
use super::common::*;

#[cfg(test)]
mod tests;

// https://www.codewars.com/kata/51ba717bb08c1cd60f00002f
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

fn acc_to_string(acc: &[i32]) -> String {
    match acc.len() {
        0 => String::new(),
        1 => format!("{}", acc[0]),
        2 => format!("{},{}", acc[0], acc[1]),
        _ => format!("{}-{}", acc.first().unwrap(), acc.last().unwrap())
    }
}

// https://www.codewars.com/kata/5672682212c8ecf83e000050
fn dbl_linear(n: u32) -> u32 {
    let mut series: Vec<u32> = Vec::with_capacity(n as usize + 1);
    series.push(1);
    let mut x_index = 0;
    let mut y_index = 0;
    let mut cur_x = 3;
    let mut cur_y = 4;
    while series.len() < series.capacity() {
        match  cur_x.cmp(&cur_y) {
            Greater => {
                series.push(cur_y);
                y_index += 1;
                cur_y = 3 * series[y_index] + 1;
            },
            Less => {
                series.push(cur_x);
                x_index += 1;
                cur_x = 2 * series[x_index] + 1;
            },
            Equal => {
                series.push(cur_x);
                x_index += 1;
                cur_x = 2 * series[x_index] + 1;
                y_index += 1;
                cur_y = 3 * series[y_index] + 1;
            }
        }
    }
    *series.last().unwrap()
}

// https://www.codewars.com/kata/5aa417aa4a6b344e2200009d
fn n_linear(m: &[u32], n: usize) -> u32 {
    let mut series: Vec<u32> = Vec::with_capacity(n as usize + 1);
    series.push(1);
    let mut indices: Vec<usize> = vec![0; m.len()];
    let mut cur_x: Vec<u32> = m.iter().map(|&x| x + 1).collect::<Vec<_>>();
    while series.len() < series.capacity() {
        let min_x = cur_x.iter().min().unwrap().to_owned();
        let min_x_indices = cur_x.iter().enumerate()
            .filter(|i_x| *i_x.1 == min_x)
            .map(|i_x| i_x.0)
            .collect::<Vec<_>>();
        series.push(min_x);
        for i in min_x_indices.into_iter() {
            indices[i] += 1;
            cur_x[i] = m[i] * series[indices[i]] + 1;
        }
    }
    *series.last().unwrap()
}

// https://www.codewars.com/kata/5629db57620258aa9d000014
fn strings_mix(s1: &str, s2: &str) -> String {
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

// https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b
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

// https://www.codewars.com/kata/53f40dff5f9d31b813000774
// Inspired by Kahn's topological sorting algorithm [https://en.wikipedia.org/wiki/Topological_sorting].
fn recover_secret(mut triplets: Vec<[char; 3]>) -> String {
    let mut res: Vec<char> = Vec::new();
    let mut i = 0usize;
    loop {
        if i >= triplets.len() {
            break;
        }
        let ch = triplets[i][0].to_owned();
        if ch == '\0' || char_does_follow_another(&ch, &triplets) {
            i += 1;
            continue;
        }
        res.push(ch);
        remove_char_from_beginning(&ch, &mut triplets);
        i = 0;
    }
    res.iter().map(|&x| x.to_string()).collect::<String>()
}

// Determines if specified char follows another char, i.e. has incoming edges.
fn char_does_follow_another(ch: &char, triplets: &[[char; 3]]) -> bool {
    for &triplet in triplets {
        if triplet[1] == *ch || triplet[2] == *ch {
            return true;
        }
    }
    false
}

// Removes specified char from the beginning of each triplet and shift the rest chars.
fn remove_char_from_beginning(ch: &char, triplets: &mut [[char; 3]]) {
    for triplet in triplets {
        if triplet[0] == *ch {
            triplet[0] = triplet[1];
            triplet[1] = triplet[2];
            triplet[2] = '\0'; // A stub.
        }
    }
}

// https://www.codewars.com/kata/54b72c16cd7f5154e9000457
// https://www.codewars.com/kata/54b724efac3d5402db00065e
pub struct MorseDecoder {
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
                ("-.--", "Y"), ("--..", "Z"), (".-.-.-", "."), ("--..--", ","), ("..--..", "?")
            ].iter().map(|(x, y)| (x.to_string(), y.to_string())).collect()
        }
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        if encoded.is_empty() {
            return "".to_string();
        }
        encoded.trim().split("   ").map(|w| w.split_ascii_whitespace()
            .map(|code| self.morse_code.get(code).unwrap().to_owned())
            .collect::<Vec<String>>()
            .join("")
        ).collect::<Vec<String>>()
            .join(" ")
    }

    pub fn decode_bits(&self, encoded: &str) -> String {
        if encoded.is_empty() {
            return "".to_string();
        }
        let trimmed = encoded.trim_matches('0');
        let sampling_rate = MorseDecoder::calc_sampling_rate(trimmed);
        let mut units: Vec<_> = trimmed
            .chars().step_by(sampling_rate)
            .map(|ch| ch.to_digit(10).unwrap() as u8)
            .collect();
        let mut morse_code: Vec<&str> = Vec::with_capacity(units.len() / sampling_rate);
        units.push(0); // A stub for one more iteration.
        let mut one_cnt = 0;
        let mut zero_cnt = 0;
        for unit in units {
            match unit {
                1 => {
                    one_cnt += 1;
                    match zero_cnt {
                        3 => morse_code.push(" "),
                        7 => morse_code.push("   "),
                        _ => {}
                    }
                    zero_cnt = 0;
                },
                0 => {
                    zero_cnt += 1;
                    match one_cnt {
                        1 => morse_code.push("."),
                        3 => morse_code.push("-"),
                        _ => {}
                    }
                    one_cnt = 0;
                },
                _ => {}
            }
        }
        morse_code.concat()
    }

    fn calc_sampling_rate(encoded: &str) -> usize {
        if encoded.is_empty() {
            return 0;
        }
        if encoded.len() == 1 {
            return 1;
        }
        let mut last_bit = encoded.chars().next().unwrap();
        let mut chars: Vec<char> = encoded.chars().skip(1).collect();
        chars.push('0'); // A stub for one more iteration.
        let mut cnt = 1;
        let mut min_cnt = 0;
        for &bit in chars.iter() {
            if last_bit == bit {
                cnt += 1;
            } else {
                if cnt == 1 {
                    return cnt;
                } else {
                    if min_cnt == 0 || cnt < min_cnt {
                        min_cnt = cnt;
                    }
                    cnt = 1;
                }
                last_bit = bit;
            }
        }
        min_cnt
    }
}