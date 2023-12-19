use axum_sqlx_demo::bootstrap;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let app = bootstrap::create_app().await;
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
