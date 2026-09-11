/*!
Marshalling between Rust types and the C types CSPICE expects.

## Description

CSPICE routines take and return C types: null terminated strings, raw pointers to scalars,
pointers to (arrays of) doubles, `SpiceBoolean` integers... Rather than teaching the
[procedural macro][`spice_derive::cspice_proc`] about every one of those conversions, the knowledge
lives here, in ordinary generic code that the compiler type checks:

+ [`SpiceArg`] describes how a Rust value is handed to a C routine, and [`In`] keeps alive whatever
  scratch storage that requires (a [`CString`], typically) for the duration of the call.
+ [`SpiceRet`] describes how an output is allocated, written to by C, then read back, and [`Out`]
  owns the buffer while the call is in flight.
+ [`SpiceReturn`] converts a value a C routine returns directly.

Every buffer handed to CSPICE is allocated and zeroed by Rust, and freed when the wrapper returns.
Nothing here leaks, and no uninitialised memory is ever read back, even when a routine fails
without writing its outputs.
*/

use crate::c::{
    SpiceBoolean, SpiceCell, SpiceChar, SpiceDLADescr, SpiceDSKDescr, SpiceDouble, SpiceEllipse,
    SpiceInt, SpicePlane,
};
use crate::MAX_LEN_OUT;
use std::ffi::{CStr, CString};

/* -------------------------------------------------------------------------------------------- */
/* Buffers                                                                                        */
/* -------------------------------------------------------------------------------------------- */

/// Inline capacity of the buffers carrying an input string; body names, frames and aberration
/// corrections all fit, so a hot loop does not hit the allocator once per argument.
const INLINE_IN: usize = 64;

/**
A null terminated buffer of `N` bytes, on the stack while what it holds fits.

Both directions go through it: an argument is copied in and handed to CSPICE as a pointer, and an
output is zeroed, written by CSPICE, then read back.
*/
pub struct Buffer<const N: usize> {
    inline: [SpiceChar; N],
    /// Used only when the content does not fit inline.
    heap: Option<Vec<SpiceChar>>,
}

impl<const N: usize> Buffer<N> {
    /**
    A null terminated copy of `value`.

    # Panics

    Panics if `value` contains an interior null byte: C has no way to represent it, so passing one
    along would silently truncate the argument.
    */
    pub fn from_text(value: &str) -> Self {
        let bytes = value.as_bytes();
        if bytes.len() < N && !bytes.contains(&0) {
            let mut inline = [0; N];
            for (target, byte) in inline.iter_mut().zip(bytes) {
                *target = *byte as SpiceChar;
            }
            return Self { inline, heap: None };
        }

        let owned = to_cstring(value);
        let heap = owned
            .as_bytes_with_nul()
            .iter()
            .map(|&byte| byte as SpiceChar)
            .collect();
        Self {
            inline: [0; N],
            heap: Some(heap),
        }
    }

    /// A zeroed buffer of `len` bytes, for CSPICE to write a string into.
    pub fn with_len(len: usize) -> Self {
        let len = len.max(1);
        if len <= N {
            return Self {
                inline: [0; N],
                heap: None,
            };
        }
        Self {
            inline: [0; N],
            heap: Some(vec![0; len]),
        }
    }

    /// The pointer to hand to CSPICE.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut SpiceChar {
        match &mut self.heap {
            Some(heap) => heap.as_mut_ptr(),
            None => self.inline.as_mut_ptr(),
        }
    }

    /// Read the buffer back as a Rust string.
    pub fn into_string(self) -> String {
        match &self.heap {
            Some(heap) => from_cbuf(heap),
            None => from_cbuf(&self.inline),
        }
    }
}

/* -------------------------------------------------------------------------------------------- */
/* Inputs                                                                                         */
/* -------------------------------------------------------------------------------------------- */

/**
A Rust value that can be handed to a CSPICE routine as an input.
*/
pub trait SpiceArg {
    /// Scratch storage that has to outlive the call, `Self` when nothing has to be allocated.
    type Owned;

    /// The value actually passed to the C routine.
    type Raw;

    /// Move the value into its scratch storage.
    fn own(self) -> Self::Owned;

    /// Borrow the scratch storage as the C representation.
    fn raw(owned: &mut Self::Owned) -> Self::Raw;
}

/**
Owns an input argument for the duration of a CSPICE call.

Dropping it releases whatever the conversion had to allocate, so a wrapper leaks nothing even when
it is called in a tight loop.
*/
pub struct In<T: SpiceArg> {
    owned: T::Owned,
}

