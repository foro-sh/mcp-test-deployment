# dummy-mcp-server-typescript

Same dummy tool set as the Python fixtures (`../uv`, `../pdm`, `../poetry`,
`../pipenv`, `../requirements`), built with the
[TypeScript MCP SDK](https://github.com/modelcontextprotocol/typescript-sdk)
instead. Used as a fixture for exercising foro.sh's Node/TypeScript build
detection (`package.json` + `tsconfig.json`).

## Run locally

```bash
npm install
npm run build
npm start
```

Or without a build step:

```bash
npm install
npx tsx server.ts
```
