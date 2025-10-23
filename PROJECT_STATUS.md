# Balatro AI Project - Wiki Scraping Complete

## Project Overview

This project is building a Balatro game emulator in Rust from scratch. The first phase involved creating a Python utilities sub-project to scrape the complete Balatro wiki from Fandom.com to gather all game rules, mechanics, and content for use by coding agents as context.

## What Was Accomplished

✅ **Complete Balatro Wiki Scraping**
- Successfully scraped **804 pages** from the Balatro wiki
- Collected **1,025,068 characters** of game data
- All data stored locally in JSON format for easy access

✅ **Python Utilities Sub-Project Created**
- `utilities/scrape_balatro_wiki.py` - Main scraping script
- `utilities/setup.py` - Environment setup script
- `utilities/requirements.txt` - Python dependencies
- `utilities/README.md` - Documentation

✅ **Data Files Generated**
- `utilities/scraped_data/balatrogame-data.json` - All wiki content (1.1MB)
- `utilities/scraped_data/balatrogame-history.json` - List of all URLs (41KB)

## Data Content

The scraped data includes comprehensive information about:

- **Jokers** - All 150+ jokers with effects, synergies, and strategies
- **Decks** - All deck types and their special properties
- **Poker Hands** - Complete hand rankings and scoring
- **Card Modifiers** - Enhancements, editions, seals, and stickers
- **Blinds & Antes** - Boss blinds, scoring requirements, and progression
- **Consumables** - Tarot cards, planet cards, spectral cards, vouchers
- **Game Mechanics** - Rules, activation sequences, and interactions
- **Updates** - All game updates and patch notes

## Usage

The scraped data is now ready to be used as context for coding agents when building the Rust emulator. The JSON format makes it easy to:

1. Parse specific game mechanics
2. Look up joker effects and synergies
3. Understand scoring systems
4. Reference card interactions
5. Implement game rules accurately

## Next Steps

With both the wiki data successfully scraped and the Rust game engine scaffolded, the project is ready to proceed with:

1. ✅ **Wiki Data Collection** - Complete Balatro wiki scraped (804 pages, 1M+ characters)
2. ✅ **Rust Project Setup** - Game engine scaffolded with proper structure
3. ✅ **Core Game Engine** - Basic game state management implemented
4. ✅ **Card System** - Card, deck, and hand management framework
5. ✅ **Joker System** - Joker effects and interactions framework
6. ✅ **Game Logic** - Basic scoring, blinds, and progression systems

### Ready for Implementation

The project now has:
- **Complete game data** from the Balatro wiki
- **Solid Rust foundation** with proper error handling and serialization
- **Modular architecture** ready for expansion
- **Comprehensive testing** framework
- **CLI interface** for development and testing

### Next Development Phase

1. **Poker Hand Detection** - Implement full poker hand recognition and scoring
2. **Specific Joker Effects** - Add actual joker implementations from wiki data
3. **Shop System** - Implement shop mechanics and consumables
4. **Blind Selection** - Add boss blind mechanics and effects
5. **Game Progression** - Complete ante progression and unlocks
6. **Save/Load System** - Implement game state persistence
7. **Performance Optimization** - Profile and optimize critical paths

The comprehensive wiki data will serve as the authoritative source for implementing all game mechanics accurately in the Rust emulator.

## Files Created

```
balatro-ai/
├── utilities/                    # Python utilities sub-project
│   ├── scrape_balatro_wiki.py    # Main scraping script
│   ├── setup.py                  # Environment setup
│   ├── requirements.txt          # Python dependencies
│   ├── README.md                 # Documentation
│   ├── scraped_data/
│   │   ├── balatrogame-data.json     # All wiki content (1.1MB)
│   │   └── balatrogame-history.json  # URL list (41KB)
│   └── fandom-scraper/          # Node.js scraper tool
└── balatro-engine/                  # Rust game engine sub-project
    ├── Cargo.toml               # Rust dependencies and config
    ├── README.md                # Engine documentation
    ├── src/
    │   ├── lib.rs               # Main library entry point
    │   ├── main.rs              # CLI executable
    │   ├── card/                # Card system
    │   ├── deck/                # Deck management
    │   ├── joker/               # Joker system
    │   ├── game/                # Core game logic
    │   ├── blind/               # Blind system
    │   ├── consumable/          # Consumable items
    │   └── error/               # Error handling
    └── target/                  # Build artifacts
```

The foundation is now in place to build a comprehensive Balatro game emulator in Rust! 🎮
