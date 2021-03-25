use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod test;

#[derive(Serialize, Deserialize)]
struct Info {
    user_id: u32,
    username: String,
}

#[derive(Deserialize)]
struct WeatherMeasurement {
    publication_time: String,
    id: u16,
    index: u16,
    field_description: String,
    measurement: f32,
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
    let measurements: Vec<WeatherMeasurement> = serde_json::from_str(&*weather_measurement).unwrap();
    for m in &measurements {
        println!("id: {}, index: {}, field description: {}", m.id, m.index, m.field_description);
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
