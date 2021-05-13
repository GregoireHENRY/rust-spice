/*!
# An idiomatic Rust layer on top of the C wrapper.

Below you will find the index of the C spice functions that are wrapped with a nice Rust interface.

It takes a long time to correctly wrap all functions of the API. Raise an issue to ask a specific
function to be implemented and we will do it immediately. Pull requests are warmly welcomed to help
speed up this process (do not forget to include a proper documentation and a test).

In the meantime, if you are in a rush and need quickly to use a function not implemented with the
Rust interface, use the unsafe C functions [here][c#functions]. You can find some inspiration in
the source of this lib to deal with the FFI types and unsafe code.

## Most common API

Chapters  | Modules
----------|--------
Spice APIs for accessing SPICE kernel data | [kernel]
Spice APIs for checking geometric conditions | [check_geometric_conditions]
*/

/// SPICE functions for checking geometric conditions.
pub mod check_geometric_conditions;
/// SPICE functions for the DAS subsystem.
pub mod das;
/// SPICE functons for accessing kernel data.
pub mod kernel;

pub use self::check_geometric_conditions::*;
pub use self::das::*;
pub use self::kernel::*;
