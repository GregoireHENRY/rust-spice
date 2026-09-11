//! The owning [`spice::Cell`] type.

use crate::common;

#[test]
#[serial]
fn integers() {
    common::reset();
    let mut cell = spice::Cell::<i32>::new(8);

    assert!(cell.is_empty());
    assert_eq!(cell.len(), 0);
    assert_eq!(cell.capacity(), 8);
    assert_eq!(cell.get(0), None);

    cell.push(3);
    cell.push(-1);
    cell.push(42);

    assert_eq!(cell.len(), 3);
    assert_eq!(cell.card, 3, "the raw descriptor agrees");
    assert_eq!(cell.to_vec(), vec![3, -1, 42]);
    assert_eq!(cell.get(1), Some(-1));
    assert_eq!(cell.get(3), None);
    assert_eq!(cell.get_data_int(2), 42);

    cell.clear();
    assert!(cell.is_empty());
    assert_eq!(cell.capacity(), 8, "clearing keeps the allocation");
}

#[test]
#[serial]
fn doubles() {
    common::reset();
    let mut cell = spice::Cell::<f64>::new(4);
    cell.push(1.5);
    cell.push(-2.5);

    assert_eq!(cell.to_vec(), vec![1.5, -2.5]);
    assert_relative_eq!(cell.get_data_double(0), 1.5, epsilon = f64::EPSILON);
    assert_eq!(cell.iter().count(), 2);
}

#[test]
#[serial]
fn strings() {
    common::reset();
    // Character cells store fixed length slots, so the slot length matters.
    let mut cell = spice::Cell::<String>::with_length(4, 16);
    cell.push("alpha".to_string());
    cell.push("beta".to_string());

    assert_eq!(cell.len(), 2);
    assert_eq!(cell.to_vec(), vec!["alpha".to_string(), "beta".to_string()]);
    assert_eq!(cell.get_data_character(1), "beta");
    assert_eq!(cell.get(2), None);
}

#[test]
#[serial]
fn cloning_is_deep() {
    common::reset();
    let mut cell = spice::Cell::<i32>::new(4);
    cell.push(1);

    let mut clone = cell.clone();
    clone.push(2);

    assert_eq!(cell.to_vec(), vec![1], "the original is untouched");
    assert_eq!(clone.to_vec(), vec![1, 2]);
}

#[test]
#[serial]
fn the_legacy_constructors_still_work() {
    common::reset();
    let mut cell = spice::Cell::new_int(4);
    cell.push(5);
    assert_eq!(cell.get_data_int(0), 5);

    let mut cell = spice::Cell::new_double(4);
    cell.push(0.5);
    assert_relative_eq!(cell.get_data_double(0), 0.5, epsilon = f64::EPSILON);

    let mut cell = spice::Cell::new_character(4, 32);
    cell.push("gamma".to_string());
    assert_eq!(cell.get_data_character(0), "gamma");

    let cell = spice::Cell::new_bool(4);
    assert!(cell.is_empty());
    assert_eq!(cell.dtype, spice::c::_SpiceDataType_SPICE_BOOL);
    let cell = spice::Cell::new_time(4);
    assert!(cell.is_empty());
}

#[test]
#[serial]
fn cspice_fills_a_caller_allocated_cell() {
    common::load();

    // The raw wrappers take the cell the caller sized, the way CSPICE does.
    let mut bodies = spice::Cell::<i32>::new(16);
    spice::raw::dskobj(&common::kernel("test.bds"), &mut bodies);
    common::assert_ok("dskobj");
    assert_eq!(bodies.to_vec(), vec![common::TARGET]);

    // Cells accumulate across calls, so asking twice still gives a set of one.
    spice::raw::dskobj(&common::kernel("test.bds"), &mut bodies);
    assert_eq!(bodies.to_vec(), vec![common::TARGET]);

    common::unload();
}

