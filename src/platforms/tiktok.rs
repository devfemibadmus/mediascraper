use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::{json, Value};
use futures::future::join_all;

pub struct TikTokv2 {
    url: String,
    cut: bool,
    client: Client,
}

impl TikTokv2 {
    pub fn new(url: &str, cut: bool, client: Client) -> Self {
        Self {
            url: url.replace("/photo", "/video"),
            cut,
            client,
        }
    }

    fn headers() -> reqwest::header::HeaderMap {
        use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, REFERER};
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        ));
        headers.insert(REFERER, HeaderValue::from_static("https://www.tiktok.com/"));
        headers
    }

    async fn get(&self, url: &str) -> reqwest::Result<reqwest::Response> {
        self.client.get(url).headers(Self::headers()).send().await
    }

    async fn fetch_json(&mut self) -> Result<Value, String> {
        if self.url.contains("vm.tiktok.com") {
            let resp = self.get(&self.url).await.map_err(|_| "Failed to follow vm.tiktok.com redirect")?;
            self.url = resp.url().to_string();
        }

        let resp = self.get(&self.url).await.map_err(|e| format!("Request error: {}", e))?;
        if resp.status() != 200 {
            return Err(format!("Failed to fetch page content: {}", resp.status()));
        }

        let text = resp.text().await.map_err(|e| format!("Read body failed: {}", e))?;
        let document = Html::parse_document(&text);
        let script_sel = Selector::parse("script#__UNIVERSAL_DATA_FOR_REHYDRATION__").unwrap();
        let script = document.select(&script_sel).next().ok_or("No script tag found")?;
        let script_text = script.text().collect::<Vec<_>>().join("").trim().to_string();
        let json_data: Value = serde_json::from_str(&script_text).map_err(|_| "Invalid JSON in script")?;
        let json_data = json_data.get("__DEFAULT_SCOPE__")
            .and_then(|x| x.get("webapp.video-detail"))
            .cloned()
            .ok_or("No webapp.video-detail in JSON")?;
        Ok(json_data)
    }

    fn err(&self, message: &str, error_message: &str) -> Value {
        json!({
            "error": true,
            "message": message,
            "error_message": error_message,
        })
    }

    pub async fn get_data(&mut self) -> (Value, u16) {
        let data = match self.fetch_json().await {
            Ok(d) => d,
            Err(e) => return (self.err(&e, &e), 502),
        };

        if !self.cut {
            return (data.clone(), 200);
        }

        let item = match data.get("itemInfo").and_then(|x| x.get("itemStruct")) {
            Some(v) => v,
            None => return (self.err("something went wrong", "unable to get item from itemStruct"), 502),
        };

        // Handle videos in parallel
        if let Some(bitrate_info) = item.get("video").and_then(|v| v.get("bitrateInfo")).and_then(|b| b.as_array()) {
            let videos_fut = bitrate_info.iter().enumerate().map(|(i, q)| async move {
                let size = q.get("PlayAddr").and_then(|p| p.get("DataSize")).cloned().unwrap_or(json!("N/A"));
                let addr = q.get("PlayAddr")
                    .and_then(|p| p.get("UrlList"))
                    .and_then(|u| u.as_array())
                    .and_then(|a| a.last())
                    .cloned()
                    .unwrap_or(json!("N/A"))
                    .as_str()
                    .unwrap_or("N/A")
                    .replace("https://www.tiktok.com", "https://api16-normal-useast5.tiktokv.us");
                json!({ format!("quality_{}", i): {"size": size, "address": addr} })
            });
            let videos: Vec<Value> = join_all(videos_fut).await;

            return (json!({
                "platform": "tiktok",
                "is_video": true,
                "content": {
                    "id": item.get("id").cloned().unwrap_or(json!("N/A")),
                    "desc": item.get("desc").cloned().unwrap_or(json!("N/A")),
                    "views": item.get("stats").and_then(|s| s.get("playCount")).cloned().unwrap_or(json!(0)),
                    "likes": item.get("stats").and_then(|s| s.get("diggCount")).cloned().unwrap_or(json!(0)),
                    "comments": item.get("stats").and_then(|s| s.get("commentCount")).cloned().unwrap_or(json!(0)),
                    "saves": item.get("stats").and_then(|s| s.get("collectCount")).cloned().unwrap_or(json!(0)),
                    "share": item.get("stats").and_then(|s| s.get("shareCount")).cloned().unwrap_or(json!(0)),
                    "cover": item.get("video").and_then(|v| v.get("cover")).cloned().unwrap_or(json!("N/A")),
                },
                "author": {
                    "name": item.get("author").and_then(|a| a.get("nickname")).cloned().unwrap_or(json!("N/A")),
                    "username": item.get("author").and_then(|a| a.get("uniqueId")).cloned().unwrap_or(json!("N/A")),
                    "verified": item.get("author").and_then(|a| a.get("verified")).cloned().unwrap_or(json!(false)),
                    "image": item.get("author").and_then(|a| a.get("avatarMedium")).cloned().unwrap_or(json!("N/A")),
                    "bio": item.get("author").and_then(|a| a.get("signature")).cloned().unwrap_or(json!("N/A")),
                },
                "videos": videos,
                "music": {
                    "author": item.get("music").and_then(|m| m.get("authorName")).cloned().unwrap_or(json!("N/A")),
                    "title": item.get("music").and_then(|m| m.get("title")).cloned().unwrap_or(json!("N/A")),
                    "cover": item.get("music").and_then(|m| m.get("coverMedium")).cloned().unwrap_or(json!("N/A")),
                    "duration": item.get("music").and_then(|m| m.get("duration")).cloned().unwrap_or(json!("N/A")),
                    "src": item.get("music").and_then(|m| m.get("playUrl")).cloned().unwrap_or(json!("N/A")),
                }
            }), 200);
        }

        // Handle images
        let images = item.get("imagePost")
            .and_then(|p| p.get("images"))
            .and_then(|x| x.as_array())
            .cloned()
            .unwrap_or_default();

        let out_images: Vec<Value> = images.iter().enumerate().map(|(i, img)| {
            let addr = img.get("imageURL")
                .and_then(|u| u.get("urlList"))
                .and_then(|l| l.as_array())
                .and_then(|a| a.last())
                .cloned()
                .unwrap_or(json!(""));
            let size = img.get("imageHeight").cloned().unwrap_or(json!("N/A"));
            json!({ format!("image_{}", i): {"address": addr, "size": size} })
        }).collect();

        (json!({
            "platform": "tiktok",
            "is_image": true,
            "content": {
                "id": item.get("id").cloned().unwrap_or(json!("N/A")),
                "desc": item.get("desc").cloned().unwrap_or(json!("N/A")),
                "title": item.get("imagePost").and_then(|p| p.get("title")).cloned().unwrap_or(json!("N/A")),
                "views": item.get("stats").and_then(|s| s.get("playCount")).cloned().unwrap_or(json!(0)),
                "likes": item.get("stats").and_then(|s| s.get("diggCount")).cloned().unwrap_or(json!(0)),
                "comments": item.get("stats").and_then(|s| s.get("commentCount")).cloned().unwrap_or(json!(0)),
                "saves": item.get("stats").and_then(|s| s.get("collectCount")).cloned().unwrap_or(json!(0)),
                "share": item.get("stats").and_then(|s| s.get("shareCount")).cloned().unwrap_or(json!(0)),
                "cover": item.get("imagePost")
                    .and_then(|p| p.get("cover"))
                    .and_then(|c| c.get("imageURL"))
                    .and_then(|u| u.get("urlList"))
                    .and_then(|l| l.as_array())
                    .and_then(|a| a.last())
                    .cloned()
                    .unwrap_or(json!("N/A")),
            },
            "author": {
                "name": item.get("author").and_then(|a| a.get("nickname")).cloned().unwrap_or(json!("N/A")),
                "username": item.get("author").and_then(|a| a.get("uniqueId")).cloned().unwrap_or(json!("N/A")),
                "verified": item.get("author").and_then(|a| a.get("verified")).cloned().unwrap_or(json!(false)),
                "image": item.get("author").and_then(|a| a.get("avatarMedium")).cloned().unwrap_or(json!("N/A")),
                "location": item.get("locationCreated").cloned().unwrap_or(json!("N/A")),
            },
            "images": out_images,
            "music": {
                "author": item.get("music").and_then(|m| m.get("authorName")).cloned().unwrap_or(json!("N/A")),
                "title": item.get("music").and_then(|m| m.get("title")).cloned().unwrap_or(json!("N/A")),
                "cover": item.get("music").and_then(|m| m.get("coverMedium")).cloned().unwrap_or(json!("N/A")),
                "duration": item.get("music").and_then(|m| m.get("duration")).cloned().unwrap_or(json!("N/A")),
                "src": item.get("playUrl").cloned().unwrap_or(json!("N/A")),
            }
        }), 200)
    }


}

#[tokio::test]
async fn tiktok() {
    let client = reqwest::Client::new();
    let mut scraper = TikTokv2::new(
        "https://www.tiktok.com/@devfemibadmus/video/7390912680883899654",
        true,
        client.clone(),
    );
    let (data, status) = scraper.get_data().await;
    assert_eq!(status, 200);
    println!("Data: {:#?}", data);
}

