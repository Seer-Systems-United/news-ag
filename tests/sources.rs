use news_ag::{
    al_jazeera::AlJazeera, atlantic::Atlantic, bbc_news::BbcNews, bloomberg::Bloomberg,
    boston_globe::BostonGlobe, calgary_herald::CalgaryHerald,
    charleston_post_and_courier::CharlestonPostAndCourier,
    cleveland_plain_dealer::ClevelandPlainDealer, cnn::Cnn, daily_caller::DailyCaller,
    daily_mail::DailyMail, daily_telegraph::DailyTelegraph, democracy_now::DemocracyNow,
    economist::Economist, edmonton_journal::EdmontonJournal, engadget::Engadget, express::Express,
    globe_and_mail::GlobeAndMail, guardian::Guardian, houston_chronicle::HoustonChronicle,
    independent::Independent, los_angeles_times::LosAngelesTimes,
    minneapolis_star_tribune::MinneapolisStarTribune, national_post::NationalPost,
    new_york_times::NewYorkTimes, new_yorker::NewYorker, npr::Npr, ottawa_citizen::OttawaCitizen,
    philadelphia_inquirer::PhiladelphiaInquirer, pittsburgh_post_gazette::PittsburghPostGazette,
    regina_leader_post::ReginaLeaderPost, richmond_times_dispatch::RichmondTimesDispatch,
    saskatoon_starphoenix::SaskatoonStarPhoenix, source::Source,
    st_louis_post_dispatch::StLouisPostDispatch, sydney_morning_herald::SydneyMorningHerald,
    techcrunch::TechCrunch, telegraph::Telegraph, the_age::TheAge, the_australian::TheAustralian,
    the_blaze::TheBlaze, the_hill::TheHill, the_intercept::TheIntercept, the_mirror::TheMirror,
    the_sun::TheSun, the_verge::TheVerge, toronto_star::TorontoStar, vancouver_sun::VancouverSun,
    vox::Vox, wall_street_journal::WallStreetJournal, washington_post::WashingtonPost, wgn::Wgn,
    winnipeg_free_press::WinnipegFreePress,
};

fn assert_source_has_articles<S: Source>(source_name: &str) {
    let endpoints = S::endpoints();

    assert!(
        !endpoints.is_empty(),
        "expected {source_name} to define at least one endpoint"
    );

    let has_article = endpoints.into_iter().any(|endpoint| {
        let articles = endpoint.get_articles();
        articles
            .iter()
            .any(|article| !article.title.trim().is_empty() && !article.url.trim().is_empty())
    });

    assert!(
        has_article,
        "expected {source_name} endpoints to return at least one article with title and url"
    );
}

#[test]
fn al_jazeera_returns_articles() {
    assert_source_has_articles::<AlJazeera>("al_jazeera");
}

#[test]
fn atlantic_returns_articles() {
    assert_source_has_articles::<Atlantic>("atlantic");
}

#[test]
fn bbc_news_returns_articles() {
    assert_source_has_articles::<BbcNews>("bbc_news");
}

#[test]
fn bloomberg_returns_articles() {
    assert_source_has_articles::<Bloomberg>("bloomberg");
}

#[test]
fn boston_globe_returns_articles() {
    assert_source_has_articles::<BostonGlobe>("boston_globe");
}

#[test]
fn calgary_herald_returns_articles() {
    assert_source_has_articles::<CalgaryHerald>("calgary_herald");
}

#[test]
fn charleston_post_and_courier_returns_articles() {
    assert_source_has_articles::<CharlestonPostAndCourier>("charleston_post_and_courier");
}

#[test]
fn cleveland_plain_dealer_returns_articles() {
    assert_source_has_articles::<ClevelandPlainDealer>("cleveland_plain_dealer");
}

#[test]
fn cnn_returns_articles() {
    assert_source_has_articles::<Cnn>("cnn");
}

#[test]
fn daily_caller_returns_articles() {
    assert_source_has_articles::<DailyCaller>("daily_caller");
}

#[test]
fn daily_mail_returns_articles() {
    assert_source_has_articles::<DailyMail>("daily_mail");
}

#[test]
fn daily_telegraph_returns_articles() {
    assert_source_has_articles::<DailyTelegraph>("daily_telegraph");
}

#[test]
fn democracy_now_returns_articles() {
    assert_source_has_articles::<DemocracyNow>("democracy_now");
}

#[test]
fn economist_returns_articles() {
    assert_source_has_articles::<Economist>("economist");
}

#[test]
fn edmonton_journal_returns_articles() {
    assert_source_has_articles::<EdmontonJournal>("edmonton_journal");
}

#[test]
fn engadget_returns_articles() {
    assert_source_has_articles::<Engadget>("engadget");
}

