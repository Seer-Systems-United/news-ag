use news_ag::{
    AbcNews, AmericanProspect, ArsTechnica, Axios, Billboard, BusinessInsider, CbsNews,
    ChristianScienceMonitor, CincinnatiEnquirer, Clarin, Cnbc, ColumbusDispatch, DailyKos,
    DailyWire, DerSpiegel, DetroitFreePress, DeutscheWelle, ElPais, Empire, FinancialTimes,
    FolhaDeSPaulo, Forbes, ForeignPolicy, Fortune, FoxNews, France24, Gizmodo, Haaretz,
    HuffingtonPost, Ign, IndianapolisStar, IrishTimes, Jacobin, JapanTimes, JerusalemPost, Kotaku,
    KyivIndependent, LeMonde, LouisvilleCourierJournal, MiddleEastEye, MoscowTimes, MotherJones,
    Msnbc, NashvilleTennessean, Nation, NationalReview, Nature, NbcNews, NewRepublic, NewScientist,
    NewStatesman, NewYorkPost, Newsweek, NikkeiAsia, OklahomaCityOklahoman, Pitchfork, Politico,
    Polygon, ProPublica, ProvidenceJournal, Quartz, Reason, RollingStone, ScienceMagazine,
    ScientificAmerican, SkyNews, SouthChinaMorningPost, Spectator, StraitsTimes, TheArtNewspaper,
    TheHollywoodReporter, Time, TimesOfIndia, UsaToday, Variety, ViceNews, Wired,
};
use news_ag::{
    AlJazeera, AtlantaJournalConstitution, Atlantic, BaltimoreSun, BbcNews, Bloomberg, BostonGlobe,
    CalgaryHerald, CharlestonGazette, CharlestonGazetteMail, CharlestonPostAndCourier,
    ChicagoTribune, ClevelandPlainDealer, Cnn, DailyBeast, DailyCaller, DailyMail, DailyTelegraph,
    DallasMorningNews, DemocracyNow, DenverPost, Economist, EdmontonJournal, Engadget, Express,
    GlobeAndMail, Guardian, HalifaxChronicleHerald, HartfordCourant, HoustonChronicle, Independent,
    LosAngelesTimes, MinneapolisStarTribune, MontrealGazette, NationalPost, NewYorkTimes,
    NewYorker, Npr, OrlandoSentinel, OttawaCitizen, PhiladelphiaInquirer, PittsburghPostGazette,
    ReginaLeaderPost, RichmondTimesDispatch, SaskatoonStarPhoenix, SeattleTimes, StJohnsTelegram,
    StLouisPostDispatch, SydneyMorningHerald, TampaBayTimes, TechCrunch, Telegraph, TheAge,
    TheAustralian, TheBlaze, TheHill, TheIntercept, TheMirror, TheSun, TheTimes, TheVerge,
    TorontoStar, VancouverSun, Vox, WallStreetJournal, WashingtonPost, Wgn, WinnipegFreePress,
    YoungTurks, source::Source,
};
use news_ag::{models::Article, source::endpoint::Endpoint};

#[cfg(not(feature = "async"))]
fn get_articles(endpoint: &Endpoint) -> Vec<Article> {
    endpoint.get_articles()
}

#[cfg(feature = "async")]
fn get_articles(endpoint: &Endpoint) -> Vec<Article> {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { endpoint.get_articles().await })
}

fn assert_source_has_articles<S: Source>(source_name: &str) {
    let endpoints = S::endpoints();

    assert!(
        !endpoints.is_empty(),
        "expected {source_name} to define at least one endpoint"
    );

    let has_article = endpoints.into_iter().any(|endpoint| {
        let articles = get_articles(&endpoint);
        articles
            .iter()
            .any(|article| !article.title.trim().is_empty() && !article.url.trim().is_empty())
    });

    assert!(
        has_article,
        "expected {source_name} endpoints to return at least one article with title and url"
    );
}

