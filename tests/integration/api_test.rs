use actix_web::{test, App};
use sqlx::{PgPool, Connection};

#[actix_rt::test]
async fn test_create_user() {
    let pool = PgPool::connect("postgres://user:password@localhost/test_db").await.unwrap();
    let app = test::init_service(App::new().app_data(web::Data::new(pool.clone())).configure(configure_routes)).await;

    let new_user = json!({"email": "test@example.com", "password": "securepassword"});
    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}

#[actix_rt::test]
async fn test_get_user() {
    let pool = PgPool::connect("postgres://user:password@localhost/test_db").await.unwrap();
    let app = test::init_service(App::new().app_data(web::Data::new(pool.clone())).configure(configure_routes)).await;

    let req = test::TestRequest::get()
        .uri("/api/users/1")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}