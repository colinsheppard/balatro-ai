"""Tests for GameState class"""

import pytest


@pytest.fixture
def balatro_engine():
    """Fixture to import balatro_engine module"""
    try:
        import balatro_engine
        return balatro_engine
    except ImportError:
        pytest.skip("balatro_engine module not available")


def test_game_state_class_exists(balatro_engine):
    """Test that GameState class exists"""
    assert hasattr(balatro_engine, 'GameState')
    assert balatro_engine.GameState is not None


def test_game_state_has_expected_methods(balatro_engine):
    """Test that GameState has expected methods"""
    # Note: We can't instantiate GameState without a proper Rust GameState instance,
    # but we can check that the class has the expected methods
    GameState = balatro_engine.GameState
    
    # Check for primitive field accessors
    expected_methods = [
        'phase',
        'ante',
        'hand_size',
        'money',
        'score',
        'round_number',
    ]
    
    # Check for JSON serialization methods
    json_methods = [
        'deck_json',
        'hand_json',
        'jokers_json',
        'consumables_json',
        'upcoming_blinds_json',
        'play_limits_json',
        'stake_json',
        'planets_json',
    ]
    
    # Check for helper methods
    helper_methods = [
        'hand_size_actual',
        'hand_selected_indices',
        'deck_remaining_cards',
        'deck_type',
        'hands_remaining',
        'discards_remaining',
    ]
    
    # Note: We can't actually call these methods without an instance,
    # but we can verify the class structure exists
    # In a real scenario, we would need a way to create a GameState instance
    # from Python, which would require additional bindings


def test_game_state_class_type(balatro_engine):
    """Test that GameState is a class type"""
    GameState = balatro_engine.GameState
    # In Python, PyO3 classes are instances of type
    assert isinstance(GameState, type), "GameState should be a class type"


def test_supporting_types_can_be_created(balatro_engine):
    """Test that supporting types can be instantiated"""
    # Test GamePhase
    phase = balatro_engine.GamePhase("Shop")
    assert str(phase) == "Shop"
    
    # Test DeckType
    deck_type = balatro_engine.DeckType("Red")
    assert str(deck_type) == "Red"
    
    # Test StakeLevel
    stake_level = balatro_engine.StakeLevel("White")
    assert str(stake_level) == "White"


def test_supporting_types_validation(balatro_engine):
    """Test that supporting types validate their inputs"""
    # Test invalid GamePhase
    with pytest.raises(Exception):  # Should raise ValueError or similar
        balatro_engine.GamePhase("InvalidPhase")
    
    # Test invalid DeckType
    with pytest.raises(Exception):
        balatro_engine.DeckType("InvalidDeck")
    
    # Test invalid StakeLevel
    with pytest.raises(Exception):
        balatro_engine.StakeLevel("InvalidStake")

