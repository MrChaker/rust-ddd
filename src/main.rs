use config::{
    database::{self, DatabaseTrait},
    parameters,
};
pub mod config;
pub mod error;
pub mod response;
pub mod routes;
pub mod user;

#[tokio::main]
async fn main() {
    parameters::init();

    let database = database::Database::init().await.unwrap();

    let host = format!("0.0.0.0:{}", parameters::get("PORT"));
    //tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, routes::root::init(database))
        .await
        .unwrap();
}
