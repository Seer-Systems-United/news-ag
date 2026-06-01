use news_ag::{
    AbcNews, AlJazeera, AlMonitor, AmericanConservative, AmericanProspect, ApNews, ArsTechnica,
    AtlantaJournalConstitution, Atlantic, Axios, BaltimoreSun, BangkokPost, BbcNews,
    BelfastTelegraph, Billboard, Bloomberg, BostonGlobe, BusinessInsider, CalgaryHerald, CbsNews,
    CharlestonGazette, CharlestonGazetteMail, CharlestonPostAndCourier, ChicagoTribune,
    ChristianScienceMonitor, CincinnatiEnquirer, Clarin, ClevelandPlainDealer, Cnbc, Cnet, Cnn,
    ColumbusDispatch, CommonDreams, CorriereDellaSera, DailyBeast, DailyCaller, DailyKos,
    DailyMail, DailyTelegraph, DailyWire, DallasMorningNews, DemocracyNow, DenverPost, DerSpiegel,
    DetroitFreePress, DeutscheWelle, DropSiteNews, Economist, EdmontonJournal, ElPais, Empire,
    Engadget, Euronews, Express, FastCompany, FinancialTimes, FolhaDeSPaulo, Forbes, ForeignPolicy,
    Fortune, FoxNews, France24, FrankfurterAllgemeineZeitung, GameSpot, Gizmodo, GlobalNews,
    GlobeAndMail, Guardian, Haaretz, HalifaxChronicleHerald, HartfordCourant, HeraldScotland,
    HoustonChronicle, HuffingtonPost, Ign, Independent, IndianapolisStar, IrishTimes, Jacobin,
    JapanTimes, JerusalemPost, Kiplinger, Kotaku, KyivIndependent, LaPresse, LaRepubblica, LeMonde,
    LosAngelesTimes, LouisvilleCourierJournal, MarketWatch, Mashable, MiddleEastEye,
    MinneapolisStarTribune, MontrealGazette, MoscowTimes, MotherJones, Msnbc, NashvilleTennessean,
    Nation, NationalPost, NationalReview, Nature, NbcNews, NewRepublic, NewScientist, NewStatesman,
    NewYorkPost, NewYorkTimes, NewYorker, Newsweek, NikkeiAsia, Nme, Npr, OklahomaCityOklahoman,
    Oregonian, OrlandoSentinel, OttawaCitizen, Pcmag, PhiladelphiaInquirer, Pitchfork,
    PittsburghPostGazette, Politico, Polygon, ProPublica, ProvidenceJournal, Quartz, Reason,
    ReginaLeaderPost, Reuters, RichmondTimesDispatch, RollingStone, SaskatoonStarPhoenix,
    ScienceMagazine, ScientificAmerican, Scotsman, ScreenRant, SeattleTimes, SkyNews,
    SouthChinaMorningPost, Spectator, StJohnsTelegram, StLouisPostDispatch, StarLedger,
    StraitsTimes, SydneyMorningHerald, TampaBayTimes, TechCrunch, Telegraph, TheAge,
    TheArtNewspaper, TheAustralian, TheBlaze, TheBulwark, TheHill, TheHindu, TheHollywoodReporter,
    TheIntercept, TheMirror, TheSun, TheTimes, TheVerge, Time, TimesOfIndia, TorontoStar,
    UnitedPressInternational, UsaToday, VancouverSun, Variety, VentureBeat, ViceNews, Vox,
    WallStreetJournal, WashingtonPost, Wgn, WinnipegFreePress, Wired, YoungTurks, Zeteo,
    source::Source,
};

const ARTICLE_ATTEMPT_LIMIT: usize = 3;

fn validate_normalized_plain_text(content: &str) -> Result<(), &'static str> {
    if content.split_whitespace().count() <= 10 {
        return Err("content did not contain enough text");
    }
    if content.contains(['\n', '\r', '\t']) {
        return Err("content contained control whitespace");
    }
    if content.contains("\\n") || content.contains("\\r") || content.contains("\\t") {
        return Err("content contained literal whitespace escapes");
    }
    if content.contains("<p>") {
        return Err("content contained HTML markup");
    }

    Ok(())
}

