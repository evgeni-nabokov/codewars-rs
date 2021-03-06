#[cfg(test)]
mod tests;

// https://www.codewars.com/kata/50654ddff44f800200000004
fn multiply(a:i32, b:i32) -> i32 {
    a * b
}

// https://www.codewars.com/kata/5ae62fcf252e66d44d00008e
fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    let nums = [a * b * c, a + b + c, (a + b) * c, a * (b + c)];
    *nums.iter().max().unwrap()
}

// https://www.codewars.com/kata/5715eaedb436cf5606000381
fn positive_sum(arr: &[i32]) -> i32 {
    arr.iter().fold(0, |mut acc, &n| acc + if n > 0 { n } else { 0 })
}
