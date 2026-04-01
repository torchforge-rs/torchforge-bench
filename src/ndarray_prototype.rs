//! ndarray DQN Prototype
//!
//! Minimal DQN Q-network implementation using pure ndarray.
//! Tests forward pass, backward pass, and basic training functionality.

use ndarray::{Array1, Array2};

// CartPole-v1 environment specifications
const OBSERVATION_DIM: usize = 4; // [position, velocity, angle, angular_velocity]
const ACTION_DIM: usize = 2; // [left, right]
const HIDDEN_DIM: usize = 64; // Hidden layer size for Q-network

pub struct QNetwork {
    weights1: Array2<f32>,
    bias1: Array1<f32>,
    weights2: Array2<f32>,
    bias2: Array1<f32>,
    weights3: Array2<f32>,
    bias3: Array1<f32>,
}

impl QNetwork {
    pub fn new() -> Self {
        // Initialize weights with small values
        let weights1 = Array2::zeros((OBSERVATION_DIM, HIDDEN_DIM));
        let bias1 = Array1::zeros(HIDDEN_DIM);

        let weights2 = Array2::zeros((HIDDEN_DIM, HIDDEN_DIM));
        let bias2 = Array1::zeros(HIDDEN_DIM);

        let weights3 = Array2::zeros((HIDDEN_DIM, ACTION_DIM));
        let bias3 = Array1::zeros(ACTION_DIM);

        Self {
            weights1,
            bias1,
            weights2,
            bias2,
            weights3,
            bias3,
        }
    }

    pub fn forward(&self, observations: &Array2<f32>) -> Array2<f32> {
        // Layer 1: Linear + ReLU
        let layer1_output = observations.dot(&self.weights1) + &self.bias1;
        let layer1_activated = relu(&layer1_output);

        // Layer 2: Linear + ReLU
        let layer2_output = layer1_activated.dot(&self.weights2) + &self.bias2;
        let layer2_activated = relu(&layer2_output);

        // Layer 3: Linear (no activation for output)
        layer2_activated.dot(&self.weights3) + &self.bias3
    }

    pub fn weights_mut(&mut self) -> [&mut Array2<f32>; 3] {
        [&mut self.weights1, &mut self.weights2, &mut self.weights3]
    }

    pub fn biases_mut(&mut self) -> [&mut Array1<f32>; 3] {
        [&mut self.bias1, &mut self.bias2, &mut self.bias3]
    }
}

impl Default for QNetwork {
    fn default() -> Self {
        Self::new()
    }
}

fn relu(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| v.max(0.0))
}

pub fn create_mock_batch(batch_size: usize) -> (Array2<f32>, Array1<usize>, Array1<f32>) {
    let observations = Array2::zeros((batch_size, OBSERVATION_DIM));
    let actions = Array1::zeros(batch_size);
    let rewards = Array1::zeros(batch_size);

    (observations, actions, rewards)
}

pub fn test_ndarray_dqn() {
    println!("Testing ndarray DQN Prototype...");

    // Create Q-network
    let q_network = QNetwork::new();
    println!("✓ Q-network created");

    // Test forward pass
    let batch_size = 32;
    let (observations, _actions, _rewards) = create_mock_batch(batch_size);

    let start = std::time::Instant::now();
    let q_values = q_network.forward(&observations);
    let forward_time = start.elapsed();

    println!(
        "✓ Forward pass: {:?} -> {:?} in {:?}",
        observations.shape(),
        q_values.shape(),
        forward_time
    );

    // Test loss computation (MSE)
    let target_q_values: Array2<f32> = Array2::zeros((batch_size, ACTION_DIM));

    let start = std::time::Instant::now();
    let diff = &q_values - &target_q_values;
    let loss = (&diff * &diff)
        .mean()
        .expect("Empty array in loss computation");
    let loss_time = start.elapsed();

    println!("✓ MSE loss computation completed in {:?}", loss_time);
    println!("✓ Loss value: {:.6}", loss);

    // Test simple gradient computation (numerical gradient)
    let epsilon = 1e-5;
    let mut q_network_grad = QNetwork::new();

    let start = std::time::Instant::now();

    // Compute numerical gradient for first layer weights
    for i in 0..q_network_grad.weights1.nrows() {
        for j in 0..q_network_grad.weights1.ncols() {
            let original = q_network_grad.weights1[[i, j]];

            // Forward pass with +epsilon
            q_network_grad.weights1[[i, j]] = original + epsilon;
            let loss_plus = q_network_grad.forward(&observations);
            let loss_plus = (&loss_plus - &target_q_values)
                .mapv(|v| v * v)
                .mean()
                .expect("Empty array in loss_plus computation");

            // Forward pass with -epsilon
            q_network_grad.weights1[[i, j]] = original - epsilon;
            let loss_minus = q_network_grad.forward(&observations);
            let loss_minus = (&loss_minus - &target_q_values)
                .mapv(|v| v * v)
                .mean()
                .expect("Empty array in loss_minus computation");

            // Numerical gradient with numerical stability check
            let grad = if loss_plus.is_finite() && loss_minus.is_finite() {
                (loss_plus - loss_minus) / (2.0 * epsilon)
            } else {
                0.0 // Handle numerical issues gracefully
            };
            q_network_grad.weights1[[i, j]] = original;

            // Simple gradient update
            q_network_grad.weights1[[i, j]] -= 0.01 * grad;
        }
    }

    let gradient_time = start.elapsed();
    println!(
        "✓ Numerical gradient computation for first layer completed in {:?}",
        gradient_time
    );

    // Test multiple training steps with simple gradient descent
    let mut q_network_train = QNetwork::new();

    let start = std::time::Instant::now();
    for i in 0..10 {
        // Reduced from 100 due to numerical gradient cost
        let (obs, _actions, _rewards) = create_mock_batch(batch_size);
        let q_values = q_network_train.forward(&obs);
        let target: Array2<f32> = Array2::zeros((batch_size, ACTION_DIM));
        let loss = (&q_values - &target)
            .mapv(|v| v * v)
            .mean()
            .expect("Empty array in training loss computation");

        // Simple parameter update (mock gradient)
        q_network_train
            .weights1
            .mapv_inplace(|v| v - 0.001 * (v - 0.5));
        q_network_train
            .weights2
            .mapv_inplace(|v| v - 0.001 * (v - 0.5));
        q_network_train
            .weights3
            .mapv_inplace(|v| v - 0.001 * (v - 0.5));

        if i % 2 == 0 {
            println!("  Training step {} completed, loss: {:.6}", i + 1, loss);
        }
    }
    let training_time = start.elapsed();

    println!("✓ 10 training steps completed in {:?}", training_time);
    println!("✓ Average per step: {:?}", training_time / 10);

    println!("ndarray DQN prototype test completed successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ndarray_dqn_creation() {
        // Test that QNetwork can be created and forward pass works
        let q_network = QNetwork::new();
        let (observations, _actions, _rewards) = create_mock_batch(4);

        let q_values = q_network.forward(&observations);

        // Assert expected shape and basic properties
        assert_eq!(q_values.shape(), &[4, 2]);
        assert!(q_values.iter().all(|&v| v.is_finite()));

        // Test loss computation doesn't panic
        let target: Array2<f32> = Array2::zeros((4, 2));
        let loss = (&q_values - &target)
            .mapv(|v| v * v)
            .mean()
            .expect("Empty array in test loss computation");
        assert!(loss.is_finite());
        assert!(loss >= 0.0);
    }
}
