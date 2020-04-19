use super::*;

#[test]
fn calc_tests() {
    assert_eq!(calc("1"), 1f64);
    assert_eq!(calc("-1"), -1f64);
    assert_eq!(calc("1-1"), 0f64);
    assert_eq!(calc("1 -1"), 0f64);
    assert_eq!(calc("1- 1"), 0f64);
    assert_eq!(calc("1 - 1"), 0f64);
    assert_eq!(calc("1- -1"), 2f64);
    assert_eq!(calc("1 - -1"), 2f64);
    assert_eq!(calc("6 + -4"), 2f64);
    assert_eq!(calc("(1 + 2) * 3 + 5"), 14f64);
    assert_eq!(calc("1 + (3 - 2)"), 2f64);
    assert_eq!(calc("6 + -(4)"), 2f64);
    assert_eq!(calc("6 + -(-4)"), 10f64);
}