macro_rules! source_test {
    ($test_name:ident, $source:ty) => {
        #[test]
        fn $test_name() {
            assert_source_has_articles::<$source>(stringify!($test_name));
        }
    };
}

source_test!(abc_news_returns_articles, AbcNews);
source_test!(american_prospect_returns_articles, AmericanProspect);
source_test!(ars_technica_returns_articles, ArsTechnica);
source_test!(axios_returns_articles, Axios);
source_test!(billboard_returns_articles, Billboard);
source_test!(business_insider_returns_articles, BusinessInsider);
source_test!(cbs_news_returns_articles, CbsNews);
source_test!(
    christian_science_monitor_returns_articles,
    ChristianScienceMonitor
);
source_test!(cincinnati_enquirer_returns_articles, CincinnatiEnquirer);
source_test!(clarin_returns_articles, Clarin);
source_test!(cnbc_returns_articles, Cnbc);
source_test!(columbus_dispatch_returns_articles, ColumbusDispatch);
source_test!(daily_kos_returns_articles, DailyKos);
source_test!(daily_wire_returns_articles, DailyWire);
source_test!(der_spiegel_returns_articles, DerSpiegel);
source_test!(detroit_free_press_returns_articles, DetroitFreePress);
source_test!(deutsche_welle_returns_articles, DeutscheWelle);
source_test!(el_pais_returns_articles, ElPais);
source_test!(empire_returns_articles, Empire);
source_test!(financial_times_returns_articles, FinancialTimes);
source_test!(folha_de_s_paulo_returns_articles, FolhaDeSPaulo);
source_test!(forbes_returns_articles, Forbes);
source_test!(foreign_policy_returns_articles, ForeignPolicy);
source_test!(fortune_returns_articles, Fortune);
source_test!(fox_news_returns_articles, FoxNews);
source_test!(france_24_returns_articles, France24);
source_test!(gizmodo_returns_articles, Gizmodo);
source_test!(haaretz_returns_articles, Haaretz);
source_test!(huffington_post_returns_articles, HuffingtonPost);
source_test!(ign_returns_articles, Ign);
source_test!(indianapolis_star_returns_articles, IndianapolisStar);
source_test!(irish_times_returns_articles, IrishTimes);
source_test!(jacobin_returns_articles, Jacobin);
source_test!(japan_times_returns_articles, JapanTimes);
source_test!(jerusalem_post_returns_articles, JerusalemPost);
source_test!(kotaku_returns_articles, Kotaku);
source_test!(kyiv_independent_returns_articles, KyivIndependent);
source_test!(le_monde_returns_articles, LeMonde);
source_test!(
    louisville_courier_journal_returns_articles,
    LouisvilleCourierJournal
);
source_test!(middle_east_eye_returns_articles, MiddleEastEye);
source_test!(moscow_times_returns_articles, MoscowTimes);
source_test!(mother_jones_returns_articles, MotherJones);
source_test!(msnbc_returns_articles, Msnbc);
source_test!(nashville_tennessean_returns_articles, NashvilleTennessean);
source_test!(nation_returns_articles, Nation);
source_test!(national_review_returns_articles, NationalReview);
source_test!(nature_returns_articles, Nature);
source_test!(nbc_news_returns_articles, NbcNews);
source_test!(new_republic_returns_articles, NewRepublic);
source_test!(new_scientist_returns_articles, NewScientist);
source_test!(new_statesman_returns_articles, NewStatesman);
source_test!(new_york_post_returns_articles, NewYorkPost);
source_test!(newsweek_returns_articles, Newsweek);
source_test!(nikkei_asia_returns_articles, NikkeiAsia);
source_test!(
    oklahoma_city_oklahoman_returns_articles,
    OklahomaCityOklahoman
);
source_test!(pitchfork_returns_articles, Pitchfork);
source_test!(politico_returns_articles, Politico);
source_test!(polygon_returns_articles, Polygon);
source_test!(propublica_returns_articles, ProPublica);
source_test!(providence_journal_returns_articles, ProvidenceJournal);
source_test!(quartz_returns_articles, Quartz);
source_test!(reason_returns_articles, Reason);
source_test!(rolling_stone_returns_articles, RollingStone);
source_test!(science_magazine_returns_articles, ScienceMagazine);
source_test!(scientific_american_returns_articles, ScientificAmerican);
source_test!(sky_news_returns_articles, SkyNews);
source_test!(
    south_china_morning_post_returns_articles,
    SouthChinaMorningPost
);
source_test!(spectator_returns_articles, Spectator);
source_test!(straits_times_returns_articles, StraitsTimes);
source_test!(the_art_newspaper_returns_articles, TheArtNewspaper);
source_test!(
    the_hollywood_reporter_returns_articles,
    TheHollywoodReporter
);
source_test!(time_returns_articles, Time);
source_test!(times_of_india_returns_articles, TimesOfIndia);
source_test!(usa_today_returns_articles, UsaToday);
source_test!(variety_returns_articles, Variety);
source_test!(vice_news_returns_articles, ViceNews);
source_test!(wired_returns_articles, Wired);

