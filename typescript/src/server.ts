// Dummy MCP server for exercising mcphost.eu deployments (TypeScript fixture).
//
// Exposes a few trivial tools over the streamable HTTP transport, on the
// 2026-07-28 protocol revision (@modelcontextprotocol/server v2). The
// platform injects MCP_PORT (the port to bind) and PROJECT_SLUG at container
// start; any project secrets are injected as additional environment
// variables.

import { createServer } from "node:http";
import { createMcpHandler, McpServer } from "@modelcontextprotocol/server";
import { toNodeHandler } from "@modelcontextprotocol/node";
import * as z from "zod/v4";

const handler = createMcpHandler(() => {
    const mcp = new McpServer({ name: "dummy-mcp-server-typescript", version: "0.1.0" });

    mcp.registerTool(
        "add",
        {
            description: "Add two integers.",
            inputSchema: z.object({ a: z.number(), b: z.number() }),
        },
        async ({ a, b }) => ({ content: [{ type: "text", text: String(a + b) }] }),
    );

    mcp.registerTool(
        "echo",
        {
            description: "Return the given message unchanged.",
            inputSchema: z.object({ message: z.string() }),
        },
        async ({ message }) => ({ content: [{ type: "text", text: message }] }),
    );

    mcp.registerTool(
        "whoami",
        { description: "Report the deployment's slug, to confirm which server answered." },
        async () => ({
            content: [{ type: "text", text: process.env.PROJECT_SLUG ?? "unknown" }],
        }),
    );

    mcp.registerTool(
        "get_env",
        {
            description:
                "Report whether an environment variable is set, and its value. Used to verify secret propagation.",
            inputSchema: z.object({ name: z.string() }),
        },
        async ({ name }) => {
            const value = process.env[name];
            return {
                content: [
                    {
                        type: "text",
                        text: JSON.stringify({ name, set: value !== undefined, value: value ?? null }),
                    },
                ],
            };
        },
    );

    return mcp;
});

const nodeHandler = toNodeHandler(handler);
const port = Number(process.env.MCP_PORT ?? 8000);

// ponytail: no Host/Origin (DNS-rebinding) guard — mcphost.eu proxies with an
// unpredictable Host header, and this is a throwaway test fixture. Add
// hostHeaderValidation/originValidation from @modelcontextprotocol/node,
// scoped to the platform's proxy hostname, if this ever serves real data.
createServer((req, res) => {
    void nodeHandler(req, res);
}).listen(port, "0.0.0.0", () => {
    console.log(`dummy-mcp-server-typescript listening on 0.0.0.0:${port}`);
});
