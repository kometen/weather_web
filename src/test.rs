use crate::controller;
use actix_web::http::header;
use actix_web::{test, App};

#[actix_rt::test]
async fn test_weather_data_ok() {
    let mut app = test::init_service(App::new().service(controller::test_weather_data_post)).await;

    let payload = r#"[{"measurement_time_default":"2021-03-24T20:50:00+01:00","id":228,"data":[{"index":201,"field_description":"relative_humidity","measurement":77.5},{"index":2501,"field_description":"precipitation_intensity","measurement":0.0},{"index":801,"field_description":"road_surface_temperature","measurement":-3.0},{"index":901,"field_description":"wind_speed","measurement":21.24},{"index":1001,"field_description":"wind_direction_bearing","measurement":176.0},{"index":101,"field_description":"air_temperature","measurement":0.2},{"index":301,"field_description":"dew_point_temperature","measurement":-3.3},{"index":1401,"field_description":"minimum_visibility_distance","measurement":9999.0},{"index":5401,"field_description":"road_surface_condition_measurements_extension","measurement":0.82}]},{"measurement_time_default":"2021-03-24T19:00:00+01:00","id":1760,"data":[]}]"#;

    let resp = test::TestRequest::post()
        .uri("/test_weather_data")
        .header(header::CONTENT_TYPE, "application/json")
        .set_payload(payload)
        .send_request(&mut app)
        .await;

    let result = test::read_body(resp).await;

    assert_eq!(result, "id: 228");
}

#[actix_rt::test]
async fn test_weather_stations_ok() {
    let mut app =
        test::init_service(App::new().service(controller::test_weather_stations_post)).await;

    let payload = r#"[{"publication_time":"2021-03-05T11:22:01.628+08:00","id":205,"name":"E6 Rosten","latitude":61.878395,"longitude":9.41545}]"#;

    let resp = test::TestRequest::post()
        .uri("/test_weather_stations")
        .header(header::CONTENT_TYPE, "application/json")
        .set_payload(payload)
        .send_request(&mut app)
        .await;

    let result = test::read_body(resp).await;

    assert_eq!(result, "id: 205, name: E6 Rosten".as_bytes());
}

#[test]
fn my_test() {
    assert_eq!(2, 1 + 1);
}
