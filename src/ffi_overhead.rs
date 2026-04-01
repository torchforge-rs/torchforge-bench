//! PyO3 FFI Overhead Benchmark
//!
//! Measures the latency overhead of calling Gymnasium environment.step() via PyO3
//! compared to native Python execution.

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct FfiOverheadResult {
    pub total_steps: usize,
    pub total_duration_ns: u128,
    pub avg_step_latency_ns: f64,
    pub min_step_latency_ns: u128,
    pub max_step_latency_ns: u128,
    pub std_deviation_ns: f64,
    pub hardware_info: HardwareInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu_model: String,
    pub os: String,
    pub rust_version: String,
    pub python_version: String,
    pub pytorch_version: String,
}

impl HardwareInfo {
    pub fn collect() -> PyResult<Self> {
        Python::with_gil(|py| {
            let platform = py.import_bound("platform")?;
            let cpu_model = platform.call_method0("processor")?.extract()?;
            let os_info = platform.call_method0("system")?.extract::<String>()?;

            let sys = py.import_bound("sys")?;
            let python_version = sys.getattr("version")?.extract()?;

            // Try to get torch version, but don't fail if not available
            let pytorch_version = match py.import_bound("torch") {
                Ok(torch) => torch.getattr("__version__")?.extract()?,
                Err(_) => "not installed".to_string(),
            };

            let rust_version = "1.85.0".to_string(); // Matches rust-toolchain.toml

            Ok(Self {
                cpu_model,
                os: os_info,
                rust_version,
                python_version,
                pytorch_version,
            })
        })
    }
}

pub fn measure_ffi_overhead(
    num_episodes: usize,
    steps_per_episode: usize,
) -> PyResult<FfiOverheadResult> {
    Python::with_gil(|py| {
        // Import gymnasium and create environment
        let gymnasium = py.import_bound("gymnasium")?;
        let env = gymnasium.call_method1("make", ("CartPole-v1",))?;

        let mut all_latencies: Vec<u128> = Vec::with_capacity(num_episodes * steps_per_episode);
        let total_start = Instant::now();

        for _episode in 0..num_episodes {
            // Reset environment
            let reset_result = env.call_method0("reset")?;
            let (_obs, _info) = reset_result.extract::<(PyObject, PyObject)>()?;

            // Run fixed-length episode with no-op policy (action = 0)
            for _step in 0..steps_per_episode {
                let step_start = Instant::now();

                // Take action 0 (no-op)
                let step_result = env.call_method1("step", (0,))?;
                let (_obs, _reward, _terminated, _truncated, _info) =
                    step_result.extract::<(PyObject, f64, bool, bool, PyObject)>()?;

                let step_duration = step_start.elapsed().as_nanos();
                all_latencies.push(step_duration);

                // If episode ended early, break and start new episode
                if _terminated || _truncated {
                    break;
                }
            }
        }

        let total_duration = total_start.elapsed().as_nanos();

        // Calculate statistics
        if all_latencies.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "No steps measured - all episodes terminated immediately",
            ));
        }

        let avg_latency = all_latencies.iter().sum::<u128>() as f64 / all_latencies.len() as f64;
        let min_latency = *all_latencies.iter().min().unwrap();
        let max_latency = *all_latencies.iter().max().unwrap();

        // Calculate standard deviation
        let variance = all_latencies
            .iter()
            .map(|&x| (x as f64 - avg_latency).powi(2))
            .sum::<f64>()
            / all_latencies.len() as f64;
        let std_dev = variance.sqrt();

        // Close environment
        env.call_method0("close")?;

        let hardware_info = HardwareInfo::collect()?;

        Ok(FfiOverheadResult {
            total_steps: all_latencies.len(),
            total_duration_ns: total_duration,
            avg_step_latency_ns: avg_latency,
            min_step_latency_ns: min_latency,
            max_step_latency_ns: max_latency,
            std_deviation_ns: std_dev,
            hardware_info,
        })
    })
}

pub fn run_ffi_benchmark() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import_bound("sys")?;
        let python_path = sys.getattr("executable")?.extract::<String>()?;
        let python_version = sys.getattr("version")?.extract::<String>()?;
        println!("Using Python: {} ({})", python_path, python_version);

        // Check if gymnasium is available
        match py.import_bound("gymnasium") {
            Ok(_) => println!("gymnasium module found"),
            Err(e) => {
                println!("gymnasium module not found: {}", e);
                return Err(e);
            }
        }

        Ok::<(), PyErr>(())
    })?;

    println!("Running PyO3 FFI Overhead Benchmark...");
    println!("Target: CartPole-v1 env.step() latency");

    // Benchmark parameters
    let num_episodes = 100;
    let steps_per_episode = 500; // CartPole max episode length

    let result = measure_ffi_overhead(num_episodes, steps_per_episode)?;

    println!("\n=== FFI Overhead Results ===");
    println!("Total steps measured: {}", result.total_steps);
    println!(
        "Average step latency: {:.2} ns ({:.2} μs)",
        result.avg_step_latency_ns,
        result.avg_step_latency_ns / 1000.0
    );
    println!(
        "Min step latency: {} ns ({:.2} μs)",
        result.min_step_latency_ns,
        result.min_step_latency_ns as f64 / 1000.0
    );
    println!(
        "Max step latency: {} ns ({:.2} μs)",
        result.max_step_latency_ns,
        result.max_step_latency_ns as f64 / 1000.0
    );
    println!("Standard deviation: {:.2} ns", result.std_deviation_ns);

    println!("\n=== Hardware Information ===");
    println!("CPU: {}", result.hardware_info.cpu_model);
    println!("OS: {}", result.hardware_info.os);
    println!("Rust version: {}", result.hardware_info.rust_version);
    println!("Python version: {}", result.hardware_info.python_version);
    println!("PyTorch version: {}", result.hardware_info.pytorch_version);

    // Save results to JSON
    let results_json = serde_json::to_string_pretty(&result).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Serialization error: {}", e))
    })?;

    std::fs::write("results/ffi_overhead/pyo3_ffi_overhead.json", results_json).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("File write error: {}", e))
    })?;

    println!("\nResults saved to: results/ffi_overhead/pyo3_ffi_overhead.json");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_overhead_measurement() {
        // Small test to verify the measurement works
        let result = measure_ffi_overhead(1, 10).unwrap();
        assert!(result.total_steps > 0);
        assert!(result.avg_step_latency_ns > 0.0);
    }
}
