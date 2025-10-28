//! Planet system for Balatro game engine
//!
//! Provides TOML-driven planet data and runtime types with poker hand detection

use serde::{Deserialize, Serialize};

use crate::card::{Card, Rank, Suit};
use crate::error::{GameError, GameResult};
use crate::scoring::HandScore;

/// Poker hand types that can be detected and upgraded by planets
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PokerHand {
    HighCard = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
    FiveOfAKind = 10,
    FlushHouse = 11,
    FlushFive = 12,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlanetFileConfig {
    #[serde(rename = "planet")]
    pub planets: Vec<PlanetDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlanetDefinition {
    pub name: String,
    pub poker_hand: String,
    pub poker_hand_name: String,
    pub addition: Vec<PlanetModifier>,
    pub hand_base_score: Vec<PlanetModifier>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlanetModifier {
    pub mult: i32,
    pub chips: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub name: String,
    pub poker_hand: PokerHand,
    pub poker_hand_name: String,
    pub base_mult: i32,
    pub base_chips: i32,
    pub add_mult: i32,
    pub add_chips: i32,
    pub level: i32,
}

impl Planet {
    pub fn upgrade(&mut self) {
        self.level += 1;
    }

    pub fn detect_poker_hand(
        &self,
        poker_hand: PokerHand,
        sorted_cards: &[Card],
        rank_counts: &std::collections::HashMap<Rank, usize>,
        suit_counts: &std::collections::HashMap<Suit, usize>,
    ) -> bool {
        // Determine if the provided cards match this planet's poker hand
        match poker_hand {
            PokerHand::HighCard => Self::is_high_card(sorted_cards, rank_counts),
            PokerHand::Pair => Self::is_pair(rank_counts),
            PokerHand::TwoPair => Self::is_two_pair(rank_counts),
            PokerHand::ThreeOfAKind => Self::is_three_of_a_kind(rank_counts),
            PokerHand::Straight => Self::is_straight(sorted_cards),
            PokerHand::Flush => Self::is_flush(suit_counts),
            PokerHand::FullHouse => Self::is_full_house(rank_counts),
            PokerHand::FourOfAKind => Self::is_four_of_a_kind(rank_counts),
            PokerHand::StraightFlush => Self::is_straight_flush(sorted_cards, rank_counts, suit_counts),
            PokerHand::FiveOfAKind => Self::is_five_of_a_kind(rank_counts),
            PokerHand::FlushHouse => Self::is_flush_house(sorted_cards, rank_counts, suit_counts),
            PokerHand::FlushFive => Self::is_flush_five(sorted_cards, rank_counts, suit_counts),
        }
    }


    fn is_high_card(_sorted_cards: &[Card], rank_counts: &std::collections::HashMap<Rank, usize>) -> bool {
        !rank_counts.values().any(|&n| n >= 2)
    }

    fn is_pair(rank_counts: &std::collections::HashMap<Rank, usize>) -> bool {
        rank_counts.values().any(|&n| n == 2)
    }

    fn is_two_pair(rank_counts: &std::collections::HashMap<Rank, usize>) -> bool {
        rank_counts.values().filter(|&&n| n == 2).count() >= 2
    }

    fn is_three_of_a_kind(rank_counts: &std::collections::HashMap<Rank, usize>) -> bool {
        rank_counts.values().any(|&n| n == 3)
    }

    fn is_straight(sorted_cards: &[Card]) -> bool {
        if sorted_cards.len() < 5 { return false; }
        // Treat Ace high straight only for now
        let mut ranks: Vec<i32> = sorted_cards.iter().map(|c| c.rank as i32).collect();
        ranks.sort_unstable();
        ranks.dedup();
        // Check any window of size 5
        for window in ranks.windows(5) {
            let mut ok = true;
            for i in 1..5 {
                if window[i] != window[i-1] + 1 { ok = false; break; }
            }
            if ok { return true; }
        }
        false
    }

    fn is_flush(suit_counts: &std::collections::HashMap<Suit, usize>) -> bool {
        suit_counts.values().any(|&n| n >= 5)
    }

    fn is_full_house(rank_counts: &std::collections::HashMap<Rank, usize>) -> bool {
        let has_three = rank_counts.values().any(|&n| n == 3);
        let has_pair = rank_counts.values().filter(|&&n| n >= 2).count() >= 2 || rank_counts.values().any(|&n| n == 2);
        has_three && has_pair
    }

    fn is_four_of_a_kind(rank_counts: &std::collections::HashMap<Rank, usize>) -> bool {
        rank_counts.values().any(|&n| n == 4)
    }

    fn is_straight_flush(
        sorted_cards: &[Card],
        _rank_counts: &std::collections::HashMap<Rank, usize>,
        suit_counts: &std::collections::HashMap<Suit, usize>,
    ) -> bool {
        // Check flush first
        if !suit_counts.values().any(|&n| n >= 5) {
            return false;
        }
        // Filter to the suit with >=5 cards and check straight within that suit
        if let Some((&suit, _)) = suit_counts.iter().find(|(_, n)| **n >= 5) {
            let suited: Vec<Card> = sorted_cards.iter().filter(|c| c.suit == suit).cloned().collect();
            return !suited.is_empty() && Self::is_straight(&suited);
        }
        false
    }

    fn is_five_of_a_kind(rank_counts: &std::collections::HashMap<Rank, usize>) -> bool {
        rank_counts.values().any(|&n| n >= 5)
    }

    fn is_flush_house(
        _sorted_cards: &[Card],
        _rank_counts: &std::collections::HashMap<Rank, usize>,
        _suit_counts: &std::collections::HashMap<Suit, usize>,
    ) -> bool { false }
    
    fn is_flush_five(
        _sorted_cards: &[Card],
        _rank_counts: &std::collections::HashMap<Rank, usize>,
        _suit_counts: &std::collections::HashMap<Suit, usize>,
    ) -> bool { false }

    fn get_base_score(&self) -> HandScore {
        let mut hand_score = HandScore::new();
        hand_score.chip_score = self.base_chips + self.add_chips * (self.level - 1);
        hand_score.mult_score = (self.base_mult + self.add_mult * (self.level - 1) )as f32;
        hand_score
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Planets {
    pub planets: Vec<Planet>,
}

impl Planets {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> GameResult<Self> {
        let content = std::fs::read_to_string(path).map_err(GameError::IoError)?;
        Self::from_str(&content)
    }

    pub fn from_str(content: &str) -> GameResult<Self> {
        let cfg: PlanetFileConfig = toml::from_str(content)
            .map_err(|e| GameError::InvalidGameState(format!("Planet TOML parsing error: {}", e)))?;

        let mut planets = Vec::new();
        for def in cfg.planets {
            let base = def.hand_base_score.get(0).cloned().unwrap_or(PlanetModifier{ mult: 0, chips: 0 });
            let add = def.addition.get(0).cloned().unwrap_or(PlanetModifier{ mult: 0, chips: 0 });
            planets.push(Planet {
                name: def.name,
                poker_hand: Self::string_to_poker_hand_enum(&def.poker_hand),
                poker_hand_name: def.poker_hand_name,
                base_mult: base.mult,
                base_chips: base.chips,
                add_mult: add.mult,
                add_chips: add.chips,
                level: 1,
            });
        }
        planets.sort_by(|a, b| b.poker_hand.cmp(&a.poker_hand));
        Ok(Self { planets })
    }

    pub fn new_default() -> GameResult<Self> {
        // Path relative to crate root
        let default_path = concat!(env!("CARGO_MANIFEST_DIR"), "/.config/planet_data.toml");
        Self::from_file(default_path)
    }

    /// Helper function to count ranks
    fn rank_counts(cards: &[Card]) -> std::collections::HashMap<Rank, usize> {
        let mut counts = std::collections::HashMap::new();
        for c in cards {
            *counts.entry(c.rank).or_insert(0) += 1;
        }
        counts
    }

    /// Helper function to count suits
    fn suit_counts(cards: &[Card]) -> std::collections::HashMap<Suit, usize> {
        let mut counts = std::collections::HashMap::new();
        for c in cards {
            *counts.entry(c.suit).or_insert(0) += 1;
        }
        counts
    }

    /// Detect poker hand from cards - returns the best matching poker hand type
    /// Hands are checked in reverse order (rarest to commonest) to find the best match
    pub fn detect_poker_hand(&self, cards: &[Card]) -> Option<PokerHand> {
        if cards.is_empty() {
            return None;
        }

        // Sort cards by rank
        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort_by_key(|c| c.rank as i32);

        // Compute counts once
        let rank_counts = Self::rank_counts(&sorted_cards);
        let suit_counts = Self::suit_counts(&sorted_cards);

        // Check hands in reverse order (rarest to commonest)
        let hand_order = vec![
            PokerHand::FlushFive,
            PokerHand::FlushHouse,
            PokerHand::FiveOfAKind,
            PokerHand::StraightFlush,
            PokerHand::FourOfAKind,
            PokerHand::FullHouse,
            PokerHand::Flush,
            PokerHand::Straight,
            PokerHand::ThreeOfAKind,
            PokerHand::TwoPair,
            PokerHand::Pair,
            PokerHand::HighCard,
        ];

        for hand in hand_order {
            // Find the planet for this hand type
            if let Some(planet) = self.planets.iter().find(|p| p.poker_hand == hand) {
                if planet.detect_poker_hand(hand, &sorted_cards, &rank_counts, &suit_counts) {
                    return Some(hand);
                }
            }
        }

        None
    }

    fn string_to_poker_hand_enum(poker_hand_str: &str) -> PokerHand {
        match poker_hand_str {
            "high_card" => PokerHand::HighCard,
            "pair" => PokerHand::Pair,
            "two_pair" => PokerHand::TwoPair,
            "three_of_a_kind" => PokerHand::ThreeOfAKind,
            "straight" => PokerHand::Straight,
            "flush" => PokerHand::Flush,
            "full_house" => PokerHand::FullHouse,
            "four_of_a_kind" => PokerHand::FourOfAKind,
            "straight_flush" => PokerHand::StraightFlush,
            "five_of_a_kind" => PokerHand::FiveOfAKind,
            "flush_house" => PokerHand::FlushHouse,
            "flush_five" => PokerHand::FlushFive,
            _ => PokerHand::HighCard, // fallback
        }
    }

}


