use super::*;

#[test]
fn range_extraction_tests() {
    assert_eq!(range_extraction(&[-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]), "-6,-3-1,3-5,7-11,14,15,17-20");
    assert_eq!(range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]), "-3--1,2,10,15,16,18-20");
}

#[test]
fn dbl_linear_tests() {
    assert_eq!(dbl_linear(10), 22);
    assert_eq!(dbl_linear(20), 57);
    assert_eq!(dbl_linear(30), 91);
    assert_eq!(dbl_linear(50), 175);
    assert_eq!(dbl_linear(100), 447);
}

#[test]
fn n_linear_pair_tests() {
    assert_eq!(n_linear(&[2, 3], 10), 22);
    assert_eq!(n_linear(&[3, 2], 10), 22);
}

#[test]
fn n_linear_triplet_tests() {
    assert_eq!(n_linear(&[5, 7, 8], 10), 64);
    assert_eq!(n_linear(&[5, 7, 8], 11), 65);
}

#[test]
fn strings_mix_tests() {
    assert_eq!(&strings_mix("Are they here", "yes, they are here"),
               "2:eeeee/2:yy/=:hh/=:rr");
    assert_eq!(&strings_mix("looping is fun but dangerous", "less dangerous than coding"),
               "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
    assert_eq!(&strings_mix(" In many languages", " there's a pair of functions"),
               "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
    assert_eq!(&strings_mix("Lords of the Fallen", "gamekult"), "1:ee/1:ll/1:oo");
    assert_eq!(&strings_mix("codewars", "codewars"), "");
    assert_eq!(&strings_mix("A generation must confront the looming ", "codewarrs"),
                            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
}

fn int_part_test(ans: &str, sol: &str) {
    assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
}

#[test]
fn int_part_tests() {
    int_part_test(&int_part(1), "Range: 0 Average: 1.00 Median: 1.00");
    int_part_test(&int_part(2), "Range: 1 Average: 1.50 Median: 1.50");
    int_part_test(&int_part(3), "Range: 2 Average: 2.00 Median: 2.00");
    int_part_test(&int_part(4), "Range: 3 Average: 2.50 Median: 2.50");
    int_part_test(&int_part(5), "Range: 5 Average: 3.50 Median: 3.50");
    int_part_test(&int_part(6), "Range: 8 Average: 4.75 Median: 4.50");
}

#[test]
fn cons_create_from_vec_test() {
    assert_eq!(Cons::from_iter(Vec::<i32>::new()), Cons::Null);

    assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5]).to_vec(),
               vec![1, 2, 3, 4, 5]);
}

#[test]
fn cons_filter_test() {
    assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5])
                   .filter(|&n| n > 3)
                   .to_vec(),
               vec![4, 5]);

    assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5])
                   .filter(|&n| n > 5),
               Cons::Null);
}

#[test]
fn cons_map_test() {
    assert_eq!(Cons::from_iter(vec!["1", "2", "3", "4", "5"])
                   .map(str::parse::<i32>)
                   .map(Result::unwrap)
                   .to_vec(),
               vec![1, 2, 3, 4, 5]);
}

#[test]
fn cans_filter_map_test() {
    assert_eq!(Cons::from_iter(vec![1, 2, 3, 4, 5])
                   .filter(|n| n % 2 == 0)
                   .map(|x| x.to_string())
                   .to_vec(),
               ["2", "4"]);
}

