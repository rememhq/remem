# remem Python SDK

This is the Python SDK for remem, providing a typed interface over the REST API.

## Installation

```bash
pip install remem
```

## Usage

```python
from remem import Memory

memory = Memory(base_url="http://localhost:7474")
response = memory.store("User prefers Python")
```
