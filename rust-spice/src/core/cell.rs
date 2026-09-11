/*!
SPICE cells, the toolkit's own dynamic arrays.

## Description

A cell is a fixed capacity array with a control area that CSPICE maintains itself. Routines such as
[`dskobj`][crate::raw::dskobj] or [`spkcov`][crate::raw::spkcov] report their results by *appending*
to one, which is why they take a cell rather than returning a vector.

[`Cell`] owns its backing storage, so it is created, grown into by CSPICE and freed like any other
Rust value:

```no_run
# #[cfg(not(feature = "lock"))]
# {
let mut ids = spice::Cell::<i32>::new(64);
spice::raw::spkobj("/path/to/kernel.bsp", &mut ids);

for id in ids.iter() {
    println!("{}", spice::bodc2n(id).0);
}
# }
```

See the [C documentation](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/req/cells.html).
*/

use crate::c::{
    _SpiceDataType_SPICE_BOOL, _SpiceDataType_SPICE_CHR, _SpiceDataType_SPICE_DP,
    _SpiceDataType_SPICE_INT, _SpiceDataType_SPICE_TIME, SpiceCell, SpiceCellDataType, SpiceChar,
    SpiceDouble, SpiceInt,
};
use crate::core::ffi::{from_cbuf, to_cstring, CELL_CTRLSZ};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

/// Default length of the slots of a character cell.
pub const CELL_MAX_LEN: usize = crate::MAX_LEN_OUT;

/// Default capacity used by the wrappers that allocate a cell on the caller's behalf.
pub const CELL_MAXID: usize = 10_000;

/**
An element a [`Cell`] can hold.

Implemented for the element types the CSPICE C API can actually read and write: [`i32`], [`f64`]
and [`String`]. Boolean cells exist in the toolkit's type enumeration but no C routine operates on
one, so [`Cell::new_bool`] hands back an integer cell tagged as boolean.
*/
pub trait CellItem: Sized {
    /// The CSPICE data type of a cell holding `Self`.
    const DTYPE: SpiceCellDataType;

    /// How `Self` is stored in the backing buffer.
    type Raw: Copy + Default;

    /// Read one element out of its slot; `slot` is `length` items long for a character cell.
    fn read(slot: &[Self::Raw]) -> Self;

    /// Append one element, through the CSPICE routine that knows how to update the control area.
    fn append(cell: &mut Cell<Self>, item: Self);
}

/**
A CSPICE cell holding elements of type `T`.

The backing storage is owned, and released when the cell is dropped.
*/
pub struct Cell<T: CellItem> {
    /// The descriptor handed to CSPICE; its `base` and `data` point into `buf`.
    raw: SpiceCell,
    /// The control area followed by the `size` element slots.
    buf: Vec<T::Raw>,
    /// Number of `T::Raw` per element: the slot length for characters, one otherwise.
    elem: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T: CellItem> Cell<T> {
    /**
    Allocate a cell able to hold `size` elements.

    Character cells get slots of [`CELL_MAX_LEN`] bytes; use [`Cell::with_length`] to choose.
    */
    pub fn new(size: usize) -> Self {
        Self::with_length(size, CELL_MAX_LEN)
    }

    /**
    Allocate a cell able to hold `size` elements whose slots are `length` items long.

    `length` is only meaningful for character cells, where it is the maximum length of an element.
    */
    pub fn with_length(size: usize, length: usize) -> Self {
        Self::build(T::DTYPE, size, length)
    }

    /// Allocate a cell of an explicit CSPICE data type, for the types with no Rust counterpart.
    fn build(dtype: SpiceCellDataType, size: usize, length: usize) -> Self {
        let character = dtype == _SpiceDataType_SPICE_CHR;
        let elem = if character { length.max(1) } else { 1 };

        let mut buf = vec![T::Raw::default(); (CELL_CTRLSZ + size) * elem];
        let base = buf.as_mut_ptr();

        Self {
            raw: SpiceCell {
                dtype,
                length: if character { elem as SpiceInt } else { 0 },
                size: size as SpiceInt,
                card: 0,
                isSet: 1,
                adjust: 0,
                init: 0,
                base: base.cast(),
                // SAFETY: `buf` holds at least `CELL_CTRLSZ * elem` items, so this is in bounds.
                data: unsafe { base.add(CELL_CTRLSZ * elem) }.cast(),
            },
            buf,
            elem,
            _marker: PhantomData,
        }
    }

    /// Number of elements currently held, the *cardinality* in SPICE terms.
    pub fn len(&self) -> usize {
        self.raw.card.max(0) as usize
    }

    /// Whether the cell holds no element.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Number of elements the cell can hold.
    pub fn capacity(&self) -> usize {
        self.raw.size.max(0) as usize
    }

    /// The element at `index`, or `None` past the cardinality.
    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        let start = (CELL_CTRLSZ + index) * self.elem;
        Some(T::read(&self.buf[start..start + self.elem]))
    }

