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
