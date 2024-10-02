
use actix_web::web::Bytes;
use actix_web::{rt::time::sleep, App, HttpServer};
use actix_ws::{WsClient, Message};
use reqwest::Client;
use std::time::Duration;

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
async fn websocket_echo_succeeds_and_healthcheck_returns_a_200_when_three_clients_invoke_these_endpoints() {
    // Arrange - this should bring up websocket and http APIs
    let app = spawn_app().await;

    // Create an HTTP client
    let client = Client::new();

    // Establish WebSocket connections for 3 clients
    let ws_url = format!("ws://{}/echo", app.address);

    let (mut ws1, _) = WsClient::new().connect(&ws_url).await.expect("Failed to connect WS1");
    let (mut ws2, _) = WsClient::new().connect(&ws_url).await.expect("Failed to connect WS2");
    let (mut ws3, _) = WsClient::new().connect(&ws_url).await.expect("Failed to connect WS3");

    // Act - Health Check for each client
    let health_check_url = format!("{}/health_check", app.address);
    let health_check_res1 = client.get(&health_check_url).send().await.expect("Failed to hit health check");
    let health_check_res2 = client.get(&health_check_url).send().await.expect("Failed to hit health check");
    let health_check_res3 = client.get(&health_check_url).send().await.expect("Failed to hit health check");

    // Each client sends their name in a loop via WebSocket
    ws1.send(WsMessage::Text("client1".into())).await.expect("Failed to send WS message");
    ws2.send(WsMessage::Text("client2".into())).await.expect("Failed to send WS message");
    ws3.send(WsMessage::Text("client3".into())).await.expect("Failed to send WS message");

    // Give some time for responses
    sleep(Duration::from_millis(100)).await;

    // Assert - Health check response should return 200
    assert_eq!(health_check_res1.status().as_u16(), 200);
    assert_eq!(health_check_res2.status().as_u16(), 200);
    assert_eq!(health_check_res3.status().as_u16(), 200);

    // Assert - Each client should get their own name echoed back
    let mut ws1_resp = ws1.next().await.expect("No WS response from client 1").expect("WS1 error");
    let mut ws2_resp = ws2.next().await.expect("No WS response from client 2").expect("WS2 error");
    let mut ws3_resp = ws3.next().await.expect("No WS response from client 3").expect("WS3 error");

    match ws1_resp {
        WsMessage::Text(text) => assert_eq!(text, "client1"),
        _ => panic!("WS1 did not receive correct echo response"),
    }
    match ws2_resp {
        WsMessage::Text(text) => assert_eq!(text, "client2"),
        _ => panic!("WS2 did not receive correct echo response"),
    }
    match ws3_resp {
        WsMessage::Text(text) => assert_eq!(text, "client3"),
        _ => panic!("WS3 did not receive correct echo response"),
    }
}
