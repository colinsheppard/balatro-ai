#!/usr/bin/env python3
"""
Debug script for stepping through a trained PPO agent's inference.

This script loads a trained model and allows you to step through each action
with detailed debugging information.
"""

import os
import sys
import argparse
from pathlib import Path
import numpy as np

# Add pylatro_ai to path
sys.path.insert(0, str(Path(__file__).parent))

from stable_baselines3 import PPO
from pylatro_ai.balatro_env import BalatroEnv


def print_step_info(step_num, observation, action, reward, terminated, truncated, info, deterministic=True):
    """Print detailed information about a step."""
    print("\n" + "=" * 80)
    print(f"STEP {step_num}")
    print("=" * 80)
    
    print(f"\n📊 OBSERVATION:")
    print(f"  Phase: {info.get('phase', 'Unknown')}")
    print(f"  Status: {info.get('status', 'Unknown')}")
    print(f"  Phase Index: {observation['phase'][0]}")
    print(f"  Valid Actions: {info.get('valid_actions', [])}")
    print(f"  Available Actions Mask: {np.where(observation['available_actions'] == 1)[0].tolist()}")
    print(f"  Game Info:")
    game_info = observation['game_info']
    print(f"    Score: {game_info[0]:.2f}")
    print(f"    Money: {game_info[1]:.2f}")
    print(f"    Ante: {game_info[2]:.0f}")
    print(f"    Round: {game_info[3]:.0f}")
    print(f"    Hands Remaining: {game_info[4]:.0f}")
    print(f"    Discards Remaining: {game_info[5]:.0f}")
    print(f"    Blind Started: {bool(game_info[6])}")
    print(f"    Blind Finished: {bool(game_info[7])}")
    print(f"    Step Count: {game_info[8]:.0f}")
    
    print(f"\n🎯 ACTION:")
    print(f"  Selected Action: {action}")
    if info.get('action_mapped', False):
        print(f"  ⚠️  Action was mapped from {info.get('original_action')} to {info.get('mapped_action')}")
    else:
        print(f"  ✓ Action is valid")
    
    print(f"\n💰 REWARD:")
    print(f"  Reward: {reward:.4f}")
    print(f"  Cumulative: {info.get('cumulative_reward', 0):.4f}")
    
    print(f"\n🏁 STATE:")
    print(f"  Terminated: {terminated}")
    print(f"  Truncated: {truncated}")
    if terminated or truncated:
        print(f"  Episode finished!")
    
    print(f"\n📝 ADDITIONAL INFO:")
    print(f"  Score: {info.get('score', 'N/A')}")
    print(f"  Money: {info.get('money', 'N/A')}")
    if info.get('finished') is not None:
        print(f"  Finished: {info.get('finished')}")


