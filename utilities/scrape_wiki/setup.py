#!/usr/bin/env python3
"""
Setup script for Balatro AI utilities.

This script sets up the environment and dependencies needed for the utilities.
"""

import subprocess
import sys
from pathlib import Path


def run_command(cmd, cwd=None):
    """Run a command and return success status."""
    try:
        result = subprocess.run(cmd, shell=True, cwd=cwd, check=True, 
                              capture_output=True, text=True)
        print(f"✓ {cmd}")
        return True
    except subprocess.CalledProcessError as e:
        print(f"✗ {cmd}")
        print(f"  Error: {e.stderr}")
        return False


def main():
    """Main setup function."""
    print("=== Balatro AI Utilities Setup ===")
    
    # Check Python version
    if sys.version_info < (3, 7):
        print("✗ Python 3.7+ required")
        sys.exit(1)
    print(f"✓ Python {sys.version}")
    
    # Install Python dependencies
    print("\nInstalling Python dependencies...")
    if not run_command("pip install -r requirements.txt"):
        print("Failed to install Python dependencies")
        sys.exit(1)
    
    # Check if Node.js is available
    print("\nChecking Node.js...")
    if not run_command("node --version"):
        print("Node.js not found. Please install Node.js and npm.")
        print("Visit: https://nodejs.org/")
        sys.exit(1)
    
    if not run_command("npm --version"):
        print("npm not found. Please install npm.")
        sys.exit(1)
    
    print("\n✓ Setup complete!")
    print("\nYou can now run:")
    print("  python scrape_balatro_wiki.py --dry-run  # See what would be done")
    print("  python scrape_balatro_wiki.py            # Run the scraper")


if __name__ == "__main__":
    main()
