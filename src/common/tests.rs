use super::*;

#[test]
fn is_prime_tests() {
    assert_eq!(is_prime(&0), false);
    assert_eq!(is_prime(&1), false);
    assert_eq!(is_prime(&7541), true);
    assert_eq!(is_prime(&62003), true);
    assert_eq!(is_prime(&62013), false);
    assert_eq!(is_prime(&15485863), true);
}

#[test]
fn mod_pow_tests() {
    assert_eq!(mod_pow(2, 5, 10), 2);
    assert_eq!(mod_pow(2546, 556, 3), 1);
}