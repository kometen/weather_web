#[macro_use]
extern crate diesel;

use chrono::{DateTime, Local};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize};
use diesel::prelude::*;
use weather_web::establish_connection;

#[cfg(test)]
mod test;

#[derive(Deserialize)]
struct Reading {
    #[serde(with = "my_date_format")]
    publication_time: DateTime<Local>,
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

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Consume weather data in json-format!")
}

#[get("/weather_data")]
async fn weather_data_get() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/weather_data")]
async fn weather_data_post(weather_measurement: String) -> Result<String> {
    let connection = establish_connection();

    let measurements: Vec<Reading> = serde_json::from_str(&*weather_measurement).unwrap();
/*
    let inserted_measurements = diesel::insert_into(readings)
        .values(&measurements)
        .execute(&connection)
        .unwrap();
    println!("status: {}", inserted_measurements);*/
    for m in &measurements {
        println!("publication_time: {}, id: {}, index: {}, field description: {}, measurement: {}",
                 m.publication_time, m.id, m.index, m.field_description, m.measurement);
    }
    Ok(format!("id: {}, index: {}", &measurements[0].id, &measurements[0].index))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();*/
    HttpServer::new(|| {
        App::new()
            .data(web::PayloadConfig::new(1 << 25))
            .service(hello)
            .service(weather_data_get)
            .service(weather_data_post)
    })
//        .bind_openssl("127.0.0.1:8080", builder)?
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
