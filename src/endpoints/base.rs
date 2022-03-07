use axum::{
    extract::{Extension},
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::Html,
    routing::{get, get_service},
    Router,
};
use minijinja::context;
use tower_http::services::fs::ServeDir;


use crate::endpoints::ApiContext;


pub fn router() -> Router {
    // Serves files inside the `static` directory at `GET /static/*`
        let serve_dir_service = get_service(ServeDir::new("static"))
            .handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            });
    Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .nest("/static", serve_dir_service)
}



async fn index(ctx: Extension<ApiContext>) -> Html<String> {
    let template = ctx.template_env.get_template("index.html").unwrap();
    Html(template.render(context!(title => "Hooksaurus Auctions: Helping Animal Sanctuaries")).unwrap())
}

async fn health_check() -> &'static str {
    "ok"
}
