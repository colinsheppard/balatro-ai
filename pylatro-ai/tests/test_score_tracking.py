"""
Test script to debug score tracking in BalatroEnv.

This test creates a BalatroEnv, steps through the game with hard-coded actions,
and prints detailed information about score and money tracking at each step.
"""

import pytest
import os
import sys
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

from pylatro_ai.balatro_env import BalatroEnv


@pytest.fixture
def balatro_engine():
    """Fixture to import balatro_engine module and set working directory"""
    try:
        import balatro_engine
        # Change to balatro-engine directory so CSV file can be found
        engine_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'balatro-engine')
        original_cwd = os.getcwd()
        os.chdir(engine_dir)
        try:
            yield balatro_engine
        finally:
            os.chdir(original_cwd)
    except ImportError:
        pytest.skip("balatro_engine module not available")


def test_score_tracking_step_by_step(balatro_engine):
    """
    Test score tracking by stepping through a game with hard-coded actions.
    
    This test allows you to set breakpoints and inspect the score/money values
    at each step to debug why score data might not be getting through.
    """
    # Create environment with quiet=False so we can see what's happening
    env = BalatroEnv(seed=12345, quiet=False)
    
    # Reset environment
    print("\n" + "=" * 80)
    print("RESETTING ENVIRONMENT")
    print("=" * 80)
    obs, info = env.reset()
    print(f"Initial Phase: {info.get('phase')}")
    print(f"Initial Valid Actions: {info.get('valid_actions')}")
    print(f"Initial Score from info: {info.get('score', 'N/A')}")
    print(f"Initial Money from info: {info.get('money', 'N/A')}")
    
    # Track cumulative score/money for debugging
    cumulative_score = 0
    cumulative_money = 0
    step_count = 0
    max_steps = 100
    
    # Hard-coded action sequence to play through a blind
    # Actions will be selected based on what's available
    action_sequence = []
    
    print("\n" + "=" * 80)
    print("STARTING STEP-BY-STEP EXECUTION")
    print("=" * 80)
    
    while step_count < max_steps:
        step_count += 1
        
        # Get valid actions
        valid_actions = info.get('valid_actions', [])
        if not valid_actions:
            print(f"\nNo valid actions at step {step_count}, stopping")
            break
        
        # Select action (use first valid action, or specific logic)
        if step_count == 1:
            # First action: should be BlindSelect - choose to play blind (action 1)
            if "BlindSelect" in str(info.get('phase', '')):
                action = 1  # Play blind
            else:
                action = valid_actions[0]
        elif step_count == 2:
            # After playing blind, we should be in Playing phase
            # Select first card (action index 2, since 0=Play, 1=Discard, 2+=cards)
            if len(valid_actions) >= 3:
                action = valid_actions[2]  # First card selection
            else:
                action = valid_actions[0]
        elif step_count == 3:
            # Continue selecting cards or play
            if 0 in valid_actions:  # Play action available
                action = 0  # Play selected cards
            elif len(valid_actions) >= 3:
                action = valid_actions[2]  # Select another card
            else:
                action = valid_actions[0]
        else:
            # For remaining steps, use a simple strategy
            if 0 in valid_actions:  # Play if available
                action = 0
            else:
                action = valid_actions[0]  # Use first available action
        
        action_sequence.append(action)
        
        print(f"\n{'=' * 80}")
        print(f"STEP {step_count}")
        print(f"{'=' * 80}")
        print(f"Phase: {info.get('phase')}")
        print(f"Valid Actions: {valid_actions}")
        print(f"Selected Action: {action}")
        print(f"Cumulative Score (before step): {cumulative_score}")
        print(f"Cumulative Money (before step): {cumulative_money}")
        
        # Step environment
        obs, reward, terminated, truncated, info = env.step(action)
        
        # Update cumulative tracking
        cumulative_score += reward
        step_score = info.get('score', 0)
        step_money = info.get('money', 0)
        
        print(f"\nAfter Step {step_count}:")
        print(f"  Reward: {reward:.4f}")
        print(f"  Cumulative Reward: {cumulative_score:.4f}")
        print(f"  Score from info: {step_score}")
        print(f"  Money from info: {step_money}")
        print(f"  Phase: {info.get('phase')}")
        print(f"  Status: {info.get('status')}")
        print(f"  Action Mapped: {info.get('action_mapped', False)}")
        if info.get('action_mapped'):
            print(f"    Original: {info.get('original_action')} -> Mapped: {info.get('mapped_action')}")
        
        # Check observation for score info
        game_info = obs.get('game_info', [])
        if len(game_info) >= 2:
            print(f"  Score from observation: {game_info[0]:.2f}")
            print(f"  Money from observation: {game_info[1]:.2f}")
        
        # Check if we can access the engine directly
        try:
            # Try to get score/money from engine if possible
            # This is for debugging - we'll see what's available
            if hasattr(env, 'engine') and env.engine:
                # Try to get step result to see what info is available
                temp_result = env.engine.step(None)
                print(f"  Engine Step Result Status: {temp_result.status}")
                print(f"  Engine Step Result Phase: {temp_result.phase}")
        except Exception as e:
            print(f"  Could not access engine directly: {e}")
        
        # Check if episode is done
        if terminated or truncated:
            print(f"\n{'=' * 80}")
            print("EPISODE FINISHED")
            print(f"{'=' * 80}")
            print(f"Total Steps: {step_count}")
            print(f"Final Reward: {cumulative_score:.4f}")
            print(f"Final Score from info: {step_score}")
            print(f"Final Money from info: {step_money}")
            break
        
        # Safety check - if we're stuck in a loop
        if step_count > 10 and len(set(action_sequence[-5:])) == 1:
            print(f"\nWarning: Possible action loop detected, stopping")
            break
    
    print(f"\n{'=' * 80}")
    print("TEST SUMMARY")
    print(f"{'=' * 80}")
    print(f"Total Steps: {step_count}")
    print(f"Cumulative Reward: {cumulative_score:.4f}")
    print(f"Action Sequence: {action_sequence}")
    
    # Final check - try to get score/money from environment state
    print(f"\nFinal Environment State:")
    print(f"  env.current_score: {env.current_score}")
    print(f"  env.current_money: {env.current_money}")
    print(f"  env.blind_started: {env.blind_started}")
    print(f"  env.blind_finished: {env.blind_finished}")
    
    env.close()


