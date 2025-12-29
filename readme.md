<!-- @format -->

# MEDIASCRAPER API

[![Rust](https://img.shields.io/badge/Rust-1.89+-orange?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE) [![Google Play](https://img.shields.io/badge/Google%20Play-Download-brightgreen?logo=google-play)](https://play.google.com/store/apps/details?id=com.blackstackhub.mediasaver) ![Views](https://komarev.com/ghpvc/?username=devfemibadmus&repo=mediascraper&color=blue)

## 📋 Overview

**MEDIASCRAPER API** is a high-performance REST API built with Rust and Actix-web for extracting media content (videos, images, audio) from social media platforms using web scraping techniques.

## ✨ Features

-   **Multi-Platform Support**: Facebook, Instagram, TikTok, Snapchat, Twitter (X)
-   **High Performance**: Async/await architecture with concurrent scraping
-   **Clean API**: Consistent JSON responses across all platforms
-   **Error Handling**: Comprehensive error messages with appropriate HTTP status codes
-   **No Rate Limits**: Bypasses platform restrictions through smart scraping

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/yourusername/mediascraper.git
cd mediascraper
cargo build --release
cargo run
```

Server runs at: `http://localhost:8080`

## 📖 API Usage

### Endpoint

```
GET or POST /api/
```

### Request Parameters

**GET Request:**

```bash
curl https://mediasaver.link/api/?url=YOUR_SOCIAL_MEDIA_URL"
```

**POST Request:**

```bash
curl -X POST https://mediasaver.link/api/ \
  -H "Content-Type: application/json" \
  -d '{"url": "YOUR_SOCIAL_MEDIA_URL"}'
```

## 📋 Supported Platforms

| Platform  | Status | Example URL Pattern                                  |
| --------- | ------ | ---------------------------------------------------- |
| Facebook  | ✅     | `facebook.com/...`, `fb.watch/...`                   |
| Instagram | ✅     | `instagram.com/p/...`, `instagram.com/reel/...`      |
| TikTok    | ✅     | `tiktok.com/...`, `vm.tiktok.com/...`                |
| Snapchat  | ✅     | `snapchat.com/t/...`                                 |
| Twitter/X | ✅     | `twitter.com/.../status/...`, `x.com/.../status/...` |

## 📊 Response Format

### Success Response (200 OK)

```json
{
	"data": ["https://media-url-1.mp4", "https://media-url-2.jpg"],
	"total": 2,
	"platform": "platform_name"
}
```

### Error Response (4xx/5xx)

```json
{
	"error": true,
	"message": "Error description",
	"error_message": "Error description"
}
```

## 🎯 Examples

### TikTok

```bash
curl https://mediasaver.link/api/?url=https://vm.tiktok.com/ZSHK8GLq32Kjh-qQ9X4/
```

### Instagram

```bash
curl https://mediasaver.link/api/?url=https://www.instagram.com/reel/DHm7knuzl1D
```

### Facebook

```bash
curl https://mediasaver.link/api/?url=https://www.facebook.com/share/v/qCRH3vKk2FbAEAUP/
```

### Snapchat

```bash
curl https://mediasaver.link/api/?url=https://snapchat.com/t/GJbX4HdO
```

### Twitter

```bash
curl https://mediasaver.link/api/?url=https://x.com/username/status/1234567890
```

## 🏗️ Project Structure

```
mediascraper/
├── src/
│   ├── main.rs
│   └── platforms/
│       ├── mod.rs
│       ├── facebook.rs
│       ├── instagram.rs
│       ├── tiktok.rs
│       ├── snapchat.rs
│       └── twitter.rs
├── Cargo.toml
└── website/
    ├── static/
    └── *.html
```

## 📦 Dependencies

Key dependencies in `Cargo.toml`:

-   `actix-web = "4.0"` - Web framework
-   `reqwest = "0.11"` - HTTP client
-   `scraper = "0.18"` - HTML parsing
-   `serde_json = "1.0"` - JSON handling
-   `regex = "1.10"` - URL validation
-   `tera = "1.19"` - Templates

## 🧪 Testing

Run tests:

```bash
cargo test
```

Run specific platform test:

```bash
cargo test facebook -- --nocapture
cargo test instagram -- --nocapture
```

## ⚠️ Disclaimer

This tool is for educational purposes only. Use responsibly and respect platform terms of service. Not responsible for misuse.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.