    /// Iterate over the elements currently held.
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        (0..self.len()).filter_map(move |index| self.get(index))
    }

    /// Collect the elements currently held.
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().collect()
    }

    /// Append an element, provided the cell is not full.
    pub fn push(&mut self, item: T) {
        T::append(self, item);
    }

    /// Drop every element, keeping the allocation.
    pub fn clear(&mut self) {
        let cell = self.as_mut_ptr();
        unsafe { crate::c::scard_c(0, cell) };
    }

    /// The cell descriptor, for calls to the [unsafe C API][crate::c].
    pub fn as_ptr(&self) -> *const SpiceCell {
        &self.raw
    }

    /**
    The cell descriptor, for calls to the [unsafe C API][crate::c].

    The `base` and `data` pointers are refreshed from the backing buffer on every call, so the
    descriptor handed out is always the one CSPICE should write through.
    */
    pub fn as_mut_ptr(&mut self) -> *mut SpiceCell {
        let base = self.buf.as_mut_ptr();
        self.raw.base = base.cast();
        // SAFETY: `buf` holds at least `CELL_CTRLSZ * elem` items, so this is in bounds.
        self.raw.data = unsafe { base.add(CELL_CTRLSZ * self.elem) }.cast();
        &mut self.raw
    }
}

impl Cell<i32> {
    /// Allocate an integer cell able to hold `size` elements.
    pub fn new_int(size: i32) -> Self {
        Self::new(size.max(0) as usize)
    }

    /// Allocate a boolean cell able to hold `size` elements.
    pub fn new_bool(size: i32) -> Self {
        Self::build(_SpiceDataType_SPICE_BOOL, size.max(0) as usize, 0)
    }

    /// The element at `index`, or `0` past the cardinality.
    pub fn get_data_int(&self, index: usize) -> i32 {
        self.get(index).unwrap_or_default()
    }

    /// The element at `index` as a boolean flag, or `0` past the cardinality.
    pub fn get_data_bool(&self, index: usize) -> i32 {
        self.get_data_int(index)
    }
}

impl Cell<f64> {
    /// Allocate a double precision cell able to hold `size` elements.
    pub fn new_double(size: i32) -> Self {
        Self::new(size.max(0) as usize)
    }

    /// Allocate a time cell able to hold `size` elements.
    pub fn new_time(size: i32) -> Self {
        Self::build(_SpiceDataType_SPICE_TIME, size.max(0) as usize, 0)
    }

    /// The element at `index`, or `0.0` past the cardinality.
    pub fn get_data_double(&self, index: usize) -> f64 {
        self.get(index).unwrap_or_default()
    }
}

impl Cell<String> {
    /// Allocate a character cell able to hold `size` elements of at most `length` bytes.
    pub fn new_character(size: i32, length: i32) -> Self {
        Self::with_length(size.max(0) as usize, length.max(1) as usize)
    }

    /// The element at `index`, or the empty string past the cardinality.
    pub fn get_data_character(&self, index: usize) -> String {
        self.get(index).unwrap_or_default()
    }
}

impl CellItem for i32 {
    const DTYPE: SpiceCellDataType = _SpiceDataType_SPICE_INT;
    type Raw = SpiceInt;

    fn read(slot: &[Self::Raw]) -> Self {
        slot[0]
    }

    fn append(cell: &mut Cell<Self>, item: Self) {
        let raw = cell.as_mut_ptr();
        unsafe { crate::c::appndi_c(item, raw) };
    }
}

impl CellItem for f64 {
    const DTYPE: SpiceCellDataType = _SpiceDataType_SPICE_DP;
    type Raw = SpiceDouble;

    fn read(slot: &[Self::Raw]) -> Self {
        slot[0]
    }

    fn append(cell: &mut Cell<Self>, item: Self) {
        let raw = cell.as_mut_ptr();
        unsafe { crate::c::appndd_c(item, raw) };
    }
}

impl CellItem for String {
    const DTYPE: SpiceCellDataType = _SpiceDataType_SPICE_CHR;
    type Raw = SpiceChar;

    fn read(slot: &[Self::Raw]) -> Self {
        from_cbuf(slot)
    }

    fn append(cell: &mut Cell<Self>, item: String) {
        let item = to_cstring(item);
        let raw = cell.as_mut_ptr();
        unsafe { crate::c::appndc_c(item.as_ptr() as *mut SpiceChar, raw) };
    }
}

/// Read only access to the raw descriptor, for the `card` and `size` fields CSPICE maintains.
impl<T: CellItem> Deref for Cell<T> {
    type Target = SpiceCell;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<T: CellItem> Clone for Cell<T> {
    fn clone(&self) -> Self {
        let mut clone = Self {
            raw: self.raw,
            buf: self.buf.clone(),
            elem: self.elem,
            _marker: PhantomData,
        };
        // Point the copied descriptor at the copied buffer rather than at the original one.
        clone.as_mut_ptr();
        clone
    }
}

impl<T: CellItem + fmt::Debug> fmt::Debug for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cell")
            .field("card", &self.len())
            .field("size", &self.capacity())
            .field("items", &self.to_vec())
            .finish()
    }
}

// SAFETY: the descriptor only points into the owned buffer, which moves with the cell.
unsafe impl<T: CellItem + Send> Send for Cell<T> {}
