use actix_web::{web, App, HttpServer};
//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[cfg(test)]
mod test;

mod controller;

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
            .service(controller::hello)
            .service(controller::weather_data_get)
            .service(controller::weather_data_post)
            .service(controller::weather_stations_get)
            .service(controller::weather_stations_post)
    })
    //        .bind_openssl("127.0.0.1:8080", builder)?
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
