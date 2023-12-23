use cli_table::{print_stdout, WithTitle};

use crate::Output;

pub fn print(results: Vec<reqwest::Response>) {
    let mut output: Vec<Output> = vec![];

    for (i, result) in results.iter().enumerate() {
        let url = result.url().as_str();
        let status = result.status();

        output.push(Output {
            num: i,
            url: String::from(url),
            status: String::from(status.as_str()),
        })
    }

    let _ = print_stdout(output.with_title()).is_ok();
}
