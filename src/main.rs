//! Whiskers Email Templating Service
//! 
#![warn(missing_docs)]

use env_logger;
use log::info;
use actix_web::middleware::Logger;
use actix_web::{post, HttpRequest, HttpResponse, HttpServer, App, Responder, web};
use std::fs;
use mustache::Template;
use serde::{Serialize, Deserialize};

mod common;
mod model;

use common::config::Config;
use common::error::WhiskerError;

use std::io;

#[derive(Serialize, Deserialize)]
struct EmailSchema {
    title: String,
    name: String,
}

#[post("/api/generate/email")]
async fn generate_email(
    body: web::Json<EmailSchema>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let html_file = fs::read_to_string("template.html").expect("Could not open template");
    
    let template = mustache::compile_str(html_file.as_str()).unwrap();

    let api_data = body.into_inner();
    
    let html = template.render_to_string(&api_data).unwrap();

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");

    env_logger::init();

    let _config = Config::new();

    info!("Starting {pkg} {ver}");
    
    HttpServer::new(move || {
        App::new()
            .service(generate_email)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0",8000))?
    .run()
    .await
}
