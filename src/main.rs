use replay::{server, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    server::run().await?;
    Ok(())
}
