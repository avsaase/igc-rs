//! This crate provides a minimal, fast parser for IGC files.
//!
//! The low level record parser mirrors the raw format of an IGC file closely, and works to
//! minimize the number of heap allocations made during parsing.
//! It is intended to be used as an unopinionated base for building higher level data structures
//! representing traces/tasks/etc..

#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(all(feature = "std", feature = "heapless"))]
// compile_error!("features `std` and `heapless` are mutually exclusive");

#[cfg(test)]
#[macro_use]
extern crate proptest;

pub mod records;
pub mod util;
