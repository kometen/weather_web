use chrono::NaiveDateTime;
use crate::schema::readings;

#[derive(Insertable, Identifiable)]
#[primary_key(publication_time)]
#[table_name="readings"]
pub struct NewReading {
    pub publication_time: NaiveDateTime,
    pub id: i32,
    index: i32,
    field_description: String,
    measurement: f32,
}
