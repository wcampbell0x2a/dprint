//! This crate provide prints macros that are not compiled in releases builds.
//!
//! ## feature: `std`
//! The `std` feature is enabled by default for `dprintln`, `dprint`, `deprintln`, and `deprint` macros.
//!
//! ```
//! use dprint::{dprint, dprintln, deprint, deprintln};
//!
//! let x = 5 * 2;
//!
//! dprintln!("x = {}", x);
//! dprint!("x");
//! dprint!(" = ");
//! dprint!("{}", x);
//! deprintln!("I'm printing to the Standard Error");
//! deprint!("I'm printing to the Standard Error");
//! ```
//!
//! ## feature: `replace`
//! The `replace` feature can be  enabled for `println`, `print`, `println`, and `print` macros to
//! replace the Rust standard library implementations.
//! ```
//! use dprint::{println, print, eprintln, eprint};
//!
//! let x = 5 * 2;
//!
//! dprintln!("x = {}", x);
//! dprint!("x");
//! dprint!(" = ");
//! dprint!("{}", x);
//! deprintln!("I'm printing to the Standard Error");
//! deprint!("I'm printing to the Standard Error");
//! ```
#[cfg(feature = "std")]
mod std;

#[cfg(feature = "replace")]
mod replace;
