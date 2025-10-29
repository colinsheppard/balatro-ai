//! Planet system for Balatro game engine
//!
//! Provides TOML-driven planet data and runtime types with poker hand detection

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use crate::{SharedCard, Utilities};
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

    pub fn get_base_score(&self) -> HandScore {
        let mut hand_score = HandScore::new();
        hand_score.chip_score = self.base_chips + self.add_chips * (self.level - 1);
        hand_score.mult_score = (self.base_mult + self.add_mult * (self.level - 1)) as f32;
        hand_score
    }
}

struct CountAndCards{
    pub count: usize,
    pub cards: Vec<SharedCard>,
}
impl CountAndCards {
    pub fn new(card: SharedCard) -> Self {
        let mut _cards = Vec::new();
        _cards.push(card);
        Self { count: 0, cards: _cards }
    }
    pub fn add_card(&mut self, card: SharedCard) {
        self.cards.push(card);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planets {
    pub planets: Vec<Planet>,
    pub planets_by_enum: HashMap<PokerHand, usize>,
}

impl Default for Planets {
    fn default() -> Self {
        Self {
            planets: Vec::new(),
            planets_by_enum: HashMap::new(),
        }
    }
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
        
        // Build HashMap for efficient lookup - store indices into the vec
        let mut planets_by_enum = HashMap::new();
        for (index, planet) in planets.iter().enumerate() {
            planets_by_enum.insert(planet.poker_hand, index);
        }
        
        Ok(Self { planets, planets_by_enum })
    }

    pub fn new_default() -> GameResult<Self> {
        // Path relative to crate root
        let default_path = concat!(env!("CARGO_MANIFEST_DIR"), "/.config/planet_data.toml");
        Self::from_file(default_path)
    }

    /// Get a reference to a planet by its poker hand type
    pub fn get_planet(&self, poker_hand: PokerHand) -> Option<&Planet> {
        self.planets_by_enum
            .get(&poker_hand)
            .map(|&index| &self.planets[index])
    }

    /// Get a mutable reference to a planet by its poker hand type
    pub fn get_planet_mut(&mut self, poker_hand: PokerHand) -> Option<&mut Planet> {
        self.planets_by_enum
            .get(&poker_hand)
            .map(|&index| &mut self.planets[index])
    }

    /// Helper function to count ranks
    fn rank_counts(cards: &[SharedCard]) -> std::collections::HashMap<Rank, CountAndCards> {
        let mut counts = std::collections::HashMap::new();
        for c in cards {
            let entry = counts.entry(c.borrow().rank).or_insert(CountAndCards::new(c.clone()));
            entry.add_card(c.clone());
            entry.count += 1;
        }
        counts
    }

    /// Helper function to count suits
    fn suit_counts(cards: &[SharedCard]) -> std::collections::HashMap<Suit, CountAndCards> {
        let mut counts = std::collections::HashMap::new();
        for c in cards {
            let entry = counts.entry(c.borrow().suit).or_insert(CountAndCards::new(c.clone()));
            entry.add_card(c.clone());
            entry.count += 1;
        }
        counts
    }

    /// Detect poker hand from cards - returns the best matching poker hand type
    /// Hands are checked in reverse order (rarest to commonest) to find the best match
    pub fn detect_poker_hand(&self, cards: &[SharedCard]) -> Option<(PokerHand,Vec<SharedCard>)> {
        if cards.is_empty() {
            return None;
        }

        // Sort cards by rank
        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort_by_key(|c| c.borrow().rank as i32);

        // Compute counts once
        let rank_counts = Self::rank_counts(&sorted_cards);
        let suit_counts = Self::suit_counts(&sorted_cards);

        // Check composite hands first by evaluating their components
        let is_five_of_a_kind = Self::is_n_of_a_kind(&rank_counts, 5);
        let is_flush = Self::is_flush_of_n(&suit_counts, 5);
        if is_five_of_a_kind && is_flush {
            return Some((PokerHand::FlushFive, sorted_cards));
        }
        let is_full_house = Self::is_full_house(&rank_counts);
        if is_full_house && is_flush {
            return Some((PokerHand::FlushHouse, sorted_cards));
        }
        if is_five_of_a_kind {
            return Some((PokerHand::FiveOfAKind, sorted_cards));
        }
        let straight_cards = Self::is_straight_of_n(&sorted_cards, 5);
        let is_straight = straight_cards.is_some();
        if is_straight && is_flush {
            return Some((PokerHand::StraightFlush, sorted_cards));
        } else if Self::is_n_of_a_kind(&rank_counts, 4) {
            let scoring_cards = Self::find_n_of_a_kind(&rank_counts, 4);
            return Some((PokerHand::FourOfAKind, scoring_cards));
        } 
        let is_three_of_a_kind = Self::is_n_of_a_kind(&rank_counts, 3);
        let is_pair = Self::is_n_of_a_kind(&rank_counts, 2);
        if is_three_of_a_kind && is_pair {
            return Some((PokerHand::FullHouse, sorted_cards));
        } else if is_flush {
            // TODO: leave this branch here for when we implement Four Fingers effect
            if true {
                return Some((PokerHand::Flush, sorted_cards));
            }else{
                let scoring_cards = Self::find_flush_of_n(&suit_counts, 4);
                return Some((PokerHand::Flush, scoring_cards));
            }
        } else if is_straight {
            return Some((PokerHand::Straight, straight_cards.unwrap()));
        } else if is_three_of_a_kind {
            let scoring_cards = Self::find_n_of_a_kind(&rank_counts, 3);
            return Some((PokerHand::ThreeOfAKind, scoring_cards));
        } else if Self::is_two_pair(&rank_counts) {
            let scoring_cards = Self::find_n_of_a_kind(&rank_counts, 2);
            return Some((PokerHand::TwoPair, scoring_cards));
        } else if is_pair {
            let scoring_cards = Self::find_n_of_a_kind(&rank_counts, 2);
            return Some((PokerHand::Pair, scoring_cards));
        } else {
            let scoring_cards = Self::find_high_card(&sorted_cards);
            return Some((PokerHand::HighCard, scoring_cards));
        }
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

    fn is_n_of_a_kind(rank_counts: &std::collections::HashMap<Rank, CountAndCards>, n: usize) -> bool {
        rank_counts.values().any(|c| c.count == n)
    }
    fn find_n_of_a_kind(rank_counts: &std::collections::HashMap<Rank, CountAndCards>, n: usize) -> Vec<SharedCard> {
        rank_counts.values().filter(|c| c.count == n).map(|c| c.cards.clone()).flatten().collect()
    }
    fn is_two_pair(rank_counts: &std::collections::HashMap<Rank, CountAndCards>) -> bool {
        rank_counts.values().filter(|c| c.count == 2).count() >= 2
    }
    fn find_flush_of_n(suit_counts: &std::collections::HashMap<Suit, CountAndCards>, n: usize) -> Vec<SharedCard> {
        suit_counts.values().filter(|c| c.count == n).map(|c| c.cards.clone()).flatten().collect()
    }
    fn find_high_card(sorted_cards: &[SharedCard]) -> Vec<SharedCard> {
        // Highest rank card should be used for High Card scoring
        vec![sorted_cards.last().cloned().unwrap()]
    }

    //TODO: This is not correct, does not treat Ace as low and high
    fn is_straight_of_n(sorted_cards: &[SharedCard], n: usize) -> Option<Vec<SharedCard>> {
        if sorted_cards.len() < n { return None; }
        // Treat Ace high straight only for now
        let mut ranks: Vec<i32> = sorted_cards.iter().map(|c| c.borrow().rank as i32).collect();
        ranks.sort_unstable();
        ranks.dedup();
        // Check any window of size 5
        for window in ranks.windows(5) {
            let mut ok = true;
            for i in 1..n {
                if window[i] != window[i-1] + 1 { ok = false; break; }
            }
            if ok { 
                return Some(sorted_cards.iter().filter(|c| window.contains(&(c.borrow().rank as i32))).cloned().collect()); 
            }
        }
        None
    }

    fn is_flush_of_n(suit_counts: &std::collections::HashMap<Suit, CountAndCards>, n: usize) -> bool {
        suit_counts.values().any(|c| c.count >= n)
    }

    fn is_full_house(rank_counts: &std::collections::HashMap<Rank, CountAndCards>) -> bool {
        let has_three = Self::is_n_of_a_kind(rank_counts, 3);
        let has_pair = Self::is_n_of_a_kind(rank_counts, 2);
        has_three && has_pair
    }
}

