use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Faield to bind random port.");
    let port = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{}", format!("{}",port));
    run(listener)?.await
}