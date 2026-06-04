#[derive(Debug, Clone)]
pub(crate) enum ArticleContentSource {
    InlineHtml(String),
    WordpressRest(reqwest::Url),
    WebPage,
}

impl Default for ArticleContentSource {
    fn default() -> Self {
        Self::WebPage
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
pub struct Article {
    pub title: String,
    pub url: String,
    pub authors: Option<Vec<String>>,
    #[cfg_attr(
        feature = "rkyv",
        rkyv(with = rkyv::with::Map<rkyv_with::UtcDateTime>)
    )]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub thumbnail_url: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip, default))]
    #[cfg_attr(feature = "rkyv", rkyv(with = rkyv::with::Skip))]
    content_source: ArticleContentSource,
}

impl Article {
    pub fn new(
        title: String,
        url: String,
        authors: Option<Vec<String>>,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
        thumbnail_url: Option<String>,
    ) -> Self {
        Self {
            title,
            url,
            authors,
            published_at,
            content_source: ArticleContentSource::WebPage,
            thumbnail_url,
        }
    }

    pub(crate) fn with_inline_content(mut self, content: &str) -> Self {
        self.content_source = ArticleContentSource::InlineHtml(content.to_string());
        self
    }

    pub(crate) fn with_wordpress_content_url(mut self, mut url: reqwest::Url) -> Self {
        url.query_pairs_mut()
            .append_pair("_fields", "content.rendered");
        self.content_source = ArticleContentSource::WordpressRest(url);
        self
    }

    pub(crate) fn content_source(&self) -> &ArticleContentSource {
        &self.content_source
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn authors(&self) -> Option<&[String]> {
        self.authors.as_deref()
    }

    pub fn published_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.published_at
    }

    pub fn thumbnail_url(&self) -> Option<&str> {
        self.thumbnail_url.as_deref()
    }

    #[cfg(not(feature = "async"))]
    pub fn get_content(&self) -> Result<String, crate::content::ContentError> {
        crate::content::get(self)
    }

    #[cfg(feature = "async")]
    pub async fn get_content(&self) -> Result<String, crate::content::ContentError> {
        crate::content::get(self).await
    }
}

#[cfg(feature = "rkyv")]
mod rkyv_with {
    use std::{error::Error, fmt};

    use chrono::{DateTime, Utc};
    use rkyv::{
        Archive, Archived, Deserialize, Place, Resolver, Serialize,
        rancor::{Fallible, Source},
        with::{ArchiveWith, DeserializeWith, SerializeWith},
    };

    type Timestamp = (i64, u32);

    pub struct UtcDateTime;

    impl ArchiveWith<DateTime<Utc>> for UtcDateTime {
        type Archived = Archived<Timestamp>;
        type Resolver = Resolver<Timestamp>;

        fn resolve_with(
            field: &DateTime<Utc>,
            resolver: Self::Resolver,
            out: Place<Self::Archived>,
        ) {
            timestamp(field).resolve(resolver, out);
        }
    }

    impl<S> SerializeWith<DateTime<Utc>, S> for UtcDateTime
    where
        S: Fallible + ?Sized,
        Timestamp: Serialize<S>,
    {
        fn serialize_with(
            field: &DateTime<Utc>,
            serializer: &mut S,
        ) -> Result<Self::Resolver, S::Error> {
            timestamp(field).serialize(serializer)
        }
    }

    impl<D> DeserializeWith<Archived<Timestamp>, DateTime<Utc>, D> for UtcDateTime
    where
        D: Fallible + ?Sized,
        D::Error: Source,
        Archived<Timestamp>: Deserialize<Timestamp, D>,
    {
        fn deserialize_with(
            field: &Archived<Timestamp>,
            deserializer: &mut D,
        ) -> Result<DateTime<Utc>, D::Error> {
            let (seconds, nanoseconds) = field.deserialize(deserializer)?;

            DateTime::from_timestamp(seconds, nanoseconds)
                .ok_or_else(|| D::Error::new(InvalidTimestamp))
        }
    }

    fn timestamp(date_time: &DateTime<Utc>) -> Timestamp {
        (date_time.timestamp(), date_time.timestamp_subsec_nanos())
    }

    #[derive(Debug)]
    struct InvalidTimestamp;

    impl fmt::Display for InvalidTimestamp {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("archived article contains an invalid timestamp")
        }
    }

    impl Error for InvalidTimestamp {}
}

#[cfg(all(test, any(feature = "rkyv", feature = "serde")))]
mod tests {
    fn article() -> super::Article {
        let published_at = chrono::DateTime::parse_from_rfc3339("2026-06-01T18:30:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);

        super::Article::new(
            "Example headline".to_string(),
            "https://example.com/article".to_string(),
            Some(vec!["Reporter".to_string()]),
            Some(published_at),
            Some("https://example.com/image.jpg".to_string()),
        )
        .with_inline_content("<p>Runtime-only content source.</p>")
    }

    fn assert_article_fields_match(actual: &super::Article, expected: &super::Article) {
        assert_eq!(actual.title(), expected.title());
        assert_eq!(actual.url(), expected.url());
        assert_eq!(actual.authors(), expected.authors());
        assert_eq!(actual.published_at(), expected.published_at());
        assert_eq!(actual.thumbnail_url(), expected.thumbnail_url());
        assert!(matches!(
            actual.content_source(),
            super::ArticleContentSource::WebPage
        ));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn article_round_trips_through_json() {
        let article = article();
        let json = serde_json::to_string(&article).unwrap();
        let decoded: super::Article = serde_json::from_str(&json).unwrap();

        assert_article_fields_match(&decoded, &article);
        assert!(!json.contains("content_source"));
    }

    #[cfg(feature = "rkyv")]
    #[test]
    fn article_round_trips_through_rkyv() {
        let article = article();
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&article).unwrap();
        let archived =
            rkyv::access::<rkyv::Archived<super::Article>, rkyv::rancor::Error>(&bytes).unwrap();
        let decoded = rkyv::deserialize::<super::Article, rkyv::rancor::Error>(archived).unwrap();

        assert_article_fields_match(&decoded, &article);
    }
}
