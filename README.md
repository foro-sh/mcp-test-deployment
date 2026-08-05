# mcp-test-deployment

Dummy [FastMCP](https://github.com/jlowin/fastmcp) servers used to test
deployments on [mcphost.eu](https://mcphost.eu). Each subfolder is an
independent, deployable MCP project — same trivial tool set, different
Python dependency manager — so this one repo can exercise every manager
foro.sh's build pipeline detects:

| Folder | Manager | Detected via |
| --- | --- | --- |
| [`uv/`](uv) | uv | `uv.lock` |
| [`pdm/`](pdm) | pdm | `pdm.lock` |
| [`poetry/`](poetry) | Poetry | `poetry.lock` / `[tool.poetry]` |
| [`pipenv/`](pipenv) | pipenv | `Pipfile` / `Pipfile.lock` |
| [`requirements/`](requirements) | uv-pip | `requirements.txt` (no `pyproject.toml`) |

Each folder has its own `foro.yaml` manifest, so the repo has more than one
deployable project — see foro-sh/platform#296 (manifest scanning + per-manager
build detection).

## Run any of them locally

```bash
cd uv        # or pdm, poetry, pipenv, requirements
uv sync && uv run server.py
```

(swap `uv sync && uv run` for the matching manager's install/run commands —
see each folder's README).

The server binds to `0.0.0.0` on `$MCP_PORT` (default `8000`) and serves MCP
at `/mcp/`.

## Deploying with mcphost.eu

Each `foro.yaml` declares that project's entrypoint, Python version, and
port. The platform locates manifests anywhere in the repo tree, builds the
image with the detected dependency manager, and injects `MCP_PORT` (the port
to bind) and `PROJECT_SLUG` at container start; project secrets arrive as
additional environment variables.

## Tools

Every fixture exposes the same tools:

- `add(a, b)` — add two integers
- `echo(message)` — return the message unchanged
- `whoami()` — report the deployment's slug (confirms which server answered)
- `get_env(name)` — report whether an env var is set and its value (see below)

## Testing secret propagation

`get_env` exists to verify that project secrets configured in mcphost.eu are
propagated into the deployed container as environment variables:

1. In the mcphost.eu dashboard, add a secret to this project, e.g.
   `TEST_SECRET=hello`.
2. Deploy (or redeploy).
3. From an MCP client, call `get_env` with `name="TEST_SECRET"`. A correctly
   propagated secret returns `{"name": "TEST_SECRET", "set": true, "value": "hello"}`.

The platform also injects `MCP_PORT` and `PROJECT_SLUG`, so `get_env("PROJECT_SLUG")`
should always report the deployment slug.
