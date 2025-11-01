"""Tests for Action types"""

import pytest


@pytest.fixture
def balatro_engine():
    """Fixture to import balatro_engine module"""
    try:
        import balatro_engine
        return balatro_engine
    except ImportError:
        pytest.skip("balatro_engine module not available")


def test_menu_action_class_exists(balatro_engine):
    """Test that MenuAction class exists"""
    assert hasattr(balatro_engine, 'MenuAction')
    assert balatro_engine.MenuAction is not None


def test_shop_action_class_exists(balatro_engine):
    """Test that ShopAction class exists"""
    assert hasattr(balatro_engine, 'ShopAction')
    assert balatro_engine.ShopAction is not None


def test_blind_action_class_exists(balatro_engine):
    """Test that BlindAction class exists"""
    assert hasattr(balatro_engine, 'BlindAction')
    assert balatro_engine.BlindAction is not None


def test_playing_action_class_exists(balatro_engine):
    """Test that PlayingAction class exists"""
    assert hasattr(balatro_engine, 'PlayingAction')
    assert balatro_engine.PlayingAction is not None


def test_round_end_action_class_exists(balatro_engine):
    """Test that RoundEndAction class exists"""
    assert hasattr(balatro_engine, 'RoundEndAction')
    assert balatro_engine.RoundEndAction is not None


def test_game_over_action_class_exists(balatro_engine):
    """Test that GameOverAction class exists"""
    assert hasattr(balatro_engine, 'GameOverAction')
    assert balatro_engine.GameOverAction is not None


def test_menu_action_creation(balatro_engine):
    """Test creating MenuAction instances"""
    # Test StartGame action
    action = balatro_engine.MenuAction("StartGame", None)
    assert action.index() == 3
    assert action.description() == "Start Game"
    assert action.is_valid() is True
    
    # Test Exit action
    action = balatro_engine.MenuAction("Exit", None)
    assert action.index() == 4
    assert action.description() == "Exit"
    
    # Test SelectDeck action
    action = balatro_engine.MenuAction("SelectDeck", "Red")
    assert action.index() == 1
    assert action.description() == "Select Deck Type"
    
    # Test SelectStake action
    action = balatro_engine.MenuAction("SelectStake", "White")
    assert action.index() == 2
    assert action.description() == "Select Stake Level"


def test_menu_action_string_representation(balatro_engine):
    """Test that MenuAction has string representation"""
    action = balatro_engine.MenuAction("StartGame", None)
    str_repr = str(action)
    assert isinstance(str_repr, str)
    assert len(str_repr) > 0


def test_shop_action_creation(balatro_engine):
    """Test creating ShopAction instances"""
    action = balatro_engine.ShopAction("NextRound")
    assert action.index() == 1
    assert action.description() == "Next Round"
    assert action.is_valid() is True


def test_blind_action_creation(balatro_engine):
    """Test creating BlindAction instances"""
    actions = [
        ("SelectBossBlind", 1, "Select Boss Blind"),
        ("SelectEliteBlind", 2, "Select Elite Blind"),
        ("SelectNormalBlind", 3, "Select Normal Blind"),
        ("ViewBlindDetails", 4, "View Blind Details"),
    ]
    
    for action_type, expected_index, expected_desc in actions:
        action = balatro_engine.BlindAction(action_type)
        assert action.index() == expected_index
        assert action.description() == expected_desc
        assert action.is_valid() is True


def test_playing_action_creation(balatro_engine):
    """Test creating PlayingAction instances"""
    # Test actions without index
    action = balatro_engine.PlayingAction("PlaySelectedCards", None)
    assert action.index() == 0
    assert action.description() == "Play selected cards"
    
    action = balatro_engine.PlayingAction("DiscardSelectedCards", None)
    assert action.index() == 1
    assert action.description() == "Discard selected cards"
    
    action = balatro_engine.PlayingAction("SortByRank", None)
    assert action.index() == 6
    assert action.description() == "Sort by rank"
    
    action = balatro_engine.PlayingAction("SortBySuit", None)
    assert action.index() == 7
    assert action.description() == "Sort by suit"
    
    # Test actions with index
    action = balatro_engine.PlayingAction("SelectCard", 0)
    assert action.index() == 2
    assert action.description() == "Select card"
    
    action = balatro_engine.PlayingAction("DeselectCard", 0)
    assert action.index() == 3
    assert action.description() == "Deselect card"
    
    action = balatro_engine.PlayingAction("MoveRight", 0)
    assert action.index() == 4
    assert action.description() == "Move right"
    
    action = balatro_engine.PlayingAction("MoveLeft", 0)
    assert action.index() == 5
    assert action.description() == "Move left"


def test_round_end_action_creation(balatro_engine):
    """Test creating RoundEndAction instances"""
    action = balatro_engine.RoundEndAction("CashOut")
    assert action.index() == 1
    assert action.description() == "Cash Out"
    assert action.is_valid() is True


def test_game_over_action_creation(balatro_engine):
    """Test creating GameOverAction instances"""
    actions = [
        ("PlayAgain", 1, "Play Again"),
        ("MainMenu", 2, "Main Menu"),
        ("ViewFinalStatistics", 3, "View Final Statistics"),
        ("Exit", 4, "Exit"),
    ]
    
    for action_type, expected_index, expected_desc in actions:
        action = balatro_engine.GameOverAction(action_type)
        assert action.index() == expected_index
        assert action.description() == expected_desc
        assert action.is_valid() is True


def test_action_types_validation(balatro_engine):
    """Test that Action types validate their inputs"""
    # Test invalid MenuAction
    with pytest.raises(Exception):
        balatro_engine.MenuAction("InvalidAction", None)
    
    # Test invalid ShopAction
    with pytest.raises(Exception):
        balatro_engine.ShopAction("InvalidAction")
    
    # Test invalid BlindAction
    with pytest.raises(Exception):
        balatro_engine.BlindAction("InvalidAction")
    
    # Test invalid PlayingAction
    with pytest.raises(Exception):
        balatro_engine.PlayingAction("InvalidAction", None)
    
    # Test invalid RoundEndAction
    with pytest.raises(Exception):
        balatro_engine.RoundEndAction("InvalidAction")
    
    # Test invalid GameOverAction
    with pytest.raises(Exception):
        balatro_engine.GameOverAction("InvalidAction")


def test_all_actions_have_string_representation(balatro_engine):
    """Test that all Action types have string representation"""
    action_classes = [
        lambda: balatro_engine.MenuAction("StartGame", None),
        lambda: balatro_engine.ShopAction("NextRound"),
        lambda: balatro_engine.BlindAction("SelectBossBlind"),
        lambda: balatro_engine.PlayingAction("PlaySelectedCards", None),
        lambda: balatro_engine.RoundEndAction("CashOut"),
        lambda: balatro_engine.GameOverAction("PlayAgain"),
    ]
    
    for action_factory in action_classes:
        action = action_factory()
        str_repr = str(action)
        assert isinstance(str_repr, str)
        assert len(str_repr) > 0

