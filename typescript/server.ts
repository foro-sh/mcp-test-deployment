// Dummy MCP server for exercising mcphost.eu deployments (TypeScript fixture).
//
// Exposes the same trivial tool set as the Python fixtures, over the
// streamable HTTP transport. The platform injects MCP_PORT (the port to
// bind) and PROJECT_SLUG at container start; any project secrets are
// injected as additional environment variables.

import { createServer } from "node:http";
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StreamableHTTPServerTransport } from "@modelcontextprotocol/sdk/server/streamableHttp.js";
import { z } from "zod";

const mcp = new McpServer({ name: "dummy-mcp-server-typescript", version: "0.1.0" });

mcp.registerTool(
  "add",
  {
    description: "Add two integers.",
    inputSchema: { a: z.number().int(), b: z.number().int() },
  },
  async ({ a, b }) => ({ content: [{ type: "text", text: String(a + b) }] }),
);

mcp.registerTool(
  "echo",
  {
    description: "Return the given message unchanged.",
    inputSchema: { message: z.string() },
  },
  async ({ message }) => ({ content: [{ type: "text", text: message }] }),
);

mcp.registerTool(
  "whoami",
  { description: "Report the deployment's slug, to confirm which server answered." },
  async () => ({
    content: [{ type: "text", text: JSON.stringify({ slug: process.env.PROJECT_SLUG ?? "unknown" }) }],
  }),
);

mcp.registerTool(
  "get_env",
  {
    description:
      "Report whether an environment variable is set, and its value. Used to verify " +
      "secret propagation: configure a project secret in mcphost.eu, deploy, then call " +
      "get_env with the secret's name to confirm it reached the running container.",
    inputSchema: { name: z.string() },
  },
  async ({ name }) => {
    const value = process.env[name];
    return {
      content: [
        { type: "text", text: JSON.stringify({ name, set: value !== undefined, value: value ?? null }) },
      ],
    };
  },
);

const transport = new StreamableHTTPServerTransport({ sessionIdGenerator: undefined });
await mcp.connect(transport);

const port = Number(process.env.MCP_PORT ?? 8000);
createServer(async (req, res) => {
  if (req.url === "/mcp/" || req.url === "/mcp") {
    await transport.handleRequest(req, res);
    return;
  }
  res.writeHead(404).end();
}).listen(port, "0.0.0.0");
