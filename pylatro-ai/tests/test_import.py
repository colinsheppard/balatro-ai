"""Tests for module import and basic availability"""

import pytest
import sys
import os


def test_balatro_engine_module_can_be_imported():
    """Test that balatro_engine module can be imported"""
    try:
        import balatro_engine
    except ImportError as e:
        pytest.skip(f"balatro_engine module not available: {e}. "
                   f"Make sure to build the Rust library with 'cargo build --features python --release' "
                   f"and ensure the extension module is in PYTHONPATH")


def test_balatro_engine_module_has_expected_attributes():
    """Test that balatro_engine module has expected classes"""
    try:
        import balatro_engine
    except ImportError:
        pytest.skip("balatro_engine module not available")
    
    # Check that GameState is available
    assert hasattr(balatro_engine, 'GameState'), "GameState class should be available"
    
    # Check that Action types are available
    action_classes = [
        'MenuAction',
        'ShopAction',
        'BlindAction',
        'PlayingAction',
        'RoundEndAction',
        'GameOverAction',
    ]
    for action_class in action_classes:
        assert hasattr(balatro_engine, action_class), f"{action_class} class should be available"
    
    # Check that supporting types are available
    supporting_types = ['GamePhase', 'DeckType', 'StakeLevel']
    for supporting_type in supporting_types:
        assert hasattr(balatro_engine, supporting_type), f"{supporting_type} class should be available"


def test_balatro_engine_module_classes_are_callable():
    """Test that the exposed classes are actually callable (constructors)"""
    try:
        import balatro_engine
    except ImportError:
        pytest.skip("balatro_engine module not available")
    
    # GameState should be a class (callable)
    assert callable(balatro_engine.GameState), "GameState should be callable"
    
    # Action types should be classes (callable)
    action_classes = [
        'MenuAction',
        'ShopAction',
        'BlindAction',
        'PlayingAction',
        'RoundEndAction',
        'GameOverAction',
    ]
    for action_class_name in action_classes:
        action_class = getattr(balatro_engine, action_class_name)
        assert callable(action_class), f"{action_class_name} should be callable"
    
    # Supporting types should be classes (callable)
    supporting_types = ['GamePhase', 'DeckType', 'StakeLevel']
    for type_name in supporting_types:
        type_class = getattr(balatro_engine, type_name)
        assert callable(type_class), f"{type_name} should be callable"

