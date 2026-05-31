use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use news_ag::{
    models::Article,
    parse::{json::wordpress, news_sitemap, rss},
};
use std::fmt::Write;
use std::hint::black_box;

const ENTRY_COUNTS: [usize; 3] = [1, 25, 100];

fn benchmark_parsers(c: &mut Criterion) {
    benchmark_parser(c, "parse/rss", rss_fixture, rss::parse_body);
    benchmark_parser(c, "parse/atom", atom_fixture, rss::parse_body);
    benchmark_parser(
        c,
        "parse/google_news_sitemap",
        google_news_sitemap_fixture,
        news_sitemap::parse_body,
    );
    benchmark_parser(
        c,
        "parse/wordpress_json",
        wordpress_fixture,
        wordpress::parse_posts,
    );
}

fn benchmark_parser(
    c: &mut Criterion,
    name: &str,
    fixture: fn(usize) -> String,
    parse: fn(&str) -> Vec<Article>,
) {
    let mut group = c.benchmark_group(name);

    for entry_count in ENTRY_COUNTS {
        let body = fixture(entry_count);
        assert_eq!(parse(&body).len(), entry_count);

        group.throughput(Throughput::Bytes(body.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("entries", entry_count),
            &body,
            |b, body| b.iter(|| black_box(parse(black_box(body.as_str())))),
        );
    }

    group.finish();
}

fn rss_fixture(entry_count: usize) -> String {
    let mut body = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:dc="http://purl.org/dc/elements/1.1/">
            <channel>
                <title>Example Feed</title>
                <link>https://example.com</link>
                <description>Example</description>
        "#,
    );

    for index in 0..entry_count {
        write!(
            body,
            r#"
                <item>
                    <title><![CDATA[Example &amp; headline {index}]]></title>
                    <link>https://example.com/article/{index}</link>
                    <dc:creator><![CDATA[Reporter {index}]]></dc:creator>
                    <pubDate>Sat, 23 May 2026 06:00:00 GMT</pubDate>
                </item>
            "#
        )
        .unwrap();
    }

    body.push_str("</channel></rss>");
    body
}

fn atom_fixture(entry_count: usize) -> String {
    let mut body = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
            <title>Example Feed</title>
            <id>https://example.com</id>
            <updated>2026-05-23T06:00:00Z</updated>
        "#,
    );

    for index in 0..entry_count {
        write!(
            body,
            r#"
                <entry>
                    <title>Example headline {index}</title>
                    <link href="https://example.com/article/{index}" rel="alternate"/>
                    <id>https://example.com/article/{index}</id>
                    <updated>2026-05-23T06:00:00Z</updated>
                    <author><name>Reporter {index}</name></author>
                </entry>
            "#
        )
        .unwrap();
    }

    body.push_str("</feed>");
    body
}

fn google_news_sitemap_fixture(entry_count: usize) -> String {
    let mut body = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
                xmlns:article="http://www.google.com/schemas/sitemap-news/0.9">
        "#,
    );

    for index in 0..entry_count {
        write!(
            body,
            r#"
                <url>
                    <loc>https://example.com/article/{index}</loc>
                    <article:news>
                        <article:publication_date>2026-05-23T06:00:00Z</article:publication_date>
                        <article:title><![CDATA[Example & headline {index}]]></article:title>
                    </article:news>
                </url>
            "#
        )
        .unwrap();
    }

    body.push_str("</urlset>");
    body
}

fn wordpress_fixture(entry_count: usize) -> String {
    let mut body = String::from("[");

    for index in 0..entry_count {
        if index > 0 {
            body.push(',');
        }

        write!(
            body,
            r#"{{
                "date": "2026-05-23T06:00:00",
                "date_gmt": "2026-05-23T06:00:00",
                "link": "https://example.com/article/{index}",
                "title": {{ "rendered": "Example <em>headline</em> {index}" }},
                "yoast_head_json": {{ "author": "Reporter {index}" }}
            }}"#
        )
        .unwrap();
    }

    body.push(']');
    body
}

criterion_group!(benches, benchmark_parsers);
criterion_main!(benches);
