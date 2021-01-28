use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Ip {
    pub ip: String,
}

pub async fn get() -> Result<String, surf::Error> {
    let uri = "https://api.ipify.org?format=json";
    let Ip { ip } = surf::get(uri).recv_json().await?;
    Ok(ip)
}