impl<T: SpiceArg> In<T> {
    /// Marshal `value` into its C representation.
    #[inline]
    pub fn new(value: T) -> Self {
        Self { owned: value.own() }
    }

    /// The pointer, or value, to hand to the C routine.
    #[inline]
    pub fn raw(&mut self) -> T::Raw {
        T::raw(&mut self.owned)
    }
}

/// Scalars are passed by value, widened or narrowed to the type CSPICE declares.
macro_rules! scalar_arg {
    ($($ty:ty => $raw:ty),* $(,)?) => {$(
        impl SpiceArg for $ty {
            type Owned = $ty;
            type Raw = $raw;

            #[inline]
            fn own(self) -> Self::Owned {
                self
            }

            #[inline]
            fn raw(owned: &mut Self::Owned) -> Self::Raw {
                *owned as $raw
            }
        }
    )*};
}

scalar_arg! {
    f32 => SpiceDouble,
    f64 => SpiceDouble,
    i8 => SpiceInt,
    i16 => SpiceInt,
    i32 => SpiceInt,
    i64 => SpiceInt,
    isize => SpiceInt,
    u8 => SpiceInt,
    u16 => SpiceInt,
    u32 => SpiceInt,
    u64 => SpiceInt,
    usize => SpiceInt,
    bool => SpiceBoolean,
}

/// Fixed size arrays and matrices are passed as a pointer to their first element.
macro_rules! array_arg {
    ($($ty:ty => $raw:ty),* $(,)?) => {$(
        impl<const N: usize> SpiceArg for [$ty; N] {
            type Owned = [$ty; N];
            type Raw = *mut $raw;

            #[inline]
            fn own(self) -> Self::Owned {
                self
            }

            #[inline]
            fn raw(owned: &mut Self::Owned) -> Self::Raw {
                owned.as_mut_ptr()
            }
        }

        impl<const M: usize, const N: usize> SpiceArg for [[$ty; N]; M] {
            type Owned = [[$ty; N]; M];
            type Raw = *mut $raw;

            #[inline]
            fn own(self) -> Self::Owned {
                self
            }

            #[inline]
            fn raw(owned: &mut Self::Owned) -> Self::Raw {
                owned.as_mut_ptr().cast()
            }
        }
    )*};
}

array_arg! {
    f64 => SpiceDouble,
    i32 => SpiceInt,
}

/// Slices are passed as a pointer to their first element; CSPICE takes the count separately.
impl<'a, T> SpiceArg for &'a [T] {
    type Owned = &'a [T];
    type Raw = *const T;

    #[inline]
    fn own(self) -> Self::Owned {
        self
    }

    #[inline]
    fn raw(owned: &mut Self::Owned) -> Self::Raw {
        owned.as_ptr()
    }
}

impl<'a, T> SpiceArg for &'a mut [T] {
    type Owned = &'a mut [T];
    type Raw = *mut T;

    #[inline]
    fn own(self) -> Self::Owned {
        self
    }

    #[inline]
    fn raw(owned: &mut Self::Owned) -> Self::Raw {
        owned.as_mut_ptr()
    }
}

/// A single character, for the few routines that take one rather than a string.
///
/// Only the low byte is passed, which is all CSPICE can represent.
impl SpiceArg for char {
    type Owned = char;
    type Raw = SpiceChar;

    #[inline]
    fn own(self) -> Self::Owned {
        self
    }

    #[inline]
    fn raw(owned: &mut Self::Owned) -> Self::Raw {
        *owned as u32 as SpiceChar
    }
}

impl SpiceArg for &str {
    type Owned = Buffer<INLINE_IN>;
    type Raw = *mut SpiceChar;

    #[inline]
    fn own(self) -> Self::Owned {
        Buffer::from_text(self)
    }

    #[inline]
    fn raw(owned: &mut Self::Owned) -> Self::Raw {
        owned.as_mut_ptr()
    }
}

impl SpiceArg for &String {
    type Owned = Buffer<INLINE_IN>;
    type Raw = *mut SpiceChar;

    #[inline]
    fn own(self) -> Self::Owned {
        Buffer::from_text(self)
    }

    #[inline]
    fn raw(owned: &mut Self::Owned) -> Self::Raw {
        owned.as_mut_ptr()
    }
}

impl SpiceArg for String {
    type Owned = Buffer<INLINE_IN>;
    type Raw = *mut SpiceChar;

    #[inline]
    fn own(self) -> Self::Owned {
        Buffer::from_text(&self)
    }

    #[inline]
    fn raw(owned: &mut Self::Owned) -> Self::Raw {
        owned.as_mut_ptr()
    }
}

