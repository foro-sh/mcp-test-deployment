"""Dummy FastMCP server for exercising mcphost.eu deployments.

Exposes a few trivial tools over the streamable HTTP transport. The platform
injects MCP_PORT (the port to bind) and PROJECT_SLUG at container start; any
project secrets are injected as additional environment variables.
"""

import os

from fastmcp import FastMCP

mcp = FastMCP("dummy-mcp-server")


@mcp.tool
def add(a: int, b: int) -> int:
    """Add two integers."""
    return a + b


@mcp.tool
def echo(message: str) -> str:
    """Return the given message unchanged."""
    return message


@mcp.tool
def echo_rollback(message: str) -> str:
    """Return the given message unchanged."""
    return message


@mcp.tool
def whoami() -> dict[str, str]:
    """Report the deployment's slug, to confirm which server answered."""
    return {"slug": os.environ.get("PROJECT_SLUG", "unknown")}


@mcp.tool
def get_env(name: str) -> dict[str, object]:
    """Report whether an environment variable is set, and its value.

    Used to verify secret propagation: configure a project secret in
    mcphost.eu, deploy, then call get_env with the secret's name to confirm it
    reached the running container as an environment variable.
    """
    value = os.environ.get(name)
    return {"name": name, "set": value is not None, "value": value}


def main() -> None:
    port = int(os.environ.get("MCP_PORT", "8000"))
    mcp.run(transport="http", host="0.0.0.0", port=port)


if __name__ == "__main__":
    main()
