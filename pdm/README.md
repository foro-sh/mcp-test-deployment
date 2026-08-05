# dummy-mcp-server-pdm

Same dummy [FastMCP](https://github.com/jlowin/fastmcp) server as `../uv`, but
dependency-managed with [pdm](https://pdm-project.org/) instead of `uv`. Used
as a fixture for foro-sh/platform#296 (dependency-manager auto-detection via
`pdm.lock`).

## Run locally

```bash
pdm install
pdm run python server.py
```
