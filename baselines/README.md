# TorchForge Baselines

This directory contains Python baseline implementations for TorchForge benchmarks. These baselines serve as reference implementations for comparing against Rust implementations.

## License Attribution

The baseline code in this directory is derived from [CleanRL](https://github.com/vwxyzjn/cleanrl), which is licensed under the MIT License. The original CleanRL license is reproduced below:

```
MIT License

Copyright (c) 2022 CleanRL contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Installation

### Prerequisites
- Python 3.12 or higher
- [uv](https://github.com/astral-sh/uv) package manager

### Setup
```bash
# Install dependencies
uv sync

# Activate virtual environment (optional)
source .venv/bin/activate  # On Windows: .venv\Scripts\activate
```

## Available Baselines

### DQN on CartPole
- **File**: `dqn_cartpole.py`
- **Environment**: CartPole-v1
- **Algorithm**: Deep Q-Network (DQN)
- **Reference**: [Playing Atari with Deep Reinforcement Learning](https://arxiv.org/abs/1312.5602)

## Running Baselines

### Basic Usage
```bash
# Run DQN on CartPole
uv run python dqn_cartpole.py

# Run with custom seed
uv run python dqn_cartpole.py --seed 42

# Run with custom parameters
uv run python dqn_cartpole.py --env CartPole-v1 --episodes 1000
```

### Available Arguments
- `--seed`: Random seed for reproducibility (default: 1)
- `--env`: Gymnasium environment name (default: CartPole-v1)
- `--episodes`: Number of episodes to run (default: 1000)
- `--save-model`: Save trained model to file
- `--capture-video`: Record video of agent performance

## Hardware Specifications

Published baseline results were obtained on the following hardware:

### Reference Hardware
- **CPU**: Intel Core i7-12700K (12 cores, 20 threads)
- **GPU**: NVIDIA RTX 4070 (12GB VRAM)
- **RAM**: 32GB DDR4-3200
- **Storage**: NVMe SSD

### Software Environment
- **OS**: Ubuntu 22.04 LTS
- **Python**: 3.12.0
- **PyTorch**: 2.4.0 (CUDA 12.1)
- **CUDA**: 12.1

## Reproducing Results

To reproduce published baseline results:

1. **Hardware**: Use similar hardware for comparable performance
2. **Software**: Ensure exact package versions (see `pyproject.toml`)
3. **Random Seed**: Use the same seed as published results
4. **Environment**: Use exact environment configuration

```bash
# Example: Reproduce DQN CartPole baseline
uv run python dqn_cartpole.py --seed 1 --episodes 1000
```

## Results Format

Baseline results are saved in JSON format compatible with the TorchForge results schema. Each result file contains:

```json
{
  "algorithm": "DQN",
  "environment": "CartPole-v1",
  "implementation": "python-baseline",
  "hardware": {
    "cpu": "Intel Core i7-12700K",
    "gpu": "NVIDIA RTX 4070",
    "ram": "32GB DDR4-3200"
  },
  "software": {
    "python": "3.12.0",
    "torch": "2.4.0",
    "gymnasium": "1.0.0"
  },
  "parameters": {
    "seed": 1,
    "episodes": 1000,
    "learning_rate": 0.001,
    "batch_size": 32
  },
  "metrics": {
    "mean_reward": 195.5,
    "std_reward": 12.3,
    "convergence_episode": 423,
    "training_time": 45.2,
    "memory_usage": 512.0
  }
}
```

## Development

### Code Style
```bash
# Format code
uv run black .

# Lint code
uv run ruff check .

# Run tests
uv run pytest
```

### Adding New Baselines
1. Create new Python file in this directory
2. Follow CleanRL coding style
3. Add documentation to this README
4. Update `pyproject.toml` if new dependencies needed
5. Add corresponding results schema documentation

## Troubleshooting

### Common Issues

#### CUDA Issues
```bash
# Check CUDA availability
python -c "import torch; print(torch.cuda.is_available())"

# Install CPU-only PyTorch if needed
uv add torch==2.4.0 --index-url https://download.pytorch.org/whl/cpu
```

#### Environment Issues
```bash
# Reset environment
uv sync --reinstall

# Check package versions
uv pip list
```

#### Performance Issues
- Ensure GPU is being used: check `torch.cuda.is_available()`
- Verify correct CUDA version compatibility
- Monitor GPU usage with `nvidia-smi`

## Contributing

When contributing to baselines:

1. **Maintain Compatibility**: Keep changes compatible with CleanRL
2. **Document Changes**: Update this README for any new features
3. **Test Thoroughly**: Ensure reproducibility across different hardware
4. **Version Pinning**: Pin exact dependency versions in `pyproject.toml`

## References

- [CleanRL GitHub](https://github.com/vwxyzjn/cleanrl)
- [Gymnasium Documentation](https://gymnasium.farama.org/)
- [PyTorch Documentation](https://pytorch.org/docs/)
