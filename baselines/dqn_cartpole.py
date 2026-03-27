#!/usr/bin/env python3
"""
DQN (Deep Q-Network) implementation for CartPole-v1.

Based on CleanRL's dqn.py implementation.
Reference: https://github.com/vwxyzjn/cleanrl
"""

import argparse
import os
import random
import time
from collections import deque

import gymnasium as gym
import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.tensorboard import SummaryWriter


class QNetwork(nn.Module):
    """Deep Q-Network for CartPole."""

    def __init__(self, env):
        super().__init__()
        self.network = nn.Sequential(
            nn.Linear(np.array(env.single_observation_space.shape).prod(), 120),
            nn.ReLU(),
            nn.Linear(120, 84),
            nn.ReLU(),
            nn.Linear(84, env.single_action_space.n),
        )

    def forward(self, x):
        return self.network(x)


def make_env(env_id, seed):
    """Create and wrap the environment."""

    def thunk():
        env = gym.make(env_id)
        env = gym.wrappers.RecordEpisodeStatistics(env)
        env.action_space.seed(seed)
        env.observation_space.seed(seed)
        return env

    return thunk


def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(description="DQN for CartPole-v1")
    parser.add_argument("--seed", type=int, default=1, help="Random seed")
    parser.add_argument("--env", type=str, default="CartPole-v1", help="Environment ID")
    parser.add_argument("--episodes", type=int, default=1000, help="Number of episodes")
    parser.add_argument("--learning-rate", type=float, default=2.5e-4, help="Learning rate")
    parser.add_argument("--buffer-size", type=int, default=10000, help="Replay buffer size")
    parser.add_argument("--gamma", type=float, default=0.99, help="Discount factor")
    parser.add_argument("--tau", type=float, default=1.0, help="Target network update rate")
    parser.add_argument("--target-update-frequency", type=int, default=500, help="Target update frequency")
    parser.add_argument("--batch-size", type=int, default=128, help="Batch size")
    parser.add_argument("--start-e", type=float, default=1.0, help="Starting epsilon")
    parser.add_argument("--end-e", type=float, default=0.05, help="Ending epsilon")
    parser.add_argument("--exploration-fraction", type=float, default=0.5, help="Exploration fraction")
    parser.add_argument("--learning-starts", type=int, default=10000, help="Learning starts step")
    parser.add_argument("--capture-video", action="store_true", help="Capture video")
    return parser.parse_args()


def main():
    """Main training loop."""
    args = parse_args()

    # Set random seeds
    random.seed(args.seed)
    np.random.seed(args.seed)
    torch.manual_seed(args.seed)
    torch.backends.cudnn.deterministic = True

    # Create environment
    env = gym.vector.SyncVectorEnv([make_env(args.env, args.seed)])

    # Create networks
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    q_network = QNetwork(env).to(device)
    target_network = QNetwork(env).to(device)
    target_network.load_state_dict(q_network.state_dict())
    optimizer = optim.Adam(q_network.parameters(), lr=args.learning_rate)

    # Replay buffer
    replay_buffer = deque(maxlen=args.buffer_size)

    # Training variables
    start_time = time.time()
    global_step = 0
    episode_rewards = deque(maxlen=100)

    # TensorBoard writer
    writer = SummaryWriter(f"runs/{args.env}_{args.seed}_{int(time.time())}")

    # Training loop
    obs, _ = env.reset(seed=args.seed)
    for episode in range(args.episodes):
        episode_reward = 0.0

        for step in range(500):  # Max steps per episode
            global_step += 1

            # Epsilon-greedy action selection
            epsilon = args.start_e + (args.end_e - args.start_e) * (
                global_step / (args.exploration_fraction * args.episodes * 500)
            )
            epsilon = max(epsilon, args.end_e)

            if random.random() < epsilon:
                actions = np.array([env.single_action_space.sample() for _ in range(env.num_envs)])
            else:
                q_values = q_network(torch.Tensor(obs).to(device))
                actions = torch.argmax(q_values, dim=1).cpu().numpy()

            # Step environment
            next_obs, rewards, terminations, truncations, infos = env.step(actions)
            episode_reward += rewards[0]

            # Store in replay buffer
            replay_buffer.append((obs[0], actions[0], rewards[0], next_obs[0], terminations[0]))

            # Training
            if global_step > args.learning_starts and global_step % 4 == 0:
                if len(replay_buffer) >= args.batch_size:
                    indices = random.sample(range(len(replay_buffer)), args.batch_size)
                    batch = [replay_buffer[i] for i in indices]
                    obs_batch, action_batch, reward_batch, next_obs_batch, done_batch = zip(*batch)

                    obs_batch = torch.Tensor(np.array(obs_batch)).to(device)
                    action_batch = torch.Tensor(np.array(action_batch)).long().to(device)
                    reward_batch = torch.Tensor(np.array(reward_batch)).to(device)
                    next_obs_batch = torch.Tensor(np.array(next_obs_batch)).to(device)
                    done_batch = torch.Tensor(np.array(done_batch)).to(device)

                    # Compute Q-learning loss
                    with torch.no_grad():
                        target_max = target_network(next_obs_batch).max(dim=1)[0]
                        td_target = reward_batch + args.gamma * target_max * (1 - done_batch)

                    q_values = q_network(obs_batch).gather(1, action_batch.unsqueeze(1)).squeeze(1)
                    loss = nn.functional.mse_loss(q_values, td_target)

                    optimizer.zero_grad()
                    loss.backward()
                    optimizer.step()

                    # Update target network
                    if global_step % args.target_update_frequency == 0:
                        for target_param, param in zip(target_network.parameters(), q_network.parameters()):
                            target_param.data.copy_(args.tau * param.data + (1.0 - args.tau) * target_param.data)

            obs = next_obs

            if "final_info" in infos:
                for info in infos["final_info"]:
                    if "episode" in info:
                        episode_rewards.append(info["episode"]["r"])
                        writer.add_scalar("charts/episode_reward", info["episode"]["r"], episode)
                        break

        if (episode + 1) % 100 == 0:
            print(
                f"Episode {episode + 1}/{args.episodes} | "
                f"Avg Reward: {np.mean(list(episode_rewards)):.2f} | "
                f"Epsilon: {epsilon:.4f} | "
                f"Time: {time.time() - start_time:.2f}s"
            )

    env.close()
    writer.close()
    print(f"Training completed in {time.time() - start_time:.2f}s")


if __name__ == "__main__":
    main()
