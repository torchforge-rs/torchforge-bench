//! PyO3 FFI Overhead Benchmark Binary
//!
//! Run this binary to measure the overhead of calling Gymnasium environments via PyO3.

use torchforge_bench::ffi_overhead::run_ffi_benchmark;

fn main() {
    match run_ffi_benchmark() {
        Ok(()) => println!("FFI benchmark completed successfully."),
        Err(e) => eprintln!("FFI benchmark failed: {}", e),
    }
}
