use news_ag::{
    SourceInfo, all_sources, sources_with_scope, us_sources, world_sources, politics_sources,
    business_sources, tech_sources, entertainment_sources, sports_sources, science_sources,
    health_sources,
    source::endpoint::EndpointScope,
};

#[test]
fn all_sources_returns_non_empty_list() {
    let sources = all_sources();
    assert!(!sources.is_empty(), "all_sources() should return at least one source");
}

#[test]
fn all_sources_have_distinct_ids() {
    let sources = all_sources();
    let mut ids: Vec<_> = sources.iter().map(|s| s.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(
        ids.len(),
        sources.len(),
        "all sources should have distinct IDs"
    );
}

#[test]
fn all_sources_have_non_empty_names() {
    let sources = all_sources();
    for source in &sources {
        assert!(
            !source.name.is_empty(),
            "source {:?} should have a non-empty name",
            source.id
        );
    }
}

#[test]
fn all_sources_have_logo_url() {
    let sources = all_sources();
    for source in &sources {
        assert!(
            !source.logo_url.is_empty(),
            "source {} should have a non-empty logo_url",
            source.name
        );
        assert!(
            source.logo_url.starts_with("http"),
            "source {} logo_url should be a URL, got {}",
            source.name,
            source.logo_url
        );
    }
}

#[test]
fn all_sources_have_at_least_one_scope() {
    let sources = all_sources();
    for source in &sources {
        assert!(
            !source.scopes.is_empty(),
            "source {} should have at least one endpoint scope",
            source.name
        );
    }
}

#[test]
fn us_sources_have_us_scope() {
    let sources = us_sources();
    assert!(!sources.is_empty());
    for source in &sources {
        assert!(
            source.scopes.contains(&EndpointScope::US),
            "source {} returned by us_sources() should have US scope",
            source.name
        );
    }
}

#[test]
fn world_sources_have_world_scope() {
    let sources = world_sources();
    assert!(!sources.is_empty());
    for source in &sources {
        assert!(
            source.scopes.contains(&EndpointScope::World),
            "source {} returned by world_sources() should have World scope",
            source.name
        );
    }
}

#[test]
fn politics_sources_have_politics_scope() {
    let sources = politics_sources();
    assert!(!sources.is_empty());
    for source in &sources {
        assert!(
            source.scopes.contains(&EndpointScope::Politics),
            "source {} returned by politics_sources() should have Politics scope",
            source.name
        );
    }
}

#[test]
fn business_sources_have_business_scope() {
    let sources = business_sources();
    assert!(!sources.is_empty());
    for source in &sources {
        assert!(
            source.scopes.contains(&EndpointScope::Business),
            "source {} returned by business_sources() should have Business scope",
            source.name
        );
    }
}

#[test]
fn tech_sources_have_technology_scope() {
    let sources = tech_sources();
    assert!(!sources.is_empty());
    for source in &sources {
        assert!(
            source.scopes.contains(&EndpointScope::Technology),
            "source {} returned by tech_sources() should have Technology scope",
            source.name
        );
    }
}

#[test]
fn entertainment_sources_have_entertainment_scope() {
    let sources = entertainment_sources();
    assert!(!sources.is_empty());
    for source in &sources {
        assert!(
            source.scopes.contains(&EndpointScope::Entertainment),
            "source {} returned by entertainment_sources() should have Entertainment scope",
            source.name
        );
    }
}

#[test]
fn sports_sources_have_sports_scope() {
    let sources = sports_sources();
    assert!(!sources.is_empty());
    for source in &sources {
        assert!(
            source.scopes.contains(&EndpointScope::Sports),
            "source {} returned by sports_sources() should have Sports scope",
            source.name
        );
    }
}

#[test]
fn science_sources_have_science_scope() {
    let sources = science_sources();
    assert!(!sources.is_empty());
    for source in &sources {
        assert!(
            source.scopes.contains(&EndpointScope::Science),
            "source {} returned by science_sources() should have Science scope",
            source.name
        );
    }
}

#[test]
fn health_sources_have_health_scope() {
    let sources = health_sources();
    assert!(!sources.is_empty());
    for source in &sources {
        assert!(
            source.scopes.contains(&EndpointScope::Health),
            "source {} returned by health_sources() should have Health scope",
            source.name
        );
    }
}

#[test]
fn sources_with_scope_matches_filtered_list() {
    let all = all_sources();
    let us = sources_with_scope(EndpointScope::US);
    let from_all: Vec<&SourceInfo> = all
        .iter()
        .filter(|s| s.scopes.contains(&EndpointScope::US))
        .collect();
    assert_eq!(us.len(), from_all.len());
}

#[test]
fn topic_sources_are_subset_of_all() {
    let all = all_sources();
    let all_names: Vec<&str> = all.iter().map(|s| s.name).collect();

    for source in us_sources() {
        assert!(
            all_names.contains(&source.name),
            "us source {} not found in all_sources()",
            source.name
        );
    }
    for source in tech_sources() {
        assert!(
            all_names.contains(&source.name),
            "tech source {} not found in all_sources()",
            source.name
        );
    }
}
