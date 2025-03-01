use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use tokio::net::TcpListener;

fn init_logs() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let level = std::env::var("LOG").unwrap_or_else(|_| "debug".into());
    if let Err(err) = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(level))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
    {
        eprintln!("unable to register tracing: {err:?}");
    }
}

fn address() -> SocketAddr {
    let host = std::env::var("HOST")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(3000);

    SocketAddr::from((host, port))
}

#[derive(Debug)]
enum EngineError {
    Parse(mrml::prelude::parser::Error),
    Render(mrml::prelude::render::Error),
}

impl IntoResponse for EngineError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Parse(ref inner) => tracing::debug!("unable to parse: {inner:?}"),
            Self::Render(ref inner) => tracing::debug!("unable to render: {inner:?}"),
        };
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("unable to convert template: {self:?}"),
        )
            .into_response()
    }
}

#[derive(Clone, Debug)]
struct Engine {
    parser: Arc<mrml::prelude::parser::AsyncParserOptions>,
    render: Arc<mrml::prelude::render::RenderOptions>,
}

impl Default for Engine {
    fn default() -> Self {
        use mrml::prelude::parser::http_loader::{AsyncReqwestFetcher, HttpIncludeLoader};
        use mrml::prelude::parser::loader::AsyncIncludeLoader;
        use mrml::prelude::parser::multi_loader::MultiIncludeLoader;
        use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
        use mrml::prelude::parser::AsyncParserOptions;

        let resolver = MultiIncludeLoader::<Box<dyn AsyncIncludeLoader + Send + Sync>>::new()
            .with_starts_with(
                "https://gist.githubusercontent.com/jdrouet",
                Box::new(HttpIncludeLoader::<AsyncReqwestFetcher>::allow_all()),
            )
            .with_any(Box::<NoopIncludeLoader>::default());

        Self {
            parser: Arc::new(AsyncParserOptions {
                include_loader: Box::new(resolver),
            }),
            render: Default::default(),
        }
    }
}

impl Engine {
    async fn handle(&self, input: String) -> Result<Response, EngineError> {
        let item = mrml::async_parse_with_options(input, self.parser.clone())
            .await
            .map_err(EngineError::Parse)?;

        let content = item
            .element
            .render(&self.render)
            .map_err(EngineError::Render)?;

        Ok(Response {
            content,
            warnings: item.warnings.into_iter().map(Warning::from).collect(),
        })
    }
}

#[derive(Debug, serde::Deserialize)]
struct Payload {
    template: String,
}

#[derive(Debug, serde::Serialize)]
struct Response {
    content: String,
    warnings: Vec<Warning>,
}

#[derive(Debug, serde::Serialize)]
struct Warning {
    code: &'static str,
    start: usize,
    end: usize,
}

impl From<mrml::prelude::parser::Warning> for Warning {
    fn from(value: mrml::prelude::parser::Warning) -> Self {
        Warning {
            code: value.kind.as_str(),
            start: value.span.start,
            end: value.span.end,
        }
    }
}

#[axum::debug_handler]
async fn handler(
    State(engine): State<Engine>,
    Json(payload): Json<Payload>,
) -> Result<Json<Response>, EngineError> {
    engine.handle(payload.template).await.map(Json)
}

fn create_app() -> axum::Router {
    axum::Router::default()
        .route("/render", axum::routing::post(handler))
        .with_state(Engine::default())
}

#[tokio::main]
async fn main() {
    init_logs();

    tracing::debug!("binding socket");
    let addr = address();
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("server listening on {addr}");
    axum::serve(listener, create_app()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{self, Request, StatusCode};
    use serde_json::json;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn should_parse_and_render() {
        super::init_logs();
        let res = super::create_app()
            .oneshot(
                Request::builder()
                    .uri("/render")
                    .method("POST")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "template": "<mjml><mj-body><mj-include path=\"https://gist.githubusercontent.com/jdrouet/b0ac80fa08a3e7262bd4c94fc8865a87/raw/ec8771f4804a6c38427ed2a9f5937e11ec2b8c27/hello-world.mjml\" /></mj-body></mjml>",
                        })).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
