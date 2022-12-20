pub struct Configuration {
    pub mysql_address: String,
}

impl Configuration {
    pub fn new() -> Result<Self, anyhow::Error> {
        let adr = spin_sdk::config::get("mysql_address")?;

        Ok(Configuration { mysql_address: adr })
    }
}
