use axum::{
    extract::{Extension, Path, Query},
    http::{header::HeaderMap, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use minijinja::context;
use tracing::{event, instrument, Level};
use uuid::Uuid;

use crate::db::tables::Table;
use crate::endpoints::admin::{AdminRow, Pagination};
use crate::endpoints::ApiContext;
use crate::error::Result;

use super::queries;

pub fn router() -> Router {
    Router::new()
        .route("/admin", get(admin_root))
        .route("/admin/tables", get(list_tables))
        .route("/admin/tables/address", get(list_address))
        .route(
            "/admin/tables/address/insert",
            get(get_address_insert_form).post(insert_address),
        )
        .route(
            "/admin/tables/address/:pk",
            get(get_address_record)
                .put(update_address_record)
                .delete(delete_address_record),
        )
}

#[instrument(skip(ctx))]
async fn admin_root(ctx: Extension<ApiContext>) -> Html<String> {
    let template = ctx.template_env.get_template("admin.html").unwrap();
    Html(
        template
            .render(context!(title => "Hooksaurus Auctions: Helping Animal Sanctuaries"))
            .unwrap(),
    )
}

#[instrument(skip(ctx))]
async fn list_tables(headers: HeaderMap, ctx: Extension<ApiContext>) -> Html<String> {
    let template;
    if headers.get("hx-request").is_some() {
        template = ctx
            .template_env
            .get_template("fragments/table_list.html")
            .unwrap();
    } else {
        template = ctx
            .template_env
            .get_template("completes/table_list.html")
            .unwrap();
    }
    let table_list: Vec<(String, String)> = Table::get_table_list()
        .iter()
        .map(|t| (t.to_url_name().to_string(), t.to_string()))
        .collect();
    let ctx = context! { table_list };
    let rendered = template
        .render(ctx)
        .map_err(|e| {
            eprintln!("{:?}", e);
            e
        })
        .unwrap();
    Html(rendered)
}

async fn list_address(
    headers: HeaderMap,
    ctx: Extension<ApiContext>,
    pagination: Option<Query<Pagination>>,
) -> (StatusCode, Html<String>) {
    let Query(pagination) = pagination.unwrap_or_default();
    list_table_records(Table::Address, pagination, headers, ctx).await
}

async fn list_table_records(
    table: Table,
    pagination: Pagination,
    headers: HeaderMap,
    ctx: Extension<ApiContext>,
) -> (StatusCode, Html<String>) {
    let template;
    if headers.get("hx-request").is_some() {
        event!(
            Level::INFO,
            event_msg = "Table list records called as fragment"
        );
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
    let next_page: usize = pagination.page + 1;
    let rows_result: Result<Vec<AdminRow>> = match table {
        Table::Address => queries::get_address_admin_rows(&pagination, &ctx.db).await,
        Table::Article => queries::get_article_admin_rows(&pagination, &ctx.db).await,
        Table::Auction => queries::get_auction_admin_rows(&pagination, &ctx.db).await,
        Table::AuctionItem => queries::get_auction_item_admin_rows(&pagination, &ctx.db).await,
        Table::AuctionItemBid => {
            queries::get_auction_item_bid_admin_rows(&pagination, &ctx.db).await
        }
        Table::AuctionItemDelivery => {
            queries::get_auction_item_delivery_admin_rows(&pagination, &ctx.db).await
        }
        Table::Organization => queries::get_organization_admin_rows(&pagination, &ctx.db).await,
        Table::User => queries::get_user_admin_rows(&pagination, &ctx.db).await,
    };
    let rows = rows_result.unwrap_or_else(|_| vec![]);

    let rendered = template
        .render(context!(
            table_url_name => table.to_url_name(),
            table_name => table.to_string(),
            records => rows,
            next_page => next_page
        ))
        .unwrap();
    (StatusCode::OK, Html(rendered))
}

#[instrument(skip(ctx))]
async fn get_address_insert_form(headers: HeaderMap, ctx: Extension<ApiContext>) -> Html<String> {
    let template;
    if headers.get("hx-request").is_some() {
        template = ctx
            .template_env
            .get_template("fragments/address_insert_modal.html")
            .unwrap();
    } else {
        template = ctx
            .template_env
            .get_template("completes/address_insert_modal.html")
            .unwrap();
    }
    let rendered = template.render(context!(false)).unwrap();
    Html(rendered)
}

async fn insert_address() -> Html<String> {
    todo!()
}

async fn get_address_record(Path(pk): Path<Uuid>) -> Html<String> {
    todo!()
}

async fn update_address_record(Path(pk): Path<Uuid>) -> Html<String> {
    todo!()
}

async fn delete_address_record(Path(pk): Path<Uuid>) -> Html<String> {
    todo!()
}
