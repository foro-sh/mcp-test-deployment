# dummy-mcp-server-pipenv

Same dummy [FastMCP](https://github.com/jlowin/fastmcp) server as `../uv`, but
dependency-managed with [pipenv](https://pipenv.pypa.io/) instead of `uv`.
Used as a fixture for foro-sh/platform#296 (dependency-manager auto-detection
via `Pipfile` / `Pipfile.lock`).

## Run locally

```bash
pipenv install
pipenv run python server.py
```
