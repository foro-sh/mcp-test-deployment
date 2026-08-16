# mcp-test-deployment

Dummy [FastMCP](https://github.com/jlowin/fastmcp) / [TypeScript MCP
SDK](https://github.com/modelcontextprotocol/typescript-sdk) servers used to
test deployments on [mcphost.eu](https://mcphost.eu). Each subfolder is an
independent, deployable MCP project — same trivial tool set, different
dependency manager (mostly Python, one Node) — so this one repo can exercise
every manager foro.sh's build pipeline detects:

| Folder | Manager | Detected via |
| --- | --- | --- |
| [`uv/`](uv) | uv | `uv.lock` |
| [`pdm/`](pdm) | pdm | `pdm.lock` |
| [`poetry/`](poetry) | Poetry | `poetry.lock` / `[tool.poetry]` |
| [`pipenv/`](pipenv) | pipenv | `Pipfile` / `Pipfile.lock` |
| [`requirements/`](requirements) | uv-pip | explicit `dependency_manager = "uv-pip"` |
| [`typescript/`](typescript) | npm | `package.json` |

Each folder has its own `pyproject.toml` (Python) or `package.json`
(`typescript/`), so the repo has more than one deployable project — see
foro-sh/platform#296 (config scanning + per-manager build detection).

`requirements/` asks for its manager by name because every deployable Python
project now carries a `pyproject.toml` (foro-sh/platform#754), and that file
is what detection reaches before it ever looks for a `requirements.txt`.

## Run any of them locally

```bash
cd uv        # or pdm, poetry, pipenv, requirements
uv sync && uv run server.py
```

(swap `uv sync && uv run` for the matching manager's install/run commands —
see each folder's README).

The server binds to `0.0.0.0` on `$PORT` (default `8000`) and serves MCP
at `/mcp/`.

## Deploying with mcphost.eu

Each `pyproject.toml` gives the platform the project's name and its
interpreter constraint; the entrypoint is `server.py` by inference, and the
`[tool.foro]` table carries only what the file itself can't say (the port
here, plus `requirements/`'s dependency manager). `typescript/`'s
`package.json` works the same way via its `"foro"` key — `main` there points
`npm run build`'s output (`dist/server.js`), so the platform can infer the
entrypoint too. The platform locates deployable directories anywhere in the
repo tree, builds the image with the detected dependency manager, and injects
`PORT` and `FASTMCP_PORT` (the port to bind — `MCP_PORT` is gone) plus
`PROJECT_SLUG` at container start; project secrets arrive as additional
environment variables.

Interpreter per fixture: `uv/`, `pdm/` and `poetry/` say `>=3.12` and so build
on the newest Python foro allows; `pipenv/` and `requirements/` cap themselves
at `<3.13` because their lockfiles were resolved against 3.12. Ports skip 8001
and 8002 — the in-container gate binds those.

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

The platform also injects `PORT` and `PROJECT_SLUG`, so `get_env("PROJECT_SLUG")`
should always report the deployment slug.