#[test]
fn recover_secret_tests() {
    assert_eq!(recover_secret(vec![
        ['t', 'u', 'p'],
        ['w', 'h', 'i'],
        ['t', 's', 'u'],
        ['a', 't', 's'],
        ['h', 'a', 'p'],
        ['t', 'i', 's'],
        ['w', 'h', 's']
    ]), "whatisup");

    assert_eq!(recover_secret(vec![
        ['t', 's', 'f'],
        ['a', 's', 'u'],
        ['m', 'a', 'f'],
        ['a', 'i', 'n'],
        ['s', 'u', 'n'],
        ['m', 'f', 'u'],
        ['a', 't', 'h'],
        ['t', 'h', 'i'],
        ['h', 'i', 'f'],
        ['m', 'h', 'f'],
        ['a', 'u', 'n'],
        ['m', 'a', 't'],
        ['f', 'u', 'n'],
        ['h', 's', 'n'],
        ['a', 'i', 's'],
        ['m', 's', 'n'],
        ['m', 's', 'u']
    ]), "mathisfun");

    assert_eq!(recover_secret(vec![
        ['o', 'x', 'y'], ['h', 'r', 'u'], ['b', 'x', 'z'], ['r', 'y', 'z'], ['v', 'y', 'z'],
        ['v', 'w', 'y'], ['o', 's', 'y'], ['i', 'u', 'z'], ['q', 'y', 'z'], ['k', 'p', 'v'],
        ['w', 'x', 'z'], ['k', 'x', 'y'], ['r', 'w', 'x'], ['a', 'n', 'w'], ['b', 'd', 't'],
        ['p', 'u', 'y'], ['n', 'v', 'z'], ['f', 'k', 'q'], ['i', 'm', 'z'], ['a', 'w', 'y'],
        ['b', 'k', 'n'], ['t', 'u', 'w'], ['x', 'y', 'z'], ['f', 'g', 'j'], ['n', 'y', 'z'],
        ['s', 'y', 'z'], ['k', 'w', 'x'], ['m', 's', 'u'], ['h', 'i', 's'], ['q', 'w', 'z'],
        ['w', 'y', 'z'], ['j', 'o', 'p'], ['r', 'v', 'y'], ['h', 'p', 'w'], ['s', 't', 'z'],
        ['j', 'k', 'r'], ['n', 'u', 'w'], ['h', 'v', 'w'], ['t', 'u', 'y'], ['l', 'q', 'y'],
        ['v', 'w', 'x'], ['r', 'w', 'z'], ['m', 'o', 'w'], ['k', 'q', 'x'], ['e', 'h', 'r'],
        ['e', 'k', 'l'], ['d', 'h', 'p'], ['r', 'u', 'w'], ['e', 'g', 'n'], ['m', 'o', 'y'],
        ['q', 'r', 's'], ['d', 'i', 'q'], ['u', 'w', 'z'], ['u', 'w', 'x'], ['u', 'x', 'z'],
        ['e', 'l', 'x'], ['p', 't', 'v'], ['k', 't', 'w'], ['v', 'x', 'y'], ['f', 'y', 'z'],
        ['v', 'w', 'z'], ['d', 'f', 'h'], ['h', 't', 'x'], ['c', 'w', 'x'], ['v', 'x', 'z'],
        ['f', 'p', 'x'], ['g', 'x', 'y'], ['g', 'v', 'w'], ['f', 'l', 's'], ['c', 'f', 'v'],
        ['g', 'q', 's'], ['d', 't', 'y'], ['j', 'p', 't'], ['d', 'k', 's'], ['s', 'w', 'x'],
        ['d', 'q', 'x'], ['o', 'r', 's'], ['l', 'v', 'y'], ['r', 't', 'y'], ['i', 'y', 'z'],
        ['g', 'r', 'w'], ['g', 'h', 'l'], ['c', 'x', 'z'], ['g', 't', 'v'], ['f', 'g', 'n'],
        ['l', 'r', 't'], ['r', 'u', 'x'], ['u', 'x', 'y'], ['s', 'x', 'y'], ['b', 'u', 'z'],
        ['l', 'w', 'y'], ['a', 'n', 'v'], ['k', 'l', 'z'], ['n', 'q', 'w'], ['m', 'u', 'z'],
        ['k', 'u', 'y'], ['t', 'v', 'z'], ['o', 'w', 'z'], ['c', 'h', 'y'], ['h', 's', 'y'],
        ['l', 'r', 'z'], ['a', 's', 'z'], ['f', 'r', 'v'], ['d', 'q', 'v'], ['u', 'v', 'y'],
        ['t', 'x', 'y'], ['b', 'w', 'y'], ['j', 'q', 'u'], ['o', 't', 'y'], ['p', 'y', 'z'],
        ['l', 'y', 'z'], ['n', 's', 'u'], ['m', 's', 'x'], ['b', 's', 'y'], ['l', 's', 'z'],
        ['d', 'm', 'u'], ['i', 'o', 'w'], ['c', 'v', 'w'], ['t', 'y', 'z'], ['l', 'n', 'y'],
        ['m', 'x', 'y'], ['n', 'v', 'x'], ['n', 'u', 'z'], ['g', 'h', 's'], ['r', 'v', 'w'],
        ['j', 'u', 'x'], ['m', 'v', 'z'], ['d', 'r', 'z'], ['o', 'v', 'x'], ['f', 'n', 'q'],
        ['a', 'b', 't'], ['h', 'v', 'x'], ['e', 'u', 'x'], ['o', 'w', 'y'], ['d', 'i', 'm'],
        ['a', 'f', 'w'], ['f', 'n', 'r'], ['d', 'm', 'x'], ['p', 'r', 'z'], ['p', 'u', 'v'],
        ['e', 'y', 'z'], ['c', 'o', 'x'], ['c', 'x', 'y'], ['a', 'i', 'w'], ['q', 'x', 'y'],
        ['c', 'i', 'n'], ['u', 'v', 'z'], ['u', 'w', 'y'], ['f', 'r', 'x'], ['t', 'w', 'z'],
        ['e', 'r', 'v'], ['o', 'q', 't'], ['m', 'w', 'x'], ['g', 'v', 'x'], ['c', 'j', 'k'],
        ['i', 's', 'y'], ['g', 's', 'u'], ['i', 'j', 's'], ['d', 'm', 'n'], ['l', 'n', 'v'],
        ['e', 's', 'w'], ['o', 'u', 'w'], ['b', 's', 'z'], ['a', 'd', 'g'], ['l', 'w', 'x'],
        ['m', 'r', 'x'], ['j', 'k', 'l'], ['f', 'p', 's'], ['p', 'r', 'v'], ['g', 'x', 'z'],
        ['o', 'u', 'z'], ['h', 'k', 's'], ['i', 'r', 'w'], ['n', 'q', 'y'], ['o', 'q', 'r'],
        ['f', 'q', 'y'], ['e', 'j', 'z'], ['e', 'o', 'u'], ['j', 'k', 'z'], ['b', 'g', 't'],
        ['f', 'v', 'w'], ['w', 'x', 'y'], ['t', 'v', 'w'], ['a', 'p', 'w'], ['c', 'l', 'x'],
        ['q', 's', 'y'], ['k', 'n', 'q'], ['d', 'y', 'z'], ['i', 'p', 'v'], ['e', 'k', 'y'],
        ['e', 'w', 'z'], ['i', 'm', 'v'], ['j', 's', 'v'], ['l', 'o', 'u'], ['e', 'o', 'q'],
        ['a', 'i', 's'], ['e', 'm', 'y'], ['b', 'y', 'z'], ['c', 'k', 'u'], ['a', 'k', 'p'],
        ['p', 'x', 'y'], ['h', 'p', 'q'], ['p', 't', 'w'], ['e', 'x', 'z'], ['l', 'p', 'y'],
        ['m', 'y', 'z'], ['l', 't', 'v'], ['d', 'g', 'n'], ['h', 'o', 't'], ['c', 't', 'x'],
        ['a', 'o', 'v'], ['m', 'v', 'x'], ['k', 'o', 'q'], ['i', 'v', 'y'], ['b', 'm', 's'],
        ['h', 'q', 'w'], ['f', 'h', 'x'], ['i', 'v', 'z'], ['f', 't', 'w'], ['l', 'v', 'z'],
        ['f', 'g', 'w'], ['s', 'w', 'z'], ['j', 'k', 'o'], ['d', 'j', 'm'], ['r', 't', 'u'],
        ['k', 'm', 'z'], ['q', 'w', 'y'], ['q', 'u', 'v'], ['g', 's', 'x'], ['p', 's', 't'],
        ['i', 'm', 't'], ['c', 'g', 'y'], ['n', 'w', 'z'], ['o', 'r', 'z'], ['h', 'i', 'm'],
        ['n', 't', 'w'], ['s', 'u', 'y'], ['s', 'x', 'z'], ['h', 'x', 'z'], ['e', 'f', 'x'],
        ['a', 'k', 'n'], ['h', 's', 'z'], ['j', 'o', 'w'], ['o', 't', 'x'], ['l', 'n', 'r'],
        ['m', 'x', 'z'], ['r', 'x', 'y'], ['b', 'w', 'z'], ['c', 'j', 'q'], ['b', 'f', 'o'],
        ['o', 'x', 'z'], ['i', 'j', 'r'], ['p', 'q', 'y'], ['j', 'p', 's'], ['m', 'r', 'w'],
        ['a', 'e', 'y'], ['u', 'y', 'z'], ['j', 'l', 'u'], ['j', 's', 'y'], ['k', 'x', 'z'],
        ['p', 'v', 'y'], ['j', 'l', 'p'], ['p', 'v', 'z'], ['f', 'h', 't'], ['k', 'n', 'x'],
        ['f', 'n', 'o'], ['p', 'v', 'w'], ['k', 'v', 'y'], ['j', 'w', 'y'], ['e', 'n', 's'],
        ['f', 'j', 'p'], ['f', 'u', 'w'], ['g', 'm', 'z'], ['n', 's', 'y'], ['m', 's', 'z'],
        ['c', 'd', 'x'], ['l', 'x', 'y'], ['g', 'y', 'z'], ['b', 't', 'w'], ['n', 'q', 'z'],
        ['r', 'w', 'y'], ['r', 't', 'w'], ['l', 't', 'x'], ['m', 'w', 'y'], ['h', 'm', 't'],
        ['k', 'n', 'v'], ['a', 'j', 'y'], ['f', 'q', 'w'], ['s', 'u', 'w'], ['p', 't', 'z'],
        ['j', 'l', 'r'], ['m', 'n', 'w'], ['n', 't', 'v'], ['n', 'p', 'r'], ['l', 'u', 'w'],
        ['g', 'j', 'o'], ['b', 'j', 'v'], ['m', 'o', 't'], ['k', 'w', 'z'], ['f', 'i', 'n'],
        ['i', 'u', 'y'], ['p', 'v', 'x'], ['k', 'l', 'u'], ['b', 'c', 'f'], ['f', 'q', 'v'],
        ['c', 'h', 'u'], ['i', 'n', 'w'], ['q', 's', 't'], ['k', 'q', 'w'], ['o', 'q', 's'],
        ['o', 'r', 'v'], ['m', 't', 'u'], ['n', 'u', 'y'], ['c', 's', 'z'], ['o', 'q', 'x'],
        ['r', 't', 'z'], ['a', 'g', 'q'], ['g', 's', 'z'], ['i', 'w', 'y'], ['j', 'l', 'y'],
        ['e', 'v', 'x'], ['e', 'n', 't'], ['f', 'g', 'v'], ['a', 'j', 'n'], ['d', 'h', 'r'],
        ['a', 'p', 'u'], ['l', 's', 'v'], ['l', 'q', 'z'], ['k', 'y', 'z'], ['r', 's', 'y'],
        ['n', 'x', 'y'], ['o', 'u', 'x'], ['n', 'q', 't'], ['c', 'f', 'h'], ['q', 's', 'x'],
        ['a', 'l', 'p'], ['l', 's', 'u'], ['e', 'r', 'y'], ['k', 'v', 'x'], ['j', 'o', 's'],
        ['o', 'p', 'q'], ['m', 'v', 'w'], ['o', 'q', 'v'], ['a', 'w', 'z'], ['l', 'u', 'x'],
        ['g', 's', 'v'], ['p', 'q', 'v'], ['b', 'o', 's'], ['o', 's', 'v'], ['f', 'h', 'y'],
        ['k', 's', 'w'], ['h', 't', 'u'], ['t', 'v', 'x'], ['q', 'v', 'w'], ['j', 'p', 'v'],
        ['c', 'l', 'u'], ['m', 's', 'w'], ['e', 'j', 'p'], ['e', 'f', 'h'], ['a', 's', 't'],
        ['i', 'k', 't'], ['j', 'l', 'm'], ['d', 'e', 'x'], ['j', 'x', 'y'], ['a', 'k', 'v'],
        ['j', 'q', 'v'], ['s', 'v', 'y'], ['d', 'k', 'q'], ['g', 'o', 's'], ['a', 'u', 'y'],
        ['h', 'u', 'x'], ['e', 'q', 's'], ['a', 'f', 'v'], ['i', 'r', 'x'], ['o', 'y', 'z'],
        ['h', 'v', 'z'], ['i', 'u', 'v'], ['h', 'p', 'x'], ['i', 't', 'z'], ['f', 'o', 'q'],
        ['a', 'x', 'y'], ['t', 'w', 'x'], ['c', 'u', 'w'], ['b', 'g', 'u'], ['q', 'v', 'y'],
        ['r', 'x', 'z'], ['s', 'u', 'x'], ['s', 'v', 'z'], ['e', 'h', 'l'], ['e', 'w', 'y'],
        ['j', 's', 'x'], ['q', 'w', 'x'], ['q', 'x', 'z'], ['f', 'l', 'n'], ['d', 'n', 'y'],
        ['j', 'r', 'u'], ['u', 'v', 'w'], ['t', 'x', 'z'], ['m', 'o', 'z'], ['f', 'm', 'q'],
        ['k', 'l', 'y'], ['f', 's', 'x'], ['m', 'w', 'z'], ['g', 'w', 'x'], ['m', 'u', 'y'],
        ['n', 'q', 'u'], ['l', 't', 'w'], ['r', 'u', 'z'], ['o', 's', 'w'], ['d', 's', 'y'],
        ['u', 'v', 'x'], ['h', 'y', 'z'], ['g', 'm', 'u'], ['a', 'c', 'l'], ['d', 'e', 'k'],
        ['p', 'q', 's'], ['g', 'j', 'l'], ['c', 'e', 'g'], ['b', 'l', 'v'], ['o', 'q', 'z'],
        ['p', 'q', 'u'], ['m', 'u', 'w'], ['j', 'n', 'y'], ['c', 'q', 'v'], ['p', 'u', 'w'],
        ['i', 'o', 'y'], ['f', 'm', 'x'], ['j', 't', 'x'], ['h', 'm', 'x'], ['c', 's', 'x'],
        ['i', 'q', 'v'], ['s', 'v', 'w'], ['i', 'w', 'x'], ['m', 'p', 't'], ['o', 'v', 'y'],
        ['p', 't', 'u'], ['e', 'w', 'x'], ['n', 'r', 's'], ['e', 'l', 'z'], ['s', 'u', 'z'],
        ['g', 'm', 't'], ['h', 'u', 'v'], ['r', 't', 'x'], ['l', 's', 'x'], ['o', 'p', 'v'],
        ['n', 'v', 'w'], ['p', 's', 'u'], ['e', 's', 'u'], ['j', 'y', 'z'], ['f', 'n', 'u'],
        ['h', 's', 'v'], ['f', 'm', 'n'], ['i', 'q', 'x'], ['d', 'j', 'l'], ['k', 't', 'v'],
        ['o', 'p', 'w'], ['e', 'k', 'm'], ['j', 'n', 'v'], ['h', 'j', 'p'], ['p', 'x', 'z'],
        ['c', 'g', 't'], ['i', 'n', 'r'], ['h', 'o', 'p'], ['c', 'h', 'v'], ['l', 'p', 'z'],
        ['q', 'v', 'z'], ['e', 't', 'w'], ['b', 't', 'x'], ['d', 'v', 'x'], ['l', 'r', 'u'],
        ['f', 'k', 'y'], ['f', 'x', 'y'], ['h', 'm', 'n'], ['s', 'v', 'x']
    ]), "abcdefghijklmnopqrstuvwxyz");
}

