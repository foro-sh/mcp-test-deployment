# dummy-mcp-server-requirements

Same dummy [FastMCP](https://github.com/jlowin/fastmcp) server as `../uv`, but
with a plain `requirements.txt` and no `pyproject.toml` at all. Used as a
fixture for foro-sh/platform#296 (dependency-manager auto-detection falling
through to the `uv-pip` / `requirements.txt` last resort).

## Run locally

```bash
uv venv
uv pip install -r requirements.txt
uv run python server.py
```
