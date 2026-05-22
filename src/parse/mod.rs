pub mod html;
pub mod json;
pub mod rss;

pub mod approach;
pub mod date;
pub mod extract;
pub mod format;
pub mod rule;
pub mod section;

use reqwest::Url;

pub use self::format::Format;

use crate::{models::Article, parse::rule::Rule};

pub fn parse(format: &Format, url: &Url, rules: &[Rule]) -> Vec<Article> {
    match format {
        Format::RSS => crate::parse::rss::parse(url, rules),
        Format::JSON => crate::parse::json::parse(url, rules),
        Format::HTML => crate::parse::html::parse(url, rules),
    }
}
