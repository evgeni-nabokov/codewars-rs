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
    assert_eq!(get_prime_factors(&10), vec![2, 5]);
    assert_eq!(get_prime_factors(&12), vec![2, 2, 3]);
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