use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    org: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Org {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleListing {
    by: ObjectId,
    title: String,
    price: Price,
    desc: String,
    wher: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Price {
    Free,
    Ask,
    Determined(String),
}
