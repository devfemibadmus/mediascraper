# WEBMEDIA

[![Google Play](https://img.shields.io/badge/Google%20Play-Download-brightgreen?logo=google-play)](https://play.google.com/store/apps/details?id=com.blackstackhub.mediasaver)
![Tests](https://github.com/<USER>/<REPO>/actions/workflows/ci.yml/badge.svg)
![Views](https://komarev.com/ghpvc/?username=devfemibadmus&repo=webmedia&color=blue)

## Overview

**WEBMEDIA**: [MediaSaver](https://github.com/devfemibadmus/mediasaver) api for videos, images, and audio from Instagram, TikTok, and Facebook via web scraping and predefined network methods. [Actix-web](https://github.com/actix/actix-web)

## :star: Features

-   **Fetch Media**: Retrieves both private and public media files

-   **Cut Data**: shrink and return normal data

-   **Full Data**: Give full data containing all fields scraped from given platform

## :rocket: Apps

-   **Web**: [Media Saver](https://mediasaver.link)

-   **iOs App**: [Add to Home Screen](https://mediasaver.link/#app)

-   **Android App**: [Google Play Store](https://play.google.com/store/apps/details?id=com.blackstackhub.mediasaver) + [WhatsApp status saver](https://github.com/devfemibadmus/mediasaver)

## :clown_face: Status

-   **YouTube**: 🔴
-   **Facebook(videos, reels & metadata)**: 🟢
-   **TikTok(videos, photos, music & metadata)**: 🟢
-   **Instagram(videos, reels, photos, music & metadata)**: 🟢

## :eyes: Checkout This

-   **Mobile**: https://github.com/devfemibadmus/mediasaver

-   **Api**: https://mediasaver.link/api/

## 📖 API Endpoint

-   **Method**: `GET` or `POST`
-   **URL**: `https://mediasaver.link/api/`
-   **Parameters**:
-   `cut`: Optional
-   `url`: Required

**Status 200 :white_check_mark:**

```json
{
	"success": true,
	"data": {}
}
```

**Status 400, 404, 500, 502 :x:**

```json
{
	"error": true,
	"message": "...",
	"error_message": "..."
}
```

#### Tiktok https://devfemibadmus.blackstackhub.com/webmedia/api/?cut=-&url=https://www.tiktok.com/@devfemibadmus/video/7390912680883899654

![TikTok](screenshot/image%20copy%206.png?raw=true)

#### Instagram https://devfemibadmus.blackstackhub.com/webmedia/api/?cut=-&url=https://www.instagram.com/p/C-TMvc4yQh6/?img_index=3 (post has been deleted)

![Instagram](screenshot/image%20copy%207.png?raw=true)

#### Facebook https://devfemibadmus.blackstackhub.com/webmedia/api/?cut=-&url=https://www.facebook.com/share/v/qCRH3vKk2FbAEAUP/

![Facebook](screenshot/image%20copy%208.png?raw=true)

| Screenshot                                                                                    | Screenshot                                                                                    |
| --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| ![post and video quality](<screenshot/127.0.0.1_5000_(iPhone%2014%20Pro%20Max).png?raw=true>) | ![author and music](<screenshot/127.0.0.1_5000_(iPhone%2014%20Pro%20Max)%20(1).png?raw=true>) |
