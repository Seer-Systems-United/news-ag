pub const BASE_URL: &str = "https://www.reuters.com";

const SECTION_API_URL: &str =
    "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-section-alias-or-id-v1";

pub const WORLD_PATH: &str = "/world/";
pub const US_PATH: &str = "/world/us/";
pub const POLITICS_PATH: &str = US_PATH;
pub const BUSINESS_PATH: &str = "/business/";
pub const TECHNOLOGY_PATH: &str = "/technology/";
pub const ENTERTAINMENT_PATH: &str = "/lifestyle/";
pub const SPORTS_PATH: &str = "/sports/";
pub const SCIENCE_PATH: &str = "/science/";
pub const HEALTH_PATH: &str = "/business/healthcare-pharmaceuticals/";

pub fn section_url(section_path: &str) -> reqwest::Url {
    let query = serde_json::json!({
        "arc-site": "reuters",
        "fetch_type": "collection",
        "offset": 0,
        "requestId": 1,
        "section_id": section_path,
        "size": "20",
        "uri": section_path,
        "website": "reuters",
    })
    .to_string();

    reqwest::Url::parse_with_params(
        SECTION_API_URL,
        [
            ("query", query.as_str()),
            ("d", "363"),
            ("mxId", "00000000"),
            ("_website", "reuters"),
        ],
    )
    .unwrap()
}

pub fn json_headers(section_path: &str) -> Vec<(String, String)> {
    vec![
        (
            "User-Agent".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
                .to_string(),
        ),
        (
            "Accept".to_string(),
            "application/json, text/plain, */*".to_string(),
        ),
        ("Accept-Language".to_string(), "en-US,en;q=0.9".to_string()),
        ("Origin".to_string(), BASE_URL.to_string()),
        ("Referer".to_string(), format!("{BASE_URL}{section_path}")),
        (
            "Sec-CH-UA".to_string(),
            "\"Chromium\";v=\"126\", \"Google Chrome\";v=\"126\", \"Not-A.Brand\";v=\"99\""
                .to_string(),
        ),
        ("Sec-CH-UA-Mobile".to_string(), "?0".to_string()),
        ("Sec-CH-UA-Platform".to_string(), "\"Windows\"".to_string()),
        ("Sec-Fetch-Dest".to_string(), "empty".to_string()),
        ("Sec-Fetch-Mode".to_string(), "cors".to_string()),
        ("Sec-Fetch-Site".to_string(), "same-origin".to_string()),
    ]
}
