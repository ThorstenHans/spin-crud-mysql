use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use spin_sdk::mysql::Decode;

#[derive(Deserialize, Debug)]
pub struct ProductCreateModel {
    pub name: String,
    pub price: f32,
}

impl ProductCreateModel {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        let r: ProductCreateModel = serde_json::from_slice(b)?;
        Ok(r)
    }
}

#[derive(Deserialize, Debug)]
pub struct ProductUpdateModel {
    pub name: String,
    pub price: f32,
}

impl ProductUpdateModel {
    pub(crate) fn from_bytes(b: &Bytes) -> Result<Self> {
        let r: ProductUpdateModel = serde_json::from_slice(b)?;
        Ok(r)
    }
}

#[derive(Serialize, Debug)]
pub struct ProductDetailsModel {
    pub id: u64,
    pub name: String,
    pub price: f32,
}

impl ProductDetailsModel {
    pub fn from_row(id: u64, row: &spin_sdk::mysql::Row) -> Result<Self> {
        let name = String::decode(&row[0])?;
        let price = f32::decode(&row[1])?;

        Ok(ProductDetailsModel { id, name, price })
    }
}

#[derive(Serialize, Debug)]
pub struct ProductListModel {
    pub id: u64,
    pub name: String,
}

impl ProductListModel {
    pub(crate) fn from_row(row: &spin_sdk::mysql::Row) -> Result<Self> {
        let id = u64::decode(&row[0])?;
        let name = String::decode(&row[1])?;
        Ok(ProductListModel { id, name })
    }
}
