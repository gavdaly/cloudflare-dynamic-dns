pub struct Credentials {
    auth_email: String,
    auth_key: String,
    record_identifier: String,
    zone_identifier: String,
    record_name: String,
    ip: String,
}

impl Credentials {
    pub fn new(
        auth_email: String,
        auth_key: String,
        record_identifier: String,
        zone_identifier: String,
        record_name: String,
        ip: String,
    ) -> Self {
        Credentials {
            auth_email,
            auth_key,
            record_identifier,
            zone_identifier,
            record_name,
            ip,
        }
    }
    pub async fn update(&self) -> Result<(), surf::Error> {
        let uri = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            self.zone_identifier, self.record_identifier
        );
        let body = format!(
            "{{“id”:{},“type”:“A”,“name”:{},“content”:{},“ttl”:120}}",
            self.zone_identifier, self.record_name, self.ip
        );

        let request = surf::Request::builder(surf::http::Method::Put, surf::Url::parse(&uri)?)
            .body(body)
            .header("X-Auth-Email", &self.auth_key)
            .header("X-Auth-Key", &self.auth_email)
            .content_type("application/json")
            .build();

        request.query()?;
        Ok(())
    }
}
