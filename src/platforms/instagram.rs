use reqwest::Client;
use serde_json::{json, Value};

pub struct Instagram {
    client: Client,
    item_id: String,
    cut: bool,
}

impl Instagram {
    pub fn new(client: Client, item_id: String, cut: bool) -> Self {
        Self { client, item_id, cut }
    }

    fn graphql() -> &'static str {
        "https://www.instagram.com/graphql/query/"
    }

    fn headers() -> reqwest::header::HeaderMap {
        use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE, USER_AGENT};
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        headers.insert(USER_AGENT, HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36 Edg/136.0.0.0"
        ));
        headers.insert("Origin", HeaderValue::from_static("https://www.instagram.com"));
        headers.insert("Referer", HeaderValue::from_static("https://www.instagram.com"));
        headers.insert("X-CSRFToken", HeaderValue::from_static("11111111111"));
        headers
    }

    pub async fn get_data(&self) -> (Value, u16) {
        let graphql_data = json!({
            "av": "0",
            "__d": "www",
            "__user": "0",
            "__a": "1",
            "__req": "a",
            "__hs": "20229.HYP:instagram_web_pkg.2.1...0",
            "dpr": "1",
            "__ccg": "GOOD",
            "__rev": "1023049274",
            "__comet_req": "7",
            "lsd": "AVqQ3As1H7g",
            "jazoest": "2855",
            "__spin_r": "1023049274",
            "__spin_b": "trunk",
            "__spin_t": "1747835843",
            "fb_api_caller_class": "RelayModern",
            "fb_api_req_friendly_name": "PolarisPostActionLoadPostQueryQuery",
            "variables": format!("{{\"shortcode\":\"{}\",\"fetch_tagged_user_count\":null,\"hoisted_comment_id\":null,\"hoisted_reply_id\":null}}", self.item_id),
            "server_timestamps": "true",
            "doc_id": "9510064595728286"
        });

        let resp = match self.client.post(Self::graphql())
            .headers(Self::headers())
            .form(&graphql_data)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return (json!({ "error_message": format!("Request failed: {}", e) }), 502);
            }
        };

        let mut data: Value = match resp.json().await {
            Ok(d) => d,
            Err(e) => {
                return (json!({ "error_message": format!("JSON parse failed: {}", e) }), 502);
            }
        };

        data["platform"] = json!("instagram");

        if !self.cut {
            return (data, 200);
        }

        let item = match data.get("data").and_then(|d| d.get("xdt_shortcode_media")) {
            Some(i) => i,
            None => return (json!({ "error_message": "Item not found" }), 502),
        };

        let desc = item.get("edge_media_to_caption")
            .and_then(|c| c.get("edges"))
            .and_then(|e| e.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|n| n.get("node"))
            .and_then(|n| n.get("text"))
            .cloned()
            .unwrap_or(json!("no desc"));

        let mut content = json!({
            "id": item.get("id").cloned().unwrap_or(json!("N/A")),
            "shortcode": item.get("shortcode").cloned().unwrap_or(json!("N/A")),
            "likes": item.get("edge_media_preview_like").and_then(|l| l.get("count")).cloned().unwrap_or(json!(0)),
            "desc": desc,
            "cover": item.get("thumbnail_src").cloned().unwrap_or(json!("N/A")),
            "is_video": item.get("is_video").cloned().unwrap_or(json!(false))
        });

        if let Some(resources) = item.get("display_resources").and_then(|r| r.as_array()) {
            if let Some(last) = resources.last() {
                content["cover"] = last.get("src").cloned().unwrap_or(json!("N/A"));
            }
        }

        let author = json!({
            "name": item.get("owner").and_then(|o| o.get("full_name")).cloned().unwrap_or(json!("N/A")),
            "username": item.get("owner").and_then(|o| o.get("username")).cloned().unwrap_or(json!("N/A")),
            "verified": item.get("owner").and_then(|o| o.get("is_verified")).cloned().unwrap_or(json!(false)),
            "image": item.get("owner").and_then(|o| o.get("profile_pic_url")).cloned().unwrap_or(json!("N/A")),
            "videos": item.get("owner").and_then(|o| o.get("edge_owner_to_timeline_media")).and_then(|v| v.get("count")).cloned().unwrap_or(json!(0)),
            "followers": item.get("owner").and_then(|o| o.get("edge_followed_by")).and_then(|f| f.get("count")).cloned().unwrap_or(json!(0))
        });

        let mut media = vec![];
        if let Some(edges) = item.get("edge_sidecar_to_children")
            .and_then(|c| c.get("edges"))
            .and_then(|a| a.as_array()) 
        {
            for edge in edges {
                if let Some(node) = edge.get("node") {
                    let mut media_item = json!({
                        "id": node.get("id").cloned().unwrap_or(json!("N/A")),
                        "shortcode": node.get("shortcode").cloned().unwrap_or(json!("N/A")),
                        "address": node.get("display_url").cloned().unwrap_or(json!("N/A")),
                        "cover": node.get("display_url").cloned().unwrap_or(json!("N/A")),
                        "is_video": node.get("video_url").is_some()
                    });
                    if let Some(resources) = node.get("display_resources").and_then(|r| r.as_array()) {
                        if let Some(last) = resources.last() {
                            media_item["address"] = last.get("src").cloned().unwrap_or(media_item["address"].clone());
                        }
                    }
                    if let Some(video) = node.get("video_url") {
                        media_item["address"] = video.clone();
                        media_item["play"] = node.get("video_play_count").cloned().unwrap_or(json!(0));
                        media_item["views"] = node.get("video_view_count").cloned().unwrap_or(json!(0));
                    }
                    media.push(media_item);
                }
            }
        } else if let Some(video_url) = item.get("video_url") {
            media.push(json!({
                "id": item.get("id").cloned().unwrap_or(json!("N/A")),
                "shortcode": item.get("shortcode").cloned().unwrap_or(json!("N/A")),
                "address": video_url.clone(),
                "is_video": item.get("is_video").cloned().unwrap_or(json!(false)),
                "cover": item.get("display_url").cloned().unwrap_or(json!("N/A"))
            }));
        }

        let result = json!({
            "platform": "instagram",
            "content": content,
            "author": author,
            "media": media
        });

        (result, 200)
    }


}

#[tokio::test]
async fn instagram() {
    let client = reqwest::Client::new();
    let scraper = Instagram { 
        client,
        item_id: "DHm7knuzl1D".to_string(),
        cut: true,
    };
    let (data, status) = scraper.get_data().await;
    assert_eq!(status, 200);
    println!("Data: {:#?}", data);
}
