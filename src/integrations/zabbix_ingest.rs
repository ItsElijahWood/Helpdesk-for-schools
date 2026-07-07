use std::sync::LazyLock;

use actix_web::{HttpResponse, HttpResponseBuilder, Result};
use awc::{Client, http::StatusCode};
use serde_json::json;

use crate::app::ErrorTypes;

static ZABBIX: LazyLock<(String, String, String)> = LazyLock::new(|| {
    (
        std::env::var("ZABBIX_ENABLED").expect("ZABBIX_ENABLED is not set"),
        std::env::var("ZABBIX_TOKEN").expect("ZABBIX_TOKEN is not set"),
        std::env::var("ZABBIX_URL").expect("ZABBIX_URL is not set"),
    )
});

pub async fn fetch() -> Result<HttpResponse, ErrorTypes> {
    // Checks if zabbix module is enabled
    if &ZABBIX.0 != "true" {
        return Ok(HttpResponseBuilder::new(StatusCode::OK).finish());
    }

    let token = &ZABBIX.1;
    let base_url = &ZABBIX.2;

    let zabbix_url = format!("{}/zabbix/api_jsonrpc.php", base_url);

    println!("{}", zabbix_url);

    let mut zabbix_servers = Client::default()
        .post(&zabbix_url)
        .insert_header(("Content-Type", "application/json"))
        .send_body({
            json!({
                "jsonrpc": "2.0",
                "method": "host.get",
                "params": {
                    "output": ["host", "active_available"]
                },
                "auth": token,
                "id": 1
            })
            .to_string()
        })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            ErrorTypes::InternalServerError
        })?;

    let body = zabbix_servers.body().limit(10_000_000).await.map_err(|e| {
        eprintln!("{}", e);
        ErrorTypes::InternalServerError
    })?;

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .content_type("application/json")
        .body(String::from_utf8_lossy(&body).into_owned()))
}
