use std::{error::Error, net::TcpListener, path::PathBuf};

use actix_web::{web, App, HttpResponse, HttpServer};
use corkscrew::{run, Settings};

#[tokio::test]
async fn test_run_e2e() -> Result<(), Box<dyn Error>> {
    let settings = Settings {
        config_path: PathBuf::from("tests/e2e-config.yml"),
        parallel: 0,
        request_names: vec![String::from("request_one"), String::from("request_two")],
    };

    start_server(7878).await;

    let results = run::go(settings).await?;

    println!("{:#?}", results);

    assert_eq!(
        200,
        results
            .get("request_one")
            .expect("should have response")
            .status(),
        "should be successful"
    );

    assert_eq!(
        200,
        results
            .get("request_two")
            .expect("should have response")
            .status(),
        "should be successful"
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
            .service(web::resource("/test_endpoint_one").post(handler_one))
            .service(web::resource("/test_endpoint_two").post(handler_two))
    })
    .listen(listener)
    .expect("unable to start server")
    .run();

    tokio::spawn(server);
}

async fn handler_one() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn handler_two() -> HttpResponse {
    HttpResponse::Ok().finish()
}
