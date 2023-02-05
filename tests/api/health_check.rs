use std::net::{SocketAddr, TcpListener};

use helsinki_bike_app::startup::run;

async fn spawn_app() -> SocketAddr {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { run(listener).await.unwrap() });
    address
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let response = reqwest::get(format!("http://{}/api/health_check", address))
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
