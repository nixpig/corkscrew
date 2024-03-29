#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::{collections::HashMap, error::Error, path::PathBuf};

    use corkscrew::{
        parser::parse,
        settings::Settings,
        types::{AuthType, Detail},
    };

    #[should_panic]
    #[test]
    fn test_invalid_content() {
        let source = "
- df908jwifoqedjklfag
";

        parse(source, vec![]);
    }

    #[test]
    fn test_empty_string() -> Result<(), Box<dyn Error>> {
        let source = "";

        let got = parse(source, vec![]);

        assert_eq!(got, vec![], "should be filtered out");

        Ok(())
    }

    #[test]
    fn test_missing_name() -> Result<(), Box<dyn Error>> {
        let source = "
- resource: /api
  requests: []
";

        let got = parse(source, vec![]);

        assert_eq!(got, vec![], "should be filtered out");

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

        parse(source, settings.request_names);
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

        let got = parse(source, settings.request_names);

        assert_eq!(got, vec![], "should be filtered out");

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

        let got = parse(source, settings.request_names);

        let want = vec![Detail {
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

        assert_eq!(got, want, "should contruct requests config");

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

        let got = parse(source, settings.request_names);

        let want = vec![
            Detail {
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
            Detail {
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
            Detail {
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
            Detail {
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
            got, want,
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

        let got = parse(source, settings.request_names);

        let want = vec![
            Detail {
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
            Detail {
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
            Detail {
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
            got, want,
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

        let got = parse(source, settings.request_names);

        let want = vec![Detail {
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
            got, want,
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

        let got = parse(source, settings.request_names);

        let want = vec![Detail {
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
            got, want,
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

        let got = parse(source, settings.request_names);

        let want = vec![
            Detail {
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
            Detail {
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
            got, want,
            "should parse all configurable request config props"
        );

        Ok(())
    }

    #[test]
    fn test_parse_single_specified_request_name() -> Result<(), Box<dyn Error>> {
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
            - name: test_patch_2
              method: patch
              resource: /api/test_2
              hash: hash_location_2
              params:
                paramA: valueA
                paramB: valueB
            - name: test_get_3
              method: get
              resource: /api/test_3
        ";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec!["test_patch_2".to_string()],
        };

        let got = parse(source, settings.request_names);

        let want = vec![Detail {
            name: Some(String::from("test_patch_2")),
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
        }];

        assert_eq!(got, want, "should only include specified request name");

        Ok(())
    }

    #[test]
    fn test_parse_multiple_specified_request_names() -> Result<(), Box<dyn Error>> {
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
            - name: test_patch_2
              method: patch
              resource: /api/test_2
              hash: hash_location_2
              params:
                paramA: valueA
                paramB: valueB
            - name: test_get_3
              method: get
              resource: /api/test_3
        ";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec!["test_post_1".to_string(), "test_get_3".to_string()],
        };

        let got = parse(source, settings.request_names);

        let want = vec![
            Detail {
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
            Detail {
                name: Some(String::from("test_get_3")),
                method: Some(String::from("get")),
                resource: Some(String::from("/api/test_3")),
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
                hash: None,
                params: None,
            },
        ];

        assert_eq!(got, want, "should only include specified request names");

        Ok(())
    }

    #[test]
    fn test_parse_form_details() -> Result<(), Box<dyn Error>> {
        let source = "
        - name: test_form_details
          host: localhost
          method: post
          resource: /api/test_form
          form:
            key_1: val_1
            key_2: val_2
        ";

        let settings = Settings {
            parallel: 0,                    // <- not used by parser
            config_path: PathBuf::from(""), // <- not used by parser
            request_names: vec![String::from("test_form_details")],
        };

        let got = parse(source, settings.request_names);

        let mut expected_form_data = HashMap::<String, String>::new();
        expected_form_data.insert(String::from("key_1"), String::from("val_1"));
        expected_form_data.insert(String::from("key_2"), String::from("val_2"));

        let want = vec![Detail {
            name: Some(String::from("test_form_details")),
            method: Some(String::from("post")),
            resource: Some(String::from("/api/test_form")),
            hash: None,
            host: Some(String::from("localhost")),
            port: None,
            timeout: None,
            scheme: None,
            requests: None,
            auth: None,
            headers: None,
            params: None,
            content: None,
            form: Some(expected_form_data),
            body: None,
        }];

        assert_eq!(got, want, "should parse the provided form data");

        Ok(())
    }
}
