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
            _ => panic!("Invalid method"),
        }
    }
}

pub async fn exec(config: &Config) -> Result<(), reqwest::Error> {
    // let methods = Method {};

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
                let method = Method {}[&r.method].clone();
                let mut url = String::new();

                url.push_str(&host.scheme);
                url.push_str("://");
                url.push_str(&host.host);
                url.push_str(":");
                url.push_str(&host.port.to_string());
                url.push_str(&r.path);

                let req = reqwest::Client::new().request(method, url).send();

                reqs.push(req);
            })
    });

    for req in reqs {
        let res = req.await?;
        println!("{:?}", res.url());
        println!("{:?}", res.status());
        let json: serde_json::Value = res.json().await?;
        println!("{:?}", json);
    }

    Ok(())
}