#[test]
fn express_returns_articles() {
    assert_source_has_articles::<Express>("express");
}

#[test]
fn globe_and_mail_returns_articles() {
    assert_source_has_articles::<GlobeAndMail>("globe_and_mail");
}

#[test]
fn guardian_returns_articles() {
    assert_source_has_articles::<Guardian>("guardian");
}

#[test]
fn houston_chronicle_returns_articles() {
    assert_source_has_articles::<HoustonChronicle>("houston_chronicle");
}

#[test]
fn independent_returns_articles() {
    assert_source_has_articles::<Independent>("independent");
}

#[test]
fn los_angeles_times_returns_articles() {
    assert_source_has_articles::<LosAngelesTimes>("los_angeles_times");
}

#[test]
fn minneapolis_star_tribune_returns_articles() {
    assert_source_has_articles::<MinneapolisStarTribune>("minneapolis_star_tribune");
}

#[test]
fn national_post_returns_articles() {
    assert_source_has_articles::<NationalPost>("national_post");
}

#[test]
fn new_york_times_returns_articles() {
    assert_source_has_articles::<NewYorkTimes>("new_york_times");
}

#[test]
fn new_yorker_returns_articles() {
    assert_source_has_articles::<NewYorker>("new_yorker");
}

#[test]
fn npr_returns_articles() {
    assert_source_has_articles::<Npr>("npr");
}

#[test]
fn ottawa_citizen_returns_articles() {
    assert_source_has_articles::<OttawaCitizen>("ottawa_citizen");
}

#[test]
fn philadelphia_inquirer_returns_articles() {
    assert_source_has_articles::<PhiladelphiaInquirer>("philadelphia_inquirer");
}

#[test]
fn pittsburgh_post_gazette_returns_articles() {
    assert_source_has_articles::<PittsburghPostGazette>("pittsburgh_post_gazette");
}

#[test]
fn regina_leader_post_returns_articles() {
    assert_source_has_articles::<ReginaLeaderPost>("regina_leader_post");
}

#[test]
fn richmond_times_dispatch_returns_articles() {
    assert_source_has_articles::<RichmondTimesDispatch>("richmond_times_dispatch");
}

#[test]
fn saskatoon_starphoenix_returns_articles() {
    assert_source_has_articles::<SaskatoonStarPhoenix>("saskatoon_starphoenix");
}

#[test]
fn st_louis_post_dispatch_returns_articles() {
    assert_source_has_articles::<StLouisPostDispatch>("st_louis_post_dispatch");
}

#[test]
fn sydney_morning_herald_returns_articles() {
    assert_source_has_articles::<SydneyMorningHerald>("sydney_morning_herald");
}

#[test]
fn techcrunch_returns_articles() {
    assert_source_has_articles::<TechCrunch>("techcrunch");
}

#[test]
fn telegraph_returns_articles() {
    assert_source_has_articles::<Telegraph>("telegraph");
}

#[test]
fn the_age_returns_articles() {
    assert_source_has_articles::<TheAge>("the_age");
}

#[test]
fn the_australian_returns_articles() {
    assert_source_has_articles::<TheAustralian>("the_australian");
}

#[test]
fn the_blaze_returns_articles() {
    assert_source_has_articles::<TheBlaze>("the_blaze");
}

#[test]
fn the_hill_returns_articles() {
    assert_source_has_articles::<TheHill>("the_hill");
}

#[test]
fn the_intercept_returns_articles() {
    assert_source_has_articles::<TheIntercept>("the_intercept");
}

#[test]
fn the_mirror_returns_articles() {
    assert_source_has_articles::<TheMirror>("the_mirror");
}

#[test]
fn the_sun_returns_articles() {
    assert_source_has_articles::<TheSun>("the_sun");
}

#[test]
fn the_verge_returns_articles() {
    assert_source_has_articles::<TheVerge>("the_verge");
}

#[test]
fn toronto_star_returns_articles() {
    assert_source_has_articles::<TorontoStar>("toronto_star");
}

#[test]
fn vancouver_sun_returns_articles() {
    assert_source_has_articles::<VancouverSun>("vancouver_sun");
}

#[test]
fn vox_returns_articles() {
    assert_source_has_articles::<Vox>("vox");
}

#[test]
fn wall_street_journal_returns_articles() {
    assert_source_has_articles::<WallStreetJournal>("wall_street_journal");
}

#[test]
fn washington_post_returns_articles() {
    assert_source_has_articles::<WashingtonPost>("washington_post");
}

#[test]
fn wgn_returns_articles() {
    assert_source_has_articles::<Wgn>("wgn");
}

#[test]
fn winnipeg_free_press_returns_articles() {
    assert_source_has_articles::<WinnipegFreePress>("winnipeg_free_press");
}
