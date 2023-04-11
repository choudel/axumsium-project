use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
mod error;
use crate::model::ModelController;

pub use self::error::{Error, Result};
mod model;
mod web;
#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    async fn main_response_mapper(res: Response) -> Response {
        println!("->> {:<12} - main response_mapper", "RES_MAPPER");
        println!();
        res
    }

    fn routes_hello() -> Router {
        Router::new()
            .route("/hello", get(handler_hello))
            .route("/hello2/:name", get(handler_hello2))
    }

    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }
    //region: ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    //endregion: ---Start Server

    //region: ---Handler Hello
    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }
    async fn handler_hello(Query(param): Query<HelloParams>) -> impl IntoResponse {
        println!("-->{:<12}-handler_hello- {param:?}", "HANDLER");
        let name = param.name.as_deref().unwrap_or("World!");
        Html(format!("Hello <strong> {name} </strong>"))
    }

    async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
        println!("-->{:<12}-handler_hello- {name:?}", "HANDLER");

        Html(format!("Hello <strong> {name} </strong>"))
    }
    //endregion: ---Handler Hello
    Ok(())
}
