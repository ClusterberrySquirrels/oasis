use actix_web::test::TestServer;

#[test]
fn test() {
    let mut srv = TestServer::new(|app| app.handler(my_handler));

    let req = srv.get().finish().unwrap();
    let response = srv.execute(req.send()).unwrap();
    assert!(response.status().is_success());
}