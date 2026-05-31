use std::collections::HashSet;

use crate::models::Article;
use serde_json::{Map, Value};

pub(crate) struct NextFlightLink {
    pub(crate) href: String,
    pub(crate) title: String,
}

pub(crate) fn json_scripts(body: &str, include: impl Fn(&str, &str) -> bool) -> Vec<Value> {
    let document = scraper::Html::parse_document(body);
    let script_selector = scraper::Selector::parse("script").unwrap();

    document
        .select(&script_selector)
        .filter_map(|script| {
            let element = script.value();
            let id = element.attr("id").unwrap_or_default();
            let script_type = element.attr("type").unwrap_or_default();

            if !include(id, script_type) {
                return None;
            }

            let text = script.text().collect::<String>();
            serde_json::from_str::<Value>(&text).ok()
        })
        .collect()
}

pub(crate) fn is_json_script_type(script_type: &str) -> bool {
    let script_type = script_type.to_ascii_lowercase();
    script_type.contains("application/json") || script_type.contains("ld+json")
}

pub(crate) fn json_assignments(body: &str, marker: &str) -> Vec<Value> {
    let mut values = Vec::new();
    let mut remainder = body;

    while let Some(marker_position) = remainder.find(marker) {
        let after_marker = &remainder[marker_position + marker.len()..];
        let Some(json_end) = json_object_end(after_marker) else {
            break;
        };

        if let Ok(value) = serde_json::from_str::<Value>(&after_marker[..json_end]) {
            values.push(value);
        }

        remainder = &after_marker[json_end..];
    }

    values
}

pub(crate) fn visit_objects(value: &Value, visit: &mut impl FnMut(&Map<String, Value>)) {
    match value {
        Value::Object(object) => {
            visit(object);

            for value in object.values() {
                visit_objects(value, visit);
            }
        }
        Value::Array(values) => {
            for value in values {
                visit_objects(value, visit);
            }
        }
        _ => {}
    }
}

pub(crate) fn article_from_object(
    object: &Map<String, Value>,
    base_url: &str,
    title_keys: &[&str],
    is_article_url: impl Fn(&Map<String, Value>, &str) -> bool,
) -> Option<Article> {
    let title = clean_text(first_string(object, title_keys)?)?;
    let raw_url = object_url(object)?;
    let url = resolve_url(base_url, raw_url)?;

    if !is_article_url(object, &url) {
        return None;
    }

    Some(Article::new(
        title,
        url,
        authors(object),
        first_string(
            object,
            &[
                "displayedDate",
                "date",
                "datePublished",
                "firstPublished",
                "publishedAt",
                "updatedTime",
            ],
        )
        .and_then(parse_date),
    ))
}

pub(crate) fn next_flight_links(body: &str) -> Vec<NextFlightLink> {
    let mut links = Vec::new();
    let mut remainder = body;
    let marker = "self.__next_f.push(";

    while let Some(marker_position) = remainder.find(marker) {
        let after_marker = &remainder[marker_position + marker.len()..];
        let Some(end_position) = after_marker.find(")</script>") else {
            break;
        };
        let argument = &after_marker[..end_position];

        if let Ok(Value::Array(values)) = serde_json::from_str::<Value>(argument) {
            if let Some(chunk) = values.get(1).and_then(Value::as_str) {
                links.extend(href_child_links(chunk));
            }
        }

        remainder = &after_marker[end_position + ")</script>".len()..];
    }

    links
}

pub(crate) fn object_string<'a>(object: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    object.get(key).and_then(Value::as_str).map(str::trim)
}