fn failure_message(source_name: &str, failures: Vec<String>) -> String {
    format!(
        "expected {source_name} to return retrievable normalized content; attempts:\n{}",
        failures.join("\n")
    )
}

#[cfg(not(feature = "async"))]
fn assert_source_has_content<S: Source>(source_name: &str) {
    let mut failures = Vec::new();
    let mut article_attempts = 0;

    'endpoints: for endpoint in S::endpoints() {
        for article in endpoint
            .get_articles()
            .into_iter()
            .take(ARTICLE_ATTEMPT_LIMIT - article_attempts)
        {
            article_attempts += 1;

            match article.get_content() {
                Ok(content) => match validate_normalized_plain_text(&content) {
                    Ok(()) => return,
                    Err(error) => failures.push(format!("{}: {error}", article.url())),
                },
                Err(error) => failures.push(format!("{}: {error}", article.url())),
            }

            if article_attempts == ARTICLE_ATTEMPT_LIMIT {
                break 'endpoints;
            }
        }
    }

    panic!("{}", failure_message(source_name, failures));
}

#[cfg(feature = "async")]
async fn assert_source_has_content<S: Source>(source_name: &str) {
    let mut failures = Vec::new();
    let mut article_attempts = 0;

    'endpoints: for endpoint in S::endpoints() {
        for article in endpoint
            .get_articles()
            .await
            .into_iter()
            .take(ARTICLE_ATTEMPT_LIMIT - article_attempts)
        {
            article_attempts += 1;

            match article.get_content().await {
                Ok(content) => match validate_normalized_plain_text(&content) {
                    Ok(()) => return,
                    Err(error) => failures.push(format!("{}: {error}", article.url())),
                },
                Err(error) => failures.push(format!("{}: {error}", article.url())),
            }

            if article_attempts == ARTICLE_ATTEMPT_LIMIT {
                break 'endpoints;
            }
        }
    }

    panic!("{}", failure_message(source_name, failures));
}