def test_direct_engine_score_access(balatro_engine):
    """
    Test direct access to engine to see if we can get score/money.
    
    This helps debug whether the issue is in the engine or in the environment wrapper.
    """
    import balatro_engine
    
    # Create engine directly
    engine_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'balatro-engine')
    original_cwd = os.getcwd()
    
    try:
        os.chdir(engine_dir)
        
        engine = balatro_engine.BalatroEngine(seed=12345)
        deck = balatro_engine.DeckType("Red")
        stake = balatro_engine.StakeLevel("White")
        engine.set_selected_deck(deck)
        engine.set_selected_stake(stake)
        engine.start_new_run_with_selections()
        
        print("\n" + "=" * 80)
        print("DIRECT ENGINE ACCESS TEST")
        print("=" * 80)
        
        # Step through a few actions
        for step_num in range(10):
            result = engine.step(None)
            print(f"\nStep {step_num}:")
            print(f"  Status: {result.status}")
            print(f"  Phase: {result.phase}")
            print(f"  Actions: {result.actions[:5]}...")  # First 5 actions
            
            # Try to get game state info if available
            # Note: game_state() might not work, but let's try
            try:
                state = engine.game_state()
                print(f"  Score from game_state(): {state.score()}")
                print(f"  Money from game_state(): {state.money()}")
            except Exception as e:
                print(f"  Could not access game_state(): {e}")
            
            # Select an action if available
            if result.actions:
                action = result.actions[0][0]  # First action
                print(f"  Executing action: {action}")
                result = engine.step(action)
                
                # Try to get score/money after action
                try:
                    state = engine.game_state()
                    print(f"  Score after action: {state.score()}")
                    print(f"  Money after action: {state.money()}")
                except Exception as e:
                    print(f"  Could not access game_state() after action: {e}")
            
            if result.status == "Finished" or (result.phase == "RoundEnd"):
                print(f"\nReached end state at step {step_num}")
                break
        
    finally:
        os.chdir(original_cwd)


def test_score_tracking_simple_flow(balatro_engine):
    """
    Simple flow test: start game, play blind, check score.
    
    This is a minimal test to see if score tracking works at all.
    """
    env = BalatroEnv(seed=12345, quiet=False)
    
    obs, info = env.reset()
    print(f"\nInitial state:")
    print(f"  Phase: {info.get('phase')}")
    print(f"  Score: {info.get('score')}")
    print(f"  Money: {info.get('money')}")
    print(f"  env.current_score: {env.current_score}")
    print(f"  env.current_money: {env.current_money}")
    
    # Play blind (action 1)
    if "BlindSelect" in str(info.get('phase')):
        obs, reward, terminated, truncated, info = env.step(1)
        print(f"\nAfter playing blind:")
        print(f"  Phase: {info.get('phase')}")
        print(f"  Reward: {reward}")
        print(f"  Score: {info.get('score')}")
        print(f"  Money: {info.get('money')}")
        print(f"  env.current_score: {env.current_score}")
        print(f"  env.current_money: {env.current_money}")
    
    env.close()

