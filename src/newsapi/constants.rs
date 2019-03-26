use std::collections::HashSet;
use std::iter::FromIterator;
use enum_map::{EnumMap, Enum};
use lazy_static;

#[allow(dead_code)]
pub const TOP_HEADLINES_URL: &str = "https://newsapi.org/v2/top-headlines";

#[allow(dead_code)]
pub const EVERYTHING_URL: &str = "https://newsapi.org/v2/everything";

#[allow(dead_code)]
pub const SOURCES_URL: &str = "https://newsapi.org/v2/sources";

lazy_static! {
    pub static ref COUNTRIES: HashSet<&'static str> = {
        let options = vec![
            "ae", "ar", "at", "au", "be", "bg", "br", "ca", "ch", "cn", "co", "cu", "cz", "de",
            "eg", "fr", "gb", "gr", "hk", "hu", "id", "ie", "il", "in", "it", "jp", "kr", "lt",
            "lv", "ma", "mx", "my", "ng", "nl", "no", "nz", "ph", "pl", "pt", "ro", "rs", "ru",
            "sa", "se", "sg", "si", "sk", "th", "tr", "tw", "ua", "us", "ve", "za",
        ];
        HashSet::from_iter(options)
    };
    pub static ref LANGUAGES: HashSet<&'static str> = {
        let options = vec![
            "ar", "en", "cn", "de", "es", "fr", "he", "it", "nl", "no", "pt", "ru", "sv", "ud",
        ];
        HashSet::from_iter(options)
    };
    pub static ref SORT_METHOD: HashSet<&'static str> = {
        let options = vec!["relevancy", "popularity", "publishedAt"];
        HashSet::from_iter(options)
    };
    pub static ref COUNTRY_LOOKUP: EnumMap<Country, &'static str> = enum_map! {
        Country::Argentina => "ar",
        Country::Australia => "au",
        Country::Austria => "at",
        Country::Belgium => "be",
        Country::Brazil => "br",
        Country::Bulgaria => "bg",
        Country::Canada => "ca",
        Country::China => "cn",
        Country::Colombia => "co",
        Country::Cuba => "cu",
        Country::Czechia => "cz",
        Country::Egypt => "eg",
        Country::France => "fr",
        Country::Germany => "de",
        Country::Greece => "gr",
        Country::HongKong => "hk",
        Country::Hungary => "hu",
        Country::India => "in",
        Country::Indonesia => "id",
        Country::Ireland => "ie",
        Country::Israel => "il",
        Country::Italy => "it",
        Country::Japan => "jp",
        Country::KoreaRepublicof => "kr",
        Country::Latvia => "lv",
        Country::Lithuania => "lt",
        Country::Malaysia => "my",
        Country::Mexico => "mx",
        Country::Morocco => "ma",
        Country::Netherlands => "nl",
        Country::NewZealand => "nz",
        Country::Nigeria => "ng",
        Country::Norway => "no",
        Country::Philippines => "ph",
        Country::Poland => "pl",
        Country::Portugal => "pt",
        Country::Romania => "ro",
        Country::RussianFederation => "ru",
        Country::SaudiArabia => "sa",
        Country::Serbia => "rs",
        Country::Singapore => "sg",
        Country::Slovakia => "sk",
        Country::Slovenia => "si",
        Country::SouthAfrica => "za",
        Country::Sweden => "se",
        Country::Switzerland => "ch",
        Country::Taiwan => "tw",
        Country::Thailand => "th",
        Country::Turkey => "tr",
        Country::Ukraine => "ua",
        Country::UnitedArabEmirates => "ae",
        Country::UnitedKingdomofGreatBritainandNorthernIreland => "gb",
        Country::UnitedStatesofAmerica => "us",
        Country::VenezuelaBolivarianRepublicof => "ve",
    };
}

#[derive(Debug)]
pub enum Category {
    Business,
    Entertainment,
    General,
    Health,
    Science,
    Sports,
    Technology,
}

#[derive(Debug, Enum)]
pub enum Country {
    Argentina,
    Australia,
    Austria,
    Belgium,
    Brazil,
    Bulgaria,
    Canada,
    China,
    Colombia,
    Cuba,
    Czechia,
    Egypt,
    France,
    Germany,
    Greece,
    HongKong,
    Hungary,
    India,
    Indonesia,
    Ireland,
    Israel,
    Italy,
    Japan,
    KoreaRepublicof,
    Latvia,
    Lithuania,
    Malaysia,
    Mexico,
    Morocco,
    Netherlands,
    NewZealand,
    Nigeria,
    Norway,
    Philippines,
    Poland,
    Portugal,
    Romania,
    RussianFederation,
    SaudiArabia,
    Serbia,
    Singapore,
    Slovakia,
    Slovenia,
    SouthAfrica,
    Sweden,
    Switzerland,
    Taiwan,
    Thailand,
    Turkey,
    Ukraine,
    UnitedArabEmirates,
    UnitedKingdomofGreatBritainandNorthernIreland,
    UnitedStatesofAmerica,
    VenezuelaBolivarianRepublicof,
}
