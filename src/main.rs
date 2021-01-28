use serde::{Deserialize, Serialize};
use async_std::task;

#[derive(Deserialize, Serialize)]
struct Ip {
    ip: String
}

fn main() -> Result<(), surf::Error> {

    task::block_on(async {

        let uri = "https://api.ipify.org?format=json";
        let Ip { ip } = surf::get(uri).recv_json().await?;
    
        println!("{}", ip);
    
        Ok(())
    })
}
