#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate fluent_validator;
#[allow(unused_imports)]

mod consts;
mod args;
mod error;
#[cfg(test)] mod unit_tests;

use fluent_validator::FluentValidator;
pub use consts::*;
pub use error::Error;
pub use args::Args;

#[allow(unused_variables)]
pub fn run(args: Args) -> Result<(), Error> {
    args.clone().validate::<Args>()?;   //TODO: Make fluent_validator::validate() non-consuming
    println!("Hello, ()-bit world!", 0usize.count_zeros());
    Ok(())
}
