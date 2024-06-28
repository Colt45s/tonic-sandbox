use anyhow::Result;

use tonic_sandbox::{app, config, logger};

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::load()?;
    logger::setup();
    app::App::new(&config).await.run().await?;
    Ok(())
}
