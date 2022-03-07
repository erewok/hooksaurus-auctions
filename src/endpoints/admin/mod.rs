use axum::{
    extract::{Extension, Path},
    http::{Request, Response, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use minijinja::{context, Environment};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

use crate::db::tables::{deserialize_dt, serialize_dt};
use crate::endpoints::ApiContext;

static ALL_TABLES: &[&str] = &[
    "article",
    "auction",
    "auction_item",
    "auction_item_bid",
    "auction_item_delivery",
    "organization",
    "user",
];

pub fn router() -> Router {
    Router::new()
        .route("/admin/tables", get(list_tables))
        .route("/admin/tables/:table", get(list_table_records))
        .route(
            "/admin/tables/:table/:pk",
            get(get_table_record)
                .post(save_table_record)
                .delete(delete_table_record),
        )
}
static TABLE_LIST_TEMP: &str = r#"
<ul class="uk-list uk-list-disc uk-list-primary">
{% for table in table_list %}
    <li><h3><a hx-boost="true" href="/admin/tables/{{ table }}">{{ table|title }}</a><h3></li>
{% endfor %}
</ul>
"#;

async fn list_tables(ctx: Extension<ApiContext>) -> Html<String> {
    let mut env = Environment::new();
    env.add_template("table_list", TABLE_LIST_TEMP).unwrap();
    let tmpl = env.get_template("table_list").unwrap();
    let rendered = tmpl.render(context!(table_list => ALL_TABLES)).unwrap();
    Html(rendered)
}

static TABLE_RECORD_TEMPLATE: &str = r#"
<h1>{{ table_name }} Records </h1>
<table class="uk-table uk-table-justify uk-table-striped">
    <thead>
        <tr>
            <th>Name</th>
            <th>Created</th>
            <th>Modified</th>
        </tr>
    </thead>
    <tbody>
        {% for row in records %}
        <tr>
            <td><a href="/admin/tables/{{ table_name }}/{{ row.pk }}">{{ row.name }}</a></td>
            <td>{{ row.created }}</td>
            <td>{{ row.modified }}</td>
        </tr>
        {% endfor %}
        {% if next_page %}
        <a href="/admin/tables/{{ table_name }}?pg={{ next_page }}" class="uk-button uk-button-primary">Next page</button>
        {% endif %}
    </tbody>

</table>
"#;

#[derive(Debug, serde::Serialize)]
struct Row {
    pub pk: Uuid,
    pub name: String,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    pub created: OffsetDateTime,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    pub modified: OffsetDateTime,
}

async fn list_table_records(Path(table_name): Path<String>) -> (StatusCode, Html<String>) {
    if !ALL_TABLES.contains(&table_name.as_str()) {
        (
            StatusCode::BAD_REQUEST,
            Html("Invalid table name".to_string()),
        )
    } else {
        let mut env = Environment::new();
        env.add_template("table_records_list", TABLE_RECORD_TEMPLATE)
            .unwrap();
        let tmpl = env.get_template("table_records_list").unwrap();
        let rows: Vec<Row> = vec![];
        let next_page: u16 = 1;
        let rendered = tmpl
            .render(context!(
                table_name => table_name,
                records => rows,
                next_page => next_page
            ))
            .unwrap();
        (StatusCode::OK, Html(rendered))
    }
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
