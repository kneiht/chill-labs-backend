mod domain;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::serve().await?;

    Ok(())
}