use crate::requests::Requests;
use crate::settings::Settings;
use crate::types::RequestData;

fn parser(src: &Vec<RequestData>, target: &mut Vec<RequestData>, parent_index: usize) {
    for request_data in src {
        if request_data.name.is_none() && request_data.requests.is_none() {
            panic!("All requests must have a name or requests.");
        }

        target.push(RequestData::new());
        let pos = target.len() - 1;

        target[pos].name = match &request_data.name {
            Some(name) => Some(name.clone()),
            None => target[parent_index].name.clone(),
        };

        target[pos].host = match &request_data.host {
            Some(host) => Some(host.clone()),
            None => target[parent_index].host.clone(),
        };

        target[pos].scheme = match &request_data.scheme {
            Some(scheme) => Some(scheme.clone()),
            None => target[parent_index].scheme.clone(),
        };

        target[pos].port = match request_data.port {
            Some(port) => Some(port),
            None => target[parent_index].port,
        };

        target[pos].timeout = match request_data.timeout {
            Some(timeout) => Some(timeout),
            None => target[parent_index].timeout,
        };

        target[pos].resource = match &request_data.resource {
            Some(resource) => Some(resource.clone()),
            None => target[parent_index].resource.clone(),
        };

        target[pos].method = match &request_data.method {
            Some(method) => Some(method.clone()),
            None => target[parent_index].method.clone(),
        };

        target[pos].hash = match &request_data.hash {
            Some(hash) => Some(hash.clone()),
            None => target[parent_index].hash.clone(),
        };

        target[pos].params = match &request_data.params {
            Some(params) => Some(params.clone()),
            None => target[parent_index].params.clone(),
        };

        target[pos].headers = match &request_data.headers {
            Some(headers) => Some(headers.clone()),
            None => target[parent_index].headers.clone(),
        };

        target[pos].auth = match &request_data.auth {
            Some(auth) => Some(auth.clone()),
            None => target[parent_index].auth.clone(),
        };

        target[pos].content = match &request_data.content {
            Some(content) => Some(content.clone()),
            None => target[parent_index].content.clone(),
        };

        target[pos].form = match &request_data.form {
            Some(form) => Some(form.clone()),
            None => target[parent_index].form.clone(),
        };

        target[pos].body = match &request_data.body {
            Some(body) => Some(body.clone()),
            None => target[parent_index].body.clone(),
        };

        if let Some(requests) = &request_data.requests {
            parser(requests, target, pos);
        }
    }
}

