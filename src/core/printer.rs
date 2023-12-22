use cli_table::{print_stdout, WithTitle};

pub fn print(results: Vec<reqwest::Response>) {
    for result in results.iter() {
        println!("{}", result.status());
    }
}
