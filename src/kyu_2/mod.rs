#[cfg(test)]
mod tests;
mod calculator;

pub fn calc(expression: &str) -> f64 {
    calculator::calc(expression)
}