pub fn parse(settings: &Settings, source: &str) -> Requests {
    let s = &serde_yaml::from_str(source).expect("Failed to parse config from provided yaml.");

    let mut request_config = vec![];

    parser(s, &mut request_config, 0);

    Requests {
        requests: request_config
            .into_iter()
            .filter(|r| {
                let has_resource = r.resource.is_some();
                let has_name = r.name.is_some();
                let run_all = settings.request_names.is_empty();

                return has_resource
                    && has_name
                    && (run_all || settings.request_names.contains(r.name.as_ref().unwrap()));
            })
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::{collections::HashMap, error::Error, path::PathBuf};

    use crate::{
        parser::parse,
        requests::Requests,
        settings::Settings,
        types::{AuthType, RequestData},
    };

    #[should_panic]
    #[test]
    fn test_invalid_content() {
        let source = "
- df908jwifoqedjklfag
";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        parse(&settings, source);
    }

    #[test]
    fn test_empty_string() -> Result<(), Box<dyn Error>> {
        let source = "";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        let got = parse(&settings, source);

        assert_eq!(got, Requests { requests: vec![] }, "should be filtered out");

        Ok(())
    }

    #[test]
    fn test_missing_name() -> Result<(), Box<dyn Error>> {
        let source = "
- resource: /api
  requests: []
";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        let got = parse(&settings, source);

        assert_eq!(got, Requests { requests: vec![] }, "should be filtered out");

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_missing_name_and_requests() {
        let source = "
- resource: /api
";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        parse(&settings, source);
    }

    #[test]
    fn test_missing_resource() -> Result<(), Box<dyn Error>> {
        let source = "
- name: test
  host: localhost
";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        let got = parse(&settings, source);

        assert_eq!(got, Requests { requests: vec![] }, "should be filtered out");

        Ok(())
    }

    #[test]
    fn test_parse_required_fields() -> Result<(), Box<dyn Error>> {
        let source = "
- name: test_required_fields
  host: localhost
  resource: /api
";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        let got = parse(&settings, source);

        let want = vec![RequestData {
            name: Some(String::from("test_required_fields")),
            host: Some(String::from("localhost")),
            port: None,
            scheme: None,
            timeout: None,
            auth: None,
            body: None,
            hash: None,
            content: None,
            form: None,
            headers: None,
            method: None,
            params: None,
            resource: Some(String::from("/api")),
            requests: None,
        }];

        assert_eq!(got.requests, want, "should contruct requests config");

        Ok(())
    }

    #[test]
    fn test_parse_nested_required_field_expansion_with_root_request() -> Result<(), Box<dyn Error>>
    {
        let source = "
    - name: test_nested_required_fields_0
      host: localhost.1
      resource: /api/0
      requests:
        - name: test_nested_required_fields_1
          resource: /api/1
        - name: test_nested_required_fields_2
          host: localhost.2
          resource: /api/2
          requests:
            - name: test_nested_required_fields_3
              host: localhost.3
              resource: /api/3
    ";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        let got = parse(&settings, source);

        let want = vec![
            RequestData {
                name: Some(String::from("test_nested_required_fields_0")),
                host: Some(String::from("localhost.1")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                content: None,
                form: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/0")),
                requests: None,
            },
            RequestData {
                name: Some(String::from("test_nested_required_fields_1")),
                host: Some(String::from("localhost.1")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                content: None,
                form: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/1")),
                requests: None,
            },
            RequestData {
                name: Some(String::from("test_nested_required_fields_2")),
                host: Some(String::from("localhost.2")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                content: None,
                form: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/2")),
                requests: None,
            },
            RequestData {
                name: Some(String::from("test_nested_required_fields_3")),
                host: Some(String::from("localhost.3")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                content: None,
                form: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/3")),
                requests: None,
            },
        ];

        assert_eq!(
            got.requests, want,
            "should expand nested request configs to root level and merge with parent properties"
        );

        Ok(())
    }

    #[test]
    fn test_parse_nested_required_field_expansion_without_root_request(
    ) -> Result<(), Box<dyn Error>> {
        let source = "
    - name: test_nested_required_fields_0
      host: localhost.1
      requests:
        - name: test_nested_required_fields_1
          resource: /api/1
        - name: test_nested_required_fields_2
          host: localhost.2
          resource: /api/2
          requests:
            - name: test_nested_required_fields_3
              host: localhost.3
              resource: /api/3
    ";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        let got = parse(&settings, source);

        let want = vec![
            RequestData {
                name: Some(String::from("test_nested_required_fields_1")),
                host: Some(String::from("localhost.1")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                content: None,
                form: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/1")),
                requests: None,
            },
            RequestData {
                name: Some(String::from("test_nested_required_fields_2")),
                host: Some(String::from("localhost.2")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                content: None,
                form: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/2")),
                requests: None,
            },
            RequestData {
                name: Some(String::from("test_nested_required_fields_3")),
                host: Some(String::from("localhost.3")),
                port: None,
                scheme: None,
                timeout: None,
                auth: None,
                content: None,
                form: None,
                body: None,
                hash: None,
                headers: None,
                method: None,
                params: None,
                resource: Some(String::from("/api/3")),
                requests: None,
            },
        ];

        assert_eq!(
            got.requests, want,
            "should expand nested request configs and merge with parents; root should be excluded"
        );

        Ok(())
    }
    #[test]
    fn test_parse_auth_basic() -> Result<(), Box<dyn Error>> {
        let source = "
    - name: test_basic_auth
      host: localhost
      resource: /api
      auth: !basic
        username: test_username
        password: test_password
    ";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        let got = parse(&settings, source);

        let want = vec![RequestData {
            name: Some(String::from("test_basic_auth")),
            host: Some(String::from("localhost")),
            port: None,
            timeout: None,
            scheme: None,
            auth: Some(AuthType::Basic {
                username: String::from("test_username"),
                password: String::from("test_password"),
            }),
            content: None,
            form: None,
            body: None,
            hash: None,
            headers: None,
            method: None,
            params: None,
            resource: Some(String::from("/api")),
            requests: None,
        }];

        assert_eq!(
            got.requests, want,
            "should parse basic auth enum and required username and password props"
        );

        Ok(())
    }

    #[test]
    fn test_parse_bearer_auth() -> Result<(), Box<dyn Error>> {
        let source = "
    - name: test_bearer_auth
      host: localhost
      resource: /api
      auth: !bearer
        token: 50m3.b34r3r.t0k?n
    ";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        let got = parse(&settings, source);

        let want = vec![RequestData {
            name: Some(String::from("test_bearer_auth")),
            host: Some(String::from("localhost")),
            port: None,
            timeout: None,
            scheme: None,
            auth: Some(AuthType::Bearer {
                token: String::from("50m3.b34r3r.t0k?n"),
            }),
            content: None,
            form: None,
            body: None,
            hash: None,
            headers: None,
            method: None,
            params: None,
            resource: Some(String::from("/api")),
            requests: None,
        }];

        assert_eq!(
            got.requests, want,
            "should parse required bearer enum and required token prop"
        );

        Ok(())
    }

    #[test]
    fn test_parse_all_unconstrained_fields() -> Result<(), Box<dyn Error>> {
        let source = "
        - name: test_unconstrained_fields
          host: localhost
          timeout: 15
          port: 3000
          scheme: https
          requests:
            - name: test_post_1
              method: post
              resource: /api/test_1
              hash: hash_location_1
              headers:
                Accept-Language: en-US,en;q=0.8
                User-Agent: Mozilla/5.0 Firefox/50.0
              params:
                param1: value1
                param2: value2
              content: json
              body:
                prop1a: val1a
                prop1b:
                  prop2a: val2a
                  prop2b:
                    prop3a: val3a
                    prop3b: val3b
            - name: test_post_2
              method: patch
              resource: /api/test_2
              hash: hash_location_2
              params:
                paramA: valueA
                paramB: valueB

        ";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![],
        };

        let got = parse(&settings, source);

        let want = vec![
            RequestData {
                name: Some(String::from("test_post_1")),
                method: Some(String::from("post")),
                resource: Some(String::from("/api/test_1")),
                hash: Some(String::from("hash_location_1")),
                host: Some(String::from("localhost")),
                port: Some(3000),
                timeout: Some(15),
                scheme: Some(String::from("https")),
                requests: None,
                auth: None,
                headers: Some(HashMap::from([
                    (
                        String::from("Accept-Language"),
                        String::from("en-US,en;q=0.8"),
                    ),
                    (
                        String::from("User-Agent"),
                        String::from("Mozilla/5.0 Firefox/50.0"),
                    ),
                ])),
                params: Some(HashMap::from([
                    (String::from("param1"), String::from("value1")),
                    (String::from("param2"), String::from("value2")),
                ])),
                content: Some(String::from("json")),
                form: None,
                body: Some(
                    serde_json::from_str(
                        "{
                                    \"prop1a\": \"val1a\",
                                    \"prop1b\": {
                                        \"prop2a\": \"val2a\",
                                        \"prop2b\": {
                                            \"prop3a\": \"val3a\",
                                            \"prop3b\": \"val3b\"
                                        }
                                    }
                                }",
                    )
                    .unwrap(),
                ),
            },
            RequestData {
                name: Some(String::from("test_post_2")),
                method: Some(String::from("patch")),
                resource: Some(String::from("/api/test_2")),
                hash: Some(String::from("hash_location_2")),
                host: Some(String::from("localhost")),
                port: Some(3000),
                timeout: Some(15),
                scheme: Some(String::from("https")),
                requests: None,
                auth: None,
                content: None,
                form: None,
                body: None,
                headers: None,
                params: Some(HashMap::from([
                    (String::from("paramA"), String::from("valueA")),
                    (String::from("paramB"), String::from("valueB")),
                ])),
            },
        ];

        assert_eq!(
            got.requests, want,
            "should parse all configurable request config props"
        );

        Ok(())
    }
}
