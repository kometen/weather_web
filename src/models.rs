use crate::schema::{locations, readings};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
pub struct Reading {
    pub measurement_time_default: DateTime<Local>,
    pub id: i32,
    pub data: serde_json::Value,
}

#[derive(Deserialize, Insertable)]
pub struct Location {
    pub publication_time: DateTime<Local>,
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}
