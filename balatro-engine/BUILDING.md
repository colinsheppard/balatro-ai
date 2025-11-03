# Building balatro-engine with Python Bindings

## Quick Start

To build the Rust library with Python bindings:

```bash
cargo build --features python --release
```

This will:
1. Compile the Rust code with PyO3 bindings
2. Link against your Python installation (detected automatically via `pyo3-build-config`)
3. Create a shared library in `target/release/`

## Extension Module Naming

After building, you'll have:
- `target/release/libbalatro_engine.dylib` (macOS) or `.so` (Linux) - the actual library
- `target/release/balatro_engine.so` - a symlink created for Python import compatibility

Python will import `balatro_engine` by looking for `balatro_engine.so` in `PYTHONPATH`.

## Troubleshooting

### Linking Errors

If you see linker errors about missing Python symbols:
- Ensure Python development headers are installed
- On macOS with pyenv: `pyenv install --keep <version>` to ensure headers
- The build script (`build.rs`) uses `pyo3-build-config` to auto-detect Python

### Import Errors in Python

If Python can't import the module:
- Ensure `PYTHONPATH` includes `target/release/` directory
- On macOS, ensure the `balatro_engine.so` symlink exists
- Check that you're using the same Python version that was detected during build

### Build Script

The `build.rs` file handles:
- Python detection via `pyo3-build-config`
- Linker configuration for Python extension modules
- Rebuild triggers when `Cargo.toml` changes
