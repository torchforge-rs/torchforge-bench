# Benchmark Results

This directory contains published benchmark results for TorchForge implementations. Each result file follows a standardized JSON schema to ensure reproducibility and comparability.

## Results Schema

All result files must follow the JSON schema below. The schema is designed for extensibility - new fields can be added in future versions without invalidating existing result files.

### Required Fields

```json
{
  "metadata": {
    "version": "0.1.0",
    "timestamp": "2024-03-26T12:00:00Z",
    "torchforge_version": "0.0.1"
  },
  "algorithm": {
    "name": "DQN",
    "implementation": "rust-native",
    "version": "0.1.0"
  },
  "environment": {
    "name": "CartPole-v1",
    "version": "1.0.0"
  },
  "hardware": {
    "cpu": {
      "model": "Intel Core i7-12700K",
      "cores": 12,
      "threads": 20,
      "base_frequency": 3.6,
      "max_frequency": 4.9
    },
    "gpu": {
      "model": "NVIDIA RTX 4070",
      "memory": 12,
      "compute_capability": "8.9"
    },
    "ram": {
      "size": 32,
      "type": "DDR4-3200",
      "channels": 2
    },
    "storage": {
      "type": "NVMe SSD",
      "size": 1000
    }
  },
  "software": {
    "os": {
      "name": "Ubuntu",
      "version": "22.04 LTS",
      "kernel": "5.15.0"
    },
    "rust": {
      "version": "1.85.0",
      "channel": "stable",
      "target": "x86_64-unknown-linux-gnu"
    },
    "python": {
      "version": "3.12.0",
      "implementation": "CPython"
    },
    "dependencies": {
      "torch": "2.4.0",
      "gymnasium": "1.0.0",
      "numpy": "1.26.0"
    }
  },
  "parameters": {
    "seed": 1,
    "episodes": 1000,
    "learning_rate": 0.001,
    "batch_size": 32,
    "buffer_size": 10000,
    "target_update_frequency": 1000,
    "epsilon_start": 1.0,
    "epsilon_end": 0.01,
    "epsilon_decay": 1000
  },
  "metrics": {
    "performance": {
      "mean_reward": 195.5,
      "std_reward": 12.3,
      "min_reward": 0.0,
      "max_reward": 500.0,
      "convergence_episode": 423,
      "final_reward": 498.0
    },
    "efficiency": {
      "training_time": 45.2,
      "episodes_per_second": 22.1,
      "steps_per_second": 2205.3
    },
    "resource": {
      "peak_memory_usage": 512.0,
      "peak_cpu_usage": 85.2,
      "peak_gpu_usage": 67.8,
      "gpu_memory_usage": 2.1
    },
    "quality": {
      "stability_score": 0.95,
      "convergence_quality": 0.88,
      "final_performance": 0.99
    }
  },
  "reproducibility": {
    "random_seed": 1,
    "deterministic": true,
    "multiple_runs": 5,
    "run_variance": 2.1
  }
}
```

### Optional Fields (for future extensibility)

```json
{
  "federation": {
    "device_count": 4,
    "aggregation_algorithm": "fedavg",
    "communication_rounds": 100,
    "local_epochs": 5,
    "bandwidth": "1Gbps",
    "latency": "10ms"
  },
  "scalability": {
    "parallel_workers": 8,
    "distributed": true,
    "cluster_size": 4,
    "speedup_factor": 3.2
  },
  "advanced_metrics": {
    "gradient_norm": 0.15,
    "loss_variance": 0.002,
    "exploration_efficiency": 0.73,
    "sample_efficiency": 0.81
  }
}
```

## File Naming Convention

Result files follow the naming convention:
```
{algorithm}_{environment}_{implementation}_{timestamp}.json
```

Examples:
- `dqn_cartpole_rust-native_2024-03-26_12-00-00.json`
- `ppo_lunarlander_python-baseline_2024-03-26_14-30-15.json`

## Directory Structure

```
results/
├── README.md                    # This file
├── schema.json                  # JSON schema validation file
├── dqn_cartpole_rust-native_2024-03-26_12-00-00.json
├── dqn_cartpole_python-baseline_2024-03-26_12-15-30.json
├── ppo_lunarlander_rust-native_2024-03-26_13-45-22.json
└── archive/                     # Old results (organized by date)
    ├── 2024-03/
    └── 2024-02/
```

## Append-Only Policy

The `results/` directory is **append-only** - never overwrite existing published results. If you need to:

1. **Correct a result**: Add a new file with corrected data and a timestamp
2. **Update methodology**: Add a new file with updated parameters
3. **Retire old results**: Move old files to the `archive/` subdirectory

## Validation

All result files should be validated against the schema:

```bash
# Validate a result file
python -c "
import json
import jsonschema

# Load schema
with open('results/schema.json', 'r') as f:
    schema = json.load(f)

# Load result
with open('results/your_result.json', 'r') as f:
    result = json.load(f)

# Validate
jsonschema.validate(result, schema)
print('Validation passed!')
"
```

## Submitting Results

When submitting benchmark results:

1. **Complete Methodology Table**: Ensure all fields in the methodology table are populated
2. **Validate Schema**: Run schema validation on your result file
3. **Document Environment**: Include complete hardware and software specifications
4. **Reproducibility**: Include random seeds and run multiple times for variance
5. **File Naming**: Use the standard naming convention
6. **CHANGELOG Entry**: Add entry under `[Added]` section

## Result Categories

### Performance Results
- Mean reward and standard deviation
- Convergence metrics
- Final performance scores

### Efficiency Results
- Training time and throughput
- Resource utilization
- Memory usage patterns

### Quality Results
- Stability and convergence quality
- Sample efficiency
- Exploration efficiency

### Reproducibility Results
- Multiple run statistics
- Variance analysis
- Determinism verification

## Comparing Results

When comparing results:

1. **Hardware Normalization**: Account for hardware differences
2. **Software Versions**: Note version differences
3. **Parameter Consistency**: Ensure comparable hyperparameters
4. **Statistical Significance**: Consider variance and confidence intervals

## Archival Policy

Results are archived based on:

- **Age**: Results older than 6 months may be archived
- **Supersession**: New results that replace old ones
- **Methodology Changes**: Results using outdated methodology

Archived results remain accessible but are not included in default comparisons.

## Tools and Utilities

### Result Analysis
```bash
# Compare two results
python scripts/compare_results.py result1.json result2.json

# Generate summary report
python scripts/generate_summary.py results/

# Plot performance trends
python scripts/plot_trends.py results/ --algorithm DQN
```

### Schema Management
```bash
# Update schema version
python scripts/update_schema.py --version 0.2.0

# Migrate old results to new schema
python scripts/migrate_results.py --from-version 0.1.0 --to-version 0.2.0
```

## Troubleshooting

### Common Validation Errors

1. **Missing Required Fields**: Ensure all required fields are present
2. **Type Mismatches**: Check that field types match schema
3. **Invalid Values**: Ensure values are within allowed ranges
4. **Timestamp Format**: Use ISO 8601 format for timestamps

### Performance Issues

1. **Large Result Files**: Consider compressing large metric arrays
2. **Slow Validation**: Use streaming validation for large files
3. **Memory Usage**: Process results in batches for large datasets

## Contributing

When contributing to the results schema:

1. **Backward Compatibility**: Ensure changes don't break existing results
2. **Documentation**: Update schema documentation
3. **Migration**: Provide migration scripts for breaking changes
4. **Validation**: Update validation tools and tests
