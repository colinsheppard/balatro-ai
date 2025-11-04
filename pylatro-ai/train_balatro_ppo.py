#!/usr/bin/env python3
"""
Train a PPO agent to play a single blind in Balatro.

This script:
1. Creates a Gymnasium environment wrapper for the Balatro engine
2. Trains a PPO agent using Stable-Baselines3
3. Uses a reward function: score + money bonus ($1 = 250 points)
4. Ends the episode when the blind is finished (on Python side)
"""

import os
import sys
import argparse
from pathlib import Path
from typing import Optional

# Add pylatro_ai to path
sys.path.insert(0, str(Path(__file__).parent))

from stable_baselines3 import PPO
from stable_baselines3.common.env_util import make_vec_env
from stable_baselines3.common.callbacks import EvalCallback, CheckpointCallback, BaseCallback
import wandb
from wandb.integration.sb3 import WandbCallback

from pylatro_ai.balatro_env import BalatroEnv


class SafeWandbCallback(BaseCallback):
    """Wrapper around WandbCallback that catches socket errors."""
    def __init__(self, wandb_callback, verbose=0):
        super().__init__(verbose)
        self.wandb_callback = wandb_callback
        self.error_count = 0
        self.max_errors = 10  # Disable after too many errors
        self.disabled = False
    
    def _on_step(self) -> bool:
        if self.disabled:
            return True
        try:
            return self.wandb_callback._on_step()
        except Exception as e:
            self.error_count += 1
            if self.error_count <= 3:  # Only print first few errors
                print(f"Warning: WandB logging error (suppressing): {type(e).__name__}")
            if self.error_count >= self.max_errors:
                print(f"WandB logging disabled after {self.max_errors} errors")
                self.disabled = True
            return True  # Always continue training
    
    def _on_training_end(self) -> None:
        if self.disabled:
            return
        try:
            self.wandb_callback._on_training_end()
        except Exception:
            pass  # Ignore errors on cleanup
    
    def _on_rollout_end(self) -> None:
        if self.disabled:
            return
        try:
            self.wandb_callback._on_rollout_end()
        except Exception:
            pass
    
    def _on_rollout_start(self) -> None:
        if self.disabled:
            return
        try:
            self.wandb_callback._on_rollout_start()
        except Exception:
            pass


def create_env(seed=None):
    """Create a single Balatro environment."""
    return BalatroEnv(seed=seed,quiet=True)


