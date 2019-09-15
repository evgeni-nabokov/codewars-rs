use super::*;

#[test]
fn max_multiple_tests() {
    assert_eq!(max_multiple(2, 7), 6);
    assert_eq!(max_multiple(3, 10), 9);
    assert_eq!(max_multiple(7, 17), 14);
    assert_eq!(max_multiple(10, 50), 50);
    assert_eq!(max_multiple(4, 0), 0);
}

#[test]
fn row_weights_tests() {
    assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
    assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
    assert_eq!(row_weights(vec![80]), (80, 0));
}

#[test]
fn min_value_tests() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}

#[test]
fn passengers_in_bus_tests() {
    assert_eq!(passengers_in_bus(&[(10, 0), (3, 5), (5, 8)]), 5);
    assert_eq!(passengers_in_bus(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)]), 17);
    assert_eq!(passengers_in_bus(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)]), 21);
}
