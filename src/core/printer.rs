use std::collections::HashMap;

use cli_table::{print_stdout, WithTitle};

use crate::Output;

pub fn print(results: HashMap<String, reqwest::Response>) {
    let mut output: Vec<Output> = vec![];

    for (request_name, result) in results {
        let url = result.url().as_str();
        let status = result.status();

        output.push(Output {
            name: request_name,
            url: String::from(url),
            status: String::from(status.as_str()),
        })
    }

    let _ = print_stdout(output.with_title()).is_ok();
}
