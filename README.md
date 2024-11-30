# CSV Reader: Rust to Python Integration

This project demonstrates how to implement a CSV reader in `Rust` and expose it to `Python` as a module using [PyO3](https://pyo3.rs/) and [setuptools-rust](https://github.com/PyO3/setuptools-rust/tree/main). 
The module reads CSV files and returns their data as Python-compatible objects

## Prerequisites
1. Install Rust: Ensure you have the Rust toolchain installed. Install it using [Rustup](https://rustup.rs/) if necessary.
2. Install Python: Have a Python interpreter installed (3.7 or later is recommended).
3. Install Required Python Tools:
   1. `pip install setuptools-rust` for building and managing Rust-based Python extensions.

## Installation 

```console
cd csv_reader
python3 -m venv .venv
source .venv/bin/activate  # on Linux or macOS
.venv\Scripts\activate     # on Windows
python3 -m pip install -e .
python3
>>> import csv_reader
```

## Test the Module in Python

Create an `example.py` file to test the module:

```python
import csv_reader

headers, records = csv_reader.read_csv("data.csv", ",", True) # Path to csv file, separator, has headers flag
print("Headers:", headers)
for record in records:
    print(record)
```
