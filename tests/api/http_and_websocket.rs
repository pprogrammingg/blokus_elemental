
use actix_web::web::Bytes;
use actix_web::{rt::time::sleep, App, HttpServer};
use actix_ws::{, Message};
use reqwest::Client;
use std::time::Duration;
use blokus_elemental::configuration::get_configuration;
use blokus_elemental::startup::Application;

// You already have this struct for your test application.
pub struct TestApp {
    pub address: String,
    pub port: u16,
}

// Your spawn_app helper function to bring up the app.
pub async fn spawn_app() -> TestApp {
    // Randomize configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a random OS port
        c.application.port = 0;
        c
    };

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    let application_port = application.port();

    // Spawn the application
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
    }
}

#[tokio::test]
async fn health_check_and_echo_succeed_when_invoked_by_multiple_clients() {
    // Arrange - this should bring up websocket and http APIs
    let app = spawn_app().await;

    // Create an HTTP client
    let client = Client::new();

    // Establish WebSocket connections for 3 clients
    let ws_url = format!("ws://{}/echo", app.address);

    let (res, mut ws) = awc::Client::new()
        .ws(ws_url)
        .connect()
        .await
        .unwrap();
    
    // Act - Health Check for each client


    // Each client sends their name in a loop via WebSocket


    // Give some time for responses
    sleep(Duration::from_millis(100)).await;

    // Assert - Health check response should return 200

    // Assert - Each client should get their own name echoed back

    match ws1_resp {
        WsMessage::Text(text) => assert_eq!(text, "client1"),
        _ => panic!("WS1 did not receive correct echo response"),
    }
    // match ws2_resp {
    //     WsMessage::Text(text) => assert_eq!(text, "client2"),
    //     _ => panic!("WS2 did not receive correct echo response"),
    // }
    // match ws3_resp {
    //     WsMessage::Text(text) => assert_eq!(text, "client3"),
    //     _ => panic!("WS3 did not receive correct echo response"),
    // }
}
