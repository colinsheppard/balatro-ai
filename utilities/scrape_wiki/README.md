# Balatro AI Utilities

This directory contains utility scripts for the Balatro AI project, including tools for scraping game data and preparing it for use in the Rust emulator.

## Scripts

### `scrape_balatro_wiki.py`

Scrapes the complete Balatro wiki from Fandom.com to gather all game rules, mechanics, and content.

**Usage:**
```bash
# Install Python dependencies
pip install -r requirements.txt

# Run the scraper
python scrape_balatro_wiki.py

# Dry run to see what would be done
python scrape_balatro_wiki.py --dry-run
```

**Requirements:**
- Node.js and npm (for the underlying fandom-scraper)
- Python 3.7+

**Output:**
- `scraped_data/balatrogame-data.json` - All wiki page content
- `scraped_data/balatrogame-history.json` - List of all scraped URLs

## Dependencies

The scraper uses the [fandom-scraper](https://github.com/lalBi94/scrapper-fandom) Node.js project as its backend, which is automatically cloned and configured for the Balatro wiki.

## Data Format

The scraped data is stored in JSON format with the following structure:

```json
[
  {
    "url": "https://balatrogame.fandom.com/wiki/Some_Page",
    "title": "Page Title",
    "content": "Cleaned text content from the page..."
  }
]
```

This data can then be used by coding agents as context when building the Balatro game emulator in Rust.
