#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate assayer;

#[cfg(test)] mod unit_tests;
mod consts;
mod args;
mod error;

use assayer::MethodSyntaxValidator;
pub use consts::*;
pub use error::Error;
pub use args::Args;

pub fn run(args: Args) -> Result<(), Error> {
    args.validate::<Args>()?;
    println!("Hello, {}-bit world!", 0usize.count_zeros());
    Ok(())
}
