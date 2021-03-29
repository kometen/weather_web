use serde::{Deserialize};
use chrono::{DateTime, Local};
use crate::schema::readings;

#[derive(Deserialize, Insertable)]
struct Reading {
    #[serde(with = "my_date_format")]
    measurement_time_default: DateTime<Local>,
    id: i32,
    index: i32,
    field_description: String,
    measurement: f32,
}

// https://serde.rs/custom-date-format.html
mod my_date_format {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    // https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#fn8
    //const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%:z";
    const FORMAT: &'static str = "%+";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Local>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Local>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Local.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
