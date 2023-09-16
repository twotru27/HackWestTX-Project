use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password_hash: String,
    major: Option<String>,
    minor: Option<String>,
    classification: Option<String>,
    phone_number: String,
    org: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Org {
    name: String,
    domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    CourseMaterial,
    Electronics,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleListing {
    by: ObjectId,
    category: Category,
    title: String,
    price: Price,
    desc: String,
    wher: String,
    posted_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedSale {
    seller: ObjectId,
    buyer: ObjectId,
    listing: SaleListing,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Price {
    Free,
    Ask,
    Set(String),
}
