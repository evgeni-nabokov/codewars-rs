use super::*;

#[test]
fn multiply_test() {
    assert_eq!(multiply(3, 5), 15)
}

#[test]
fn expressions_matter_tests() {
    assert_eq!(expressions_matter(2, 1, 2), 6);
    assert_eq!(expressions_matter(1, 1, 1), 3);
    assert_eq!(expressions_matter(2, 1, 1), 4);
    assert_eq!(expressions_matter(1, 2, 3), 9);
    assert_eq!(expressions_matter(1, 3, 1), 5);
    assert_eq!(expressions_matter(2, 2, 2), 8);

    assert_eq!(expressions_matter(5, 1, 3), 20);
    assert_eq!(expressions_matter(3, 5, 7), 105);
    assert_eq!(expressions_matter(5, 6, 1), 35);
    assert_eq!(expressions_matter(1, 6, 1), 8);
    assert_eq!(expressions_matter(2, 6, 1), 14);
    assert_eq!(expressions_matter(6, 7, 1), 48);

    assert_eq!(expressions_matter(2, 10, 3), 60);
    assert_eq!(expressions_matter(1, 8, 3), 27);
    assert_eq!(expressions_matter(9, 7, 2), 126);
    assert_eq!(expressions_matter(1, 1, 10), 20);
    assert_eq!(expressions_matter(9, 1, 1), 18);
    assert_eq!(expressions_matter(10, 5, 6), 300);
    assert_eq!(expressions_matter(1, 10, 1), 12);
}

#[test]
fn positive_sum_tests() {
    assert_eq!(positive_sum(&[1,2,3,4,5]), 15);
    assert_eq!(positive_sum(&[1,-2,3,4,5]), 13);
    assert_eq!(positive_sum(&[-1,2,3,4,-5]), 9);
}
