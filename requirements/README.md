# dummy-mcp-server-requirements

Same dummy [FastMCP](https://github.com/jlowin/fastmcp) server as `../uv`, but
with a plain `requirements.txt` for its dependencies. The `pyproject.toml`
carries no dependencies of its own: it is there because foro reads the name and
the interpreter constraint from it (foro-sh/platform#754), and it declares
`dependency_manager = "uv-pip"` because its own presence would otherwise send
detection down the plain-`uv` path. Used as a fixture for
foro-sh/platform#296 (dependency-manager detection, `uv-pip` install path).

## Run locally

```bash
uv venv
uv pip install -r requirements.txt
uv run python server.py
```
