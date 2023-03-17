#![feature(linkage)]
#![feature(link_llvm_intrinsics)]
#![feature(core_intrinsics)]
#![doc = include_str!("../README.md")]

/// Interesting values that can be used during mutations
mod dictionary;
/// Main fuzzing driver/entrypoint code
mod driver;
/// Exports for interfacing with Fazi via FFI
pub mod exports;
/// Main Fazi state management code
mod fazi;
/// Function hooks for builtin functions
mod hooks;
/// Contains the public mutation API
mod mutate;
/// Runtime configuration options
mod options;
mod protobuf;
/// SanitizerCoverage callbacks
mod sancov;
/// Signal handling code
mod signal;
/// Module for weak imports pulled from the Rust standard library
mod weak;
/// Weakly linked imports
mod weak_imports;
/// proto specific things for WA
mod proto_callback;

// generated protobuf files
mod protocol;
mod mms_retry;
mod e2e;

pub use fazi::*;

#[doc(hidden)]
pub use rand;
use rand::rngs::StdRng;
