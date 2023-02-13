mod config;
mod demo;
mod models;
mod utils;

use anyhow::Result;
use config::Configuration;
use http::HeaderValue;
use models::{ProductCreateModel, ProductDetailsModel, ProductListModel, ProductUpdateModel};
use utils::{bad_request, internal_server_error, method_not_allowed, no_content, not_found, ok};
use demo::{handle_create};
use spin_sdk::{
    http::{Request, Response},
    http_component,
    mysql::{ParameterValue},
};

enum Api {
    Create(String, f32),
    ReadAll,
    ReadById(u64),
    Update(u64, String, f32),
    Delete(u64),
    BadRequest,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
}

#[http_component]
fn crud_with_my_sql(req: Request) -> Result<Response> {
    let cfg = Configuration::new()?;

    match api_from_request(req) {
        Api::BadRequest => bad_request(),
        Api::InternalServerError => internal_server_error(String::from("")),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::Create(name, price) => handle_create(&cfg.mysql_address, name, price),
        Api::Update(id, name, price) => handle_update(&cfg.mysql_address, id, name, price),
        Api::ReadAll => handle_read_all(&cfg.mysql_address),
        Api::ReadById(id) => handle_read_by_id(&cfg.mysql_address, id),
        Api::Delete(id) => handle_delete_by_id(&cfg.mysql_address, id),
        _ => not_found(),
    }
}

fn get_id_from_route(header_value: &HeaderValue) -> Result<Option<u64>, ()> {
    match header_value.to_str() {
        Ok(value) => {
            let segment = value.split('/').last();
            match segment {
                Some("") => Ok(None),
                Some(id_as_str) => match id_as_str.parse::<u64>() {
                    Ok(id) => Ok(Some(id)),
                    Err(_) => Err(()),
                },
                _ => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

fn api_from_request(req: Request) -> Api {
    match *req.method() {
        http::Method::POST => {
            match ProductCreateModel::from_bytes(&req.body().clone().unwrap_or_default()) {
                Ok(model) => Api::Create(model.name, model.price),
                Err(_) => Api::BadRequest,
            }
        }
        http::Method::GET => match req.headers().get("spin-path-info") {
            None => Api::InternalServerError,
            Some(v) => match get_id_from_route(v) {
                Ok(Some(id)) => Api::ReadById(id),
                Ok(None) => Api::ReadAll,
                Err(()) => Api::NotFound,
            },
        },
        http::Method::PUT => match req.headers().get("spin-path-info") {
            None => Api::InternalServerError,
            // 🤔 can we join those two matches somehow?
            Some(v) => match get_id_from_route(v) {
                Ok(Some(id)) => {
                    match ProductUpdateModel::from_bytes(&req.body().clone().unwrap_or_default()) {
                        Ok(model) => Api::Update(id, model.name, model.price),
                        Err(_) => Api::BadRequest,
                    }
                }
                Ok(None) => Api::NotFound,
                Err(()) => Api::NotFound,
            },
        },
        http::Method::DELETE => match req.headers().get("spin-path-info") {
            None => Api::InternalServerError,
            Some(v) => match get_id_from_route(v) {
                Ok(Some(id)) => Api::Delete(id),
                Ok(None) => Api::NotFound,
                Err(()) => Api::NotFound,
            },
        },
        _ => Api::MethodNotAllowed,
    }
}



fn handle_read_by_id(adr: &str, id: u64) -> Result<Response> {
    let statement = "SELECT Name, Price FROM Products WHERE Id=?";
    let params = vec![ParameterValue::Uint64(id)];

    let rowset = spin_sdk::mysql::query(adr, statement, &params)?;
    match rowset.rows.first() {
        Some(row) => {
            let product = ProductDetailsModel::from_row(id, row)?;
            let payload = serde_json::to_string(&product)?;
            ok(payload)
        }
        None => not_found(),
    }
}

fn handle_read_all(adr: &str) -> Result<Response> {
    let statement = "SELECT Id, Name FROM Products";
    let params = vec![];

    let rowset = spin_sdk::mysql::query(adr, statement, &params)?;
    let mut products = vec![];
    for row in rowset.rows {
        let p = ProductListModel::from_row(&row)?;
        products.push(p)
    }
    let payload = serde_json::to_string(&products)?;

    ok(payload)
}

fn handle_update(adr: &str, id: u64, name: String, price: f32) -> Result<Response> {
    let statement = "UPDATE Products SET Name=?, Price=? WHERE Id=?";
    let params = vec![
        ParameterValue::Str(name.as_str()),
        ParameterValue::Floating32(price),
        ParameterValue::Uint64(id),
    ];
    spin_sdk::mysql::execute(adr, statement, &params)?;
    handle_read_by_id(adr, id)
}

fn handle_delete_by_id(adr: &str, id: u64) -> Result<Response> {
    let statement = "DELETE FROM Products WHERE Id = ?";
    let params = vec![ParameterValue::Uint64(id)];
    match spin_sdk::mysql::execute(adr, statement, &params) {
        Ok(_) => no_content(),
        Err(_) => internal_server_error(String::from("Error while deleting product")),
    }
}
