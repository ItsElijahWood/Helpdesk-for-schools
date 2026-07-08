use actix_files::Files;
use actix_web::{
    App, HttpResponse, HttpServer, error,
    http::{StatusCode, header::ContentType},
    web,
};
use derive_more::Display;

use crate::integrations::zabbix_ingest;
use crate::public::home::home;
use crate::{integrations::fresh_service_ingest, misc::fs::notify};

#[derive(Debug, Display)]
pub enum ErrorTypes {
    // Requests
    #[display("not found")]
    NotFound,

    #[display("internal server error")]
    InternalServerError,

    #[display("bad request")]
    BadClientData,

    #[display("timeout")]
    Timeout,

    // Askama
    TemplateError(String),
}

impl From<askama::Error> for ErrorTypes {
    fn from(e: askama::Error) -> Self {
        ErrorTypes::TemplateError(e.to_string())
    }
}

impl error::ResponseError for ErrorTypes {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ErrorTypes::NotFound => StatusCode::NOT_FOUND,
            ErrorTypes::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorTypes::BadClientData => StatusCode::BAD_REQUEST,
            ErrorTypes::Timeout => StatusCode::GATEWAY_TIMEOUT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[actix_web::main]
pub async fn app() -> std::io::Result<()> {
    println!("Webserver started on http://localhost:3000");

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/assets", "./assets").show_files_listing())
            // Pages
            .route("/", web::get().to(home))
            // Integrations
            .route(
                "/api/integrations/zabbix",
                web::get().to(zabbix_ingest::fetch),
            )
            .route(
                "/api/integrations/fresh-service",
                web::get().to(fresh_service_ingest::fetch),
            )
            // Misc
            .route("/api/misc/fs/notify", web::get().to(notify::play))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
