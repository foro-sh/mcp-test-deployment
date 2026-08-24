//! Smoke test: exercises every tool against a locally running server
//! (`MCP_PORT=8123 cargo run` in another shell) via the reference rmcp client.
use anyhow::Result;
use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, ClientCapabilities, ClientInfo, Implementation},
    transport::StreamableHttpClientTransport,
};

#[tokio::main]
async fn main() -> Result<()> {
    let transport = StreamableHttpClientTransport::from_uri("http://localhost:8123/mcp");
    let client_info = ClientInfo::new(
        ClientCapabilities::default(),
        Implementation::new("smoke-test", "0.0.1"),
    );
    let client = client_info.serve(transport).await?;

    let tools = client.list_tools(Default::default()).await?;
    let names: Vec<_> = tools.tools.iter().map(|t| t.name.clone()).collect();
    assert_eq!(
        {
            let mut n = names.clone();
            n.sort();
            n
        },
        vec!["add", "echo", "get_env", "whoami"]
    );

    let add = client
        .call_tool(
            CallToolRequestParams::new("add")
                .with_arguments(serde_json::json!({"a": 2, "b": 3}).as_object().cloned().unwrap()),
        )
        .await?;
    println!("add(2, 3) -> {add:?}");

    let echo = client
        .call_tool(
            CallToolRequestParams::new("echo").with_arguments(
                serde_json::json!({"message": "hi"}).as_object().cloned().unwrap(),
            ),
        )
        .await?;
    println!("echo(hi) -> {echo:?}");

    let whoami = client
        .call_tool(CallToolRequestParams::new("whoami"))
        .await?;
    println!("whoami() -> {whoami:?}");

    let get_env = client
        .call_tool(
            CallToolRequestParams::new("get_env").with_arguments(
                serde_json::json!({"name": "TEST_SECRET"})
                    .as_object()
                    .cloned()
                    .unwrap(),
            ),
        )
        .await?;
    println!("get_env(TEST_SECRET) -> {get_env:?}");

    client.cancel().await?;
    println!("smoke test passed");
    Ok(())
}
