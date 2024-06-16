use std::net::TcpListener;
use zero2prod::startup::run;
use sqlx::PgPool;
use zero2prod::configration::get_configration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configration = get_configration().expect("Faield to deserialize settings.");
    let connection_pool = PgPool::connect(&configration.database.connection_string())
        .await
        .expect("Faield to connect to postgres");
    let address = format!("127.0.0.1:{}", configration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}