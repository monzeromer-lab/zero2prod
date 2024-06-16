use std::net::TcpListener;

use zero2prod::{configration::get_configration, run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configration = get_configration().expect("Faield to deserialize settings.");
    let listener = TcpListener::bind("0.0.0.0:0").expect("Faield to bind random port.");
    println!("http://127.0.0.1:{}", format!("{}", configration.application_port));
    run(listener)?.await
}