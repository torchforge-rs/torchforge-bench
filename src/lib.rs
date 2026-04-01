//! # torchforge-bench
//!
//! ML benchmark suite for reproducing paper results in Rust.
//!
//! The goal: reproduce high-profile ML paper architectures in Rust and
//! demonstrate measurable performance and memory advantages over Python
//! reference implementations — particularly on edge hardware.
//!
//! ## Modules

pub mod ffi_overhead;
pub mod ndarray_prototype;
