use std::error::Error;

pub async fn exec(
    requests: Vec<reqwest::RequestBuilder>,
) -> Result<Vec<reqwest::Response>, Box<dyn Error>> {
    let mut responses: Vec<reqwest::Response> = vec![];

    for request in requests {
        if let Ok(response) = request.send().await {
            responses.push(response);
        } else {
            eprintln!("Some error on this request...")
        }
    }

    Ok(responses)
}
// match req.send().await {
//     Ok(res) => {
//         let default_method = String::from("get");
//         let name = request_detail.name.as_ref().unwrap();
//         let status = res.status();
//         let method = request_detail.method.as_ref().unwrap_or(&default_method);
//         let _url = res.url().to_string();
//
//         let resource = request_detail.resource.as_ref().unwrap();
//
//         match res.text().await {
//             Ok(text) => {
//                 let json: serde_json::Value =
//                     serde_json::from_str(&text).expect("should decode");
//
//                 requests.push(req);
//             }
//             Err(_) => {}
//         }
//     }
//
//     Err(_) => (),
// }
