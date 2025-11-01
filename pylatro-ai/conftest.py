"""Pytest configuration and fixtures"""

import pytest
import sys
import os
from pathlib import Path


def pytest_configure(config):
    """Configure pytest with custom markers"""
    config.addinivalue_line(
        "markers", "requires_build: mark test as requiring the Rust extension to be built"
    )


@pytest.fixture(scope="session")
def ensure_extension_available():
    """Fixture to ensure the balatro_engine extension is available"""
    # Try to import the module
    try:
        import balatro_engine
        return True
    except ImportError:
        # Try to add the release directory to path
        project_root = Path(__file__).parent.parent
        engine_dir = project_root / "balatro-engine"
        release_dir = engine_dir / "target" / "release"
        
        if release_dir.exists():
            sys.path.insert(0, str(release_dir))
            try:
                import balatro_engine
                return True
            except ImportError:
                pass
        
        # Also check tests directory
        tests_dir = Path(__file__).parent / "tests"
        if tests_dir.exists():
            sys.path.insert(0, str(tests_dir))
            try:
                import balatro_engine
                return True
            except ImportError:
                pass
        
        pytest.skip(
            "balatro_engine extension not available. "
            "Build it with: cd ../balatro-engine && cargo build --features python --release"
        )

