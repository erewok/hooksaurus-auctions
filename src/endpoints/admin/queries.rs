use sqlx::PgPool;
use sql_builder::SqlBuilder;

use crate::db::Table;
use crate::endpoints::admin::{Pagination, Row};


pub async fn get_table_rows(table: Table, pagination: Pagination, db: PgPool) -> Vec<Row> {
    let sql = SqlBuilder::select_from(table.to_postgres_name())
        .field("title")
        .field("price")
        .and_where("price > 100")
        .and_where_like_left("title", "Harry Potter")
        .sql()?;
}