#[cfg(test)]
mod tests;

use super::common::*;

// https://www.codewars.com/kata/5518a860a73e708c0a000027
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
                _ => mod_pow(a, 3, 10)
            },
            _ => 1
        },
        x => x
    }
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

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let mut res = vec![vec![0i8; size]; size];
    let mut curr_size = size;
    let mut left_col = 0;
    let mut top_row = 0;
    while curr_size >= 5 {
        // Top & bottom.
        for col in 0..curr_size {
            res[top_row][left_col + col] = 1;
            res[top_row + curr_size - 1][left_col + col] = 1;
        }
        // Left & right.
        for row in 2..curr_size {
            res[top_row + row][left_col] = 1;
            res[top_row + row][left_col + curr_size - 1] = 1;
        }
        // Special cases.
        res[top_row + 2][left_col + 1] = 1;
        res[top_row + 1][left_col + curr_size - 1] = 1;
        // Move to the smaller circle.
        curr_size -= 4;
        left_col += 2;
        top_row += 2;
    }
    // Fill the rest depending on the core's size.
    if curr_size == 1 {
        res[top_row][left_col] = 1;
    } else {
        let rem: Vec<Vec<Vec<i8>>> = vec![
            vec![vec![1, 1],
                 vec![0, 1]],
            vec![vec![1, 1, 1],
                 vec![0, 0, 1],
                 vec![1, 1, 1]],
            vec![vec![1, 1, 1, 1],
                 vec![0, 0, 0, 1],
                 vec![1, 0, 0, 1],
                 vec![1, 1, 1, 1]],
        ];
        let v = &rem[curr_size - 2];
        for row in 0..curr_size {
            for col in 0..curr_size {
                res[top_row + row][left_col + col] = v[row][col];
            }
        }
    }
    res
}