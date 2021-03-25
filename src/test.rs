use super::*;
use actix_web::{http, test, web};
use web::Bytes;

/*
#[actix_rt::test]
async fn test_user_ok() {
    let mut app = test::init_service(
        App::new().service(identify)
    ).await;

    let payload = r#"{"username":"Claus","user_id":1967}"#.as_bytes();

    let resp = test::TestRequest::post()
        .uri("/identify")
        .header(header::CONTENT_TYPE, "application/json")
        .set_payload(payload)
        .send_request(&mut app)
        .await;

    let result = test::read_body(resp).await;

    assert_eq!(result, "Welcome Claus, your id is 1967");
}
*/

#[test]
fn my_test() {
    assert_eq!(2, 1 + 1);
}
