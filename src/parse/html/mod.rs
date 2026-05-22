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

pub fn parse(url: &reqwest::Url, rules: &[Rule]) -> Vec<Article> {
    let get_html = reqwest::blocking::get(url.as_str()).unwrap().bytes();

    // get areas of interest from rules
    let parser =
        scraper::Html::parse_document(&String::from_utf8(get_html.unwrap().to_vec()).unwrap());

    let mut area_of_interest_selectors = Vec::new();
    let mut title_selectors = Vec::new();
    let mut url_selectors = Vec::new();
    let mut date_selectors = Vec::new();

    for rule in rules {
        match &rule.section {
            ParseSection::AreaOfInterest => match &rule.approach {
                ParseApproach::UseClass { class_name } => {
                    let selector = scraper::Selector::parse(&format!(".{class_name}")).unwrap();
                    area_of_interest_selectors.push(selector);
                }
            },
            ParseSection::Title { extract_method } => match &rule.approach {
                ParseApproach::UseClass { class_name } => {
                    let selector = scraper::Selector::parse(&format!(".{class_name}")).unwrap();
                    title_selectors.push((selector, extract_method));
                }
            },
            ParseSection::Link { extract_method } => match &rule.approach {
                ParseApproach::UseClass { class_name } => {
                    let selector = scraper::Selector::parse(&format!(".{class_name}")).unwrap();
                    url_selectors.push((selector, extract_method));
                }
            },
            ParseSection::Date {
                extract_method,
                date_format,
            } => match &rule.approach {
                ParseApproach::UseClass { class_name } => {
                    let selector = scraper::Selector::parse(&format!(".{class_name}")).unwrap();
                    date_selectors.push((selector, extract_method, date_format));
                }
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
                articles.push(Article {
                    title,
                    url,
                    author: None,
                    published_at: date,
                });
            }
        }
    }

    articles
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
