//! Dummy FastMCP-equivalent server for exercising mcphost.eu deployments.
//!
//! Rust counterpart of the Python/TypeScript fixtures in this repo: same
//! trivial tool set, served over streamable HTTP. The platform injects PORT
//! (the port to bind) and PROJECT_SLUG at container start; any project
//! secrets arrive as additional environment variables.

use rmcp::{
    Json, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AddRequest {
    a: i64,
    b: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct EchoRequest {
    message: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct WhoamiResponse {
    slug: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct GetEnvRequest {
    name: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
struct GetEnvResponse {
    name: String,
    set: bool,
    value: Option<String>,
}

#[derive(Clone, Default)]
struct DummyServer {
    #[allow(dead_code)] // read by the #[tool_handler] macro, not by name
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl DummyServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Add two integers.")]
    fn add(&self, Parameters(AddRequest { a, b }): Parameters<AddRequest>) -> Json<i64> {
        Json(a + b)
    }

    #[tool(description = "Return the given message unchanged.")]
    fn echo(&self, Parameters(EchoRequest { message }): Parameters<EchoRequest>) -> Json<String> {
        Json(message)
    }

    #[tool(description = "Report the deployment's slug, to confirm which server answered.")]
    fn whoami(&self) -> Json<WhoamiResponse> {
        Json(WhoamiResponse {
            slug: std::env::var("PROJECT_SLUG").unwrap_or_else(|_| "unknown".to_string()),
        })
    }

    /// Used to verify secret propagation: configure a project secret in
    /// mcphost.eu, deploy, then call get_env with the secret's name to
    /// confirm it reached the running container as an environment variable.
    #[tool(description = "Report whether an environment variable is set, and its value.")]
    fn get_env(
        &self,
        Parameters(GetEnvRequest { name }): Parameters<GetEnvRequest>,
    ) -> Json<GetEnvResponse> {
        let value = std::env::var(&name).ok();
        Json(GetEnvResponse {
            set: value.is_some(),
            name,
            value,
        })
    }
}

#[tool_handler]
impl ServerHandler for DummyServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(
            Implementation::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        )
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let service = StreamableHttpService::new(
        || Ok(DummyServer::new()),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default(),
    );
    let router = axum::Router::new().nest_service("/mcp", service);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
