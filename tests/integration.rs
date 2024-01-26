use actix_web::{web, App, HttpResponse, HttpServer};
use corkscrew::Settings;
use serial_test::serial;
use std::{error::Error, net::TcpListener, path::PathBuf};

#[tokio::test]
#[serial]
async fn test_minimal_config() -> Result<(), Box<dyn Error>> {
    let settings = Settings {
        config_path: PathBuf::from("tests/minimal-config.yml"),
        parallel: 0,
        request_names: vec![],
    };

    start_server(7878).await;

    let results = corkscrew::run::go(settings).await?;

    let response_test_minimal_config = results
        .get("test_minimal_config")
        .expect("item for request should be in the results");

    assert_eq!(
        "http://localhost:7878/api",
        response_test_minimal_config.url().to_string(),
        "response should be for provided URL"
    );

    assert_eq!(
        200,
        response_test_minimal_config.status(),
        "should be a success response"
    );

    Ok(())
}

async fn start_server(port: u32) {
    let listener =
        TcpListener::bind(format!("localhost:{}", port)).expect("unable to bind to port");

    let server = HttpServer::new(move || App::new().route("/api", web::get().to(ok_handler)))
        .listen(listener)
        .expect("must be able to start server")
        .run();

    tokio::spawn(server);
}

async fn ok_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}
