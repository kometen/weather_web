use crate::schema::{locations, readings};
use chrono::{DateTime, Local};
use serde::Deserialize;
use bigdecimal::BigDecimal;

#[derive(Deserialize, Insertable)]
pub struct Reading {
    pub measurement_time_default: DateTime<Local>,
    pub id: i32,
    pub index: i32,
    pub field_description: String,
    pub measurement: BigDecimal,
}

#[derive(Deserialize, Insertable)]
pub struct Location {
    pub publication_time: DateTime<Local>,
    pub id: i32,
    pub name: String,
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}
