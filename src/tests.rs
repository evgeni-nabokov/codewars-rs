use super::*;

#[test]
fn good_vs_evil_tests() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}

#[test]
fn min_value_tests() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}


#[test]
fn passengers_in_bus_tests() {
    assert_eq!(passengers_in_bus(&[(10, 0),(3, 5),(5, 8)]), 5);
    assert_eq!(passengers_in_bus(&[(3, 0),(9, 1),(4, 10),(12, 2),(6, 1),(7, 10)]), 17);
    assert_eq!(passengers_in_bus(&[(3, 0),(9, 1),(4, 8),(12, 2),(6, 1),(7, 8)]), 21);
}

#[test]
fn duplicate_encode_tests() {
    assert_eq!(duplicate_encode("din"),"(((");
    assert_eq!(duplicate_encode("recede"),"()()()");
    assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
    assert_eq!(duplicate_encode("(( @"),"))((");
}

fn comp_test(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

#[test]
fn comp_tests() {
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    comp_test(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    comp_test(a1, a2, false);
}

fn backwards_prime_test(start: u64, stop: u64, exp: Vec<u64>) -> () {
    assert_eq!(backwards_prime(start, stop), exp)
}

#[test]
fn backwards_prime_tests() {
    let a = vec![13, 17, 31, 37, 71, 73, 79, 97];
    backwards_prime_test(1, 100, a);
    let a = vec![13, 17, 31];
    backwards_prime_test(1, 31, a);
    let a = vec![701, 709, 733, 739, 743, 751, 761, 769];
    backwards_prime_test(701, 799, a);
    let a = vec![1095047, 1095209, 1095319, 1095403];
    backwards_prime_test(1095000, 1095403, a);
    let a = vec![700000031];
    backwards_prime_test(700000008, 700000050, a);
}

fn travel_test(r: &str, zipcode:&str, exp: &str) -> () {
    //println!("r:{:?}", r);
    println!(" zipcode:{:?}", zipcode);
    let ans = travel(r, zipcode);
    println!("actual: {:?}", ans);
    println!("expect: {:?}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
}

#[test]
fn travel_tests() {
    let r = &String::from("123 Main Street St. Louisville OH 43071,
      432 Main Long Road St. Louisville OH 43071, 786 High Street Pollocksville NY 56432,
      54 Holy Grail Street Niagara Town ZP 32908, 3200 Main Rd. Bern AE 56210,1 Gordon St. Atlanta RE 13000,
      10 Pussy Cat Rd. Chicago EX 34342, 10 Gordon St. Atlanta RE 13000, 58 Gordon Road Atlanta RE 13000,
      22 Tokyo Av. Tedmondville SW 43098, 674 Paris bd. Abbeville AA 45521, 10 Surta Alley Goodtown GG 30654,
      45 Holy Grail Al. Niagara Town ZP 32908, 320 Main Al. Bern AE 56210, 14 Gordon Park Atlanta RE 13000,
      100 Pussy Cat Rd. Chicago EX 34342, 2 Gordon St. Atlanta RE 13000, 5 Gordon Road Atlanta RE 13000,
      2200 Tokyo Av. Tedmondville SW 43098, 67 Paris St. Abbeville AA 45521, 11 Surta Avenue Goodtown GG 30654,
      45 Holy Grail Al. Niagara Town ZP 32918, 320 Main Al. Bern AE 56215, 14 Gordon Park Atlanta RE 13200,
      100 Pussy Cat Rd. Chicago EX 34345, 2 Gordon St. Atlanta RE 13222, 5 Gordon Road Atlanta RE 13001,
      2200 Tokyo Av. Tedmondville SW 43198, 67 Paris St. Abbeville AA 45522, 11 Surta Avenue Goodville GG 30655,
      2222 Tokyo Av. Tedmondville SW 43198, 670 Paris St. Abbeville AA 45522, 114 Surta Avenue Goodville GG 30655,
      2 Holy Grail Street Niagara Town ZP 32908, 3 Main Rd. Bern AE 56210, 77 Gordon St. Atlanta RE 13000,
      100 Pussy Cat Rd. Chicago OH 13201");
    travel_test(r, "AA 45522", "AA 45522:Paris St. Abbeville,Paris St. Abbeville/67,670");
    travel_test(r, "OH 430", "OH 430:/");
}

#[test]
fn sum_pairs_tests() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
}

fn prime_gap_test(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(prime_gap(g, m, n), exp)
}

#[test]
fn prime_gap_tests() {
    prime_gap_test(2,100,110, Some((101, 103)));
    prime_gap_test(4,100,110, Some((103, 107)));
    prime_gap_test(6,100,110, None);
    prime_gap_test(8,300,400, Some((359, 367)));
}

fn part_test(ans: &str, sol: &str) {
    assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
}

#[test]
fn part_tests() {
    part_test(&int_part(1), "Range: 0 Average: 1.00 Median: 1.00");
    part_test(&int_part(2), "Range: 1 Average: 1.50 Median: 1.50");
    part_test(&int_part(3), "Range: 2 Average: 2.00 Median: 2.00");
    part_test(&int_part(4), "Range: 3 Average: 2.50 Median: 2.50");
    part_test(&int_part(5), "Range: 5 Average: 3.50 Median: 3.50");
    part_test(&int_part(6), "Range: 8 Average: 4.75 Median: 4.50");
}


#[test]
fn longest_consec_tests() {
    longest_consec_test(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    longest_consec_test(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1,
                        "oocccffuucccjjjkkkjyyyeehh");
    longest_consec_test(vec![], 3, "");
    longest_consec_test(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    longest_consec_test(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    longest_consec_test(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}

#[test]
fn product_fib_tests() {
    assert_eq!(product_fib(4895), (55, 89, true));
    assert_eq!(product_fib(5895), (89, 144, false));
}

fn mix_test(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}

#[test]
fn mix_tests() {
    mix_test("Are they here", "yes, they are here",
             "2:eeeee/2:yy/=:hh/=:rr");
    mix_test("looping is fun but dangerous", "less dangerous than coding",
             "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
    mix_test(" In many languages", " there's a pair of functions",
             "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
    mix_test("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    mix_test("codewars", "codewars", "");
    mix_test("A generation must confront the looming ", "codewarrs",
             "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");

}

#[test]
fn get_prime_factors_tests() -> () {
    assert_eq!(get_prime_factors(&11), vec![]);
    assert_eq!(get_prime_factors(&10), vec![2, 5]);
    assert_eq!(get_prime_factors(&12), vec![2, 2, 3]);
    assert_eq!(get_prime_factors(&7775460), vec![2, 2, 3, 3, 3, 5, 7, 11, 11, 17]);
}

fn get_kprimes_test(k: usize, start: u64, nd: u64, exp: Vec<u64>) -> () {
    assert_eq!(get_kprimes(k, start, nd), exp)
}

#[test]
fn get_kprimes_tests() {
    get_kprimes_test(5, 1000, 1100, vec![1020, 1026, 1032, 1044, 1050, 1053, 1064, 1072, 1092, 1100]);
    get_kprimes_test(12, 100000, 100100, vec![]);
}

fn puzzle_test(n: u64, exp: u8) -> () {
    assert_eq!(puzzle(n), exp)
}

#[test]
fn puzzle_tests() {
    puzzle_test(100, 0);
    puzzle_test(144, 0);
    puzzle_test(143, 2);
}

#[test]
fn last_digit_tests() {
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10","10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376",
                          "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723",
                          "68819615221552997273737174557165657483427362207517952651"), 7);
}

#[test]
fn cons_create_from_vec_test() {
    assert_eq!(Cons::from_iter(Vec::<i32>::new()), Cons::Null);

    assert_eq!(Cons::from_iter(vec![1,2,3,4,5]).to_vec(),
               vec![1,2,3,4,5]);
}

#[test]
fn cons_filter_test() {
    assert_eq!(Cons::from_iter(vec![1,2,3,4,5])
                   .filter(|&n| n > 3)
                   .to_vec(),
               vec![4,5]);

    assert_eq!(Cons::from_iter(vec![1,2,3,4,5])
                   .filter(|&n| n > 5),
               Cons::Null);
}

#[test]
fn cons_map_test() {
    assert_eq!(Cons::from_iter(vec!["1","2","3","4","5"])
                   .map(str::parse::<i32>)
                   .map(Result::unwrap)
                   .to_vec(),
               vec![1,2,3,4,5]);
}

#[test]
fn cans_filter_map_test() {
    assert_eq!(Cons::from_iter(vec![1,2,3,4,5])
                   .filter(|n| n % 2 == 0)
                   .map(|x| x.to_string())
                   .to_vec(),
               ["2","4"]);
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
fn row_weights_tests() {
    assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
    assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
    assert_eq!(row_weights(vec![80]), (80,0));
}

#[cfg(test)]

fn meeting_test(s: &str, exp: &str) -> () {
    let ans = meeting(s);
    assert_eq!(ans, exp);
}

#[test]
fn meeting_tests() {
    meeting_test("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn",
                 "(ARNO, ANN)(BELL, JOHN)(CORNWELL, ALEX)(DORNY, ABBA)(KERN, LEWIS)(KORN, ALEX)(META, GRACE)(SCHWARZ, VICTORIA)(STAN, MADISON)(STAN, MEGAN)(WAHL, ALEXIS)");
    meeting_test("John:Gates;Michael:Wahl;Megan:Bell;Paul:Dorries;James:Dorny;Lewis:Steve;Alex:Meta;Elizabeth:Russel;Anna:Korn;Ann:Kern;Amber:Cornwell",
                 "(BELL, MEGAN)(CORNWELL, AMBER)(DORNY, JAMES)(DORRIES, PAUL)(GATES, JOHN)(KERN, ANN)(KORN, ANNA)(META, ALEX)(RUSSEL, ELIZABETH)(STEVE, LEWIS)(WAHL, MICHAEL)");
    meeting_test("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern",
                 "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");
}

#[test]
fn modpow_tests() {
    assert_eq!(modpow(2, 5, 10), 2);
    assert_eq!(modpow(2546, 556, 3), 1);
}

#[test]
fn last_digit2_tests() {
    let tests = vec![
        (vec![], 1),
        (vec![0], 0),
        (vec![0, 0], 1),
        (vec![0, 0, 0], 0),
        (vec![0, 0, 0, 0, 0, 0, 0, 0], 1),
        (vec![1, 2], 1),
        (vec![3, 4, 5], 1),
        (vec![4, 3, 6], 4),
        (vec![7, 6, 21], 1),
        (vec![12, 30, 21], 6),
        (vec![2, 2, 2, 0], 4),
        (vec![937640, 767456, 981242], 0),
        (vec![123232, 694022, 140249], 6),
        (vec![499942, 898102, 846073], 6),
        (vec![2147483647, 2147483647, 2147483647, 2147483647], 3)
    ];

    for test in tests {
        assert_eq!(last_digit2(&test.0), test.1);
    }
}

#[test]
fn range_extraction_tests() {
    assert_eq!(range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]), "-6,-3-1,3-5,7-11,14,15,17-20");
    assert_eq!(range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]), "-3--1,2,10,15,16,18-20");
}

#[test]
fn recover_secret_tests() {
    assert_eq!(recover_secret(vec![
        ['t','u','p'],
        ['w','h','i'],
        ['t','s','u'],
        ['a','t','s'],
        ['h','a','p'],
        ['t','i','s'],
        ['w','h','s']
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

fn buddy_numbers_test(start: u64, limit: u64, exp: Option<(u64, u64)>) -> () {
    println!("start:{}", start);
    println!("limit:{}", limit);
    let ans = buddy_numbers(start, limit);
    println!("actual:\n{:?}", ans);
    println!("expect:\n{:?}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
}

#[test]
fn buddy_numbers_tests() {
    buddy_numbers_test(10, 50,  Some((48, 75)));
    buddy_numbers_test(1081180, 1103735, Some((1081184, 1331967)));
    buddy_numbers_test(271, 5128, Some((1050 , 1925)));
    buddy_numbers_test(305047, 309143, None);
}

#[test]
fn max_multiple_tests() {
    assert_eq!(max_multiple(2,7),6);
    assert_eq!(max_multiple(3,10),9);
    assert_eq!(max_multiple(7,17),14);
    assert_eq!(max_multiple(10,50),50);
    assert_eq!(max_multiple(4,0), 0);
}

fn find_x_of_pseries_test(m: f64, expect: f64) -> () {
    let merr = 1e-12;
    println!("{:?}", m);
    let actual = find_x_of_pseries(m);
    println!("Actual {:e}", actual);
    println!("Expect {:e}", expect);
    let inrange = (actual - expect).abs() <= merr;
    if inrange == false {
        println!("Expected value near: {:e} but got: {:e}", actual, expect);
    }
    assert_eq!(inrange, true);
}

#[test]
fn find_x_of_pseries_tests() {
    find_x_of_pseries_test(2.00, 5.000000000000e-01);
    find_x_of_pseries_test(4.00, 6.096117967978e-01);
    find_x_of_pseries_test(5.00, 6.417424305044e-01);
}

fn sum_of_series_test(actual: f64, expected: f64) {
    let merr = 1.0e-6;
    let inrange =
        if expected == 0.0 {
            (actual.abs() <= merr)
        } else {
            ((actual - expected).abs() / expected <= merr)
        };
    if inrange == false {
        println!("Expected value must be near: {:e} but was:{:e}", expected, actual);
    } else {
        //println!("....... GOOD\n");
    }
    assert_eq!(true, inrange);
}

#[test]
fn sum_of_series_tests() {
    sum_of_series_test(sum_of_series(5), 1.275);
    sum_of_series_test(sum_of_series(6), 1.2125);
    sum_of_series_test(sum_of_series(7), 1.173214);
    sum_of_series_test(sum_of_series(8), 1.146651);
    sum_of_series_test(sum_of_series(200), 1.005025);
    sum_of_series_test(sum_of_series(379), 1.002645);
}

fn get_phone_directory_input() -> String {
    let res = r#"/+1-541-754-3010 156 Alphand_St. <J Steeve>
133, Green, Rd. <E Kustur> NY-56423 ;+1-541-914-3010;
+1-541-984-3012 <P Reed> /PO Box 530; Pollocksville, NC-28573
:+1-321-512-2222 <Paul Dive> Sequoia Alley PQ-67209
+1-741-984-3090 <Peter Reedgrave> _Chicago
:+1-921-333-2222 <Anna Stevens> Haramburu_Street AA-67209
+1-111-544-8973 <Peter Pan> LA
+1-921-512-2222 <Wilfrid Stevens> Wild Street AA-67209
<Peter Gone> LA ?+1-121-544-8974
<R Steell> Quora Street AB-47209 +1-481-512-2222!
<Arthur Clarke> San Antonio $+1-121-504-8974 TT-45120
<Ray Chandler> Teliman Pk. !+1-681-512-2222! AB-47209,
<Sophia Loren> +1-421-674-8974 Bern TP-46017
<Peter O'Brien> High Street +1-908-512-2222; CC-47209
<Anastasia> +48-421-674-8974 Via Quirinal Roma
<P Salinger> Main Street, +1-098-512-2222, Denver
<C Powel> *+19-421-674-8974 Chateau des Fosses Strasbourg F-68000
<Bernard Deltheil> +1-498-512-2222; Mount Av.  Eldorado
+1-099-500-8000 <Peter Crush> Labrador Bd.
+1-931-512-4855 <William Saurin> Bison Street CQ-23071
<P Salinge> Main Street, +1-098-512-2222, Denve
/+5-541-754-3010 156 Alphandria_Street. <Jr Part>
1333, Green, Road <F Fulgur> NW-46423 ;+6-541-914-3010!
+5-541-984-3012 <Peter Reeves> /PO Box 5300; Albertville, SC-28573
:+5-321-512-2222 <Paulo Divino> Boulder Alley ZQ-87209
+3-741-984-3090 <F Flanaghan> _Chicago Av.
:+3-921-333-2222 <Roland Scorsini> Bellevue_Street DA-67209
+8-111-544-8973 <Laurence Pantow> SA
+8-921-512-2222 <Raymond Stevenson> Joly Street EE-67209
<John Freeland> Mantow ?+2-121-544-8974
<Robert Mitch> Eleonore Street QB-87209 +2-481-512-2222?
<Arthur Paternos> San Antonio $+7-121-504-8974 TT-45121
<Ray Charles> Stevenson Pk. !+7-681-512-2222! CB-47209,
<JP Gorce> +9-421-674-8974 New-Bern TP-16017
<P McDon> Revolution Street +2-908-512-2222; PP-47209
<Elizabeth Corber> +8-421-674-8974 Via Papa Roma
<C Saborn> Main Street, +15-098-512-2222, Boulder
<Colin Marshall> *+9-421-674-8974 Edinburgh UK
<Bernard Povit> +3-498-512-2222; Hill Av.  Cameron
+12-099-500-8000 <Pete Highman> Ontario Bd.
+8-931-512-4855 <W Mount> Oxford Street CQ-23071
<Donald Drinkaw> Moon Street, +3-098-512-2222, Peterville
"#;
    return String::from(res);
}

fn phone_directory_test(dir: &str, num: &str, exp: &str) -> () {
    println!("num:{}", num);
    let ans = phone_directory(dir, num);
    println!("actual:\n{}", ans);
    println!("expect:\n{}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
}

#[test]
fn phone_directory_tests() {
    let dir = &get_phone_directory_input();
    phone_directory_test(dir, "48-421-674-8974", "Phone => 48-421-674-8974, Name => Anastasia, Address => Via Quirinal Roma");
    phone_directory_test(dir, "1-921-512-2222", "Phone => 1-921-512-2222, Name => Wilfrid Stevens, Address => Wild Street AA-67209");
    phone_directory_test(dir, "1-908-512-2222", "Phone => 1-908-512-2222, Name => Peter O'Brien, Address => High Street CC-47209");
    phone_directory_test(dir, "1-541-754-3010", "Phone => 1-541-754-3010, Name => J Steeve, Address => 156 Alphand St.");
    phone_directory_test(dir, "1-098-512-2222", "Error => Too many people: 1-098-512-2222");
    phone_directory_test(dir, "5-555-555-5555", "Error => Not found: 5-555-555-5555");

}

fn consec_kprimes_test(k: usize, arr: Vec<u64>, exp: u64) -> () {
    assert_eq!(consec_kprimes(k, arr), exp)
}

#[test]
fn consec_kprimes_tests() {
    consec_kprimes_test(2, vec![10081, 10071, 10077, 10065, 10060, 10070, 10086, 10083, 10078, 10076, 10089, 10085, 10063, 10074, 10068, 10073, 10072, 10075], 2);
    consec_kprimes_test(6, vec![10064], 0);
    consec_kprimes_test(1, vec![10054, 10039, 10053, 10051, 10047, 10043, 10037, 10034], 0);
    consec_kprimes_test(3, vec![10158, 10182, 10184, 10172, 10179, 10168, 10156, 10165, 10155, 10161, 10178, 10170], 5);
    consec_kprimes_test(2, vec![10110, 10102, 10097, 10113, 10123, 10109, 10118, 10119, 10117, 10115, 10101, 10121, 10122], 7);
    consec_kprimes_test(1, vec![10054, 10061, 10059, 10058, 10067, 10066, 10053, 10079, 10069, 10082], 1);
}

fn factorial_decomp_test(n: u64, exp: &str) -> () {
    println!("n:{:?}", n);
    let ans = factorial_decomp(n);
    println!("actual: {:?}", ans);
    println!("expect: {:?}", exp.to_string());
    println!("{}", ans == exp.to_string());
    assert_eq!(ans, exp.to_string());
    println!("{}", "-");
}

#[test]
fn factorial_decomp_tests() {
    factorial_decomp_test(17, "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
    factorial_decomp_test(5, "2^3 * 3 * 5");
    factorial_decomp_test(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
    factorial_decomp_test(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
    factorial_decomp_test(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");
    factorial_decomp_test(1, "1");
}


fn prime_factors_test(n: u64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn prime_factors_tests() {
    prime_factors_test(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    prime_factors_test(17*17*93*677, "(3)(17**2)(31)(677)");
    prime_factors_test(7919, "(7919)");
    prime_factors_test(933555431, "(7537)(123863)");
}

#[test]
fn rgb_tests() {
    assert_eq!(rgb(0, 0, 0), "000000");
    assert_eq!(rgb(1, 2, 3), "010203");
    assert_eq!(rgb(255, 255, 255), "FFFFFF");
    assert_eq!(rgb(254, 253, 252), "FEFDFC");
    assert_eq!(rgb(-20, 275, 125), "00FF7D");
}


fn perimeter_test(n: u64, exp: u64) -> () {
    assert_eq!(perimeter(n), exp)
}

#[test]
fn perimeter_tests() {
    perimeter_test(5, 80);
    perimeter_test(7, 216);
    perimeter_test(20, 114624);
    perimeter_test(30, 14098308);
}

fn john_test(n: u32, exp: Vec<u32>) -> () {
    assert_eq!(john(n), exp)
}
fn ann_test(n: u32, exp: Vec<u32>) -> () {
    assert_eq!(ann(n), exp)
}
fn sum_john_test(n: u32, exp: u32) -> () {
    assert_eq!(sum_john(n), exp)
}
fn sum_ann_test(n: u32, exp: u32) -> () {
    assert_eq!(sum_ann(n), exp)
}

#[test]
fn john_tests() {
    john_test(11, vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]);
    john_test(14, vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8]);
}
#[test]
fn ann_tests() {
    ann_test(6, vec![1, 1, 2, 2, 3, 3]);
    ann_test(15, vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9]);
}
#[test]
fn sum_john_tests() {
    sum_john_test(75, 1720);
    sum_john_test(78, 1861);
}
#[test]
fn sum_ann_tests() {
    sum_ann_test(115, 4070);
    sum_ann_test(150, 6930);
}

fn dbl_linear_test(n: u32, exp: u32) -> () {
    assert_eq!(dbl_linear(n), exp)
}

#[test]
fn dbl_linear_tests() {
    dbl_linear_test(10, 22);
    dbl_linear_test(20, 57);
    dbl_linear_test(30, 91);
    dbl_linear_test(50, 175);
    dbl_linear_test(100, 447);
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