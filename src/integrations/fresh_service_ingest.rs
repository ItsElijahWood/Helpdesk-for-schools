use actix_web::{HttpResponse, HttpResponseBuilder};
use awc::{Client, http::StatusCode};
use chrono::Utc;
use serde_json::{Map, Value};
use std::sync::LazyLock;
use url::Url;

use base64::{Engine, engine::general_purpose::STANDARD as b64};

use crate::app::ErrorTypes;

static FRESHSERVICE: LazyLock<(String, String, String)> = LazyLock::new(|| {
    (
        std::env::var("FS_TOKEN").expect("failed to fetch FS_TOKEN from .env"),
        std::env::var("FS_URL").expect("failed to fetch FS_TOKEN from .env"),
        std::env::var("FS_WORKSPACE_ID").expect("failed to fetch FS_WORKSPACE_ID from .env"),
    )
});

pub async fn fetch() -> Result<HttpResponse, ErrorTypes> {
    let (fs_token, fs_url, fs_workspace_id) = (&FRESHSERVICE.0, &FRESHSERVICE.1, &FRESHSERVICE.2);

    let base = fs_url.trim_end_matches("/");

    let iso_time: String = Utc::now()
        .to_rfc3339()
        .split("T")
        .next()
        .unwrap_or("")
        .to_string();

    let date: &str = &iso_time;

    let filters = vec![
        format!(r#""tov(status:2 OR status:3 OR status:6 OR status:8 OR status:9 OR status:12)""#), // Total open value
        format!(r#""ov(status:2)""#), // Open value
        format!(r#""toh(status:3 OR status:6 OR status:8 OR status:9 OR status:12)""#), // Total on hold
        format!(r#"ut(agent_id:null)""#), // Unassigned tickets
        format!(r#""ot(status:2 AND created_at:>'{date}')""#), // Open today
        format!(r#""dt(status:2 OR status:9) AND due_by:>'{date}'""#), // Due today
    ];

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .finish();

    let mut fs_results: Vec<serde_json::Value> = Vec::new();

    for i in filters {
        let basic = format!("Basic {}", b64.encode(fs_token));

        let (identifier, query) = &i
            .split_once('(')
            .map(|(identifier, query)| {
                let i = identifier.trim_end_matches(")").trim_end_matches('"');
                let q = format!(r#""({query}"#);

                (i, q)
            })
            .unwrap();

        let mut url = Url::parse(base).unwrap();
        url.query_pairs_mut()
            .clear()
            .append_pair("workspace_id", fs_workspace_id)
            .append_pair("query", query);

        let mut fs_resp = client
            .get(url.as_str())
            .insert_header(("Authorization", basic))
            .insert_header(("Content-Type", "application/json"))
            .send()
            .await
            .map_err(|e| {
                eprintln!("request error: {e}");
                ErrorTypes::InternalServerError
            })?;

        let body = fs_resp.body().await.map_err(|e| {
            eprintln!("body error: {e}");
            ErrorTypes::InternalServerError
        })?;

        let v: serde_json::Value = serde_json::from_slice(&body).unwrap();

        let mut m = Map::new();
        m.insert(identifier.to_string(), v);

        let wrapped = Value::Object(m);

        fs_results.push(wrapped);
    }

    let body = serde_json::to_vec(&fs_results).unwrap();

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .content_type("application/json")
        .body(body))
}
