
use uuid::Uuid;
use wiremock::MockServer;
use blokus_elemental::{
    configuration::{
        get_configuration,
    },
    startup::{
        Application,
    },
};

// Ensure that the `tracing` stack is only initialised once using `once_cell`

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

impl TestApp {
    // pub async fn send_websocket_message(&self, body: String) -> reqwest::Response {
    //     // reqwest::Client::new()
    //     //     .post(&format!("{}/echo", &self.address))
    //     //     .header("Content-Type", "application/x-www-form-urlencoded")
    //     //     .body(body)
    //     //     .send()
    //     //     .await
    //     //     .expect("Failed to execute request.")
    // }
}


pub async fn spawn_app() -> TestApp {
    

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration."); 
        // Use a random OS port
        c.application.port = 0;
        // randomise email server url
        c
    };

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    let application_port = application.port();

    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
    }
}
