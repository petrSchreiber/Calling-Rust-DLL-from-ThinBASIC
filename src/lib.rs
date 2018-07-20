/*

  This Library represents DLL, which can be called from ThinBASIC.

  The purpose of this code to give idea how to setup Rust to talk to ThinBASIC.

*/

// Include the functions from other files, and make them public = exported
mod thinbasic;

pub mod unsigned_integers;
pub mod signed_integers;
pub mod floating_points;

pub mod fixed_length_strings;
pub mod dynamic_strings;

pub mod user_defined_types;
