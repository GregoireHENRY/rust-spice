//! Searching, sorting and the string utilities.

use crate::common;

const NUMBERS: [f64; 5] = [-3.0, 0.5, 0.5, 2.0, 7.0];
const INTEGERS: [i32; 5] = [-3, 0, 0, 2, 7];

#[test]
#[serial]
fn binary_search() {
    common::reset();

    // The arrays are sorted, which is what a binary search needs.
    assert_eq!(spice::bsrchd(2.0, &NUMBERS), 3);
    assert_eq!(spice::bsrchd(-3.0, &NUMBERS), 0);
    assert_eq!(spice::bsrchd(9.0, &NUMBERS), -1, "absent values give -1");
    assert_eq!(spice::bsrchi(2, &INTEGERS), 3);
    assert_eq!(spice::bsrchi(9, &INTEGERS), -1);

    let names = ["alpha", "beta", "gamma"];
    assert_eq!(spice::bsrchc("beta", &names), 1);
    assert_eq!(spice::bsrchc("delta", &names), -1);

    // The equivalence search ignores case and trailing blanks, and does not need a sorted array.
    assert_eq!(spice::esrchc("GAMMA  ", &names), 2);
    assert_eq!(spice::esrchc("delta", &names), -1);

    common::assert_ok("the searches");
}

#[test]
#[serial]
fn last_element_searches() {
    common::reset();

    // The last index at or below a value, and strictly below it, differ on an exact match.
    assert_eq!(spice::lstled(2.0, &NUMBERS), 3);
    assert_eq!(spice::lstltd(2.0, &NUMBERS), 2);
    assert_eq!(spice::lstled(-9.0, &NUMBERS), -1);
    assert_eq!(spice::lstled(9.0, &NUMBERS), 4);

    assert_eq!(spice::lstlei(2, &INTEGERS), 3);
    assert_eq!(spice::lstlti(2, &INTEGERS), 2);

    let names = ["alpha", "beta", "gamma"];
    assert_eq!(spice::lstlec("beta", &names), 1);
    assert_eq!(spice::lstltc("beta", &names), 0);

    common::assert_ok("the last element searches");
}

#[test]
#[serial]
fn ordering_and_sorting() {
    common::reset();

    let unsorted = [5.0, -1.0, 3.0];
    let order = spice::orderd(&unsorted);
    assert_eq!(order, vec![1, 2, 0], "the indices that put it in order");
    assert!(spice::isordv(&order));
    assert!(!spice::isordv(&[0, 0, 1]));

    // Applying the order vector sorts the array.
    let mut copy = unsorted;
    spice::reordd(&order, &mut copy);
    assert_eq!(copy, [-1.0, 3.0, 5.0]);

    // And so does the shell sort, in place.
    let mut copy = unsorted;
    spice::shelld(&mut copy);
    assert_eq!(copy, [-1.0, 3.0, 5.0]);

    let unsorted = [5, -1, 3];
    assert_eq!(spice::orderi(&unsorted), vec![1, 2, 0]);
    let mut copy = unsorted;
    spice::reordi(&spice::orderi(&unsorted), &mut copy);
    assert_eq!(copy, [-1, 3, 5]);
    let mut copy = unsorted;
    spice::shelli(&mut copy);
    assert_eq!(copy, [-1, 3, 5]);

    // Logical arrays reorder too; CSPICE stores them as integers.
    let mut flags = [1, 0, 1];
    spice::reordl(&[1, 0, 2], &mut flags);
    assert_eq!(flags, [0, 1, 1]);

    let names = ["gamma", "alpha", "beta"];
    assert_eq!(spice::orderc(&names), vec![1, 2, 0]);
    assert_eq!(
        spice::reordc(&spice::orderc(&names), &names),
        vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()]
    );
    assert_eq!(
        spice::shellc(&names),
        vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()]
    );

    common::assert_ok("ordering and sorting");
}

#[test]
#[serial]
fn ordered_search() {
    common::reset();

    // An unsorted array, searched through the order vector that would sort it.
    let values = [5, -1, 3];
    let order = spice::orderi(&values);
    assert_eq!(spice::bschoi(3, &values, &order), 2);
    assert_eq!(spice::bschoi(9, &values, &order), -1);

    let names = ["gamma", "alpha", "beta"];
    let order = spice::orderc(&names);
    assert_eq!(spice::bschoc("beta", &names, &order), 2);
    assert_eq!(spice::bschoc("delta", &names, &order), -1);

    common::assert_ok("the ordered searches");
}

