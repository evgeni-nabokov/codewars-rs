use super::*;
use super::super::kyu_4::MorseDecoder;

#[test]
fn duplicate_encode_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}

#[test]
fn meeting_tests() {
    assert_eq!(meeting("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn"),
               "(ARNO, ANN)(BELL, JOHN)(CORNWELL, ALEX)(DORNY, ABBA)(KERN, LEWIS)(KORN, ALEX)(META, GRACE)(SCHWARZ, VICTORIA)(STAN, MADISON)(STAN, MEGAN)(WAHL, ALEXIS)");
    assert_eq!(meeting("John:Gates;Michael:Wahl;Megan:Bell;Paul:Dorries;James:Dorny;Lewis:Steve;Alex:Meta;Elizabeth:Russel;Anna:Korn;Ann:Kern;Amber:Cornwell"),
               "(BELL, MEGAN)(CORNWELL, AMBER)(DORNY, JAMES)(DORRIES, PAUL)(GATES, JOHN)(KERN, ANN)(KORN, ANNA)(META, ALEX)(RUSSEL, ELIZABETH)(STEVE, LEWIS)(WAHL, MICHAEL)");
    assert_eq!(meeting("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern"),
               "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");
}

#[test]
fn longest_consec_tests() {
    assert_eq!(&longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2), "abigailtheta");
    assert_eq!(&longest_consec(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1),
               "oocccffuucccjjjkkkjyyyeehh");
    assert_eq!(&longest_consec(vec![], 3), "");
    assert_eq!(&longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3), "ixoyx3452zzzzzzzzzzzz");
    assert_eq!(&longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15), "");
    assert_eq!(&longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0), "");
}

#[test]
fn comp_tests() {
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11 * 11, 121 * 121, 144 * 144, 19 * 19, 161 * 161, 19 * 19, 144 * 144, 19 * 19];
    assert_eq!(comp(a1, a2), true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11 * 21, 121 * 121, 144 * 144, 19 * 19, 161 * 161, 19 * 19, 144 * 144, 19 * 19];
    assert_eq!(comp(a1, a2), false);
}

#[test]
fn backwards_prime_tests() {
    assert_eq!(backwards_prime(1, 100), vec![13, 17, 31, 37, 71, 73, 79, 97]);
    assert_eq!(backwards_prime(1, 31), vec![13, 17, 31]);
    assert_eq!(backwards_prime(701, 799), vec![701, 709, 733, 739, 743, 751, 761, 769]);
    assert_eq!(backwards_prime(1_095_000, 1095403), vec![1_095_047, 1_095_209, 1_095_319, 1_095_403]);
    assert_eq!(backwards_prime(700_000_008, 700_000_050), vec![700_000_031]);
}

fn travel_test(r: &str, zip_code: &str, exp: &str) -> () {
    println!("zip code:{:?}", zip_code);
    let ans = travel(r, zip_code);
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
fn good_vs_evil_tests() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}

fn assert_fuzzy_equals(actual: f64, expected: f64) {
    let merr = 1.0e-12;
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
    //println!("{}", inrange);
    assert_eq!(true, inrange)
}

#[test]
fn fp_approx_tests() {
    assert_fuzzy_equals(fp_approx(2.6e-08), 1.29999999155e-08);
    assert_fuzzy_equals(fp_approx(1.4e-09), 6.999999997549999e-10);
}

#[test]
fn decode_morse_test() {
    let decoder = MorseDecoder::new();
    assert_eq!(decoder.decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
}