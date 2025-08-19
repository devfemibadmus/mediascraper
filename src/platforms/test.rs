struct TwitterDownloader;

impl TwitterDownloader {
    fn download(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        use reqwest::blocking::Client;
        use regex::Regex;
        use std::fs::File;
        use std::io::copy;

        let client = Client::builder().user_agent("Mozilla/5.0").build()?;
        let res = client.get(url).send()?.text()?;

        let re = Regex::new(r#"property="og:video" content="(.*?)""#)?;
        let video_url = re.captures(&res).ok_or("No video found")?[1].to_string();

        let mut resp = client.get(&video_url).send()?;
        let mut out = File::create("video.mp4")?;
        copy(&mut resp, &mut out)?;

        println!("Downloaded video.mp4");
        Ok(())
    }
}

#[test]
fn quick_test() -> Result<(), Box<dyn std::error::Error>> {
    let dl = TwitterDownloader;
    dl.download("https://twitter.com/username/status/TWEET_ID")?;
    assert!(std::path::Path::new("video.mp4").exists());
    Ok(())
}