#[test]
#[serial]
fn set_operations() {
    common::reset();

    let mut a = spice::Cell::<i32>::new(16);
    let mut b = spice::Cell::<i32>::new(16);
    for item in [1, 3, 5, 7] {
        spice::insrti(item, &mut a);
    }
    for item in [3, 4, 5, 6] {
        spice::insrti(item, &mut b);
    }
    common::assert_ok("building the sets");

    // Insertion keeps a set sorted and free of duplicates.
    spice::insrti(3, &mut a);
    assert_eq!(a.to_vec(), vec![1, 3, 5, 7]);

    let mut result = spice::Cell::<i32>::new(16);
    spice::union(&mut a, &mut b, &mut result);
    assert_eq!(result.to_vec(), vec![1, 3, 4, 5, 6, 7]);

    spice::inter(&mut a, &mut b, &mut result);
    assert_eq!(result.to_vec(), vec![3, 5]);

    spice::diff(&mut a, &mut b, &mut result);
    assert_eq!(result.to_vec(), vec![1, 7]);
    common::assert_ok("set operations");

    // Membership and removal.
    assert!(spice::elemi(5, &mut a));
    assert!(!spice::elemi(4, &mut a));
    spice::removi(5, &mut a);
    assert!(!spice::elemi(5, &mut a));
    assert_eq!(a.to_vec(), vec![1, 3, 7]);

    common::unload();
}

#[test]
#[serial]
fn sets_of_doubles_and_strings() {
    common::reset();

    let mut numbers = spice::Cell::<f64>::new(16);
    for item in [2.5, -1.0, 2.5] {
        spice::insrtd(item, &mut numbers);
    }
    assert_eq!(numbers.to_vec(), vec![-1.0, 2.5]);
    assert!(spice::elemd(2.5, &mut numbers));
    spice::removd(2.5, &mut numbers);
    assert!(!spice::elemd(2.5, &mut numbers));

    let mut names = spice::Cell::<String>::with_length(16, 32);
    for item in ["beta", "alpha", "beta"] {
        spice::insrtc(item, &mut names);
    }
    common::assert_ok("string set");
    assert_eq!(
        names.to_vec(),
        vec!["alpha".to_string(), "beta".to_string()]
    );
    assert!(spice::elemc("alpha", &mut names));
    spice::removc("alpha", &mut names);
    assert!(!spice::elemc("alpha", &mut names));
    assert_eq!(names.to_vec(), vec!["beta".to_string()]);

    common::unload();
}

#[test]
#[serial]
fn cardinality_and_size() {
    common::reset();

    let mut cell = spice::Cell::<i32>::new(16);
    for item in [4, 2, 4, 9] {
        spice::appndi(item, &mut cell);
    }
    common::assert_ok("appending");

    // Appending keeps duplicates and the insertion order; it is a cell, not yet a set.
    assert_eq!(cell.to_vec(), vec![4, 2, 4, 9]);
    assert_eq!(spice::card(&mut cell), 4);
    assert_eq!(spice::card(&mut cell), cell.len() as i32);
    assert_eq!(spice::size(&mut cell), 16);
    assert_eq!(spice::size(&mut cell), cell.capacity() as i32);

    // Validating turns it into a set.
    spice::valid(16, 4, &mut cell);
    assert_eq!(cell.to_vec(), vec![2, 4, 9]);

    // Truncating and copying.
    spice::scard(2, &mut cell);
    assert_eq!(cell.to_vec(), vec![2, 4]);

    let mut copy = spice::Cell::<i32>::new(16);
    spice::copy(&mut cell, &mut copy);
    assert_eq!(copy.to_vec(), cell.to_vec());
    common::assert_ok("copying");

    // `ssize` resizes and empties.
    spice::ssize(8, &mut copy);
    assert_eq!(spice::size(&mut copy), 8);
    assert!(copy.is_empty());

    // And the double and character forms of append.
    let mut numbers = spice::Cell::<f64>::new(4);
    spice::appndd(1.5, &mut numbers);
    assert_eq!(numbers.to_vec(), vec![1.5]);

    let mut names = spice::Cell::<String>::with_length(4, 32);
    spice::appndc("delta", &mut names);
    assert_eq!(names.to_vec(), vec!["delta".to_string()]);

    common::unload();
}
