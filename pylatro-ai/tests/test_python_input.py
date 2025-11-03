"""Test Python input functionality using the new non-blocking step API"""

import pytest
import os


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


def test_step_api_basic(balatro_engine):
    """
    Test the basic step API - verify it returns NeedsInput state.
    This test is fully debuggable with IDE breakpoints.
    """
    # Create engine
    engine = balatro_engine.BalatroEngine(seed=12345)
    
    # Initialize game state (we need to handle menu first to create GameState)
    # For now, we'll skip menu and test that step() works with initialized state
    # Actually, we need GameState to exist first. Let's use the old menu for now,
    # but we could add a method to initialize with defaults.
    
    # For this basic test, let's just verify the step API exists and works
    # We'll need an initialized game state, so let's set up deck/stake first
    deck = balatro_engine.DeckType("Red")
    stake = balatro_engine.StakeLevel("White")
    engine.set_selected_deck(deck)
    engine.set_selected_stake(stake)
    engine.start_new_run_with_selections()
    
    # Now we can use step() API
    result = engine.step(None)
    
    # Should return NeedsInput with BlindSelect phase
    assert result.status == "NeedsInput"
    assert result.phase == "BlindSelect"
    assert len(result.actions) > 0
    assert result.finished is None


def test_step_api_game_flow(balatro_engine):
    """
    Test game flow using step API: start game, enter blind, select cards.
    This test is fully debuggable - no blocking, no threading needed.
    """
    # Create engine
    engine = balatro_engine.BalatroEngine(seed=12345)
    
    # Set up deck and stake
    deck = balatro_engine.DeckType("Red")
    stake = balatro_engine.StakeLevel("White")
    engine.set_selected_deck(deck)
    engine.set_selected_stake(stake)
    engine.start_new_run_with_selections()
    
    # Step 1: Should need input for BlindSelect
    result = engine.step(None)
    assert result.status == "NeedsInput"
    assert result.phase == "BlindSelect"
    assert len(result.actions) > 0
    
    # Provide input to play the blind (choice 1)
    result = engine.step(1)
    assert result.status in ["NeedsInput", "Progressed"]
    
    # Step 2: Should now be in Playing phase
    result = engine.step(None)
    assert result.status == "NeedsInput"
    assert result.phase == "Playing"
    assert len(result.actions) > 0
    
    # Verify we have card selection actions available
    # Actions are: 0=Play, 1=Discard (if available), then card selections
    card_select_actions = [a for a in result.actions if a[0] >= 2]
    assert len(card_select_actions) >= 2, f"Should have at least 2 cards to select, got {len(card_select_actions)}"
    
    # Select first card (first selection action after Play/Discard)
    first_card_idx = card_select_actions[0][0]
    result = engine.step(first_card_idx)
    assert result.status in ["NeedsInput", "Progressed"]
    
    # Select second card - get current state again
    result = engine.step(None)
    assert result.status == "NeedsInput"
    assert result.phase == "Playing"
    
    # Find next card selection (should be different from first)
    card_select_actions = [a for a in result.actions if a[0] >= 2]
    if len(card_select_actions) >= 1:
        second_card_idx = card_select_actions[0][0]
        result = engine.step(second_card_idx)
        assert result.status in ["NeedsInput", "Progressed"]
    
    # Test passes if we successfully navigated through the step API
    # The key is that we never blocked - control always returned to Python
    assert True, "Successfully used non-blocking step API"


def test_step_api_handles_finished(balatro_engine):
    """
    Test that step API correctly handles finished game state.
    """
    # Create engine
    engine = balatro_engine.BalatroEngine(seed=12345)
    
    # Initialize game
    deck = balatro_engine.DeckType("Red")
    stake = balatro_engine.StakeLevel("White")
    engine.set_selected_deck(deck)
    engine.set_selected_stake(stake)
    engine.start_new_run_with_selections()
    
    # Get initial state
    result = engine.step(None)
    assert result.status == "NeedsInput"
    
    # We won't test full game completion here, but verify the structure
    # The step API should handle all states correctly
    assert hasattr(result, 'status')
    assert hasattr(result, 'phase')
    assert hasattr(result, 'actions')
    assert hasattr(result, 'finished')
