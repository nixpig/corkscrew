use crate::config::Config;
use std::ops::Index;

struct Method {}
impl Index<&'_ str> for Method {
    type Output = reqwest::Method;

    fn index(&self, index: &str) -> &reqwest::Method {
        match index {
            "get" => &reqwest::Method::GET,
            "post" => &reqwest::Method::POST,
            "put" => &reqwest::Method::PUT,
            "patch" => &reqwest::Method::PATCH,
            "delete" => &reqwest::Method::DELETE,
            _ => &reqwest::Method::GET,
        }
    }
}

pub async fn exec(config: &Config) -> Result<(), reqwest::Error> {
    let mut reqs = vec![];

    config.hosts.iter().for_each(|host| {
        host.requests
            .iter()
            .filter(|x| {
                #[allow(clippy::len_zero)] // is_empty() doesn't work on this
                if config.request_names.len() != 0 {
                    config.request_names.contains(&x.name)
                } else {
                    true
                }
            })
            .for_each(|r| {
                let mut url = String::new();

                let method = match &r.method {
                    Some(v) => Method {}[v].clone(),
                    None => Method {}["get"].clone(),
                };

                let scheme = match &host.scheme {
                    Some(v) => v,
                    None => "http",
                };

                url.push_str(scheme);
                url.push_str("://");
                url.push_str(&host.host);

                if let Some(port) = &host.port {
                    url.push_str(&':'.to_string());
                    url.push_str(&port.to_string());
                }

                if let Some(path) = &r.path {
                    url.push_str(path)
                }

                if let Some(hash) = &r.hash {
                    url.push_str(hash);
                }

                if let Some(params) = &r.params {
                    let p: Vec<String> =
                        params.iter().map(|(k, v)| format!("{}={}", k, v)).collect();

                    let mut query = String::from("?");
                    query.push_str(p.join("&").as_str());

                    url.push_str(&query);
                }

                let req = reqwest::Client::new().request(method, &url).send();

                println!("{:?}", url.to_string());

                reqs.push(req);
            })
    });

    for req in reqs {
        let res = req.await?;
        // println!("{:?}", res.url());
        println!(" - {:?}", res.status());
        // let json: serde_json::Value = res.json().await?;
        // println!("{:?}", json);
    }

    Ok(())
}
