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

#[tokio::test]
async fn test_http_methods() -> Result<(), Box<dyn Error>> {
    let settings = Settings {
        config_path: PathBuf::from("tests/e2e-config.yml"),
        parallel: 0,
        request_names: vec![
            String::from("http_put"),
            String::from("http_patch"),
            String::from("http_delete"),
            String::from("http_get"),
            String::from("http_default"),
        ],
    };

    start_server(7878).await;

    let mut results = run::go(settings).await?;

    let put = results
        .remove("http_put")
        .expect("should have the put response");

    println!("PUT: {:#?}", put);

    assert_eq!(
        put.status(),
        reqwest::StatusCode::OK,
        "put should return ok"
    );
    assert_eq!(
        put.text().await.expect("should have response text"),
        "put_ok"
    );

    let patch = results
        .remove("http_patch")
        .expect("should have the patch response");

    assert_eq!(
        patch.status(),
        reqwest::StatusCode::OK,
        "patch should return ok"
    );
    assert_eq!(
        patch.text().await.expect("should have response text"),
        "patch_ok"
    );

    let delete = results
        .remove("http_delete")
        .expect("should have the delete response");

    assert_eq!(
        delete.status(),
        reqwest::StatusCode::OK,
        "delete should return ok"
    );
    assert_eq!(
        delete.text().await.expect("should have response text"),
        "delete_ok"
    );

    let get = results
        .remove("http_get")
        .expect("should have the get response");

    assert_eq!(
        get.status(),
        reqwest::StatusCode::OK,
        "get should return ok"
    );
    assert_eq!(
        get.text().await.expect("should have response text"),
        "get_ok"
    );

    let default = results
        .remove("http_default")
        .expect("should have the default (get) response");

    assert_eq!(
        default.status(),
        reqwest::StatusCode::OK,
        "default (get) should return ok",
    );
    assert_eq!(
        default.text().await.expect("should have response text"),
        "get_ok"
    );

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
            .service(
                web::resource("/test_http_methods")
                    .route(web::delete().to(handler_methods))
                    .route(web::put().to(handler_methods))
                    .route(web::patch().to(handler_methods))
                    .route(web::get().to(handler_methods)),
            )
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

async fn handler_methods(req: HttpRequest) -> HttpResponse {
    println!("should be put: {:#?}", req);
    match *req.method() {
        reqwest::Method::GET => HttpResponse::Ok().body("get_ok"),
        reqwest::Method::PUT => HttpResponse::Ok().body("put_ok"),
        reqwest::Method::PATCH => HttpResponse::Ok().body("patch_ok"),
        reqwest::Method::DELETE => HttpResponse::Ok().body("delete_ok"),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
