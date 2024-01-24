use crate::types::Detail;

fn parser(source: &Vec<Detail>, target: &mut Vec<Detail>, parent_index: usize) {
    for request_data in source {
        if request_data.name.is_none() && request_data.requests.is_none() {
            panic!("All requests must have a name or requests.");
        }

        target.push(Detail::new());
        let pos = target.len() - 1;

        target[pos] = Detail {
            requests: None,

            name: match &request_data.name {
                Some(name) => Some(name.clone()),
                None => target[parent_index].name.clone(),
            },

            host: match &request_data.host {
                Some(host) => Some(host.clone()),
                None => target[parent_index].host.clone(),
            },

            scheme: match &request_data.scheme {
                Some(scheme) => Some(scheme.clone()),
                None => target[parent_index].scheme.clone(),
            },

            port: match request_data.port {
                Some(port) => Some(port),
                None => target[parent_index].port,
            },

            timeout: match request_data.timeout {
                Some(timeout) => Some(timeout),
                None => target[parent_index].timeout,
            },

            resource: match &request_data.resource {
                Some(resource) => Some(resource.clone()),
                None => target[parent_index].resource.clone(),
            },

            method: match &request_data.method {
                Some(method) => Some(method.clone()),
                None => target[parent_index].method.clone(),
            },

            hash: match &request_data.hash {
                Some(hash) => Some(hash.clone()),
                None => target[parent_index].hash.clone(),
            },

            params: match &request_data.params {
                Some(params) => Some(params.clone()),
                None => target[parent_index].params.clone(),
            },

            headers: match &request_data.headers {
                Some(headers) => Some(headers.clone()),
                None => target[parent_index].headers.clone(),
            },

            auth: match &request_data.auth {
                Some(auth) => Some(auth.clone()),
                None => target[parent_index].auth.clone(),
            },

            content: match &request_data.content {
                Some(content) => Some(content.clone()),
                None => target[parent_index].content.clone(),
            },

            form: match &request_data.form {
                Some(form) => Some(form.clone()),
                None => target[parent_index].form.clone(),
            },

            body: match &request_data.body {
                Some(body) => Some(body.clone()),
                None => target[parent_index].body.clone(),
            },
        };

        if let Some(requests) = &request_data.requests {
            parser(requests, target, pos);
        }
    }
}

pub fn parse(source: &str, request_names: Vec<String>) -> Vec<Detail> {
    let s = &serde_yaml::from_str(source).expect("Failed to parse config from provided yaml.");

    let mut request_config = vec![];

    parser(s, &mut request_config, 0);

    request_config
        .into_iter()
        .filter(|r| {
            let has_resource = r.resource.is_some();
            let has_name = r.name.is_some();
            let run_all = request_names.is_empty();

            return has_resource
                && has_name
                && (run_all || request_names.contains(r.name.as_ref().unwrap()));
        })
        .collect()
}
