#[cfg(test)]
mod test {

    use actix_web::{web, App, HttpResponse, HttpServer};
    use corkscrew::executor;
    use reqwest::StatusCode;
    use std::{collections::HashMap, error::Error, net::TcpListener};

    #[tokio::test]
    async fn test_executor() -> Result<(), Box<dyn Error>> {
        let mut requests = HashMap::<String, reqwest::Request>::new();

        requests.insert(
            String::from("test_http_get"),
            reqwest::Client::new()
                .get("http://localhost:7878/test_http_methods_endpoint")
                .build()
                .expect("should build request"),
        );

        requests.insert(
            String::from("test_http_post"),
            reqwest::Client::new()
                .post("http://localhost:7878/test_http_methods_endpoint")
                .build()
                .expect("should build request"),
        );

        requests.insert(
            String::from("test_http_unknown"),
            reqwest::Client::new()
                .get("http://localhost:2323")
                .build()
                .expect("should build request"),
        );

        start_server(7878).await;

        let responses = executor::exec(requests).await?;

        let get_response = responses
            .get("test_http_get")
            .expect("response should be there");

        assert_eq!(StatusCode::OK, get_response.status(), "status should be ok");

        let post_response = responses
            .get("test_http_post")
            .expect("response should be there");

        assert_eq!(
            StatusCode::OK,
            post_response.status(),
            "status should be ok"
        );

        assert!(
            !responses.contains_key("test_http_unknown"),
            "responses should not contain key for unknown request"
        );

        Ok(())
    }

    async fn start_server(port: u32) {
        let listener =
            TcpListener::bind(format!("localhost:{}", port)).expect("unable to bind to port");

        let server = HttpServer::new(move || {
            App::new().service(
                web::resource("/test_http_methods_endpoint")
                    .route(web::get().to(test_http_get_handler))
                    .route(web::post().to(test_http_post_handler)),
            )
        })
        .listen(listener)
        .expect("must be able to start server")
        .run();

        tokio::spawn(server);
    }

    async fn test_http_get_handler() -> HttpResponse {
        HttpResponse::Ok().body("http_get_ok")
    }

    async fn test_http_post_handler() -> HttpResponse {
        HttpResponse::Ok().body("http_post_ok")
    }
}
