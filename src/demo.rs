use crate::utils::internal_server_error;
use anyhow::Result;
use spin_sdk::{http::Response, mysql::{Decode, ParameterValue}};


pub fn handle_create(adr: &str, name: String, price: f32) -> Result<Response> {
    let statement = "INSERT INTO Products (Name, Price) VALUES (?, ?)";
    let params = vec![
        ParameterValue::Str(name.as_str()),
        ParameterValue::Floating32(price),
    ];

    spin_sdk::mysql::execute(adr, statement, &params)?;

    let rowset = spin_sdk::mysql::query(adr, "SELECT LAST_INSERT_ID()", &[])?;
    match rowset.rows.first() {
        Some(row) => {
            let id = u64::decode(&row[0])?;
            Ok(http::Response::builder()
                .status(http::StatusCode::CREATED)
                .header(http::header::LOCATION, format!("/{}", id))
                .body(None)?)
        }
        None => internal_server_error(String::from("Could not persist product")),
    }
}