pub(crate) fn clean_text(value: &str) -> Option<String> {
    let fragment = scraper::Html::parse_fragment(value);
    let text = fragment.root_element().text().collect::<Vec<_>>().join(" ");
    let text = text
        .replace('\u{a0}', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if text.is_empty() { None } else { Some(text) }
}

pub(crate) fn resolve_url(base_url: &str, value: &str) -> Option<String> {
    let value = value.trim();

    if value.is_empty() || value.starts_with('#') || value.starts_with("javascript:") {
        return None;
    }

    reqwest::Url::parse(base_url)
        .ok()
        .and_then(|base| base.join(value).ok())
        .map(|url| url.to_string())
}

pub(crate) fn parse_date(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    let value = value.trim();

    chrono::DateTime::parse_from_rfc3339(value)
        .map(|date| date.with_timezone(&chrono::Utc))
        .ok()
        .or_else(|| {
            chrono::NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
                .map(|date| date.and_utc())
                .ok()
        })
        .or_else(|| {
            chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
                .ok()
                .and_then(|date| date.and_hms_opt(0, 0, 0))
                .map(|date| date.and_utc())
        })
}

pub(crate) fn dedupe_articles(articles: Vec<Article>) -> Vec<Article> {
    let mut seen = HashSet::new();
    articles
        .into_iter()
        .filter(|article| {
            !article.title.trim().is_empty()
                && !article.url.trim().is_empty()
                && seen.insert(article.url.clone())
        })
        .collect()
}

fn href_child_links(chunk: &str) -> Vec<NextFlightLink> {
    let mut links = Vec::new();
    let mut cursor = 0;
    let href_key = "\"href\":\"";

    while let Some(relative_position) = chunk[cursor..].find(href_key) {
        let href_start = cursor + relative_position + href_key.len();
        let Some((href, href_end)) = read_json_string(chunk, href_start) else {
            break;
        };

        let next_href = chunk[href_end..]
            .find(href_key)
            .map(|position| href_end + position)
            .unwrap_or(chunk.len());
        let article_chunk = &chunk[href_end..next_href];

        if let Some(title) = best_child_title(article_chunk).and_then(|title| clean_text(&title)) {
            links.push(NextFlightLink { href, title });
        }

        cursor = next_href;
    }

    links
}

fn best_child_title(chunk: &str) -> Option<String> {
    let mut best = None;
    let mut cursor = 0;
    let child_key = "\"children\":\"";

    while let Some(relative_position) = chunk[cursor..].find(child_key) {
        let child_start = cursor + relative_position + child_key.len();
        let Some((child, child_end)) = read_json_string(chunk, child_start) else {
            break;
        };

        if is_article_title(&child) {
            let replace = best
                .as_ref()
                .map(|best: &String| child.len() > best.len())
                .unwrap_or(true);
            if replace {
                best = Some(child);
            }
        }

        cursor = child_end;
    }

    best
}

fn read_json_string(value: &str, start: usize) -> Option<(String, usize)> {
    let mut escaped = false;

    for (relative_position, ch) in value[start..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            '"' => {
                let raw = &value[start..start + relative_position];
                let parsed = serde_json::from_str::<String>(&format!("\"{raw}\""))
                    .unwrap_or_else(|_| raw.to_string());
                return Some((parsed, start + relative_position + 1));
            }
            _ => {}
        }
    }

    None
}

fn json_object_end(value: &str) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut started = false;

    for (position, ch) in value.char_indices() {
        if !started {
            if ch.is_whitespace() {
                continue;
            }
            if ch != '{' {
                return None;
            }
            started = true;
        }

        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(position + ch.len_utf8());
                }
            }
            _ => {}
        }
    }

    None
}

fn object_url(object: &Map<String, Value>) -> Option<&str> {
    first_string(
        object,
        &["url", "href", "canonicalUrl", "canonical_url", "webUrl"],
    )
    .or_else(|| {
        object
            .get("link")
            .and_then(Value::as_object)
            .and_then(|link| object_string(link, "url"))
    })
}

fn first_string<'a>(object: &'a Map<String, Value>, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|key| object_string(object, key))
}

fn authors(object: &Map<String, Value>) -> Option<Vec<String>> {
    if let Some(author) = object_string(object, "author").and_then(clean_text) {
        return Some(vec![author]);
    }

    if let Some(byline) = object_string(object, "byline").and_then(clean_text) {
        return Some(vec![byline]);
    }

    let authors = object.get("authors")?.as_array()?;
    let authors = authors
        .iter()
        .filter_map(author_name)
        .collect::<Vec<String>>();

    if authors.is_empty() {
        None
    } else {
        Some(authors)
    }
}

fn author_name(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => clean_text(value),
        Value::Object(object) => {
            first_string(object, &["name", "displayName", "fullName", "byline"])
                .and_then(clean_text)
        }
        _ => None,
    }
}

fn is_article_title(value: &str) -> bool {
    let value = value.trim();

    value.len() >= 16
        && value.split_whitespace().count() >= 3
        && !value.starts_with("http")
        && !matches!(
            value,
            "Subscribe" | "Sign In" | "Latest News" | "Top Stories"
        )
}

#[cfg(test)]
mod tests {
    #[test]
    fn extracts_json_assignment() {
        let values = super::json_assignments(
            r#"<script>window.example = {"nested":{"value":"ok; still json"}}</script>"#,
            "window.example = ",
        );

        assert_eq!(values.len(), 1);
        assert_eq!(values[0]["nested"]["value"], "ok; still json");
    }

    #[test]
    fn extracts_next_flight_links() {
        let links = super::next_flight_links(
            r#"
            <script>self.__next_f.push([1,"1:[\"$\",\"a\",null,{\"href\":\"/example/2026/05/story/\",\"children\":[\"$\",\"h3\",null,{\"children\":\"City council approves new neighborhood partnership\"}]}]"])</script>
            "#,
        );

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].href, "/example/2026/05/story/");
        assert_eq!(
            links[0].title,
            "City council approves new neighborhood partnership"
        );
    }
}
