use async_std::task;
use std::env;
mod cloudflare;
mod ip;

fn main() -> Result<(), surf::Error> {
    task::block_on(async {
        let ip = ip::get().await?;

        let cf = cloudflare::Credentials::new(
            env::var("AUTH_EMAIL")?,
            env::var("AUTH_KEY")?,
            env::var("RECORD_ID")?,
            env::var("ZONE_IDENTIFIER")?,
            env::var("RECORD_NAME")?,
            ip,
        );
        cf.update().await?;
        Ok(())
    })
}
