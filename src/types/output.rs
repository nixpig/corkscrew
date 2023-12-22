#[derive(Debug, cli_table::Table)]
pub struct Output {
    #[table(title = "#")]
    pub num: usize,
    #[table(title = "Name")]
    pub name: String,
    #[table(title = "Method")]
    pub method: String,
    #[table(title = "Resource")]
    pub resource: String,
    #[table(title = "Status")]
    pub status: String,
    // #[table(data = "Data")]
    // pub data: String,
}