def train(
    total_timesteps: int = 100000,
    seed: int = 42,
    use_wandb: bool = True,
    log_dir: str = "./logs/balatro_ppo",
    save_dir: str = "./models/balatro_ppo",
    resume_from: Optional[str] = None,
    n_envs: int = 4,
):
    """
    Train a PPO agent on the Balatro environment.
    
    Args:
        total_timesteps: Total number of training timesteps (target, will train remaining if resuming)
        seed: Random seed for reproducibility
        use_wandb: Whether to use Weights & Biases for logging
        log_dir: Directory for logs
        save_dir: Directory for saved models
        resume_from: Path to checkpoint file to resume from (e.g., ./models/balatro_ppo/ppo_balatro_100000_steps.zip)
        n_envs: Number of parallel environments to run (default: 4, use 4-8 for 8-core systems)
    """
    # Create directories
    os.makedirs(log_dir, exist_ok=True)
    os.makedirs(save_dir, exist_ok=True)
    
    # Initialize WandB if requested
    if use_wandb:
        try:
            # Use offline mode to avoid socket connection issues
            # Set WANDB_MODE=offline environment variable or use mode="offline"
            wandb_mode = os.environ.get("WANDB_MODE", "online")
            
            # Option to use HTTP transport instead of service socket (more stable)
            # Set WANDB_TRANSPORT=http to use HTTP instead of service
            use_http = os.environ.get("WANDB_TRANSPORT", "").lower() == "http"
            
            # Configure WandB settings to prevent connection resets
            # The ConnectionResetError is happening in the async service client's socket
            # Root cause: The asyncio stream connection (_writer.drain()) is being reset
            # Solutions: Use valid settings, disable unnecessary features, use HTTP mode
            wandb_settings = wandb.Settings(
                # Connection settings - only use valid settings
                _disable_stats=True,  # Disable stats collection that can cause socket issues
                _disable_meta=True,   # Disable metadata collection
                _service_wait=60,      # Wait up to 60s for service (increased from 30)
                
                # Disable features that cause additional async connections
                console="off",          # Disable console output to reduce overhead
                
                # Network/buffer settings
                _network_buffer=65536,  # Increase network buffer size (64KB)
            )
            
            # If using HTTP transport, configure it
            if use_http:
                # Force HTTP mode - this bypasses the service socket that's causing issues
                os.environ["WANDB_SERVICE"] = "false"
                print("Using HTTP transport mode for WandB (more stable than service socket)")
            
            wandb.init(
                project="balatro-ppo",
                name="single-blind-training",
                config={
                    "total_timesteps": total_timesteps,
                    "seed": seed,
                    "algorithm": "PPO",
                    "env": "BalatroEnv",
                    "n_envs": n_envs,
                },
                mode=wandb_mode,  # Use offline mode if set, otherwise online
                sync_tensorboard=(wandb_mode == "online"),  # Only sync if online
                settings=wandb_settings,
                reinit=True,  # Allow reinitialization if needed
            )
        except Exception as e:
            print(f"Warning: WandB initialization failed: {e}")
            print("Continuing without WandB logging...")
            print("Tip: Use --no-wandb to disable WandB, or set WANDB_MODE=offline for offline mode")
            use_wandb = False
    
    # Create vectorized environment with parallel environments
    print(f"Creating {n_envs} parallel environments...")
    vec_env = make_vec_env(create_env, n_envs=n_envs, seed=seed)
    print(f"✓ Vectorized environment created with {n_envs} parallel environments")
    
    # Create or load PPO agent
    print("Loading/creating PPO agent...")
    if resume_from and os.path.exists(resume_from):
        print(f"Resuming from checkpoint: {resume_from}")
        model = PPO.load(resume_from)
        
        # Get the number of timesteps already trained
        # The model's num_timesteps attribute tracks this
        already_trained = model.num_timesteps
        remaining_timesteps = total_timesteps - already_trained
        
        if remaining_timesteps <= 0:
            print(f"Model already trained for {already_trained} timesteps, which exceeds target {total_timesteps}")
            print("Set a higher --timesteps value to continue training")
            return model
        
        print(f"Already trained: {already_trained} timesteps")
        print(f"Remaining: {remaining_timesteps} timesteps")
        
        # Update total_timesteps to only train the remaining amount
        total_timesteps = remaining_timesteps
        
        # Set the environment for the loaded model
        model.set_env(vec_env)
    else:
        if resume_from:
            print(f"Warning: Checkpoint not found at {resume_from}, starting fresh training")
        
        # Create new PPO agent
        print("Creating new PPO agent...")
        model = PPO(
            "MultiInputPolicy",  # Use MultiInputPolicy for Dict observation space
            vec_env,
            verbose=1,
            tensorboard_log=log_dir,
            seed=seed,
            learning_rate=3e-4,
            n_steps=2048,
            batch_size=64,
            n_epochs=10,
            gamma=0.99,
            gae_lambda=0.95,
            clip_range=0.2,
            ent_coef=0.01,
            vf_coef=0.5,
            max_grad_norm=0.5,
        )
    
    # Set up callbacks
    callbacks = []
    
    # Evaluation callback (use single env for evaluation)
    eval_env = create_env(seed=seed + 1000)  # Different seed for eval
    eval_callback = EvalCallback(
        eval_env,
        best_model_save_path=save_dir,
        log_path=log_dir,
        eval_freq=5000,
        deterministic=True,
        render=False,
    )
    callbacks.append(eval_callback)
    
    # Checkpoint callback
    checkpoint_callback = CheckpointCallback(
        save_freq=10e3,
        save_path=save_dir,
        name_prefix="ppo_balatro",
    )
    callbacks.append(checkpoint_callback)
    
    # WandB callback - reduce logging frequency to prevent connection overload
    if use_wandb:
        try:
            # Increase logging frequency intervals to reduce connection pressure
            # This reduces the number of socket writes and connection attempts
            wandb_callback = WandbCallback(
                gradient_save_freq=10e3,  # Increased from 1000 - save gradients less frequently
                model_save_path=save_dir,
                verbose=1,  # Reduced verbosity
                log="all",  # Log all metrics but less frequently
            )
            # Wrap in safe callback to catch socket errors (as backup)
            safe_wandb_callback = SafeWandbCallback(wandb_callback)
            callbacks.append(safe_wandb_callback)
        except Exception as e:
            print(f"Warning: WandB callback creation failed: {e}")
            print("Continuing without WandB callback...")
            use_wandb = False
    
    # Train the agent
    print(f"Training for {total_timesteps} timesteps...")
    model.learn(
        total_timesteps=total_timesteps,
        callback=callbacks,
        progress_bar=True,
    )
    
    # Save final model
    final_model_path = os.path.join(save_dir, "ppo_balatro_final")
    model.save(final_model_path)
    print(f"Final model saved to {final_model_path}")
    
    # Test the trained agent
    print("\nRun the following command to test the trained agent:\n\npython debug_agent.py --model {final_model_path}\n\n")
    
    # Close WandB
    if use_wandb:
        try:
            wandb.finish()
        except Exception as e:
            print(f"Warning: Error closing WandB: {e}")
            pass  # Continue even if WandB close fails
    
    print("\nTraining complete!")
    return model


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Train PPO agent on Balatro")
    parser.add_argument(
        "--timesteps",
        type=int,
        default=100000,
        help="Total number of training timesteps (default: 100000)",
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=42,
        help="Random seed (default: 42)",
    )
    parser.add_argument(
        "--no-wandb",
        action="store_true",
        help="Disable Weights & Biases logging",
    )
    parser.add_argument(
        "--log-dir",
        type=str,
        default="./logs/balatro_ppo",
        help="Directory for logs (default: ./logs/balatro_ppo)",
    )
    parser.add_argument(
        "--save-dir",
        type=str,
        default="./models/balatro_ppo",
        help="Directory for saved models (default: ./models/balatro_ppo)",
    )
    parser.add_argument(
        "--resume-from",
        type=str,
        default=None,
        help="Path to checkpoint to resume from (e.g., ./models/balatro_ppo/ppo_balatro_100000_steps.zip)",
    )
    parser.add_argument(
        "--n-envs",
        type=int,
        default=4,
        help="Number of parallel environments to run (default: 4, recommended: 4-8 for 8-core systems)",
    )
    
    args = parser.parse_args()
    
    train(
        total_timesteps=args.timesteps,
        seed=args.seed,
        use_wandb=not args.no_wandb,
        log_dir=args.log_dir,
        save_dir=args.save_dir,
        resume_from=args.resume_from,
        n_envs=args.n_envs,
    )


if __name__ == "__main__":
    main()

