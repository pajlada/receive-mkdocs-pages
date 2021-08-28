use axum::{
    async_trait,
    body::{Bytes, Empty},
    extract::{Extension, FromRequest, Query, RequestParts},
    handler::get,
    handler::post,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    AddExtensionLayer, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr};

static PASSWORD_HEADER: &str = "X-Receive-Password";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config();

    let app = Router::new()
        .route("/", get(index))
        .route("/test", post(test))
        .route("/push", post(push))
        .layer(AddExtensionLayer::new(config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Clone)]
struct Config {
    password: String,
}

fn config() -> Config {
    return Config {
        password: env::var("RECEIVE_MKDOCS_PAGES_PASSWORD")
            .expect("Missing RECEIVE_MKDOCS_PAGES_PASSWORD!"),
    };
}

async fn index() -> impl IntoResponse {
    "POST to /push!"
}

#[derive(Serialize)]
struct SuccessResponse {
    message: String,
}

async fn push(_: PasswordAuthenticated) -> impl IntoResponse {
    return "Good password!".to_string();
}

async fn test(_: PasswordAuthenticated) -> impl IntoResponse {
    return Json(SuccessResponse {
        message: "aaaaaaaa".to_string(),
    });
}

struct PasswordAuthenticated;

#[async_trait]
impl<B> FromRequest<B> for PasswordAuthenticated
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(config) = Extension::<Config>::from_request(req)
            .await
            .expect("`Config` extension missing");

        let headers = req.headers().expect("other extractor taken headers");

        let password = headers
            .get(PASSWORD_HEADER)
            .ok_or((StatusCode::BAD_REQUEST, "Missing X-Receive-Password header"))?
            .to_str()
            .unwrap();

        if password == config.password {
            Ok(PasswordAuthenticated)
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                "Bad password in X-Receive-Password header",
            ))
        }
    }
}
