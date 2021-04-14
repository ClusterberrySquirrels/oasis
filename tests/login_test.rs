use std::cell::RefMut;
use actix_web::http::{HeaderValue, header};
use actix_web::cookie::Cookie;
use actix_web::test::TestServer;
use std::time::Duration;
use actix_web::HttpMessage;

#[actix_rt::test]
async fn test() {
    create_user();

    let srv = server_test();

    let (crsf_token, request_cookie) = login(srv.borrow_mut()).await;

    fn create_user() -> User {
        use diesel::RunQueryDsl;
        use ::mystore_lib::schema::users;
        use chrono::Local;

        let connection = establish_connection();
        let pg_pool = connection.get().unwrap();

        diesel::delete(users::table).execute(&pg_pool).unwrap();

        diesel::insert_into(users::table)
            .values(NewUser {
                email: "jhon@doe.com".to_string(),
                password: User::hash_password("12345678".to_string()).unwrap(),
                created_at: Local::now().naive_local()
            })
            .get_result::<User>(&pg_pool).unwrap()
    }

    async fn login(srv: RefMut<'_, TestServer>) -> (HeaderValue, Cookie<'_>) {
        let request = srv
            .post("/login")
            .header(header::CONTENT_TYPE, "application/json")
            .timeout(std_duration::from_secs(600));
        let response =
            request
                .send_body(r#"{"email":"jhon@doe.com","password":"12345678"}"#)
                .await
                .unwrap();

        let csrf_token = response.headers().get("x-csrf-token").unwrap();
        let cookies = response.cookies().unwrap();
        let cookie = cookies[0].clone().into_owned().value().to_string();

        let request_cookie = Cookie::build("mystorejwt", cookie)
            .domain("localhost")
            .path("/")
            // .max_age(Duration::days(1).num_seconds())
            .secure(false)
            .http_only(false)
            .finish();
        (csrf_token.clone(), request_cookie.clone())
    }
}