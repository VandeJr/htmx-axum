use askama::Template;
use axum::{Router, routing::get, response::IntoResponse};
use tower_http::services::ServeDir;


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String
}


#[derive(Template)]
#[template(path = "partial.html")]
struct PartialTemplate {
}

async fn root() -> impl IntoResponse {
    IndexTemplate {
        name: "Askama".to_string()
    }
}

async fn partial() -> impl IntoResponse {
    PartialTemplate {}
}

#[tokio::main]
async fn main() {
    let app = Router::new()
            .nest_service("/public", ServeDir::new("public"))
            .route("/", get(root))
            .route("/partial", get(partial));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
