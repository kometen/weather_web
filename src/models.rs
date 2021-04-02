use crate::schema::{locations, readings};
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize, Insertable)]
pub struct Reading {
    pub measurement_time_default: DateTime<Local>,
    pub id: i32,
    pub index: i32,
    pub field_description: String,
    pub measurement: f32,
}

#[derive(Deserialize, Insertable)]
pub struct Location {
    pub publication_time: DateTime<Local>,
    pub id: i32,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
}
