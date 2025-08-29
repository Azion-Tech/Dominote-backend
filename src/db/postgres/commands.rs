use postgres::Row;
use crate::db::postgres::connection::get_db_client;

/// Fetches all rows from a specified table and columns.
/// # Arguments
/// * `table` - The name of the table to query.
/// * `cols` - A vector of column names to select, if you want to get all data, only place ["*"] as parameter
/// example: 
/// ```
/// // This shows all columns from the table
/// show_table("players", vec!["*"])
/// 
/// // Also you can specify the columns you want to fetch
/// show_table("players", vec!["name", "id"])
/// 
/// ```
pub async  fn show_table(table: &str, cols: Vec<&str>) -> Result<Vec<Row>, tokio_postgres::Error> {
    let columns = cols.join(", ");
    let client = get_db_client().await?;
    return client.query(&format!("SELECT {} FROM {}", columns, table), &[]).await
}