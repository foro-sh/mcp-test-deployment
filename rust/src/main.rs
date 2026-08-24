//! Dummy MCP server for exercising mcphost.eu deployments.
//!
//! Exposes a few trivial tools over the streamable HTTP transport, on the
//! 2026-07-28 protocol revision (rmcp 3.x). The platform injects MCP_PORT
//! (the port to bind) and PROJECT_SLUG at container start; any project
//! secrets are injected as additional environment variables.

use rmcp::{
    ServerHandler,
    handler::server::wrapper::{Json, Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
    },
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Default)]
pub struct DummyServer {
    #[allow(dead_code)]
    tool_router: rmcp::handler::server::router::tool::ToolRouter<DummyServer>,
}

#[derive(Debug, schemars::JsonSchema, Deserialize)]
pub struct AddRequest {
    pub a: i64,
    pub b: i64,
}

#[derive(Debug, schemars::JsonSchema, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

#[derive(Debug, schemars::JsonSchema, Deserialize)]
pub struct GetEnvRequest {
    pub name: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct WhoamiResponse {
    pub slug: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct GetEnvResponse {
    pub name: String,
    pub set: bool,
    pub value: Option<String>,
}

#[tool_router]
impl DummyServer {
    fn new() -> Self {
        Self::default()
    }

    #[tool(description = "Add two integers.")]
    async fn add(&self, Parameters(AddRequest { a, b }): Parameters<AddRequest>) -> Json<i64> {
        Json(a + b)
    }

    #[tool(description = "Return the given message unchanged.")]
    async fn echo(&self, Parameters(EchoRequest { message }): Parameters<EchoRequest>) -> String {
        message
    }

    #[tool(description = "Report the deployment's slug, to confirm which server answered.")]
    async fn whoami(&self) -> Json<WhoamiResponse> {
        Json(WhoamiResponse {
            slug: env::var("PROJECT_SLUG").unwrap_or_else(|_| "unknown".to_string()),
        })
    }

    #[tool(
        description = "Report whether an environment variable is set, and its value. Used to verify secret propagation."
    )]
    async fn get_env(
        &self,
        Parameters(GetEnvRequest { name }): Parameters<GetEnvRequest>,
    ) -> Json<GetEnvResponse> {
        let value = env::var(&name).ok();
        Json(GetEnvResponse {
            set: value.is_some(),
            value,
            name,
        })
    }
}

#[tool_handler]
impl ServerHandler for DummyServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::new(ServerCapabilities::builder().enable_tools().build());
        info.server_info = Implementation::new("dummy-mcp-server-rust", env!("CARGO_PKG_VERSION"));
        info
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port: u16 = env::var("MCP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let ct = tokio_util::sync::CancellationToken::new();

    let service = StreamableHttpService::new(
        || Ok(DummyServer::new()),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default().with_cancellation_token(ct.child_token()),
    );

    let router = axum::Router::new().nest_service("/mcp", service);
    let tcp_listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.ok();
            ct.cancel();
        })
        .await?;
    Ok(())
}
