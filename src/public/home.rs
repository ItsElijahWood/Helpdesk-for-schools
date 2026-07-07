use actix_web::{HttpResponse, Result};
use askama::Template;

use crate::app::ErrorTypes;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    title: &'a str,
}

pub async fn home() -> Result<HttpResponse, ErrorTypes> {
    let t = HomeTemplate {
        title: "School Helpdesk",
    };
    let html = t.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
