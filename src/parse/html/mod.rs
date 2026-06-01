use crate::{
    models::Article,
    parse::{
        approach::ParseApproach,
        date::{DataFormat, parse_date},
        extract::ExtractMethod,
        rule::Rule,
        section::ParseSection,
    },
};

#[cfg(not(feature = "async"))]
pub fn parse(url: &reqwest::Url, rules: &[Rule]) -> Vec<Article> {
    let get_html = reqwest::blocking::get(url.as_str()).unwrap().bytes();

    parse_body(
        &String::from_utf8(get_html.unwrap().to_vec()).unwrap(),
        rules,
    )
}

#[cfg(feature = "async")]
pub async fn parse(url: &reqwest::Url, rules: &[Rule]) -> Vec<Article> {
    let get_html = reqwest::get(url.as_str()).await.unwrap().bytes().await;

    parse_body(
        &String::from_utf8(get_html.unwrap().to_vec()).unwrap(),
        rules,
    )
}

fn parse_body(body: &str, rules: &[Rule]) -> Vec<Article> {
    // get areas of interest from rules
    let parser = scraper::Html::parse_document(body);

    let mut area_of_interest_selectors = Vec::new();
    let mut title_selectors = Vec::new();
    let mut url_selectors = Vec::new();
    let mut thumbnail_selectors = Vec::new();
    let mut date_selectors = Vec::new();

    for rule in rules {
        match &rule.section {
            ParseSection::AreaOfInterest => match &rule.approach {
                ParseApproach::UseClass { class_name } => {
                    let selector = scraper::Selector::parse(&format!(".{class_name}")).unwrap();
                    area_of_interest_selectors.push(selector);
                }
                _ => (),
            },
            ParseSection::Title { extract_method } => match &rule.approach {
                ParseApproach::UseClass { class_name } => {
                    let selector = scraper::Selector::parse(&format!(".{class_name}")).unwrap();
                    title_selectors.push((selector, extract_method));
                }
                _ => (),
            },
            ParseSection::Link { extract_method } => match &rule.approach {
                ParseApproach::UseClass { class_name } => {
                    let selector = scraper::Selector::parse(&format!(".{class_name}")).unwrap();
                    url_selectors.push((selector, extract_method));
                }
                _ => (),
            },
            ParseSection::Thumbnail { extract_method } => match &rule.approach {
                ParseApproach::UseClass { class_name } => {
                    let selector = scraper::Selector::parse(&format!(".{class_name}")).unwrap();
                    thumbnail_selectors.push((selector, extract_method));
                }
                _ => (),
            },
            ParseSection::Date {
                extract_method,
                date_format,
            } => match &rule.approach {
                ParseApproach::UseClass { class_name } => {
                    let selector = scraper::Selector::parse(&format!(".{class_name}")).unwrap();
                    date_selectors.push((selector, extract_method, date_format));
                }
                _ => (),
            },
            _ => (),
        }
    }

    let mut articles = Vec::new();

    for area_selector in &area_of_interest_selectors {
        for section in parser.select(area_selector) {
            let date = extract_date_from_section(section, &date_selectors);

            if let (Some(title), Some(url)) = (
                extract_first_from_section(section, &title_selectors),
                extract_first_from_section(section, &url_selectors),
            ) {
                let thumbnail_url = extract_first_from_section(section, &thumbnail_selectors);
                articles.push(Article::new(title, url, None, date, thumbnail_url));
            }
        }
    }

    articles
}

#[cfg(test)]
mod tests {
    use crate::parse::{
        approach::ParseApproach, extract::ExtractMethod, rule::Rule, section::ParseSection,
    };

    #[test]
    fn extracts_thumbnail_from_rule() {
        let rules = [
            rule(ParseSection::AreaOfInterest, "card"),
            rule(
                ParseSection::Title {
                    extract_method: ExtractMethod::Text,
                },
                "title",
            ),
            rule(
                ParseSection::Link {
                    extract_method: ExtractMethod::Attribute {
                        name: "href".to_string(),
                    },
                },
                "link",
            ),
            rule(
                ParseSection::Thumbnail {
                    extract_method: ExtractMethod::Attribute {
                        name: "src".to_string(),
                    },
                },
                "image",
            ),
        ];

        let articles = super::parse_body(
            r#"<div class="card">
                <a class="link" href="https://example.com/article">
                    <span class="title">Example headline</span>
                </a>
                <img class="image" src="https://example.com/image.jpg">
            </div>"#,
            &rules,
        );

        assert_eq!(articles.len(), 1);
        assert_eq!(
            articles[0].thumbnail_url(),
            Some("https://example.com/image.jpg")
        );
    }

    fn rule(section: ParseSection, class_name: &str) -> Rule {
        Rule {
            section,
            approach: ParseApproach::UseClass {
                class_name: class_name.to_string(),
            },
        }
    }
}

fn extract_first_from_section(
    section: scraper::ElementRef<'_>,
    selectors: &[(scraper::Selector, &ExtractMethod)],
) -> Option<String> {
    for (selector, extract_method) in selectors {
        for element in section.select(selector) {
            let value = extract_value(element, extract_method);
            if !value.is_empty() {
                return Some(value);
            }
        }
    }

    None
}

fn extract_value(element: scraper::ElementRef<'_>, extract_method: &ExtractMethod) -> String {
    match extract_method {
        ExtractMethod::Text => element
            .text()
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .collect::<Vec<_>>()
            .join(" "),
        ExtractMethod::Attribute { name } => element.value().attr(name).unwrap_or("").to_string(),
    }
}

fn extract_date_from_section(
    section: scraper::ElementRef<'_>,
    selectors: &[(scraper::Selector, &ExtractMethod, &DataFormat)],
) -> Option<chrono::DateTime<chrono::Utc>> {
    for (selector, extract_method, date_format) in selectors {
        for element in section.select(selector) {
            let value = extract_value(element, extract_method);
            if let Some(date) = parse_date(&value, date_format) {
                return Some(date);
            }
        }
    }

    None
}
