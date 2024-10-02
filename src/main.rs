//! main.rs
//! main.rs
use blokus_elemental::{
    configuration::get_configuration,
    startup::Application,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application
        .run_until_stopped()
        .await?;
    Ok(())
}