/// The descriptors, planes and ellipses are plain C structs CSPICE reads through a pointer.
macro_rules! struct_arg {
    ($($ty:ty),* $(,)?) => {$(
        impl SpiceArg for $ty {
            type Owned = $ty;
            type Raw = *mut $ty;

            #[inline]
            fn own(self) -> Self::Owned {
                self
            }

            #[inline]
            fn raw(owned: &mut Self::Owned) -> Self::Raw {
                owned as *mut $ty
            }
        }
    )*};
}

struct_arg!(SpiceDLADescr, SpiceDSKDescr, SpicePlane, SpiceEllipse);

/* -------------------------------------------------------------------------------------------- */
/* Outputs                                                                                        */
/* -------------------------------------------------------------------------------------------- */

/**
A Rust value a CSPICE routine can write through an output pointer.
*/
pub trait SpiceRet: Sized {
    /// Buffer CSPICE writes into.
    type Buf;

    /// The pointer handed to the C routine.
    type Raw;

    /// A zeroed buffer of the default size.
    fn buf() -> Self::Buf;

    /// A zeroed buffer sized by the caller; only string outputs care.
    fn buf_with_len(len: usize) -> Self::Buf {
        let _ = len;
        Self::buf()
    }

    /// Borrow the buffer as the pointer to pass to C.
    fn raw(buf: &mut Self::Buf) -> Self::Raw;

    /// Read the value back once the call returned.
    fn get(buf: Self::Buf) -> Self;
}

/**
Owns an output buffer for the duration of a CSPICE call.
*/
pub struct Out<T: SpiceRet> {
    buf: T::Buf,
}

impl<T: SpiceRet> Out<T> {
    /// A zeroed output of the default size.
    #[inline]
    pub fn new() -> Self {
        Self { buf: T::buf() }
    }

    /// A zeroed output of `len` bytes, for the string outputs whose size the caller chooses.
    #[inline]
    pub fn with_len(len: usize) -> Self {
        Self {
            buf: T::buf_with_len(len),
        }
    }

    /// The pointer to hand to the C routine.
    #[inline]
    pub fn raw(&mut self) -> T::Raw {
        T::raw(&mut self.buf)
    }

    /// Read the output back.
    #[inline]
    pub fn get(self) -> T {
        T::get(self.buf)
    }
}

impl<T: SpiceRet> Default for Out<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Scalar outputs: a single zeroed cell CSPICE writes through.
macro_rules! scalar_ret {
    ($($ty:ty => $raw:ty, $zero:expr, $read:expr);* $(;)?) => {$(
        impl SpiceRet for $ty {
            type Buf = $raw;
            type Raw = *mut $raw;

            #[inline]
            fn buf() -> Self::Buf {
                $zero
            }

            #[inline]
            fn raw(buf: &mut Self::Buf) -> Self::Raw {
                buf as *mut $raw
            }

            #[inline]
            fn get(buf: Self::Buf) -> Self {
                #[allow(clippy::redundant_closure_call)]
                ($read)(buf)
            }
        }
    )*};
}

scalar_ret! {
    f64 => SpiceDouble, 0.0, |value| value;
    i32 => SpiceInt, 0, |value| value;
    bool => SpiceBoolean, 0, |value: SpiceBoolean| value != 0;
}

/// Array outputs: zeroed, so a routine that fails without writing still yields a readable value.
macro_rules! array_ret {
    ($($ty:ty => $raw:ty),* $(,)?) => {$(
        impl<const N: usize> SpiceRet for [$ty; N] {
            type Buf = [$ty; N];
            type Raw = *mut $raw;

            #[inline]
            fn buf() -> Self::Buf {
                [<$ty>::default(); N]
            }

            #[inline]
            fn raw(buf: &mut Self::Buf) -> Self::Raw {
                buf.as_mut_ptr()
            }

            #[inline]
            fn get(buf: Self::Buf) -> Self {
                buf
            }
        }

        impl<const M: usize, const N: usize> SpiceRet for [[$ty; N]; M] {
            type Buf = [[$ty; N]; M];
            type Raw = *mut $raw;

            #[inline]
            fn buf() -> Self::Buf {
                [[<$ty>::default(); N]; M]
            }

            #[inline]
            fn raw(buf: &mut Self::Buf) -> Self::Raw {
                buf.as_mut_ptr().cast()
            }

            #[inline]
            fn get(buf: Self::Buf) -> Self {
                buf
            }
        }
    )*};
}

array_ret! {
    f64 => SpiceDouble,
    i32 => SpiceInt,
}

impl SpiceRet for String {
    type Buf = Buffer<MAX_LEN_OUT>;
    type Raw = *mut SpiceChar;