macro_rules! content_test {
    ($(#[$attribute:meta])* $test_name:ident: $source:ty) => {
        #[cfg(not(feature = "async"))]
        #[test]
        $(#[$attribute])*
        fn $test_name() {
            assert_source_has_content::<$source>(stringify!($test_name));
        }

        #[cfg(feature = "async")]
        #[tokio::test]
        $(#[$attribute])*
        async fn $test_name() {
            assert_source_has_content::<$source>(stringify!($test_name)).await;
        }
    };
}

macro_rules! content_tests {
    ($($test_name:ident: $source:ty),+ $(,)?) => {
        $(content_test!($test_name: $source);)+
    };
}

macro_rules! ignored_content_tests {
    ($reason:literal; $($test_name:ident: $source:ty),+ $(,)?) => {
        $(content_test!(#[ignore = $reason] $test_name: $source);)+
    };
}

content_tests!(
    abc_news_returns_content: AbcNews,
    al_jazeera_returns_content: AlJazeera,
    al_monitor_returns_content: AlMonitor,
    american_conservative_returns_content: AmericanConservative,
    american_prospect_returns_content: AmericanProspect,
    apnews_returns_content: ApNews,
    ars_technica_returns_content: ArsTechnica,
    atlanta_journal_constitution_returns_content: AtlantaJournalConstitution,
    atlantic_returns_content: Atlantic,
    axios_returns_content: Axios,
    baltimore_sun_returns_content: BaltimoreSun,
    bangkok_post_returns_content: BangkokPost,
    bbc_news_returns_content: BbcNews,
    belfast_telegraph_returns_content: BelfastTelegraph,
    billboard_returns_content: Billboard,
    boston_globe_returns_content: BostonGlobe,
    business_insider_returns_content: BusinessInsider,
    calgary_herald_returns_content: CalgaryHerald,
    cbs_news_returns_content: CbsNews,
    charleston_gazette_returns_content: CharlestonGazette,
    charleston_gazette_mail_returns_content: CharlestonGazetteMail,
    charleston_post_and_courier_returns_content: CharlestonPostAndCourier,
    chicago_tribune_returns_content: ChicagoTribune,
    christian_science_monitor_returns_content: ChristianScienceMonitor,
    clarin_returns_content: Clarin,
    cleveland_plain_dealer_returns_content: ClevelandPlainDealer,
    cnbc_returns_content: Cnbc,
    cnet_returns_content: Cnet,
    cnn_returns_content: Cnn,
    common_dreams_returns_content: CommonDreams,
    corriere_della_sera_returns_content: CorriereDellaSera,
    daily_beast_returns_content: DailyBeast,
    daily_caller_returns_content: DailyCaller,
    daily_kos_returns_content: DailyKos,
    daily_mail_returns_content: DailyMail,
    daily_wire_returns_content: DailyWire,
    dallas_morning_news_returns_content: DallasMorningNews,
    democracy_now_returns_content: DemocracyNow,
    denver_post_returns_content: DenverPost,
    der_spiegel_returns_content: DerSpiegel,
    deutsche_welle_returns_content: DeutscheWelle,
    drop_site_news_returns_content: DropSiteNews,
    edmonton_journal_returns_content: EdmontonJournal,
    el_pais_returns_content: ElPais,
    empire_returns_content: Empire,
    engadget_returns_content: Engadget,
    euronews_returns_content: Euronews,
    express_returns_content: Express,
    folha_de_s_paulo_returns_content: FolhaDeSPaulo,
    forbes_returns_content: Forbes,
    foreign_policy_returns_content: ForeignPolicy,
    fortune_returns_content: Fortune,
    fox_news_returns_content: FoxNews,
    frankfurter_allgemeine_zeitung_returns_content: FrankfurterAllgemeineZeitung,
    gizmodo_returns_content: Gizmodo,
    global_news_returns_content: GlobalNews,
    globe_and_mail_returns_content: GlobeAndMail,
    guardian_returns_content: Guardian,
    haaretz_returns_content: Haaretz,
    halifax_chronicle_herald_returns_content: HalifaxChronicleHerald,
    hartford_courant_returns_content: HartfordCourant,
    herald_scotland_returns_content: HeraldScotland,
    houston_chronicle_returns_content: HoustonChronicle,
    huffington_post_returns_content: HuffingtonPost,
    ign_returns_content: Ign,
    independent_returns_content: Independent,
    irish_times_returns_content: IrishTimes,
    jacobin_returns_content: Jacobin,
    japan_times_returns_content: JapanTimes,
    jerusalem_post_returns_content: JerusalemPost,
    kiplinger_returns_content: Kiplinger,
    kotaku_returns_content: Kotaku,
    kyiv_independent_returns_content: KyivIndependent,
    la_presse_returns_content: LaPresse,
    la_repubblica_returns_content: LaRepubblica,
    le_monde_returns_content: LeMonde,
    los_angeles_times_returns_content: LosAngelesTimes,
    mashable_returns_content: Mashable,
    middle_east_eye_returns_content: MiddleEastEye,
    montreal_gazette_returns_content: MontrealGazette,
    moscow_times_returns_content: MoscowTimes,
    mother_jones_returns_content: MotherJones,
    msnbc_returns_content: Msnbc,
    nation_returns_content: Nation,
    national_post_returns_content: NationalPost,
    national_review_returns_content: NationalReview,
    nature_returns_content: Nature,
    nbc_news_returns_content: NbcNews,
    new_republic_returns_content: NewRepublic,
    new_scientist_returns_content: NewScientist,
    new_statesman_returns_content: NewStatesman,
    new_york_post_returns_content: NewYorkPost,
    new_yorker_returns_content: NewYorker,
    newsweek_returns_content: Newsweek,
    nikkei_asia_returns_content: NikkeiAsia,
    nme_returns_content: Nme,
    npr_returns_content: Npr,
    oregonian_returns_content: Oregonian,
    orlando_sentinel_returns_content: OrlandoSentinel,
    ottawa_citizen_returns_content: OttawaCitizen,
    pcmag_returns_content: Pcmag,
    philadelphia_inquirer_returns_content: PhiladelphiaInquirer,
    pitchfork_returns_content: Pitchfork,
    pittsburgh_post_gazette_returns_content: PittsburghPostGazette,
    politico_returns_content: Politico,
    polygon_returns_content: Polygon,
    propublica_returns_content: ProPublica,
    quartz_returns_content: Quartz,
    reason_returns_content: Reason,
    regina_leader_post_returns_content: ReginaLeaderPost,
    richmond_times_dispatch_returns_content: RichmondTimesDispatch,
    rolling_stone_returns_content: RollingStone,
    saskatoon_starphoenix_returns_content: SaskatoonStarPhoenix,
    scientific_american_returns_content: ScientificAmerican,
    scotsman_returns_content: Scotsman,
    screen_rant_returns_content: ScreenRant,
    seattle_times_returns_content: SeattleTimes,
    south_china_morning_post_returns_content: SouthChinaMorningPost,
    spectator_returns_content: Spectator,
    st_johns_telegram_returns_content: StJohnsTelegram,
    st_louis_post_dispatch_returns_content: StLouisPostDispatch,
    star_ledger_returns_content: StarLedger,
    straits_times_returns_content: StraitsTimes,
    sydney_morning_herald_returns_content: SydneyMorningHerald,
    tampa_bay_times_returns_content: TampaBayTimes,
    techcrunch_returns_content: TechCrunch,
    the_age_returns_content: TheAge,
    the_art_newspaper_returns_content: TheArtNewspaper,
    the_blaze_returns_content: TheBlaze,
    the_bulwark_returns_content: TheBulwark,
    the_hill_returns_content: TheHill,
    the_hindu_returns_content: TheHindu,
    the_hollywood_reporter_returns_content: TheHollywoodReporter,
    the_intercept_returns_content: TheIntercept,
    the_mirror_returns_content: TheMirror,
    the_times_returns_content: TheTimes,
    the_verge_returns_content: TheVerge,
    time_returns_content: Time,
    times_of_india_returns_content: TimesOfIndia,
    toronto_star_returns_content: TorontoStar,
    united_press_international_returns_content: UnitedPressInternational,
    usa_today_returns_content: UsaToday,
    vancouver_sun_returns_content: VancouverSun,
    variety_returns_content: Variety,
    vice_news_returns_content: ViceNews,
    vox_returns_content: Vox,
    wgn_returns_content: Wgn,
    winnipeg_free_press_returns_content: WinnipegFreePress,
    wired_returns_content: Wired,
    young_turks_returns_content: YoungTurks,
    zeteo_returns_content: Zeteo,
);

ignored_content_tests!(
    "publisher blocks anonymous article content requests";
    bloomberg_returns_content: Bloomberg,
    economist_returns_content: Economist,
    fast_company_returns_content: FastCompany,
    financial_times_returns_content: FinancialTimes,
    france_24_returns_content: France24,
    gamespot_returns_content: GameSpot,
    marketwatch_returns_content: MarketWatch,
    minneapolis_star_tribune_returns_content: MinneapolisStarTribune,
    new_york_times_returns_content: NewYorkTimes,
    reuters_returns_content: Reuters,
    science_magazine_returns_content: ScienceMagazine,
    sky_news_returns_content: SkyNews,
    telegraph_returns_content: Telegraph,
    venturebeat_returns_content: VentureBeat,
    wall_street_journal_returns_content: WallStreetJournal,
    washington_post_returns_content: WashingtonPost,
);

ignored_content_tests!(
    "publisher article URLs do not resolve to anonymous body pages";
    cincinnati_enquirer_returns_content: CincinnatiEnquirer,
    columbus_dispatch_returns_content: ColumbusDispatch,
    detroit_free_press_returns_content: DetroitFreePress,
    indianapolis_star_returns_content: IndianapolisStar,
    louisville_courier_journal_returns_content: LouisvilleCourierJournal,
    nashville_tennessean_returns_content: NashvilleTennessean,
    oklahoma_city_oklahoman_returns_content: OklahomaCityOklahoman,
    providence_journal_returns_content: ProvidenceJournal,
);

ignored_content_tests!(
    "Google News feed URLs do not expose publisher article bodies";
    daily_telegraph_returns_content: DailyTelegraph,
    the_sun_returns_content: TheSun,
    the_australian_returns_content: TheAustralian,
);
