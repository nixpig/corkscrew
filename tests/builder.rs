#[cfg(test)]
mod test {
    use itertools::Itertools;
    use std::{collections::HashMap, error::Error, time::Duration};

    use corkscrew::{builder, AuthType, Detail};

    #[test]
    fn test_build_required_props() -> Result<(), Box<dyn Error>> {
        let details = vec![Detail {
            name: Some(String::from("test_required_props")),
            requests: None,
            host: Some(String::from("localhost")),
            scheme: None,
            port: None,
            timeout: None,
            resource: Some(String::from("/test_required_props_resource")),
            method: None,
            hash: None,
            params: None,
            headers: None,
            auth: None,
            content: None,
            body: None,
            form: None,
        }];

        let requests = builder::build(details).expect("should build requests");

        let r1 = requests
            .get("test_required_props")
            .expect("should have request");

        assert_eq!(
            String::from("http://localhost/test_required_props_resource"),
            r1.url().to_string(),
            "url should be constructed correctly"
        );
        assert_eq!(
            &Duration::from_secs(10),
            r1.timeout().expect("timeout should be present"),
            "timeout should be correct"
        );
        assert_eq!(
            reqwest::Method::GET,
            r1.method(),
            "method should be correct"
        );

        Ok(())
    }

    #[test]
    fn test_build_optional_props() -> Result<(), Box<dyn Error>> {
        let details = vec![
            Detail {
                name: Some(String::from("test_form_post")),
                host: Some(String::from("localhost")),
                requests: None,
                scheme: Some(String::from("https")),
                port: Some(7878),
                timeout: Some(30),
                resource: Some(String::from("/test_form_post_resource")),
                method: Some(String::from("post")),
                hash: Some(String::from("fragment")),
                params: Some(HashMap::from([
                    (
                        String::from("param_one_name"),
                        String::from("param_one_value"),
                    ),
                    (
                        String::from("param_two_name"),
                        String::from("param_two_value"),
                    ),
                ])),
                headers: Some(HashMap::from([(
                    String::from("x-test-header"),
                    String::from("test header value"),
                )])),
                auth: Some(AuthType::Basic {
                    username: String::from("name"),
                    password: String::from("p4ssw0rd"),
                }),
                content: None,
                body: None,
                form: Some(HashMap::from([
                    (String::from("field_one"), String::from("value one")),
                    (String::from("field_two"), String::from("value two")),
                ])),
            },
            Detail {
                name: Some(String::from("test_json_post")),
                host: Some(String::from("localhost")),
                requests: None,
                scheme: None,
                port: None,
                timeout: None,
                resource: Some(String::from("/test_json_post_resource")),
                method: Some(String::from("post")),
                hash: None,
                params: None,
                headers: None,
                auth: Some(AuthType::Bearer {
                    token: String::from("abcd1234"),
                }),
                content: None,
                body: Some(serde_json::from_str("{\"foo\": \"bar\"}").expect("should parse json")),
                form: None,
            },
        ];

        let requests = builder::build(details).expect("should build requests");

        let r1 = requests
            .get("test_form_post")
            .expect("should have expected request");

        assert_eq!(
            "fragment",
            r1.url().fragment().expect("should have fragment"),
            "fragment hash should match"
        );

        assert_eq!(
            "localhost",
            r1.url().host().expect("should have host").to_string(),
            "hostname should match"
        );

        assert_eq!(
            7878,
            r1.url().port().expect("should have port"),
            "port should match"
        );

        assert_eq!(
            "/test_form_post_resource",
            r1.url().path(),
            "should have path"
        );

        let exp = HashMap::from([
            (
                String::from("param_one_name"),
                String::from("param_one_value"),
            ),
            (
                String::from("param_two_name"),
                String::from("param_two_value"),
            ),
        ]);

        let e = exp.iter().sorted();

        let got: HashMap<String, String> = r1
            .url()
            .query_pairs()
            .map(|(k, v)| (k.to_string().clone(), v.to_string().clone()))
            .collect();
        let g = got.iter().sorted();

        assert!(g.eq(e), "query params should match");

        assert_eq!("https", r1.url().scheme(), "http scheme should match");

        assert_eq!(
            "Basic bmFtZTpwNHNzdzByZA==",
            r1.headers()
                .get("authorization")
                .expect("should have authorization header")
                .to_str()
                .unwrap(),
            "should have correct credentials"
        );

        let r2 = requests.get("test_json_post").expect("should have request");

        assert_eq!(
            "Bearer abcd1234",
            r2.headers()
                .get("authorization")
                .expect("should have authorization header"),
            "should have correct bearer auth token"
        );

        let body = String::from_utf8(
            r2.body()
                .expect("should have body")
                .as_bytes()
                .unwrap()
                .to_vec(),
        )
        .unwrap();

        assert_eq!(
            String::from("{\"foo\":\"bar\"}"),
            body,
            "body content should match"
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_panic_on_missing_name() {
        let details = vec![Detail::new()];

        let _ = builder::build(details);
    }
}
