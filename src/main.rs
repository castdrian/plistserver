use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde::{Deserialize, Serialize};
use env_logger::Env;
use log::info;

#[derive(Deserialize)]
struct PlistQuery {
    bundleid: String,
    name: String,
    version: String,
    fetchurl: String,
}

#[derive(Serialize)]
struct Asset {
    kind: String,
    url: String,
}

#[derive(Serialize)]
struct Metadata {
    #[serde(rename = "bundle-identifier")]
    bundle_identifier: String,
    #[serde(rename = "bundle-version")]
    bundle_version: String,
    kind: String,
    title: String,
}

#[derive(Serialize)]
struct PlistItem {
    assets: Vec<Asset>,
    metadata: Metadata,
}

#[derive(Serialize)]
struct PlistRoot {
    items: Vec<PlistItem>,
}

async fn generate_plist(query: web::Query<PlistQuery>) -> impl Responder {
    let plist = PlistRoot {
        items: vec![PlistItem {
            assets: vec![Asset {
                kind: "software-package".to_string(),
                url: query.fetchurl.clone(),
            }],
            metadata: Metadata {
                bundle_identifier: query.bundleid.clone(),
                bundle_version: query.version.clone(),
                kind: "software".to_string(),
                title: query.name.clone(),
            },
        }],
    };

    let mut buf = Vec::new();
    plist::to_writer_xml(&mut buf, &plist).unwrap();
    let plist_xml = String::from_utf8(buf).unwrap();

    HttpResponse::Ok()
        .content_type("application/x-plist")
        .body(plist_xml)
}

async fn status() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "running",
        "endpoints": {
            "GET /": "Server status and documentation",
            "GET /genPlist": "Generate iOS installation manifest (query params: bundleid, name, version, fetchurl)"
        }
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting server at http://0.0.0.0:3788");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(status))
            .route("/genPlist", web::get().to(generate_plist))
    })
    .bind("0.0.0.0:3788")?
    .run()
    .await
}
