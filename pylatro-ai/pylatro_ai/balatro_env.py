"""
Gymnasium-compatible environment wrapper for Balatro engine.
"""

import os
import sys
import contextlib
import gymnasium as gym
from gymnasium import spaces
import numpy as np
import json
from typing import Optional, Tuple, Dict, Any

BLIND_COMPLETION_BASE_REWARD = 1000.0
STEP_REWARD = -0.01

class BalatroEnv(gym.Env):
    """
    Gymnasium environment for Balatro game.
    
    This environment wraps the Balatro engine and provides a Gymnasium-compatible
    interface for reinforcement learning.
    """
    
    metadata = {"render_modes": ["human"], "render_fps": 4}
    
    def __init__(
        self,
        seed: Optional[int] = None,
        deck_type: str = "Red",
        stake_level: str = "White",
        max_episode_steps: int = 1000,
        quiet: bool = True,
    ):
        """
        Initialize the Balatro environment.
        
        Args:
            seed: Random seed for the engine
            deck_type: Type of deck to use (e.g., "Red", "Blue", "Yellow")
            stake_level: Stake level (e.g., "White", "Red", "Green")
            max_episode_steps: Maximum number of steps per episode
            quiet: If True, suppress Rust engine print statements (default: True)
        """
        super().__init__()
        self.quiet = quiet
        
        # Import balatro_engine (will fail gracefully if not available)
        try:
            import balatro_engine
            self.balatro_engine = balatro_engine
        except ImportError:
            # Try to find and add the balatro-engine build directory to path
            import sys
            current_file = os.path.abspath(__file__)
            project_root = os.path.dirname(os.path.dirname(os.path.dirname(current_file)))
            engine_dir = os.path.join(project_root, "balatro-engine")
            build_dir = os.path.join(engine_dir, "target", "release")
            
            if build_dir not in sys.path:
                sys.path.insert(0, build_dir)
            
            # Try importing again
            try:
                import balatro_engine
                self.balatro_engine = balatro_engine
            except ImportError:
                raise ImportError(
                    f"balatro_engine module not available. "
                    f"Make sure the balatro-engine is built with: "
                    f"cd {engine_dir} && cargo build --features python --release\n"
                    f"Then ensure the module is in PYTHONPATH or at {build_dir}"
                )
        
        # Store configuration
        self.seed = seed if seed is not None else np.random.randint(0, 2**31)
        self.deck_type_str = deck_type
        self.stake_level_str = stake_level
        self.max_episode_steps = max_episode_steps
        
        # Initialize engine
        self.engine = None
        self.engine_dir = None
        
        # Track game state
        self.initial_score = 0
        self.initial_money = 0
        self.current_score = 0
        self.current_money = 0
        self.step_count = 0
        self.blind_started = False
        self.blind_finished = False
        self.prev_phase = None
        self.previous_score = 0  # Track previous score to calcluate marginal
        self.previous_money = 0  # Track previous score to calcluate marginal
        self.last_valid_actions = []  # Cache valid actions from last observation
        
        # Action space: variable size based on available actions
        # We'll use a discrete space with a large max value
        # The actual valid actions will be provided in the observation
        self.action_space = spaces.Discrete(100)  # Large enough for any action
        
        # Observation space: we'll use a dict space with:
        # - phase: current game phase (one-hot encoded)
        # - available_actions: binary mask of valid actions
        # - hand_info: information about current hand (simplified)
        # - game_info: score, money, etc.
        phases = ["Shop", "ShopPackSelection", "BlindSelect", "Playing", "RoundEnd", "GameOver"]
        self.observation_space = spaces.Dict({
            "phase": spaces.MultiDiscrete([len(phases)]),
            "available_actions": spaces.MultiBinary(100),  # Binary mask for valid actions
            "game_info": spaces.Box(
                low=-np.inf,
                high=np.inf,
                shape=(10,),  # score, money, ante, round, hands_remaining, discards_remaining, etc.
                dtype=np.float32,
            ),
        })
        
        self.phases = phases
        self.phase_to_idx = {phase: i for i, phase in enumerate(phases)}
    
    def _get_engine_dir(self):
        """Get the balatro-engine directory path."""
        if self.engine_dir is None:
            # Navigate to balatro-engine directory
            current_file = os.path.abspath(__file__)
            project_root = os.path.dirname(os.path.dirname(os.path.dirname(current_file)))
            self.engine_dir = os.path.join(project_root, "balatro-engine")
        return self.engine_dir
    
    def _reset_engine(self):
        """Reset the engine and initialize a new game."""
        # Change to engine directory (needed for CSV files)
        engine_dir = self._get_engine_dir()
        original_cwd = os.getcwd()
        
        try:
            os.chdir(engine_dir)
            
            # Create new engine
            self.engine = self.balatro_engine.BalatroEngine(seed=self.seed)
            
            # Set deck and stake
            deck = self.balatro_engine.DeckType(self.deck_type_str)
            stake = self.balatro_engine.StakeLevel(self.stake_level_str)
            self.engine.set_selected_deck(deck)
            self.engine.set_selected_stake(stake)
            
            # Start new run (with output suppression if quiet mode)
            with self._suppress_output():
                self.engine.start_new_run_with_selections()
            
            # Reset tracking variables
            self.initial_score = 0
            self.initial_money = 0
            self.current_score = 0
            self.current_money = 0
            self.step_count = 0
            self.previous_score = 0
            self.previous_money = 0
            self.prev_phase = None
            self.last_valid_actions = []
            
        finally:
            os.chdir(original_cwd)
    
    def _get_game_info(self) -> Dict[str, Any]:
        """Extract game information from the engine state."""
        # Try to get game state info
        # Since game_state() method isn't available, we'll track manually
        # and use step result information
        
        # For now, return basic info - we'll enhance this as we track state
        return {
            "score": self.current_score,
            "previous_score": self.previous_money,
            "money": self.current_money,
            "previous_money": self.previous_money,
            "ante": 1,  # Will be tracked from phase
            "round": 1,  # Will be tracked from phase
            "hands_remaining": 4,  # Default, will track
            "discards_remaining": 3,  # Default, will track
            "blind_started": 1.0 if self.blind_started else 0.0,
            "blind_finished": 1.0 if self.blind_finished else 0.0,
            "step_count": self.step_count,
            "phase_idx": 0,  # Will be set from observation
        }
    
    def _extract_observation(self, step_result) -> Dict[str, np.ndarray]:
        """Extract observation from step result."""
        # Get phase index
        phase_idx = 0
        if step_result.phase:
            phase_idx = self.phase_to_idx.get(step_result.phase, 0)
        
        # Create available actions mask
        available_actions = np.zeros(100, dtype=np.int8)
        valid_actions = [action[0] for action in step_result.actions]
        for action_idx in valid_actions:
            if action_idx < 100:
                available_actions[action_idx] = 1
        
        # Get game info
        game_info_dict = self._get_game_info()
        game_info_dict["phase_idx"] = phase_idx
        game_info = np.array([
            game_info_dict["score"],
            game_info_dict["money"],
            game_info_dict["ante"],
            game_info_dict["round"],
            game_info_dict["hands_remaining"],
            game_info_dict["discards_remaining"],
            game_info_dict["blind_started"],
            game_info_dict["blind_finished"],
            game_info_dict["step_count"],
            phase_idx,
        ], dtype=np.float32)
        
        return {
            "phase": np.array([phase_idx], dtype=np.int64),
            "available_actions": available_actions,
            "game_info": game_info,
        }
    
    def _calculate_reward(self, step_result) -> float:
        """
        Calculate reward based on game state.
        
        Reward = score achieved + money bonus ($1 = 250 scoring points)
        Only awarded when blind is finished (RoundEnd phase).
        """
        # Calculate reward based on score and money gained during this blind
        # Score reward is the score achieved during the blind
        score_reward = max(0, self.current_score - self.previous_score)
        
        # Money reward: $1 = 250 scoring points
        money_reward = max(0, self.current_money - self.previous_money) * 250.0

        # STEP_REWARD is a small negative reward for each step to encourage efficiency
        total_reward = score_reward + money_reward + STEP_REWARD

        # Give a extra reward when blind is finished
        if step_result.phase == "RoundEnd" and self.blind_started and self.blind_finished:
            total_reward += BLIND_COMPLETION_BASE_REWARD
        
        return total_reward

    
    def _is_done(self, step_result) -> bool:
        """Check if episode is done."""
        # Episode is done when:
        # 1. Blind is finished (RoundEnd phase after playing blind)
        # 2. Game is over
        # 3. Max steps reached
        
        if step_result.status == "Finished":
            return True
        
        if step_result.phase == "RoundEnd" and self.blind_started:
            return True
        
        if self.step_count >= self.max_episode_steps:
            return True
        
        return False
    
    def reset(
        self,
        seed: Optional[int] = None,
        options: Optional[Dict] = None,
    ) -> Tuple[Dict[str, np.ndarray], Dict[str, Any]]:
        """Reset the environment."""
        if seed is not None:
            self.seed = seed
        
        # Reset engine
        self._reset_engine()
        
        # Get initial observation with output suppression
        with self._suppress_output():
            step_result = self.engine.step(None)
        observation = self._extract_observation(step_result)
        
        # Cache valid actions
        self.last_valid_actions = [action_tuple[0] for action_tuple in step_result.actions]
        
        # Info dict - only include picklable data (no StepResult object)
        info = {
            "phase": step_result.phase,
            "status": step_result.status,
            "valid_actions": self.last_valid_actions.copy(),
        }
        
        # Initialize prev_phase
        self.prev_phase = step_result.phase
        
        return observation, info
    
    @contextlib.contextmanager
    def _suppress_output(self):
        """Context manager to suppress stdout/stderr from Rust engine.
        
        Rust's println! writes directly to file descriptors, so we need to
        redirect at the FD level, not just sys.stdout/sys.stderr.
        """
        if self.quiet:
            # Save original file descriptors
            original_stdout_fd = os.dup(1)  # stdout is FD 1
            original_stderr_fd = os.dup(2)  # stderr is FD 2
            
            try:
                # Open /dev/null for writing
                devnull = os.open(os.devnull, os.O_WRONLY)
                
                # Redirect stdout and stderr to /dev/null at the FD level
                os.dup2(devnull, 1)  # Redirect stdout
                os.dup2(devnull, 2)  # Redirect stderr
                
                # Also redirect Python's sys.stdout/stderr for good measure
                original_stdout = sys.stdout
                original_stderr = sys.stderr
                sys.stdout = open(os.devnull, 'w')
                sys.stderr = open(os.devnull, 'w')
                
                try:
                    yield
                finally:
                    # Restore Python's sys.stdout/stderr
                    sys.stdout.close()
                    sys.stderr.close()
                    sys.stdout = original_stdout
                    sys.stderr = original_stderr
                    
                    # Restore original file descriptors
                    os.dup2(original_stdout_fd, 1)
                    os.dup2(original_stderr_fd, 2)
                    
                    # Close the duplicate FDs
                    os.close(original_stdout_fd)
                    os.close(original_stderr_fd)
                    os.close(devnull)
            except Exception:
                # If anything goes wrong, try to restore
                try:
                    os.dup2(original_stdout_fd, 1)
                    os.dup2(original_stderr_fd, 2)
                    os.close(original_stdout_fd)
                    os.close(original_stderr_fd)
                except:
                    pass
                raise
        else:
            yield
    
    def _try_get_score_money(self):
        """Try to get score and money from engine state."""
        try:
            # Use game_state() to get an owned copy of the game state
            state = self.engine.game_state()
            return state.score(), state.money()
        except Exception as e:
            # If game_state() doesn't exist or fails, return None to indicate we need to track manually
            return None, None
    
    def step(
        self, action: int
    ) -> Tuple[Dict[str, np.ndarray], float, bool, bool, Dict[str, Any]]:
        """Execute one step in the environment."""
        self.step_count += 1
        
        # Filter invalid actions using cached valid actions from last observation
        # If the action is not in the valid actions list, map it to the first valid action
        if action not in self.last_valid_actions:
            # Invalid action selected - map to first valid action
            if self.last_valid_actions:
                mapped_action = self.last_valid_actions[0]
                # Give a small penalty for invalid action selection
                invalid_action_penalty = -10.0
            else:
                # No valid actions available - use 0 as fallback
                mapped_action = 0
                invalid_action_penalty = -0.5
        else:
            mapped_action = action
            invalid_action_penalty = 0.0
        
        # Execute the (possibly mapped) action with output suppression
        with self._suppress_output():
            step_result = self.engine.step(mapped_action)
        
        # Update tracking based on phase transitions
        if step_result.phase == "BlindSelect":
            # About to start a blind - reset blind tracking
            self.blind_started = False
            self.blind_finished = False
        elif step_result.phase == "Playing" and self.prev_phase != "Playing":
            # Just entered playing phase - blind started
            self.blind_started = True
            # Try to get initial score/money
            score, money = self._try_get_score_money()
            if score is not None and money is not None:
                self.blind_score_at_start = score
                self.blind_money_at_start = money
            else:
                # Fallback: use tracked values
                self.blind_score_at_start = self.current_score
                self.blind_money_at_start = self.current_money
        elif step_result.phase == "RoundEnd" and self.blind_started:
            # Blind finished - try to get final score/money
            self.blind_finished = True
            score, money = self._try_get_score_money()
            if score is not None and money is not None:
                # Calculate score/money gained during this blind
                self.current_score = score - self.blind_score_at_start
                self.current_money = money - self.blind_money_at_start
        elif step_result.phase == "Playing" and self.blind_started:
            # During playing phase, periodically update score/money
            # This ensures we have current values even if blind doesn't complete
            score, money = self._try_get_score_money()
            if score is not None and money is not None:
                # Update current score/money relative to blind start
                self.current_score = score - self.blind_score_at_start
                self.current_money = money - self.blind_money_at_start
        
        # Update prev_phase for next step
        self.prev_phase = step_result.phase
        
        # Extract observation
        observation = self._extract_observation(step_result)
        
        # Update cached valid actions for next step
        self.last_valid_actions = [action_tuple[0] for action_tuple in step_result.actions]
        
        # Calculate reward
        reward = self._calculate_reward(step_result)

        self.previous_score = self.current_score
        self.previous_money = self.current_money
        
        # Apply penalty for invalid action if one was selected
        reward += invalid_action_penalty
        
        # Check if done
        terminated = self._is_done(step_result)
        truncated = False  # Never truncate, only terminate
        
        # Info dict - only include picklable data (no StepResult object)
        info = {
            "phase": step_result.phase,
            "status": step_result.status,
            "score": self.current_score,
            "money": self.current_money,
            "action_mapped": action != mapped_action,  # Track if action was filtered
            "original_action": action,
            "mapped_action": mapped_action,
            "valid_actions": self.last_valid_actions.copy(),
            "finished": step_result.finished,
        }
        
        return observation, reward, terminated, truncated, info
    
    def render(self):
        """Render the environment (not implemented)."""
        pass
    
    def close(self):
        """Clean up resources."""
        self.engine = None

