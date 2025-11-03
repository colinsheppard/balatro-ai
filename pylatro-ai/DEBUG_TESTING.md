# Debug Testing Guide

## Quick Debug Commands

### Basic Debug Mode (Verbose + Show Prints)
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest -v -s
```

### Run Specific Test with Full Debug Output
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest tests/test_import.py -v -s
```

### Maximum Verbosity (vvv)
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest -vvv -s tests/test_import.py
```

## Interactive Debugging

### Drop into Debugger on Failure
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest --pdb tests/test_python_input.py
```
This will drop you into `pdb` (Python debugger) when a test fails.

### Drop into Debugger at Test Start
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest --pdb-trace tests/test_python_input.py
```
This drops into the debugger at the start of each test.

### Drop into Debugger on Failure or Error
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest --pdb --pdbcls=IPython.terminal.debugger:Pdb tests/test_python_input.py
```
(Requires ipython: `pip install ipython`)

## Logging Debug Mode

### Show All Log Output
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest --log-cli-level=DEBUG -s tests/test_python_input.py
```

### Show Specific Log Levels
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest --log-cli-level=INFO -s tests/test_python_input.py
```

## Useful Debug Flags Combined

### Comprehensive Debug Session
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest -vvv -s --log-cli-level=DEBUG --tb=short tests/test_python_input.py
```

### Run Single Test with Full Debug
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest -vvv -s --pdb tests/test_python_input.py::test_python_input_game_flow
```

### Show Test Output Immediately (no buffering)
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest -s --tb=short tests/test_python_input.py
```

## Traceback Options

```bash
# Short traceback (default)
pytest --tb=short

# Long traceback (full details)
pytest --tb=long

# Line traceback (one line per failure)
pytest --tb=line

# No traceback (just summary)
pytest --tb=no
```

## Debugging Specific Issues

### Debug the Python Input Test
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest -vvv -s --log-cli-level=DEBUG --tb=long tests/test_python_input.py::test_python_input_game_flow
```

### Debug with Timeout (if using pytest-timeout)
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest --timeout=30 -vvv -s tests/test_python_input.py
```

## Using Python Debugger (pdb) Manually

Add this to your test code:
```python
import pdb; pdb.set_trace()
```

Then run:
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest -s tests/test_python_input.py
```

## Debugging in VS Code or IDE

### VS Code Launch Configuration
Create `.vscode/launch.json`:
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Python: Pytest Current File",
            "type": "python",
            "request": "launch",
            "module": "pytest",
            "args": [
                "${file}",
                "-vvv",
                "-s",
                "--log-cli-level=DEBUG"
            ],
            "env": {
                "PYTHONPATH": "${workspaceFolder}/balatro-engine/target/release"
            },
            "console": "integratedTerminal",
            "justMyCode": false
        }
    ]
}
```

## Common Debug Scenarios

### Debug Import Issues
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
python -c "import balatro_engine; print(balatro_engine)"
pytest -vvv -s tests/test_import.py
```

### Debug Test Collection
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest --collect-only -v
```

### Debug with Coverage (already enabled by default)
```bash
export PYTHONPATH="../balatro-engine/target/release:$PYTHONPATH"
pytest --cov=pylatro_ai --cov-report=html -vvv -s
# Then open htmlcov/index.html
```


