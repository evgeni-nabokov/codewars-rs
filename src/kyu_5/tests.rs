use super::*;

#[test]
fn sum_pairs_tests() {
    assert_eq!(sum_pairs(&[1, 4, 8, 7, 3, 15], 8), Some((1, 7)));
    assert_eq!(sum_pairs(&[1, -2, 3, 0, -6, 1], -6), Some((0, -6)));
    assert_eq!(sum_pairs(&[20, -13, 40], -7), None);
    assert_eq!(sum_pairs(&[1, 2, 3, 4, 1, 0], 2), Some((1, 1)));
    assert_eq!(sum_pairs(&[10, 5, 2, 3, 7, 5], 10), Some((3, 7)));
    assert_eq!(sum_pairs(&[4, -2, 3, 3, 4], 8), Some((4, 4)));
    assert_eq!(sum_pairs(&[0, 2, 0], 0), Some((0, 0)));
    assert_eq!(sum_pairs(&[5, 9, 13, -3], 10), Some((13, -3)));
}

#[test]
fn prime_gap_tests() {
    assert_eq!(prime_gap(2, 100, 110), Some((101, 103)));
    assert_eq!(prime_gap(4, 100, 110), Some((103, 107)));
    assert_eq!(prime_gap(6, 100, 110), None);
    assert_eq!(prime_gap(8, 300, 400), Some((359, 367)));
}

#[test]
fn product_fib_tests() {
    assert_eq!(product_fib(4895), (55, 89, true));
    assert_eq!(product_fib(5895), (89, 144, false));
}

#[test]
fn get_prime_factors_tests() -> () {
    assert_eq!(get_prime_factors(&11), vec![11]);
    assert_eq!(get_prime_factors(&10), vec![2, 5]);
    assert_eq!(get_prime_factors(&12), vec![2, 2, 3]);
    assert_eq!(get_prime_factors(&7775460), vec![2, 2, 3, 3, 3, 5, 7, 11, 11, 17]);
}

#[test]
fn get_kprimes_tests() {
    assert_eq!(get_kprimes(5, 1000, 1100), vec![1020, 1026, 1032, 1044, 1050, 1053, 1064, 1072, 1092, 1100]);
    assert_eq!(get_kprimes(12, 100000, 100100), vec![]);
}

#[test]
fn puzzle_tests() {
    assert_eq!(puzzle(100), 0);
    assert_eq!(puzzle(144), 0);
    assert_eq!(puzzle(143), 2);
}

#[test]
fn last_digit_tests() {
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10", "10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376",
                          "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723",
                          "68819615221552997273737174557165657483427362207517952651"), 7);
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
    buddy_numbers_test(10, 50, Some((48, 75)));
    buddy_numbers_test(1081180, 1103735, Some((1081184, 1331967)));
    buddy_numbers_test(271, 5128, Some((1050, 1925)));
    buddy_numbers_test(305047, 309143, None);
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

#[test]
fn consec_kprimes_tests() {
    assert_eq!(consec_kprimes(2, vec![10081, 10071, 10077, 10065, 10060, 10070, 10086, 10083, 10078, 10076, 10089, 10085, 10063, 10074, 10068, 10073, 10072, 10075]), 2);
    assert_eq!(consec_kprimes(6, vec![10064]), 0);
    assert_eq!(consec_kprimes(1, vec![10054, 10039, 10053, 10051, 10047, 10043, 10037, 10034]), 0);
    assert_eq!(consec_kprimes(3, vec![10158, 10182, 10184, 10172, 10179, 10168, 10156, 10165, 10155, 10161, 10178, 10170]), 5);
    assert_eq!(consec_kprimes(2, vec![10110, 10102, 10097, 10113, 10123, 10109, 10118, 10119, 10117, 10115, 10101, 10121, 10122]), 7);
    assert_eq!(consec_kprimes(1, vec![10054, 10061, 10059, 10058, 10067, 10066, 10053, 10079, 10069, 10082]), 1);
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

#[test]
fn prime_factors_tests() {
    assert_eq!(&prime_factors(7775460), "(2**2)(3**3)(5)(7)(11**2)(17)");
    assert_eq!(&prime_factors(17 * 17 * 93 * 677), "(3)(17**2)(31)(677)");
    assert_eq!(&prime_factors(7919), "(7919)");
    assert_eq!(&prime_factors(933555431), "(7537)(123863)");
}

#[test]
fn rgb_tests() {
    assert_eq!(rgb(0, 0, 0), "000000");
    assert_eq!(rgb(1, 2, 3), "010203");
    assert_eq!(rgb(255, 255, 255), "FFFFFF");
    assert_eq!(rgb(254, 253, 252), "FEFDFC");
    assert_eq!(rgb(-20, 275, 125), "00FF7D");
}

#[test]
fn perimeter_tests() {
    assert_eq!(perimeter(5), 80);
    assert_eq!(perimeter(7), 216);
    assert_eq!(perimeter(20), 114624);
    assert_eq!(perimeter(30), 14098308);
}

#[test]
fn john_tests() {
    assert_eq!(john(11), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]);
    assert_eq!(john(14), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8]);
}

#[test]
fn ann_tests() {
    assert_eq!(ann(6), vec![1, 1, 2, 2, 3, 3]);
    assert_eq!(ann(15), vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9]);
}

#[test]
fn sum_john_tests() {
    assert_eq!(sum_john(75), 1720);
    assert_eq!(sum_john(78), 1861);
}

#[test]
fn sum_ann_tests() {
    assert_eq!(sum_ann(115), 4070);
    assert_eq!(sum_ann(150), 6930);
}

#[test]
fn get_prime_factors_with_power_tests() {
    assert_eq!(get_prime_factors_with_power(&11), hashmap!{ 11 => 1 });
    assert_eq!(get_prime_factors_with_power(&10), hashmap!{ 2 => 1, 5 => 1 });
    assert_eq!(get_prime_factors_with_power(&24), hashmap!{ 2 => 3, 3 => 1 });
    assert_eq!(get_prime_factors_with_power(&7775460), hashmap!{ 2 => 2, 3 => 3, 5 => 1, 7 => 1, 11 => 2, 17 => 1 });
}

#[test]
fn test_add() {
    assert_eq!(convert_fracts(vec![(1, 2), (1, 3), (1, 4)]), vec![(6, 12), (4, 12), (3, 12)]);
    assert_eq!(convert_fracts(vec![(69, 130), (87, 1310), (3, 4)]), vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    assert_eq!(convert_fracts(vec![(690, 1300), (87, 1310), (30, 40)]), vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
}