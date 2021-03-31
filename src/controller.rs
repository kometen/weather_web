use actix_web::{get, post, HttpResponse, Responder, Result};
use diesel::RunQueryDsl;
use models::{Location, Reading};
use weather_web::schema::locations::dsl::locations;
use weather_web::schema::readings::dsl::readings;
use weather_web::{establish_connection, models};

#[get("/")]
pub(crate) async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Consume weather data in json-format!")
}

#[get("/weather_data")]
pub(crate) async fn weather_data_get() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/weather_data")]
pub(crate) async fn weather_data_post(weather_data: String) -> Result<String> {
    let connection = establish_connection();

    let wd: Vec<Reading> = serde_json::from_str(&*weather_data).unwrap();

    let inserted_wd = diesel::insert_into(readings)
        .values(&wd)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
    println!("count: {}", inserted_wd);
    /*    for m in &measurements {
        println!("measurement_time_default: {}, id: {}, index: {}, field description: {}, measurement: {}",
                 m.measurement_time_default, m.id, m.index, m.field_description, m.measurement);
    }*/
    Ok(format!("id: {}, index: {}", &wd[0].id, &wd[0].index))
}

#[get("/weather_stations")]
pub(crate) async fn weather_stations_get() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/weather_stations")]
pub(crate) async fn weather_stations_post(weather_stations: String) -> Result<String> {
    let connection = establish_connection();

    let ws: Vec<Location> = serde_json::from_str(&*weather_stations).unwrap();

    let inserted_ws = diesel::insert_into(locations)
        .values(&ws)
        .on_conflict_do_nothing()
        .execute(&connection)
        .unwrap();
    println!("count: {}", inserted_ws);
    /*    for l in &locations {
        println!("publication_time: {}, id: {}, name: {}, latitude: {}, longitude: {}",
                 l.publication_time, l.id, l.name, l.latitude, l.longitude);
    }*/
    Ok(format!("id: {}, name: {}", &ws[0].id, &ws[0].name))
}