#[test]
fn decode_bits_test() {
    let decoder = MorseDecoder::new();
    let cases = [
        ["1", "E"],
        ["1110111", "M"],
        ["01110", "E"],
        [
            "1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011",
            "HEY JUDE"
        ],
        [
            "00011100010101010001000000011101110101110001010111000101000111010111010001110101110000000111010101000101110100011101110111000101110111000111010000000101011101000111011101110001110101011100000001011101110111000101011100011101110001011101110100010101000000011101110111000101010111000100010111010000000111000101010100010000000101110101000101110001110111010100011101011101110000000111010100011101110111000111011101000101110101110101110",
            "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG."
        ]
    ];
    for case in cases.iter() {
        let mut decoded_bits = decoder.decode_bits(case[0]);
        assert_eq!(decoder.decode_morse(&decoded_bits), case[1]);
    }
}

#[test]
fn next_bigger_number_tests() {
    let cases = [
        (9, -1),
        (10, -1),
        (11, -1),
        (531, -1),
        (12, 21),
        (122, 212),
        (513, 531),
        (2017, 2071),
        (14321, 21134),
        (1234567890, 1234567908),
        (59884848459853, 59884848483559)
    ];
    for case in cases.iter() {
        assert_eq!(next_bigger_number(case.0), case.1);
    }
}