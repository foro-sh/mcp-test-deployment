# dummy-mcp-server-poetry

Same dummy [FastMCP](https://github.com/jlowin/fastmcp) server as `../uv`, but
dependency-managed with [Poetry](https://python-poetry.org/) instead of `uv`.
Used as a fixture for foro-sh/platform#296 (dependency-manager auto-detection
via `poetry.lock` / `[tool.poetry]`).

## Run locally

```bash
poetry install
poetry run python server.py
```
