use enum_map::{enum_map, Enum, EnumMap};

pub const TOP_HEADLINES_URL: &str = "https://newsapi.org/v2/top-headlines";

pub const EVERYTHING_URL: &str = "https://newsapi.org/v2/everything";

pub const SOURCES_URL: &str = "https://newsapi.org/v2/sources";

lazy_static! {
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
    pub static ref LANG_LOOKUP: EnumMap<Language, &'static str> = enum_map! {
        Language::Arabic => "ar",
        Language::German => "de",
        Language::English => "en",
        Language::Spanish => "es",
        Language::French => "fr",
        Language::Hebrew => "he",
        Language::Italian => "it",
        Language::Dutch => "nl",
        Language::Norwegian => "no",
        Language::Portuguese => "pt",
        Language::Russian => "ru",
        Language::NorthernSami => "se",
        // "ud" below is an incorrect ISO-639 code. The correct one for Urdu is "ur"
        // @biomunky has emailed the NewsAPI devs and we are awaiting a response
        Language::Urdu => "ud",
        Language::Chinese => "zh",
    };
    pub static ref SORT_METHOD_LOOKUP: EnumMap<SortMethod, &'static str> = enum_map! {
        SortMethod::Relevancy => "relevancy",
        SortMethod::Popularity => "popularity",
        SortMethod::PublishedAt => "publishedAt",
    };

}

#[derive(Debug, Enum)]
pub enum SortMethod {
    Relevancy,
    Popularity,
    PublishedAt,
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

// Language list below obtained from NewsAPI docs on 01/04/2019 https://newsapi.org/docs/endpoints/everything
// "ar", "en", "cn", "de", "es", "fr", "he", "it", "nl", "no", "pt", "ru", "sv", "ud",
#[derive(Debug, Enum)]
pub enum Language {
    Arabic,
    German,
    English,
    Spanish,
    French,
    Hebrew,
    Italian,
    Dutch,
    Norwegian,
    Portuguese,
    Russian,
    NorthernSami,
    Urdu,
    Chinese,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "entertainment")]
    Entertainment,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "health")]
    Health,
    #[serde(rename = "science")]
    Science,
    #[serde(rename = "sports")]
    Sports,
    #[serde(rename = "technology")]
    Technology,
}