def debug_agent(
    model_path: str,
    seed: int = 42,
    max_steps: int = 1000,
    deterministic: bool = True,
    interactive: bool = False,
):
    """
    Debug a trained agent by stepping through inference.
    
    Args:
        model_path: Path to the saved model
        seed: Random seed for the environment
        max_steps: Maximum number of steps to run
        deterministic: Whether to use deterministic action selection
        interactive: If True, pause after each step for inspection
    """
    print("=" * 80)
    print("DEBUGGING TRAINED AGENT")
    print("=" * 80)
    print(f"\nModel Path: {model_path}")
    print(f"Seed: {seed}")
    print(f"Deterministic: {deterministic}")
    print(f"Interactive Mode: {interactive}")
    print(f"Max Steps: {max_steps}")
    
    # Load the trained model
    print("\n" + "-" * 80)
    print("Loading model...")
    print("-" * 80)
    if not os.path.exists(model_path):
        print(f"❌ Error: Model file not found at {model_path}")
        print(f"\nAvailable models in ./models/balatro_ppo/:")
        model_dir = Path("./models/balatro_ppo")
        if model_dir.exists():
            for file in model_dir.glob("*.zip"):
                print(f"  - {file}")
        sys.exit(1)
    
    try:
        model = PPO.load(model_path)
        print(f"✓ Model loaded successfully")
        print(f"  Policy: {model.policy}")
        print(f"  Device: {model.device}")
    except Exception as e:
        print(f"❌ Error loading model: {e}")
        sys.exit(1)
    
    # Create environment
    print("\n" + "-" * 80)
    print("Creating environment...")
    print("-" * 80)
    try:
        # In debug mode, we might want to see output, so allow quiet to be disabled
        # For now, keep it quiet by default but could add a flag
        env = BalatroEnv(seed=seed, quiet=True)  # Set quiet=False to see Rust output
        print(f"✓ Environment created")
        print(f"  Observation Space: {env.observation_space}")
        print(f"  Action Space: {env.action_space}")
        print(f"  Quiet Mode: {env.quiet}")
    except Exception as e:
        print(f"❌ Error creating environment: {e}")
        sys.exit(1)
    
    # Reset environment
    print("\n" + "-" * 80)
    print("Resetting environment...")
    print("-" * 80)
    obs, info = env.reset()
    print(f"✓ Environment reset")
    print(f"  Initial Phase: {info.get('phase', 'Unknown')}")
    print(f"  Initial Valid Actions: {info.get('valid_actions', [])}")
    
    # Run inference
    print("\n" + "-" * 80)
    print("Starting inference...")
    print("-" * 80)
    
    cumulative_reward = 0.0
    step_count = 0
    
    try:
        while step_count < max_steps:
            # Get action from model
            action, _states = model.predict(obs, deterministic=deterministic)
            action = int(action)  # Convert to int
            
            # Step environment
            obs, reward, terminated, truncated, info = env.step(action)
            
            # Update cumulative reward
            cumulative_reward += reward
            info['cumulative_reward'] = cumulative_reward
            
            # Print step information
            print_step_info(
                step_count + 1,
                obs,
                action,
                reward,
                terminated,
                truncated,
                info,
                deterministic=deterministic,
            )
            
            step_count += 1
            
            # Check if done
            if terminated or truncated:
                print("\n" + "=" * 80)
                print("EPISODE FINISHED")
                print("=" * 80)
                print(f"Total Steps: {step_count}")
                print(f"Total Reward: {cumulative_reward:.4f}")
                print(f"Final Phase: {info.get('phase', 'Unknown')}")
                break
            
            # Interactive mode: pause for inspection
            if interactive:
                user_input = input("\nPress Enter to continue, 'q' to quit, or 'i' for info: ")
                if user_input.lower() == 'q':
                    print("\nStopped by user")
                    break
                elif user_input.lower() == 'i':
                    print("\n" + "-" * 80)
                    print("DETAILED STATE INFO")
                    print("-" * 80)
                    print(f"Observation keys: {list(obs.keys())}")
                    print(f"Info keys: {list(info.keys())}")
                    print(f"Observation shapes:")
                    for key, value in obs.items():
                        print(f"  {key}: {value.shape if hasattr(value, 'shape') else type(value)}")
                    input("\nPress Enter to continue...")
    
    except KeyboardInterrupt:
        print("\n\nStopped by user (Ctrl+C)")
    except Exception as e:
        print(f"\n❌ Error during inference: {e}")
        import traceback
        traceback.print_exc()
    finally:
        print("\n" + "=" * 80)
        print("SUMMARY")
        print("=" * 80)
        print(f"Steps Executed: {step_count}")
        print(f"Cumulative Reward: {cumulative_reward:.4f}")
        env.close()
        print("\n✓ Environment closed")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Debug a trained PPO agent with step-by-step inference"
    )
    parser.add_argument(
        "--model",
        type=str,
        default="./models/balatro_ppo/ppo_balatro_final.zip",
        help="Path to the trained model (default: ./models/balatro_ppo/ppo_balatro_final.zip)",
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=42,
        help="Random seed for the environment (default: 42)",
    )
    parser.add_argument(
        "--max-steps",
        type=int,
        default=1000,
        help="Maximum number of steps to run (default: 1000)",
    )
    parser.add_argument(
        "--stochastic",
        action="store_true",
        help="Use stochastic action selection instead of deterministic",
    )
    parser.add_argument(
        "--interactive",
        action="store_true",
        help="Pause after each step for inspection (interactive mode)",
    )
    
    args = parser.parse_args()
    
    debug_agent(
        model_path=args.model,
        seed=args.seed,
        max_steps=args.max_steps,
        deterministic=not args.stochastic,
        interactive=args.interactive,
    )


if __name__ == "__main__":
    main()