    #[inline]
    fn buf() -> Self::Buf {
        Buffer::with_len(MAX_LEN_OUT)
    }

    #[inline]
    fn buf_with_len(len: usize) -> Self::Buf {
        Buffer::with_len(len)
    }

    #[inline]
    fn raw(buf: &mut Self::Buf) -> Self::Raw {
        buf.as_mut_ptr()
    }

    #[inline]
    fn get(buf: Self::Buf) -> Self {
        buf.into_string()
    }
}

/// These are all plain old data, so a zeroed struct is a valid, readable, starting point.
macro_rules! struct_ret {
    ($($ty:ty),* $(,)?) => {$(
        impl SpiceRet for $ty {
            type Buf = $ty;
            type Raw = *mut $ty;

            #[inline]
            fn buf() -> Self::Buf {
                // SAFETY: every field is an integer or a float, for which all-zero is valid.
                unsafe { std::mem::zeroed() }
            }

            #[inline]
            fn raw(buf: &mut Self::Buf) -> Self::Raw {
                buf as *mut $ty
            }

            #[inline]
            fn get(buf: Self::Buf) -> Self {
                buf
            }
        }
    )*};
}

struct_ret!(SpiceDLADescr, SpiceDSKDescr, SpicePlane, SpiceEllipse);

/* -------------------------------------------------------------------------------------------- */
/* Direct returns                                                                                 */
/* -------------------------------------------------------------------------------------------- */

/**
A Rust value a CSPICE routine returns directly, rather than through an output pointer.
*/
pub trait SpiceReturn {
    /// What the C routine returns.
    type Raw;

    /// Convert it to the Rust type.
    ///
    /// # Safety
    ///
    /// `raw` must be what the C routine actually returned; a pointer return has to point at a
    /// null terminated string that outlives the call.
    unsafe fn from_c(raw: Self::Raw) -> Self;
}

impl SpiceReturn for f64 {
    type Raw = SpiceDouble;

    #[inline]
    unsafe fn from_c(raw: Self::Raw) -> Self {
        raw
    }
}

impl SpiceReturn for i32 {
    type Raw = SpiceInt;

    #[inline]
    unsafe fn from_c(raw: Self::Raw) -> Self {
        raw
    }
}

impl SpiceReturn for bool {
    type Raw = SpiceBoolean;

    #[inline]
    unsafe fn from_c(raw: Self::Raw) -> Self {
        raw != 0
    }
}

impl SpiceReturn for String {
    type Raw = *mut SpiceChar;

    #[inline]
    unsafe fn from_c(raw: Self::Raw) -> Self {
        if raw.is_null() {
            return String::new();
        }
        // SAFETY: the caller guarantees `raw` points at a null terminated string; CSPICE returns
        // one of its own statics here.
        unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned()
    }
}

/* -------------------------------------------------------------------------------------------- */
/* Helpers                                                                                        */
/* -------------------------------------------------------------------------------------------- */

/**
Build the null terminated string CSPICE expects.

# Panics

Panics if `string` contains an interior null byte: C has no way to represent it, so passing one
along would silently truncate the argument.
*/
pub fn to_cstring<S: AsRef<str>>(string: S) -> CString {
    let string = string.as_ref();
    CString::new(string).unwrap_or_else(|_| {
        panic!("a string passed to CSPICE must not contain a null byte, got {string:?}")
    })
}

/**
Read back a string CSPICE wrote into a buffer.

Stops at the first null byte, then trims the blank padding CSPICE inherits from Fortran. Invalid
UTF-8 is replaced rather than rejected, so this never panics on whatever the toolkit produced.
*/
pub fn from_cbuf(buf: &[SpiceChar]) -> String {
    let bytes = buf.iter().map(|&byte| byte as u8).collect::<Vec<u8>>();
    let end = bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end])
        .trim_end()
        .to_string()
}

/// The size, in elements, of the control area CSPICE keeps at the front of a cell.
pub(crate) const CELL_CTRLSZ: usize = crate::c::SPICE_CELL_CTRLSZ as usize;

/// A pointer to a cell, for the wrappers that take one as an input.
impl<'a, T: crate::core::cell::CellItem> SpiceArg for &'a mut crate::core::cell::Cell<T> {
    type Owned = &'a mut crate::core::cell::Cell<T>;
    type Raw = *mut SpiceCell;

    #[inline]
    fn own(self) -> Self::Owned {
        self
    }

    #[inline]
    fn raw(owned: &mut Self::Owned) -> Self::Raw {
        owned.as_mut_ptr()
    }
}