#[test]
#[serial]
fn arithmetic_helpers() {
    common::reset();

    assert_relative_eq!(spice::sumad(&NUMBERS), 7.0, epsilon = 1e-14);
    assert_eq!(spice::sumai(&INTEGERS), 6);

    // Bracketing clamps into the interval, whichever way round the endpoints are given.
    assert_relative_eq!(spice::brcktd(9.0, 0.0, 5.0), 5.0, epsilon = 0.0);
    assert_relative_eq!(spice::brcktd(-9.0, 0.0, 5.0), 0.0, epsilon = 0.0);
    assert_relative_eq!(spice::brcktd(3.0, 5.0, 0.0), 3.0, epsilon = 0.0);
    assert_eq!(spice::brckti(9, 0, 5), 5);
    assert_eq!(spice::brckti(-9, 0, 5), 0);
}

#[test]
#[serial]
fn case_and_comparison() {
    common::reset();

    assert_eq!(spice::lcase("Hello World"), "hello world");
    assert_eq!(spice::ucase("Hello World"), "HELLO WORLD");

    // Equivalence ignores case and surrounding blanks.
    assert!(spice::eqstr("  hello ", "HELLO"));
    assert!(!spice::eqstr("hello", "hell"));

    // The raw form truncates to the buffer it is given.
    assert_eq!(spice::raw::ucase("Hello", 4), "HEL");
}

#[test]
#[serial]
fn templates() {
    common::reset();

    // `*` stands for any run of characters, `%` for exactly one.
    assert!(spice::matchw("ALPHA BETA", "ALPHA*", '*', '%'));
    assert!(spice::matchw("ALPHA BETA", "*BETA", '*', '%'));
    assert!(spice::matchw("ALPHA", "AL%HA", '*', '%'));
    assert!(!spice::matchw("ALPHA", "AL%%%HA", '*', '%'));

    // `matchw` respects case, `matchi` does not.
    assert!(!spice::matchw("alpha", "ALPHA", '*', '%'));
    assert!(spice::matchi("alpha", "ALPHA", '*', '%'));
}

#[test]
#[serial]
fn words_and_lists() {
    common::reset();

    assert_eq!(
        spice::nextwd("  alpha beta gamma"),
        ("alpha".to_string(), "beta gamma".to_string())
    );
    assert_eq!(spice::nextwd("alpha"), ("alpha".to_string(), String::new()));

    // A single delimiter keeps the empty fields between repeats.
    assert_eq!(
        spice::lparse("alpha,beta,,gamma", ","),
        vec![
            "alpha".to_string(),
            "beta".to_string(),
            String::new(),
            "gamma".to_string()
        ]
    );
    // Several delimiters, with runs treated as one.
    assert_eq!(
        spice::lparsm("alpha, beta;gamma", ",; "),
        vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()]
    );

    // Compressing runs of a delimiter.
    assert_eq!(spice::cmprss(' ', 1, "alpha    beta"), "alpha beta");
    assert_eq!(spice::cmprss('-', 2, "a-----b"), "a--b");

    common::assert_ok("words and lists");
}

#[test]
#[serial]
fn markers() {
    common::reset();

    assert_eq!(
        spice::repmc("Invalid # value", "#", "operation"),
        "Invalid operation value"
    );
    assert_eq!(
        spice::repmi("There are # items", "#", 5),
        "There are 5 items"
    );
    assert_eq!(
        spice::repmd("Value is #", "#", 1.0 / 3.0, 4),
        "Value is 3.333E-01"
    );
    assert_eq!(
        spice::raw::repmf("Value is #", "#", 1234.5, 5, 'F', 64),
        "Value is 1234.5"
    );
    assert_eq!(
        spice::raw::repmot("The # element", "#", 3, 'L', 64),
        "The third element"
    );
    assert_eq!(
        spice::raw::repml("The answer is #", "#", true, 'U', 64),
        "The answer is TRUE"
    );

    common::assert_ok("marker replacement");
}
