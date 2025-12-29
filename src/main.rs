use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, route, web};
use regex::Regex;
use std::collections::HashMap;
use tera::Tera;

mod platforms;
use platforms::{
    facebook::Facebook, instagram::Instagram, snapchat::Snapchat, tiktok::TikTok, twitter::Twitter,
};

struct Validator;

impl Validator {
    fn validate(url: &str) -> &'static str {
        let patterns = [
            (r"tiktok\.com/.*/", "TikTok"),
            (
                r"instagram\.com/(p|reel|reels|tv)/([A-Za-z0-9_-]+)/?",
                "Instagram",
            ),
            (r"(facebook\.com/.*/|fb\.watch/.*/)", "Facebook"),
            (r"snapchat\.com/t/", "Snapchat"),
            (r"(twitter\.com/|x\.com/).*/status/", "Twitter"),
        ];

        for (pattern, platform) in patterns.iter() {
            let re = Regex::new(pattern).unwrap();
            if re.is_match(url) {
                return platform;
            }
        }

        "Invalid URL"
    }
}

#[route("/api/", method = "GET", method = "POST")]
async fn api_handler(
    client: web::Data<reqwest::Client>,
    body: web::Bytes,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let json: HashMap<String, serde_json::Value> =
        serde_json::from_slice(&body).unwrap_or_default();
    let url = json
        .get("url")
        .and_then(|v| v.as_str())
        .or_else(|| query.get("url").map(|s| s.as_str()));

    if url.is_none() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": true,
            "message": "URL is required",
            "error_message": "URL is required"
        }));
    }

    let url = url.unwrap();
    let platform = Validator::validate(url);

    match platform {
        "Facebook" => {
            let mut fb = Facebook::new(
                client.get_ref().clone(),
                &url.replace("web.facebook", "www.facebook"),
            );
            fb.get_data().await
        }
        "Instagram" => {
            let insta = Instagram::new(client.get_ref().clone(), url.to_string());
            insta.get_data().await
        }
        "TikTok" => {
            let tiktok = TikTok::new(client.get_ref().clone(), url.to_string());
            tiktok.get_data().await
        }
        "Snapchat" => {
            let snap = Snapchat::new(client.get_ref().clone(), url.to_string());
            snap.get_data().await
        }
        "Twitter" => {
            let twitter = Twitter::new(client.get_ref().clone(), url.to_string());
            twitter.get_data().await
        }
        _ => HttpResponse::BadRequest().json(serde_json::json!({
            "error": true,
            "message": "Unsupported URL",
            "error_message": "Unsupported URL"
        })),
    }
}

#[get("/")]
async fn home(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = tera::Context::new();
    match tmpl.render("home.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Template error"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = reqwest::Client::new();
    let tera = match Tera::new("website/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Template error: {}", e);
            return Ok(());
        }
    };

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(home)
            .service(api_handler)
            .service(Files::new("/static", "website/static").show_files_listing())
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(client.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
