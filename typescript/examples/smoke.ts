// Smoke test: exercises every tool against a locally running server
// (`MCP_PORT=8124 npm run dev` in another shell) via the reference client SDK.
import { Client, StreamableHTTPClientTransport } from "@modelcontextprotocol/client";
import assert from "node:assert/strict";

async function main() {
    const client = new Client({ name: "smoke-test", version: "0.0.1" });
    const transport = new StreamableHTTPClientTransport(new URL("http://localhost:8124/mcp"));
    await client.connect(transport);

    const tools = await client.listTools();
    const names = tools.tools.map((t) => t.name).sort();
    assert.deepEqual(names, ["add", "echo", "get_env", "whoami"]);

    const add = await client.callTool({ name: "add", arguments: { a: 2, b: 3 } });
    console.log("add(2, 3) ->", add);

    const echo = await client.callTool({ name: "echo", arguments: { message: "hi" } });
    console.log("echo(hi) ->", echo);

    const whoami = await client.callTool({ name: "whoami", arguments: {} });
    console.log("whoami() ->", whoami);

    const getEnv = await client.callTool({ name: "get_env", arguments: { name: "TEST_SECRET" } });
    console.log("get_env(TEST_SECRET) ->", getEnv);

    await transport.terminateSession();
    await client.close();
    console.log("smoke test passed");
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
