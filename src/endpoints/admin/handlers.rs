use axum::{
    extract::{Extension, Path, Query},
    http::{header::HeaderMap, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use minijinja::context;
use uuid::Uuid;

use crate::db::tables::Table;
use crate::endpoints::admin::{AdminRow, Pagination};
use crate::endpoints::ApiContext;

use super::queries;

pub fn router() -> Router {
    Router::new()
        .route("/admin", get(admin_root))
        .route("/admin/tables", get(list_tables))
        .route("/admin/tables/:table", get(list_table_records))
        .route(
            "/admin/tables/:table/:pk",
            get(get_table_record)
                .post(save_table_record)
                .delete(delete_table_record),
        )
}

async fn admin_root(ctx: Extension<ApiContext>) -> Html<String> {
    let template = ctx.template_env.get_template("admin.html").unwrap();
    Html(
        template
            .render(context!(title => "Hooksaurus Auctions: Helping Animal Sanctuaries"))
            .unwrap(),
    )
}

async fn list_tables(ctx: Extension<ApiContext>) -> Html<String> {
    let template = ctx
        .template_env
        .get_template("fragments/table_list.html")
        .unwrap();
    let rendered = template
        .render(context!(table_list => Table::get_table_list()))
        .unwrap();
    Html(rendered)
}

async fn list_table_records(
    pagination: Option<Query<Pagination>>,
    headers: HeaderMap,
    ctx: Extension<ApiContext>,
    Path(table_requested): Path<Option<Table>>,
) -> (StatusCode, Html<String>) {
    let template;
    if headers.get("hx-request").is_some() {
        template = ctx
            .template_env
            .get_template("fragments/table_list_records.html")
            .unwrap();
    } else {
        template = ctx
            .template_env
            .get_template("completes/table_list_records.html")
            .unwrap();
    }
    let Query(pagination) = pagination.unwrap_or_default();
    let next_page: usize = pagination.page + 1;
    let rows: Vec<AdminRow> = match table_requested {
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Html("Invalid table name".to_string()),
            )
        }
        Some(Table::Address) => queries::get_address_admin_rows(&pagination, &ctx.db).await,
        Some(Table::Article) => vec![],
        Some(Table::Auction) => vec![],
        Some(Table::AuctionItem) => vec![],
        Some(Table::AuctionItemBid) => vec![],
        Some(Table::AuctionItemDelivery) => vec![],
        Some(Table::Organization) => vec![],
        Some(Table::User) => vec![],
    };

    let rendered = template
        .render(context!(
            table_name => table_requested.unwrap_or_default().to_string(),
            records => rows,
            next_page => next_page
        ))
        .unwrap();
    (StatusCode::OK, Html(rendered))
}

async fn get_table_record(Path(table): Path<String>, Path(pk): Path<Uuid>) -> Html<String> {
    todo!()
}

async fn save_table_record(Path(table): Path<String>, Path(pk): Path<Uuid>) -> Html<String> {
    todo!()
}

async fn delete_table_record(Path(table): Path<String>, Path(pk): Path<Uuid>) -> Html<String> {
    todo!()
}