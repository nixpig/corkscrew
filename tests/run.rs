use std::{error::Error, net::TcpListener, path::PathBuf};

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use corkscrew::{run, Settings};

#[derive(serde::Deserialize)]
struct TestUrlParams {
    param_one: String,
    param_two: String,
}

#[derive(PartialEq, Eq, Debug, serde::Deserialize)]
struct TestJsonL1 {
    field_one: String,
    field_two_obj: TestJsonL2,
}

#[derive(PartialEq, Eq, Debug, serde::Deserialize)]
struct TestJsonL2 {
    field_two_a: String,
    field_two_b: String,
    num: i32,
    bool: bool,
    arr: Vec<i32>,
}

#[tokio::test]
async fn test_run_e2e() -> Result<(), Box<dyn Error>> {
    let settings = Settings {
        config_path: PathBuf::from("tests/e2e-config.yml"),
        parallel: 0,
        request_names: vec![String::from("request_one"), String::from("request_two")],
    };

    start_server(7878).await;

    let mut results = run::go(settings).await?;

    let r1 = results.remove("request_one").expect("should have response");
    assert_eq!(200, r1.status(), "should be successful");

    let t1 = r1.text().await?;
    assert_eq!("handler_one_ok", t1);

    let r2 = results.remove("request_two").expect("should have response");
    assert_eq!(200, r2.status(), "should be successful");

    let t2 = r2.text().await?;
    assert_eq!("handler_two_ok", t2);

    Ok(())
}

#[should_panic]
#[tokio::test]
async fn test_panics_on_missing_config_file() {
    let settings = Settings {
        config_path: PathBuf::from("tests/missing.yml"),
        request_names: vec![],
        parallel: 0,
    };

    let _ = run::go(settings).await;
}

async fn start_server(port: u16) {
    let listener =
        TcpListener::bind(format!("localhost:{}", port)).expect("unable to bind to port");

    let server = HttpServer::new(move || {
        App::new()
            .service(web::resource("/test_endpoint_one").post(handler_one))
            .service(web::resource("/test_endpoint_two").post(handler_two))
    })
    .listen(listener)
    .expect("unable to start server")
    .run();

    tokio::spawn(server);
}

async fn handler_one(req: HttpRequest, params: web::Query<TestUrlParams>) -> HttpResponse {
    assert_eq!(req.method(), reqwest::Method::POST);
    assert_eq!(req.path(), "/test_endpoint_one");
    assert_eq!(
        "Basic Y29ya3M6cDRzc3cwcmQ=",
        req.headers()
            .get("authorization")
            .expect("authorization header should be present"),
    );

    assert_eq!("val_one", params.param_one);
    assert_eq!("val_two", params.param_two);

    HttpResponse::Ok().body("handler_one_ok")
}

async fn handler_two(req: HttpRequest, body: web::Json<TestJsonL1>) -> HttpResponse {
    assert_eq!(req.method(), reqwest::Method::POST);
    assert_eq!(req.path(), "/test_endpoint_two");
    assert_eq!(
        "Bearer saoidhfjoiasdjhfw3980ufj029348jfiowejf89weaf",
        req.headers()
            .get("authorization")
            .expect("should have authorization header")
    );

    let tj = TestJsonL1 {
        field_one: String::from("val_one"),
        field_two_obj: TestJsonL2 {
            field_two_a: String::from("val_two_a"),
            field_two_b: String::from("val_two_b"),
            num: 23,
            bool: true,
            arr: vec![1, 2, 3],
        },
    };

    assert_eq!(body.into_inner(), tj);

    HttpResponse::Ok().body("handler_two_ok")
}
