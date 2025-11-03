# Changelog

## Build Fix - November 2024

### Problem
The initial build failed with linking errors when trying to compile the Python extension module. The linker couldn't find Python symbols because `pyo3-build-config` wasn't properly configured.

### Solution
1. **Added `pyo3-build-config` dependency**: Added to both `[dependencies]` and `[build-dependencies]` sections in `Cargo.toml`
2. **Created `build.rs` script**: Added a build script that calls `pyo3_build_config::add_extension_module_link_args()` to configure the linker
3. **Module naming fix**: Created a `balatro_engine.so` symlink on macOS since Python expects `.so` extension even though the actual file is `.dylib`

### Files Changed
- `balatro-engine/Cargo.toml`: Added `pyo3-build-config` dependency
- `balatro-engine/build.rs`: New build script for Python linking
- `pylatro-ai/setup_test_env.sh`: Updated to create the `.so` symlink automatically
- `balatro-engine/BUILDING.md`: New documentation for building with Python bindings

### Verification
The module now successfully:
- Compiles without linking errors
- Can be imported in Python: `import balatro_engine`
- Exposes all expected classes: GameState, all Action types, and supporting types

