#!/usr/bin/env python3
"""
Balatro Wiki Scraper

This script scrapes the Balatro wiki from Fandom.com to gather all game rules,
mechanics, and content for use in building a Balatro game emulator.

It uses the Node.js fandom-scraper project as a backend and provides a Python
interface for configuration and execution.
"""

import os
import sys
import json
import subprocess
import shutil
from pathlib import Path
from typing import Dict, List, Optional
import argparse


class BalatroWikiScraper:
    """Main class for scraping the Balatro wiki using the fandom-scraper Node.js tool."""
    
    def __init__(self, base_dir: str = None):
        """
        Initialize the scraper.
        
        Args:
            base_dir: Base directory for the utilities project. If None, uses current directory.
        """
        self.base_dir = Path(base_dir) if base_dir else Path(__file__).parent
        self.fandom_scraper_dir = self.base_dir / "fandom-scraper"
        self.output_dir = self.base_dir / "scraped_data"
        
        # Balatro-specific configuration
        self.balatro_config = {
            "from": "https://balatrogame.fandom.com",
            "entry_point_from_all_pages": "https://balatrogame.fandom.com/wiki/Special:AllPages",
            "sub_dir": "balatrogame",
            "filename_data": "balatrogame-data.json",
            "filename_history": "balatrogame-history.json"
        }
    
    def check_dependencies(self) -> bool:
        """
        Check if Node.js and npm are available.
        
        Returns:
            True if dependencies are available, False otherwise.
        """
        try:
            # Check Node.js
            result = subprocess.run(["node", "--version"], 
                                  capture_output=True, text=True, check=True)
            print(f"✓ Node.js version: {result.stdout.strip()}")
            
            # Check npm
            result = subprocess.run(["npm", "--version"], 
                                  capture_output=True, text=True, check=True)
            print(f"✓ npm version: {result.stdout.strip()}")
            
            return True
        except (subprocess.CalledProcessError, FileNotFoundError) as e:
            print(f"✗ Missing dependency: {e}")
            print("Please install Node.js and npm to use this scraper.")
            return False
    
    def setup_fandom_scraper(self) -> bool:
        """
        Set up the fandom-scraper Node.js project.
        
        Returns:
            True if setup successful, False otherwise.
        """
        if not self.fandom_scraper_dir.exists():
            print(f"✗ Fandom scraper directory not found: {self.fandom_scraper_dir}")
            return False
        
        try:
            # Install dependencies
            print("Installing Node.js dependencies...")
            subprocess.run(["npm", "install"], 
                         cwd=self.fandom_scraper_dir, 
                         check=True)
            print("✓ Dependencies installed successfully")
            return True
        except subprocess.CalledProcessError as e:
            print(f"✗ Failed to install dependencies: {e}")
            return False
    
    def configure_scraper(self) -> bool:
        """
        Configure the fandom-scraper for Balatro wiki.
        
        Returns:
            True if configuration successful, False otherwise.
        """
        main_js_path = self.fandom_scraper_dir / "main.mjs"
        
        if not main_js_path.exists():
            print(f"✗ Main script not found: {main_js_path}")
            return False
        
        try:
            # Read the current main.mjs file
            with open(main_js_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Replace the configuration with Balatro-specific settings
            replacements = [
                ('const from = "https://solo-leveling.fandom.com";', 
                 f'const from = "{self.balatro_config["from"]}";'),
                ('const entry_point_from_all_pages =\n    "https://solo-leveling.fandom.com/fr/wiki/Sp\\xC3\\xA9cial:Toutes_les_pages";',
                 f'const entry_point_from_all_pages =\n    "{self.balatro_config["entry_point_from_all_pages"]}";'),
            ]
            
            for old, new in replacements:
                content = content.replace(old, new)
            
            # Write the modified content back
            with open(main_js_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            print("✓ Scraper configured for Balatro wiki")
            return True
            
        except Exception as e:
            print(f"✗ Failed to configure scraper: {e}")
            return False
    
    def run_scraper(self) -> bool:
        """
        Run the fandom scraper to collect Balatro wiki data.
        
        Returns:
            True if scraping successful, False otherwise.
        """
        try:
            print("Starting Balatro wiki scraping...")
            print("This may take several minutes depending on the wiki size...")
            
            # Run the scraper
            result = subprocess.run(["npm", "start"], 
                                  cwd=self.fandom_scraper_dir, 
                                  check=True, 
                                  capture_output=True, 
                                  text=True)
            
            print("✓ Scraping completed successfully")
            print("Scraper output:")
            print(result.stdout)
            
            return True
            
        except subprocess.CalledProcessError as e:
            print(f"✗ Scraping failed: {e}")
            if e.stdout:
                print("STDOUT:", e.stdout)
            if e.stderr:
                print("STDERR:", e.stderr)
            return False
    
    def copy_results(self) -> bool:
        """
        Copy the scraped results to our output directory.
        
        Returns:
            True if copy successful, False otherwise.
        """
        try:
            # Create output directory
            self.output_dir.mkdir(exist_ok=True)
            
            # Source directory in fandom-scraper
            source_dir = self.fandom_scraper_dir / "out" / self.balatro_config["sub_dir"]
            
            if not source_dir.exists():
                print(f"✗ Source directory not found: {source_dir}")
                return False
            
            # Copy files
            for filename in [self.balatro_config["filename_data"], 
                           self.balatro_config["filename_history"]]:
                source_file = source_dir / filename
                if source_file.exists():
                    dest_file = self.output_dir / filename
                    shutil.copy2(source_file, dest_file)
                    print(f"✓ Copied {filename}")
                else:
                    print(f"⚠ File not found: {filename}")
            
            return True
            
        except Exception as e:
            print(f"✗ Failed to copy results: {e}")
            return False
    
    def analyze_results(self) -> Dict:
        """
        Analyze the scraped results and provide summary.
        
        Returns:
            Dictionary with analysis results.
        """
        data_file = self.output_dir / self.balatro_config["filename_data"]
        history_file = self.output_dir / self.balatro_config["filename_history"]
        
        analysis = {
            "data_file_exists": data_file.exists(),
            "history_file_exists": history_file.exists(),
            "total_pages": 0,
            "total_characters": 0,
            "sample_pages": []
        }
        
        if data_file.exists():
            try:
                with open(data_file, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                
                analysis["total_pages"] = len(data)
                analysis["total_characters"] = sum(len(page.get("content", "")) for page in data)
                
                # Get sample pages
                analysis["sample_pages"] = [
                    {
                        "title": page.get("title", "Unknown"),
                        "url": page.get("url", "Unknown"),
                        "content_length": len(page.get("content", ""))
                    }
                    for page in data[:5]  # First 5 pages
                ]
                
            except Exception as e:
                print(f"⚠ Failed to analyze data file: {e}")
        
        if history_file.exists():
            try:
                with open(history_file, 'r', encoding='utf-8') as f:
                    history = json.load(f)
                analysis["total_links"] = len(history)
            except Exception as e:
                print(f"⚠ Failed to analyze history file: {e}")
        
        return analysis
    
    def run_full_scrape(self) -> bool:
        """
        Run the complete scraping process.
        
        Returns:
            True if all steps successful, False otherwise.
        """
        print("=== Balatro Wiki Scraper ===")
        print(f"Base directory: {self.base_dir}")
        print(f"Output directory: {self.output_dir}")
        print()
        
        # Step 1: Check dependencies
        print("Step 1: Checking dependencies...")
        if not self.check_dependencies():
            return False
        print()
        
        # Step 2: Setup fandom scraper
        print("Step 2: Setting up fandom scraper...")
        if not self.setup_fandom_scraper():
            return False
        print()
        
        # Step 3: Configure scraper
        print("Step 3: Configuring scraper for Balatro...")
        if not self.configure_scraper():
            return False
        print()
        
        # Step 4: Run scraper
        print("Step 4: Running scraper...")
        if not self.run_scraper():
            return False
        print()
        
        # Step 5: Copy results
        print("Step 5: Copying results...")
        if not self.copy_results():
            return False
        print()
        
        # Step 6: Analyze results
        print("Step 6: Analyzing results...")
        analysis = self.analyze_results()
        
        print("=== Scraping Complete ===")
        print(f"Total pages scraped: {analysis['total_pages']}")
        print(f"Total characters: {analysis['total_characters']:,}")
        print(f"Data file: {self.output_dir / self.balatro_config['filename_data']}")
        print(f"History file: {self.output_dir / self.balatro_config['filename_history']}")
        
        if analysis["sample_pages"]:
            print("\nSample pages:")
            for page in analysis["sample_pages"]:
                print(f"  - {page['title']} ({page['content_length']} chars)")
        
        return True


def main():
    """Main entry point for the script."""
    parser = argparse.ArgumentParser(description="Scrape Balatro wiki from Fandom.com")
    parser.add_argument("--base-dir", type=str, help="Base directory for the utilities project")
    parser.add_argument("--dry-run", action="store_true", help="Show what would be done without executing")
    
    args = parser.parse_args()
    
    scraper = BalatroWikiScraper(args.base_dir)
    
    if args.dry_run:
        print("=== Dry Run Mode ===")
        print(f"Would scrape from: {scraper.balatro_config['from']}")
        print(f"Would use entry point: {scraper.balatro_config['entry_point_from_all_pages']}")
        print(f"Would output to: {scraper.output_dir}")
        print("Dependencies check:")
        scraper.check_dependencies()
        return
    
    success = scraper.run_full_scrape()
    
    if success:
        print("\n🎉 Balatro wiki scraping completed successfully!")
        print("The scraped data is now available for use in your Rust emulator project.")
    else:
        print("\n❌ Scraping failed. Please check the error messages above.")
        sys.exit(1)


if __name__ == "__main__":
    main()
