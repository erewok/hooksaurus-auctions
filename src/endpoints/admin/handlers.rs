use axum::{
    extract::{Extension, Form, Path, Query},
    http::{header::HeaderMap, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use minijinja::context;
use serde::Deserialize;
use tracing::{event, instrument, Level};
use uuid::Uuid;

use crate::db::tables::{self, Table};
use crate::endpoints::admin::{AdminRow, Pagination, ToForm};
use crate::endpoints::ApiContext;
use crate::error::Result;

use super::queries;

pub fn router() -> Router {
    Router::new()
        .route("/admin", get(admin_root))
        .route("/admin/tables", get(list_tables))
        // address
        .route(
            "/admin/tables/:table/:pk",
            get(get_table_record)
                .put(update_table_record)
                .delete(delete_table_record),
        )
        .route(
            "/admin/tables/:table/insert",
            get(get_insert_form).post(insert_table_record),
        )
        .route("/admin/tables/:table", get(list_table_records))
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
            .get_template("fragments/list_all_tables.html")
            .unwrap();
    } else {
        template = ctx
            .template_env
            .get_template("completes/list_all_tables.html")
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

async fn list_table_records(
    headers: HeaderMap,
    ctx: Extension<ApiContext>,
    Path(table): Path<Table>,
    pagination: Option<Query<Pagination>>,
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
    let Query(pagination) = pagination.unwrap_or_default();
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
async fn get_insert_form(
    headers: HeaderMap,
    ctx: Extension<ApiContext>,
    Path(table): Path<Table>,
) -> Html<String> {
    let template;
    if headers.get("hx-request").is_some() {
        template = ctx
            .template_env
            .get_template("fragments/form_insert_modal.html")
            .unwrap();
    } else {
        template = ctx
            .template_env
            .get_template("completes/form_insert_modal.html")
            .unwrap();
    }
    let form = match table {
        Table::Address => tables::address::Address::to_empty_form(),
        Table::Article => todo!(),
        Table::Auction => tables::auction::Auction::to_empty_form(),
        Table::AuctionItem => todo!(),
        Table::AuctionItemBid => todo!(),
        Table::AuctionItemDelivery => todo!(),
        Table::Organization => todo!(),
        Table::User => todo!(),
    };

    let rendered = template
        .render(context!(
            table_name => table.to_string(),
            form => form,
            insert_record_url => format!("/admin/tables/{}/insert", table.to_url_name()),
        ))
        .unwrap();
    Html(rendered)
}

#[derive(Deserialize)]
struct TableDetailParams {
    table: tables::Table,
    pk: Uuid,
}

#[instrument(skip(ctx))]
async fn insert_table_record(
    headers: HeaderMap,
    ctx: Extension<ApiContext>,
    Path(table): Path<Table>,
    form: Form<tables::address::AddressFromForm>,
) -> (StatusCode, Html<String>) {
    let address: tables::address::AddressFromForm = form.0;
    // insert new address
    event!(Level::INFO, event_msg = "Inserting new address", address=?address);
    match queries::insert_address_from_form(address, &ctx.db).await {
        Ok(_) => list_table_records(headers, ctx, Path(table), None).await,
        Err(e) => {
            event!(Level::ERROR, event_msg="Error inserting Address record", err=?e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("An error occurred".to_string()),
            )
        }
    }
    // send back listings again
}

async fn get_table_record(
    headers: HeaderMap,
    ctx: Extension<ApiContext>,
    Path(TableDetailParams { table, pk }): Path<TableDetailParams>,
) -> (StatusCode, Html<String>) {
    let template;
    if headers.get("hx-request").is_some() {
        template = ctx
            .template_env
            .get_template("fragments/form_insert_modal.html")
            .unwrap();
    } else {
        template = ctx
            .template_env
            .get_template("completes/form_insert_modal.html")
            .unwrap();
    }
    match queries::get_table_detail(&table, pk, &ctx.db).await {
        Err(e) => {
            event!(Level::ERROR, event_msg="Error retrieving Address record", err=?e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("An error occurred".to_string()),
            )
        }
        Ok(None) => (StatusCode::NOT_FOUND, Html("".to_string())),
        Ok(Some(form_thing)) => (
            StatusCode::OK,
            Html(
                template
                    .render(context!(
                        action => "Update",
                        table_name => table.to_string(),
                        form => form_thing.to_form(),
                        record_save_url => format!("/admin/tables/{}/{}", table, pk),
                    ))
                    .unwrap(),
            ),
        ),
    }
}

async fn update_table_record(
    Path(TableDetailParams { table, pk }): Path<TableDetailParams>,
    headers: HeaderMap,
    ctx: Extension<ApiContext>,
) -> Html<String> {
    todo!()
}

async fn delete_table_record(
    Path(TableDetailParams { table, pk }): Path<TableDetailParams>,
    headers: HeaderMap,
    ctx: Extension<ApiContext>,
) -> Html<String> {
    todo!()
}
