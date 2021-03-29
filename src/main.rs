#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use weather_web::establish_connection;
use crate::schema::readings::dsl::readings;
use crate::schema::locations::dsl::locations;
use diesel::RunQueryDsl;

#[cfg(test)]
mod test;

mod models;
mod schema;

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

    let measurements: Vec<models::Reading> = serde_json::from_str(&*weather_measurement).unwrap();

    let inserted_measurements = diesel::insert_into(readings)
        .values(&measurements)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
    println!("count: {}", inserted_measurements);
/*    for m in &measurements {
        println!("measurement_time_default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                 m.measurement_time_default, m.id, m.index, m.field_description, m.measurement);
    }*/
    Ok(format!("id: {}, index: {}", &measurements[0].id, &measurements[0].index))
}

#[get("/weather_stations")]
async fn weather_stations_get() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/weather_stations")]
async fn weather_stations_post(weather_stations: String) -> Result<String> {
    let connection = establish_connection();

    let stations: Vec<models::Location> = serde_json::from_str(&*weather_stations).unwrap();

    let inserted_locations = diesel::insert_into(locations)
        .values(&stations)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
    println!("count: {}", inserted_locations);
/*    for l in &locations {
            println!("publication_time: {}, id: {}, name: {}, latitude: {}, longitude: {}",
                     l.publication_time, l.id, l.name, l.latitude, l.longitude);
        }*/
    Ok(format!("id: {}, name: {}", &stations[0].id, &stations[0].name))
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
            .service(weather_stations_get)
            .service(weather_stations_post)
    })
//        .bind_openssl("127.0.0.1:8080", builder)?
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
