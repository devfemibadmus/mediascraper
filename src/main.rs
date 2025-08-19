use actix_web::{route, web, get, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use tera::Tera;
use regex::Regex;
use serde::Serialize;

mod platforms;
use platforms::{tiktok::TikTokv2, facebook::Facebook, instagram::Instagram};

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
    error_message: Option<String>,
}

struct Validator;

impl Validator {
    fn validate(url: &str) -> (&'static str, Option<String>) {
        let tiktok_pattern = Regex::new(r"tiktok\.com/.*/").unwrap();
        let instag_pattern = Regex::new(r"instagram\.com/(p|reel|tv)/([A-Za-z0-9_-]+)/?").unwrap();
        let facebook_pattern = Regex::new(r"(facebook\.com/.*/|fb\.watch/.*/)").unwrap();

        if tiktok_pattern.is_match(url) {
            return ("TikTok", Some(url.to_string()));
        }
        if facebook_pattern.is_match(url) {
            return ("Facebook", Some(url.to_string()));
        }
        if let Some(cap) = instag_pattern.captures(url) {
            return ("Instagram", Some(cap[2].to_string()));
        }

        ("Invalid URL", None)
    }
}

#[route("/api/", method = "GET", method = "POST")]
async fn api_handler(
        client: web::Data<reqwest::Client>, 
        form: Option<web::Form<std::collections::HashMap<String, String>>>, 
        query: Option<web::Query<std::collections::HashMap<String, String>>>,
    ) -> impl Responder {
    let form = form.map_or_else(|| std::collections::HashMap::new(), |f| f.into_inner());
    let query = query.map_or_else(|| std::collections::HashMap::new(), |q| q.into_inner());
    let url = form.get("url").or(query.get("url"));
    let cut: bool = form.get("cut")
    .or(query.get("cut"))
    .map(|s| s == "true")
    .unwrap_or(false);


    if url.is_none() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>{
            success: false,
            data: None,
            message: Some("URL is required".to_string()),
            error_message: None,
        });
    }

    let url = url.unwrap();
    let (source, item_id) = Validator::validate(url);

    match source {
        "TikTok" => {
            let mut tiktok = TikTokv2::new(url, cut, client.get_ref().clone());
            let (data, status) = tiktok.get_data().await;
            if status == 200 {
                HttpResponse::Ok().json(ApiResponse { success: true, data: Some(data), message: None, error_message: None })
            } else {
                let code = actix_web::http::StatusCode::from_u16(status as u16).unwrap();
                HttpResponse::build(code)
                    .json(ApiResponse { 
                        success: false, 
                        data: Some(data.clone()), 
                        message: data.get("message").and_then(|v| v.as_str().map(|s| s.to_string())), 
                        error_message: data.get("error_message").and_then(|v| v.as_str().map(|s| s.to_string())) 
                    })
            }
        },
        "Facebook" => {
            let mut fb = Facebook::new(url, cut, client.get_ref().clone());
            let (data, status) = fb.get_data().await;
            if status == 200 {
                HttpResponse::Ok().json(ApiResponse { success: true, data: Some(data), message: None, error_message: None })
            } else {
                let code = actix_web::http::StatusCode::from_u16(status as u16).unwrap();
                HttpResponse::build(code)
                    .json(ApiResponse { 
                        success: false, 
                        data: Some(data.clone()), 
                        message: data.get("message").and_then(|v| v.as_str().map(|s| s.to_string())), 
                        error_message: data.get("error_message").and_then(|v| v.as_str().map(|s| s.to_string())) 
                    })
            }
        },
        "Instagram" => {
            if let Some(id) = item_id {
                let insta = Instagram::new(client.get_ref().clone(), id, cut);
                let (data, status) = insta.get_data().await;
                if status == 200 {
                    HttpResponse::Ok().json(ApiResponse { success: true, data: Some(data), message: None, error_message: None })
                } else {
                    let code = actix_web::http::StatusCode::from_u16(status as u16).unwrap();
                    HttpResponse::build(code)
                        .json(ApiResponse { 
                            success: false, 
                            data: Some(data.clone()), 
                            message: data.get("message").and_then(|v| v.as_str().map(|s| s.to_string())), 
                            error_message: data.get("error_message").and_then(|v| v.as_str().map(|s| s.to_string())) 
                        })
                }
            } else {
                HttpResponse::BadRequest().json(ApiResponse::<()>{
                    success: false,
                    data: None,
                    message: Some("Invalid Instagram video URL".to_string()),
                    error_message: None,
                })
            }
        },
        _ => HttpResponse::BadRequest().json(ApiResponse::<()>{
            success: false,
            data: None,
            message: Some("Unsupported URL".to_string()),
            error_message: None,
        })
    }
}

#[get("/")]
async fn home(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = tmpl.render("home.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = reqwest::Client::new();
    let tera = Tera::new("website/*html").unwrap();

    HttpServer::new(move || {
        let client = client.clone();
        App::new()
            .service(home)
            .service(api_handler)
            .service(Files::new("/static", "website/static").show_files_listing())
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(client))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

