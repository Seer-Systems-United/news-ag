pub(crate) mod ssr;
pub mod wordpress;

#[cfg(not(feature = "async"))]
pub fn parse(
    url: &reqwest::Url,
    rules: &[crate::parse::rule::Rule],
) -> Vec<crate::models::Article> {
    match &rules.get(0).unwrap().approach {
        super::approach::ParseApproach::UseJSONParser {
            function,
            headers,
            http1_only,
        } => {
            let res = fetch(url, headers, *http1_only);
            function(&res)
        }
        _ => {
            panic!("Invalid approach for JSON format")
        }
    }
}

#[cfg(feature = "async")]
pub async fn parse(
    url: &reqwest::Url,
    rules: &[crate::parse::rule::Rule],
) -> Vec<crate::models::Article> {
    match &rules.get(0).unwrap().approach {
        super::approach::ParseApproach::UseJSONParser {
            function,
            headers,
            http1_only,
        } => {
            let res = fetch(url, headers, *http1_only).await;
            function(&res)
        }
        _ => {
            panic!("Invalid approach for JSON format")
        }
    }
}

#[cfg(not(feature = "async"))]
fn fetch(url: &reqwest::Url, headers: &[(String, String)], http1_only: bool) -> String {
    let mut client = reqwest::blocking::Client::builder();

    if http1_only {
        client = client.http1_only();
    }

    let client = client.build().unwrap();
    let mut request = client.get(url.as_str());

    for (name, value) in headers {
        request = request.header(name, value);
    }

    request.send().unwrap().text().unwrap()
}

#[cfg(feature = "async")]
async fn fetch(url: &reqwest::Url, headers: &[(String, String)], http1_only: bool) -> String {
    let mut client = reqwest::Client::builder();

    if http1_only {
        client = client.http1_only();
    }

    let client = client.build().unwrap();
    let mut request = client.get(url.as_str());

    for (name, value) in headers {
        request = request.header(name, value);
    }

    request.send().await.unwrap().text().await.unwrap()
}
