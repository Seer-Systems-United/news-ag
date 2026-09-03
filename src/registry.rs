use crate::source::{Source, endpoint::EndpointScope};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
pub struct SourceInfo {
    pub id: uuid::Uuid,
    pub name: &'static str,
    pub scopes: Vec<EndpointScope>,
}

fn source_info<S: Source>() -> SourceInfo {
    let scopes = S::endpoints().into_iter().map(|e| e.scope).collect();
    SourceInfo {
        id: S::id(),
        name: S::name(),
        scopes,
    }
}

pub fn all_sources() -> Vec<SourceInfo> {
    let mut sources = Vec::new();

    macro_rules! add {
        ($ty:ty) => {
            sources.push(source_info::<$ty>());
        };
    }

    add!(crate::AbcNews);
    add!(crate::AlJazeera);
    add!(crate::AlMonitor);
    add!(crate::Alternet);
    add!(crate::AmericanConservative);
    add!(crate::AmericanProspect);
    add!(crate::ApNews);
    add!(crate::ArabNews);
    add!(crate::ArsTechnica);
    add!(crate::AtlantaJournalConstitution);
    add!(crate::Atlantic);
    add!(crate::Axios);
    add!(crate::BaltimoreSun);
    add!(crate::BangkokPost);
    add!(crate::BangkokPostBusiness);
    add!(crate::BangkokPostWorld);
    add!(crate::BalkanInsight);
    add!(crate::BbcNews);
    add!(crate::BelfastTelegraph);
    add!(crate::Benzinga);
    add!(crate::Billboard);
    add!(crate::Bloomberg);
    add!(crate::BostonGlobe);
    add!(crate::BusinessInsider);
    add!(crate::CalgaryHerald);
    add!(crate::CbsNews);
    add!(crate::CharlestonGazette);
    add!(crate::CharlestonGazetteMail);
    add!(crate::CharlestonPostAndCourier);
    add!(crate::ChicagoTribune);
    add!(crate::ChristianScienceMonitor);
    add!(crate::CincinnatiEnquirer);
    add!(crate::Clarin);
    add!(crate::ClevelandPlainDealer);
    add!(crate::Cnbc);
    add!(crate::Cnet);
    add!(crate::Cnn);
    add!(crate::ColumbusDispatch);
    add!(crate::CommonDreams);
    add!(crate::Conversation);
    add!(crate::CorriereDellaSera);
    add!(crate::CyprusMail);
    add!(crate::DailyBeast);
    add!(crate::DailyCaller);
    add!(crate::DailyKos);
    add!(crate::DailyMail);
    add!(crate::DailyTelegraph);
    add!(crate::DailyWire);
    add!(crate::DallasMorningNews);
    add!(crate::DemocracyNow);
    add!(crate::DenverPost);
    add!(crate::DerSpiegel);
    add!(crate::DetroitFreePress);
    add!(crate::DeutscheWelle);
    add!(crate::DropSiteNews);
    add!(crate::Economist);
    add!(crate::EdmontonJournal);
    add!(crate::ElPais);
    add!(crate::Empire);
    add!(crate::Engadget);
    add!(crate::Eurasianet);
    add!(crate::Euronews);
    add!(crate::Express);
    add!(crate::FastCompany);
    add!(crate::FinancialTimes);
    add!(crate::FolhaDeSPaulo);
    add!(crate::Forbes);
    add!(crate::ForeignPolicy);
    add!(crate::Fortune);
    add!(crate::FoxNews);
    add!(crate::France24);
    add!(crate::FranceInfo);
    add!(crate::FrankfurterAllgemeineZeitung);
    add!(crate::GameSpot);
    add!(crate::Gizmodo);
    add!(crate::GlobalNews);
    add!(crate::GlobalVoices);
    add!(crate::GlobeAndMail);
    add!(crate::Guardian);
    add!(crate::Haaretz);
    add!(crate::HalifaxChronicleHerald);
    add!(crate::HartfordCourant);
    add!(crate::HeraldScotland);
    add!(crate::HoustonChronicle);
    add!(crate::HuffingtonPost);
    add!(crate::Ign);
    add!(crate::Independent);
    add!(crate::IndependentUK);
    add!(crate::IndianapolisStar);
    add!(crate::InterceptFirstLook);
    add!(crate::InvestorsBusinessDaily);
    add!(crate::IrishTimes);
    add!(crate::Jacobin);
    add!(crate::JapanTimes);
    add!(crate::JerusalemPost);
    add!(crate::Kiplinger);
    add!(crate::Kotaku);
    add!(crate::KyivIndependent);
    add!(crate::LaPresse);
    add!(crate::LaRepubblica);
    add!(crate::LeMonde);
    add!(crate::LosAngelesTimes);
    add!(crate::LouisvilleCourierJournal);
    add!(crate::MarketWatch);
    add!(crate::Mashable);
    add!(crate::MiddleEastEye);
    add!(crate::MilitaryTimes);
    add!(crate::MinneapolisStarTribune);
    add!(crate::MirrorUK);
    add!(crate::MontrealGazette);
    add!(crate::MoscowTimes);
    add!(crate::MotherJones);
    add!(crate::Msnbc);
    add!(crate::NashvilleTennessean);
    add!(crate::Nation);
    add!(crate::NationalPost);
    add!(crate::NationalReview);
    add!(crate::Nature);
    add!(crate::NbcNews);
    add!(crate::NerdWallet);
    add!(crate::NewRepublic);
    add!(crate::NewScientist);
    add!(crate::NewStatesman);
    add!(crate::NewYorkPost);
    add!(crate::NewYorkTimes);
    add!(crate::NewYorker);
    add!(crate::Newsweek);
    add!(crate::NHKWorld);
    add!(crate::NikkeiAsia);
    add!(crate::NikkeiEnglish);
    add!(crate::Nme);
    add!(crate::Npr);
    add!(crate::OklahomaCityOklahoman);
    add!(crate::Oregonian);
    add!(crate::OrlandoSentinel);
    add!(crate::OttawaCitizen);
    add!(crate::Pcmag);
    add!(crate::PhiladelphiaInquirer);
    add!(crate::Pitchfork);
    add!(crate::PittsburghPostGazette);
    add!(crate::Politico);
    add!(crate::Polygon);
    add!(crate::ProPublica);
    add!(crate::ProvidenceJournal);
    add!(crate::Quartz);
    add!(crate::Reason);
    add!(crate::RFI);
    add!(crate::RawStory);
    add!(crate::ReginaLeaderPost);
    add!(crate::Reuters);
    add!(crate::RichmondTimesDispatch);
    add!(crate::RollingStone);
    add!(crate::Salon);
    add!(crate::SaskatoonStarPhoenix);
    add!(crate::ScienceMagazine);
    add!(crate::ScientificAmerican);
    add!(crate::Scotsman);
    add!(crate::ScreenRant);
    add!(crate::SeattleTimes);
    add!(crate::Slashdot);
    add!(crate::SkyNews);
    add!(crate::SouthChinaMorningPost);
    add!(crate::Spectator);
    add!(crate::StarLedger);
    add!(crate::StJohnsTelegram);
    add!(crate::StLouisPostDispatch);
    add!(crate::StraitsTimes);
    add!(crate::SydneyMorningHerald);
    add!(crate::TampaBayTimes);
    add!(crate::TechCrunch);
    add!(crate::Techdirt);
    add!(crate::Techmeme);
    add!(crate::Telegraph);
    add!(crate::TelegraphUK);
    add!(crate::TheAge);
    add!(crate::TheArtNewspaper);
    add!(crate::TheAustralian);
    add!(crate::TheBlaze);
    add!(crate::TheBulwark);
    add!(crate::TheHill);
    add!(crate::TheHindu);
    add!(crate::TheHollywoodReporter);
    add!(crate::TheIntercept);
    add!(crate::TheMirror);
    add!(crate::TheSun);
    add!(crate::TheTimes);
    add!(crate::TheVerge);
    add!(crate::TexasTribune);
    add!(crate::Time);
    add!(crate::TimesOfIndia);
    add!(crate::TomsGuide);
    add!(crate::TorontoStar);
    add!(crate::Truthout);
    add!(crate::UnitedPressInternational);
    add!(crate::UsaToday);
    add!(crate::VancouverSun);
    add!(crate::Variety);
    add!(crate::VentureBeat);
    add!(crate::ViceNews);
    add!(crate::Vox);
    add!(crate::WallStreetJournal);
    add!(crate::WashingtonPost);
    add!(crate::Wgn);
    add!(crate::WinnipegFreePress);
    add!(crate::Wired);
    add!(crate::YoungTurks);
    add!(crate::Zeteo);

    sources
}

pub fn sources_with_scope(scope: EndpointScope) -> Vec<SourceInfo> {
    all_sources()
        .into_iter()
        .filter(|s| s.scopes.contains(&scope))
        .collect()
}

pub fn us_sources() -> Vec<SourceInfo> {
    sources_with_scope(EndpointScope::US)
}

pub fn world_sources() -> Vec<SourceInfo> {
    sources_with_scope(EndpointScope::World)
}

pub fn politics_sources() -> Vec<SourceInfo> {
    sources_with_scope(EndpointScope::Politics)
}

pub fn business_sources() -> Vec<SourceInfo> {
    sources_with_scope(EndpointScope::Business)
}

pub fn tech_sources() -> Vec<SourceInfo> {
    sources_with_scope(EndpointScope::Technology)
}

pub fn entertainment_sources() -> Vec<SourceInfo> {
    sources_with_scope(EndpointScope::Entertainment)
}

pub fn sports_sources() -> Vec<SourceInfo> {
    sources_with_scope(EndpointScope::Sports)
}

pub fn science_sources() -> Vec<SourceInfo> {
    sources_with_scope(EndpointScope::Science)
}

pub fn health_sources() -> Vec<SourceInfo> {
    sources_with_scope(EndpointScope::Health)
}
