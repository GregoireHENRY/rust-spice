/*!
# SPICE functions for the DAS subsystem

*/

use spice_derive::make_answer;

make_answer!();

#[test]
fn test() {
    println!("{}", answer());
}
