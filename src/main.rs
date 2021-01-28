use async_std::task;
mod ip;

fn main() -> Result<(), surf::Error> {
    task::block_on(async {
        let ip = ip::get().await?;

        println!("{}", ip);

        Ok(())
    })
}