#[test]
fn al_jazeera_returns_articles() {
    assert_source_has_articles::<AlJazeera>("al_jazeera");
}

#[test]
fn atlantic_returns_articles() {
    assert_source_has_articles::<Atlantic>("atlantic");
}

#[test]
fn atlanta_journal_constitution_returns_articles() {
    assert_source_has_articles::<AtlantaJournalConstitution>("atlanta_journal_constitution");
}

#[test]
fn baltimore_sun_returns_articles() {
    assert_source_has_articles::<BaltimoreSun>("baltimore_sun");
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
fn charleston_gazette_returns_articles() {
    assert_source_has_articles::<CharlestonGazette>("charleston_gazette");
}

#[test]
fn charleston_gazette_mail_returns_articles() {
    assert_source_has_articles::<CharlestonGazetteMail>("charleston_gazette_mail");
}

#[test]
fn charleston_post_and_courier_returns_articles() {
    assert_source_has_articles::<CharlestonPostAndCourier>("charleston_post_and_courier");
}

#[test]
fn chicago_tribune_returns_articles() {
    assert_source_has_articles::<ChicagoTribune>("chicago_tribune");
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
fn daily_beast_returns_articles() {
    assert_source_has_articles::<DailyBeast>("daily_beast");
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
fn dallas_morning_news_returns_articles() {
    assert_source_has_articles::<DallasMorningNews>("dallas_morning_news");
}

#[test]
fn democracy_now_returns_articles() {
    assert_source_has_articles::<DemocracyNow>("democracy_now");
}

#[test]
fn denver_post_returns_articles() {
    assert_source_has_articles::<DenverPost>("denver_post");
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
fn halifax_chronicle_herald_returns_articles() {
    assert_source_has_articles::<HalifaxChronicleHerald>("halifax_chronicle_herald");
}

#[test]
fn hartford_courant_returns_articles() {
    assert_source_has_articles::<HartfordCourant>("hartford_courant");
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
fn montreal_gazette_returns_articles() {
    assert_source_has_articles::<MontrealGazette>("montreal_gazette");
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
fn orlando_sentinel_returns_articles() {
    assert_source_has_articles::<OrlandoSentinel>("orlando_sentinel");
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
fn seattle_times_returns_articles() {
    assert_source_has_articles::<SeattleTimes>("seattle_times");
}

#[test]
fn st_johns_telegram_returns_articles() {
    assert_source_has_articles::<StJohnsTelegram>("st_johns_telegram");
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
fn tampa_bay_times_returns_articles() {
    assert_source_has_articles::<TampaBayTimes>("tampa_bay_times");
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
fn the_times_returns_articles() {
    assert_source_has_articles::<TheTimes>("the_times");
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

#[test]
fn young_turks_returns_articles() {
    assert_source_has_articles::<YoungTurks>("young_turks");
}
