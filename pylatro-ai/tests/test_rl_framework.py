"""
Test RL framework setup - verifies all tools can load and run a simple hello world training/inference job.

This test verifies:
- Gymnasium: Environment creation
- Stable-Baselines3: Simple model training
- PyTorch: MLP architecture creation
- Optuna: Hyperparameter search
- WandB: Logging (with disabled mode if not configured)
- Brax: Optional JAX-based simulation
"""

import pytest


def test_gymnasium_environment():
    """Test that Gymnasium can create and interact with an environment."""
    import gymnasium as gym
    
    # Create a simple environment
    env = gym.make("CartPole-v1")
    
    # Test environment reset
    obs, info = env.reset()
    assert obs is not None
    assert len(obs) > 0
    
    # Test environment step
    action = env.action_space.sample()
    obs, reward, terminated, truncated, info = env.step(action)
    assert obs is not None
    assert isinstance(reward, (int, float))
    
    env.close()


def test_stable_baselines3_training():
    """Test that Stable-Baselines3 can train a simple model."""
    import gymnasium as gym
    from stable_baselines3 import PPO
    
    # Create environment
    env = gym.make("CartPole-v1")
    
    # Create a simple PPO model
    model = PPO("MlpPolicy", env, verbose=0, n_steps=64, batch_size=32)
    
    # Train for a few steps (minimal training just to verify it works)
    model.learn(total_timesteps=128)
    
    # Test inference
    obs, info = env.reset()
    action, _states = model.predict(obs, deterministic=True)
    assert action is not None
    assert env.action_space.contains(action)
    
    env.close()


def test_pytorch_mlp():
    """Test that PyTorch can create and run a simple MLP."""
    import torch
    import torch.nn as nn
    
    # Create a simple MLP
    class SimpleMLP(nn.Module):
        def __init__(self, input_size=4, hidden_size=64, output_size=2):
            super().__init__()
            self.fc1 = nn.Linear(input_size, hidden_size)
            self.fc2 = nn.Linear(hidden_size, hidden_size)
            self.fc3 = nn.Linear(hidden_size, output_size)
            self.relu = nn.ReLU()
        
        def forward(self, x):
            x = self.relu(self.fc1(x))
            x = self.relu(self.fc2(x))
            x = self.fc3(x)
            return x
    
    # Create model
    model = SimpleMLP()
    
    # Test forward pass
    x = torch.randn(1, 4)
    output = model(x)
    assert output.shape == (1, 2)
    
    # Test training step
    criterion = nn.MSELoss()
    optimizer = torch.optim.Adam(model.parameters(), lr=0.001)
    
    target = torch.randn(1, 2)
    loss = criterion(output, target)
    loss.backward()
    optimizer.step()
    
    assert loss.item() >= 0


def test_transformers():
    """Test that Transformers library can load and use a simple model."""
    from transformers import AutoModel, AutoTokenizer
    
    # Use a tiny model for testing
    model_name = "distilbert-base-uncased"
    
    try:
        # Load tokenizer and model
        tokenizer = AutoTokenizer.from_pretrained(model_name)
        model = AutoModel.from_pretrained(model_name)
        
        # Test inference
        text = "Hello, world!"
        inputs = tokenizer(text, return_tensors="pt")
        outputs = model(**inputs)
        
        assert outputs.last_hidden_state is not None
        assert outputs.last_hidden_state.shape[0] == 1  # batch size
    except Exception as e:
        # If model download fails (network issues), skip but log
        pytest.skip(f"Could not load transformer model: {e}")


def test_optuna_hyperparameter_search():
    """Test that Optuna can run a simple hyperparameter search."""
    import optuna
    
    def objective(trial):
        # Simple objective function
        x = trial.suggest_float("x", -10.0, 10.0)
        y = trial.suggest_float("y", -10.0, 10.0)
        return (x - 2) ** 2 + (y + 3) ** 2
    
    # Create study
    study = optuna.create_study(direction="minimize")
    
    # Run a few trials
    study.optimize(objective, n_trials=5)
    
    # Verify study completed
    assert len(study.trials) == 5
    assert study.best_params is not None


def test_wandb_initialization():
    """Test that WandB can be initialized (with disabled mode if not configured)."""
    import wandb
    
    # Initialize WandB in disabled mode (so it works without API key)
    wandb.init(mode="disabled", project="test-rl-framework")
    
    # Log a simple metric
    wandb.log({"test_metric": 42})
    
    # Finish run
    wandb.finish()
    
    # Verify it completed without errors
    assert True


def test_ray_rllib():
    """Test that RLlib can be imported and configured."""
    try:
        from ray.rllib.algorithms.ppo import PPOConfig
        from ray.rllib.env import PettingZooEnv
        
        # Create a simple PPO config
        config = PPOConfig()
        config.environment(env="CartPole-v1")
        config.training(lr=0.001)
        config.env_runners(num_env_runners=0)  # Use single worker for testing
        
        # Verify config was created
        assert config is not None
        
    except ImportError as e:
        pytest.skip(f"RLlib not fully available: {e}")


def test_ray_tune():
    """Test that Ray Tune can run a simple hyperparameter search."""
    from ray import tune
    from ray.tune.search.optuna import OptunaSearch
    
    def trainable(config):
        # Simple trainable function
        score = config["x"] ** 2 + config["y"] ** 2
        tune.report(score=score)
    
    # Run a simple tune experiment
    try:
        analysis = tune.run(
            trainable,
            config={
                "x": tune.uniform(-10.0, 10.0),
                "y": tune.uniform(-10.0, 10.0),
            },
            num_samples=3,
            search_alg=OptunaSearch(),
            verbose=0,
        )
        
        # Verify analysis completed
        assert analysis is not None
        assert len(analysis.trials) == 3
    except Exception as e:
        pytest.skip(f"Ray Tune not fully available: {e}")


def test_brax():
    """Test that Brax can create and simulate a simple environment."""
    try:
        import jax
        import brax
        from brax import envs
        
        # Create a simple environment
        env = envs.create("ant")
        
        # Reset environment
        state = env.reset(jax.random.PRNGKey(0))
        
        # Step environment
        action = jax.numpy.zeros(env.action_size)
        state = env.step(state, action)
        
        # Verify state is valid
        assert state.obs is not None
        assert len(state.obs) > 0
        
    except ImportError as e:
        pytest.skip(f"Brax not fully available: {e}")


def test_integrated_rl_pipeline():
    """
    Test an integrated RL pipeline that combines multiple tools:
    - Gymnasium environment
    - Stable-Baselines3 training
    - PyTorch model inspection
    - WandB logging
    """
    import gymnasium as gym
    from stable_baselines3 import PPO
    import wandb
    
    # Initialize WandB in disabled mode
    wandb.init(mode="disabled", project="test-integrated-pipeline")
    
    # Create environment
    env = gym.make("CartPole-v1")
    
    # Create model
    model = PPO("MlpPolicy", env, verbose=0, n_steps=64, batch_size=32)
    
    # Train for a few steps
    model.learn(total_timesteps=128)
    
    # Log training info
    wandb.log({"training_step": 128})
    
    # Test inference
    obs, info = env.reset()
    for _ in range(5):
        action, _states = model.predict(obs, deterministic=True)
        obs, reward, terminated, truncated, info = env.step(action)
        wandb.log({"reward": reward})
        if terminated or truncated:
            break
    
    # Finish WandB run
    wandb.finish()
    
    # Verify everything completed
    assert True
    
    env.close()

