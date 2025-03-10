// DO NOT TOUCH THIS FILE. (Auto-generated by `keshvar-code-generator/src/currency_code.rs`)

use crate::SearchError;

#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
/// An enum containing currency codes.
///
/// # Example
/// ```
/// use keshvar::{CurrencyCode, Alpha2};
///
/// assert_eq!(Ok(CurrencyCode::QAR), CurrencyCode::try_from("qAr")); // not case-sensitive
/// let eur_alpha2_country_list: Vec<Alpha2> = CurrencyCode::EUR
///     .alpha2_list()
///     .iter()
///     .filter_map(|alpha2_str| Alpha2::try_from(*alpha2_str).ok())
///     .collect();
/// assert!(eur_alpha2_country_list.contains(&Alpha2::ES)); // Spain
/// assert!(eur_alpha2_country_list.contains(&Alpha2::NL)); // Netherlands
/// ```
/// We usually need to convert [`Alpha2`](crate::Alpha2) to [`Country`](crate::Country) and use that object instead.
pub enum CurrencyCode {
    /// * The United Arab Emirates (Asia)
    AED,
    /// * The Islamic Republic of Afghanistan (Asia)
    AFN,
    /// * The Republic of Albania (Europe)
    ALL,
    /// * The Republic of Armenia (Asia)
    AMD,
    /// * The Country of Curaçao (Americas)
    /// * Sint Maarten (Americas)
    ANG,
    /// * The Republic of Angola (Africa)
    AOA,
    /// * The Argentine Republic (Americas)
    ARS,
    /// * The Commonwealth of Australia (Oceania)
    /// * The Territory of Cocos (Keeling) Islands (Oceania)
    /// * The Territory of Christmas Island (Oceania)
    /// * The Territory of Heard Island and McDonald Islands
    /// * The Republic of Kiribati (Oceania)
    /// * The Territory of Norfolk Island (Oceania)
    /// * The Republic of Nauru (Oceania)
    /// * Tuvalu (Oceania)
    AUD,
    /// * Aruba (Americas)
    AWG,
    /// * The Republic of Azerbaijan (Asia)
    AZN,
    /// * Bosnia and Herzegovina (Europe)
    BAM,
    /// * Barbados (Americas)
    BBD,
    /// * The People's Republic of Bangladesh (Asia)
    BDT,
    /// * The Republic of Bulgaria (Europe)
    BGN,
    /// * The Kingdom of Bahrain (Asia)
    BHD,
    /// * The Republic of Burundi (Africa)
    BIF,
    /// * Bermuda (Americas)
    BMD,
    /// * The Nation of Brunei, the Abode of Peace (Asia)
    BND,
    /// * The Plurinational State of Bolivia (Americas)
    BOB,
    /// * The Federative Republic of Brazil (Americas)
    BRL,
    /// * The Commonwealth of The Bahamas (Americas)
    BSD,
    /// * The Kingdom of Bhutan (Asia)
    BTN,
    /// * The Republic of Botswana (Africa)
    BWP,
    /// * The Republic of Belarus (Europe)
    BYN,
    /// * Belize (Americas)
    BZD,
    /// * Canada (Americas)
    CAD,
    /// * The Democratic Republic of the Congo (Africa)
    CDF,
    /// * The Swiss Confederation (Europe)
    /// * The Principality of Liechtenstein (Europe)
    CHF,
    /// * The Republic of Chile (Americas)
    CLP,
    /// * The People's Republic of China (Asia)
    CNY,
    /// * The Republic of Colombia (Americas)
    COP,
    /// * The Republic of Costa Rica (Americas)
    CRC,
    /// * The Republic of Cuba (Americas)
    CUP,
    /// * The Republic of Cabo Verde (Africa)
    CVE,
    /// * The Czech Republic (Europe)
    CZK,
    /// * The Republic of Djibouti (Africa)
    DJF,
    /// * The Kingdom of Denmark (Europe)
    /// * The Faroe Islands (Europe)
    /// * Kalaallit Nunaat (Americas)
    DKK,
    /// * The Dominican Republic (Americas)
    DOP,
    /// * The People's Democratic Republic of Algeria (Africa)
    DZD,
    /// * The Arab Republic of Egypt (Africa)
    EGP,
    /// * The State of Eritrea (Africa)
    ERN,
    /// * The Federal Democratic Republic of Ethiopia (Africa)
    ETB,
    /// * The Principality of Andorra (Europe)
    /// * The Republic of Austria (Europe)
    /// * Åland (Europe)
    /// * The Kingdom of Belgium (Europe)
    /// * The Collectivity of Saint-Barthélemy (Americas)
    /// * The Republic of Cyprus (Asia)
    /// * The Federal Republic of Germany (Europe)
    /// * The Republic of Estonia (Europe)
    /// * The Kingdom of Spain (Europe)
    /// * The Republic of Finland (Europe)
    /// * The French Republic (Europe)
    /// * Guyane (Americas)
    /// * Guadeloupe (Americas)
    /// * The Hellenic Republic (Europe)
    /// * The Republic of Croatia (Europe)
    /// * Ireland (Europe)
    /// * The Italian Republic (Europe)
    /// * The Republic of Lithuania (Europe)
    /// * The Grand Duchy of Luxembourg (Europe)
    /// * The Republic of Latvia (Europe)
    /// * The Principality of Monaco (Europe)
    /// * Montenegro (Europe)
    /// * The Collectivity of Saint-Martin (Americas)
    /// * Martinique (Americas)
    /// * The Republic of Malta (Europe)
    /// * The Kingdom of the Netherlands (Europe)
    /// * The Overseas Collectivity of Saint-Pierre and Miquelon (Americas)
    /// * The Portuguese Republic (Europe)
    /// * Réunion (Africa)
    /// * The Republic of Slovenia (Europe)
    /// * The Slovak Republic (Europe)
    /// * The Republic of San Marino (Europe)
    /// * The French Southern and Antarctic Lands (Africa)
    /// * The Holy See (Europe)
    /// * The Department of Mayotte (Africa)
    EUR,
    /// * The Republic of Fiji (Oceania)
    FJD,
    /// * The Falkland Islands (Americas)
    FKP,
    /// * The United Kingdom of Great Britain and Northern Ireland (Europe)
    /// * The Bailiwick of Guernsey (Europe)
    /// * South Georgia and the South Sandwich Islands (Americas)
    /// * The Isle of Man (Europe)
    /// * The Bailiwick of Jersey (Europe)
    GBP,
    /// * Georgia (Asia)
    GEL,
    /// * The Republic of Ghana (Africa)
    GHS,
    /// * Gibraltar (Europe)
    GIP,
    /// * The Republic of The Gambia (Africa)
    GMD,
    /// * The Republic of Guinea (Africa)
    GNF,
    /// * The Republic of Guatemala (Americas)
    GTQ,
    /// * The Co-operative Republic of Guyana (Americas)
    GYD,
    /// * The Hong Kong Special Administrative Region of China (Asia)
    HKD,
    /// * The Republic of Honduras (Americas)
    HNL,
    /// * The Republic of Haiti (Americas)
    HTG,
    /// * Hungary (Europe)
    HUF,
    /// * The Republic of Indonesia (Asia)
    /// * The Democratic Republic of Timor-Leste (Asia)
    IDR,
    /// * The State of Israel (Asia)
    /// * The State of Palestine (Asia)
    ILS,
    /// * The Republic of India (Asia)
    INR,
    /// * The Republic of Iraq (Asia)
    IQD,
    /// * The Islamic Republic of Iran (Asia)
    IRR,
    /// * Iceland (Europe)
    ISK,
    /// * Jamaica (Americas)
    JMD,
    /// * The Hashemite Kingdom of Jordan (Asia)
    JOD,
    /// * Japan (Asia)
    JPY,
    /// * The Republic of Kenya (Africa)
    KES,
    /// * The Kyrgyz Republic (Asia)
    KGS,
    /// * The Kingdom of Cambodia (Asia)
    KHR,
    /// * The Union of the Comoros (Africa)
    KMF,
    /// * The Democratic People's Republic of Korea (Asia)
    KPW,
    /// * The Republic of Korea (Asia)
    KRW,
    /// * The State of Kuwait (Asia)
    KWD,
    /// * The Cayman Islands (Americas)
    KYD,
    /// * The Republic of Kazakhstan (Asia)
    KZT,
    /// * The Lao People's Democratic Republic (Asia)
    LAK,
    /// * The Lebanese Republic (Asia)
    LBP,
    /// * The Democratic Socialist Republic of Sri Lanka (Asia)
    LKR,
    /// * The Republic of Liberia (Africa)
    LRD,
    /// * The Kingdom of Lesotho (Africa)
    LSL,
    /// * The State of Libya (Africa)
    LYD,
    /// * The Sahrawi Arab Democratic Republic (Africa)
    /// * The Kingdom of Morocco (Africa)
    MAD,
    /// * The Republic of Moldova (Europe)
    MDL,
    /// * The Republic of Madagascar (Africa)
    MGA,
    /// * The Republic of North Macedonia (Europe)
    MKD,
    /// * The Republic of the Union of Myanmar (Asia)
    MMK,
    /// * Mongolia (Asia)
    MNT,
    /// * The Macao Special Administrative Region of China (Asia)
    MOP,
    /// * The Islamic Republic of Mauritania (Africa)
    MRU,
    /// * The Republic of Mauritius (Africa)
    MUR,
    /// * The Republic of Maldives (Asia)
    MVR,
    /// * The Republic of Malawi (Africa)
    MWK,
    /// * The United Mexican States (Americas)
    MXN,
    /// * Malaysia (Asia)
    MYR,
    /// * The Republic of Mozambique (Africa)
    MZN,
    /// * The Republic of Namibia (Africa)
    NAD,
    /// * The Federal Republic of Nigeria (Africa)
    NGN,
    /// * The Republic of Nicaragua (Americas)
    NIO,
    /// * Bouvet Island
    /// * The Kingdom of Norway (Europe)
    /// * Svalbard and Jan Mayen (Europe)
    NOK,
    /// * The Federal Democratic Republic of Nepal (Asia)
    NPR,
    /// * The Cook Islands (Oceania)
    /// * Niue (Oceania)
    /// * New Zealand (Oceania)
    /// * The Pitcairn, Henderson, Ducie and Oeno Islands (Oceania)
    /// * Tokelau (Oceania)
    NZD,
    /// * The Sultanate of Oman (Asia)
    OMR,
    /// * The Republic of Panamá (Americas)
    PAB,
    /// * The Republic of Perú (Americas)
    PEN,
    /// * The Independent State of Papua New Guinea (Oceania)
    PGK,
    /// * The Republic of the Philippines (Asia)
    PHP,
    /// * The Islamic Republic of Pakistan (Asia)
    PKR,
    /// * The Republic of Poland (Europe)
    PLN,
    /// * The Republic of Paraguay (Americas)
    PYG,
    /// * The State of Qatar (Asia)
    QAR,
    /// * Romania (Europe)
    RON,
    /// * The Republic of Serbia (Europe)
    RSD,
    /// * The Russian Federation (Europe)
    RUB,
    /// * The Republic of Rwanda (Africa)
    RWF,
    /// * The Kingdom of Saudi Arabia (Asia)
    SAR,
    /// * The Solomon Islands (Oceania)
    SBD,
    /// * The Republic of Seychelles (Africa)
    SCR,
    /// * The Republic of the Sudan (Africa)
    SDG,
    /// * The Kingdom of Sweden (Europe)
    SEK,
    /// * The Republic of Singapore (Asia)
    SGD,
    /// * Saint Helena, Ascension and Tristan da Cunha (Africa)
    SHP,
    /// * The Republic of Sierra Leone (Africa)
    SLL,
    /// * The Federal Republic of Somalia (Africa)
    SOS,
    /// * The Republic of Suriname (Americas)
    SRD,
    /// * The Republic of South Sudan (Africa)
    SSP,
    /// * The Democratic Republic of São Tomé and Príncipe (Africa)
    STD,
    /// * The Syrian Arab Republic (Asia)
    SYP,
    /// * The Kingdom of Eswatini (Africa)
    SZL,
    /// * The Kingdom of Thailand (Asia)
    THB,
    /// * The Republic of Tajikistan (Asia)
    TJS,
    /// * Turkmenistan (Asia)
    TMT,
    /// * The Republic of Tunisia (Africa)
    TND,
    /// * The Kingdom of Tonga (Oceania)
    TOP,
    /// * The Republic of Türkiye (Asia)
    TRY,
    /// * The Republic of Trinidad and Tobago (Americas)
    TTD,
    /// * Taiwan, Province of China (Asia)
    TWD,
    /// * The United Republic of Tanzania (Africa)
    TZS,
    /// * Ukraine (Europe)
    UAH,
    /// * The Republic of Uganda (Africa)
    UGX,
    /// * Antarctica
    /// * The Territory of American Samoa (Oceania)
    /// * Bonaire, Sint Eustatius and Saba (Americas)
    /// * The Republic of Ecuador (Americas)
    /// * The Federated States of Micronesia (Oceania)
    /// * The Territory of Guam (Oceania)
    /// * The British Indian Ocean Territory (Africa)
    /// * The Republic of the Marshall Islands (Oceania)
    /// * The Commonwealth of the Northern Mariana Islands (Oceania)
    /// * The Commonwealth of Puerto Rico (Americas)
    /// * The Republic of Palau (Oceania)
    /// * The Republic of El Salvador (Americas)
    /// * The Turks and Caicos Islands (Americas)
    /// * United States Minor Outlying Islands (Americas)
    /// * The United States of America (Americas)
    /// * The Virgin Islands (Americas)
    /// * The Virgin Islands of the United States (Americas)
    /// * The Republic of Zimbabwe (Africa)
    USD,
    /// * The Oriental Republic of Uruguay (Americas)
    UYU,
    /// * The Republic of Uzbekistan (Asia)
    UZS,
    /// * The Bolivarian Republic of Venezuela (Americas)
    VES,
    /// * The Socialist Republic of Viet Nam (Asia)
    VND,
    /// * The Republic of Vanuatu (Oceania)
    VUV,
    /// * The Independent State of Samoa (Oceania)
    WST,
    /// * The Central African Republic (Africa)
    /// * The Republic of the Congo (Africa)
    /// * The Republic of Cameroon (Africa)
    /// * The Gabonese Republic (Africa)
    /// * The Republic of Equatorial Guinea (Africa)
    /// * The Republic of Chad (Africa)
    XAF,
    /// * Antigua and Barbuda (Americas)
    /// * Anguilla (Americas)
    /// * The Commonwealth of Dominica (Americas)
    /// * Grenada (Americas)
    /// * Saint Kitts and Nevis (Americas)
    /// * Saint Lucia (Americas)
    /// * Montserrat (Americas)
    /// * Saint Vincent and the Grenadines (Americas)
    XCD,
    /// * Burkina Faso (Africa)
    /// * The Republic of Benin (Africa)
    /// * The Republic of Côte d'Ivoire (Africa)
    /// * The Republic of Guinea-Bissau (Africa)
    /// * The Republic of Mali (Africa)
    /// * The Republic of the Niger (Africa)
    /// * The Republic of Senegal (Africa)
    /// * The Togolese Republic (Africa)
    XOF,
    /// * New Caledonia (Oceania)
    /// * French Polynesia (Oceania)
    /// * The Territory of the Wallis and Futuna Islands (Oceania)
    XPF,
    /// * The Republic of Yemen (Asia)
    YER,
    /// * The Republic of South Africa (Africa)
    ZAR,
    /// * The Republic of Zambia (Africa)
    ZMW,
}

impl CurrencyCode {
    pub fn alpha2_list(&self) -> &[&str] {
        match self {
            Self::AED => &["AE"],
            Self::AFN => &["AF"],
            Self::ALL => &["AL"],
            Self::AMD => &["AM"],
            Self::ANG => &["CW", "SX"],
            Self::AOA => &["AO"],
            Self::ARS => &["AR"],
            Self::AUD => &["AU", "CC", "CX", "HM", "KI", "NF", "NR", "TV"],
            Self::AWG => &["AW"],
            Self::AZN => &["AZ"],
            Self::BAM => &["BA"],
            Self::BBD => &["BB"],
            Self::BDT => &["BD"],
            Self::BGN => &["BG"],
            Self::BHD => &["BH"],
            Self::BIF => &["BI"],
            Self::BMD => &["BM"],
            Self::BND => &["BN"],
            Self::BOB => &["BO"],
            Self::BRL => &["BR"],
            Self::BSD => &["BS"],
            Self::BTN => &["BT"],
            Self::BWP => &["BW"],
            Self::BYN => &["BY"],
            Self::BZD => &["BZ"],
            Self::CAD => &["CA"],
            Self::CDF => &["CD"],
            Self::CHF => &["CH", "LI"],
            Self::CLP => &["CL"],
            Self::CNY => &["CN"],
            Self::COP => &["CO"],
            Self::CRC => &["CR"],
            Self::CUP => &["CU"],
            Self::CVE => &["CV"],
            Self::CZK => &["CZ"],
            Self::DJF => &["DJ"],
            Self::DKK => &["DK", "FO", "GL"],
            Self::DOP => &["DO"],
            Self::DZD => &["DZ"],
            Self::EGP => &["EG"],
            Self::ERN => &["ER"],
            Self::ETB => &["ET"],
            Self::EUR => &[
                "AD", "AT", "AX", "BE", "BL", "CY", "DE", "EE", "ES", "FI", "FR", "GF", "GP", "GR",
                "HR", "IE", "IT", "LT", "LU", "LV", "MC", "ME", "MF", "MQ", "MT", "NL", "PM", "PT",
                "RE", "SI", "SK", "SM", "TF", "VA", "YT",
            ],
            Self::FJD => &["FJ"],
            Self::FKP => &["FK"],
            Self::GBP => &["GB", "GG", "GS", "IM", "JE"],
            Self::GEL => &["GE"],
            Self::GHS => &["GH"],
            Self::GIP => &["GI"],
            Self::GMD => &["GM"],
            Self::GNF => &["GN"],
            Self::GTQ => &["GT"],
            Self::GYD => &["GY"],
            Self::HKD => &["HK"],
            Self::HNL => &["HN"],
            Self::HTG => &["HT"],
            Self::HUF => &["HU"],
            Self::IDR => &["ID", "TL"],
            Self::ILS => &["IL", "PS"],
            Self::INR => &["IN"],
            Self::IQD => &["IQ"],
            Self::IRR => &["IR"],
            Self::ISK => &["IS"],
            Self::JMD => &["JM"],
            Self::JOD => &["JO"],
            Self::JPY => &["JP"],
            Self::KES => &["KE"],
            Self::KGS => &["KG"],
            Self::KHR => &["KH"],
            Self::KMF => &["KM"],
            Self::KPW => &["KP"],
            Self::KRW => &["KR"],
            Self::KWD => &["KW"],
            Self::KYD => &["KY"],
            Self::KZT => &["KZ"],
            Self::LAK => &["LA"],
            Self::LBP => &["LB"],
            Self::LKR => &["LK"],
            Self::LRD => &["LR"],
            Self::LSL => &["LS"],
            Self::LYD => &["LY"],
            Self::MAD => &["EH", "MA"],
            Self::MDL => &["MD"],
            Self::MGA => &["MG"],
            Self::MKD => &["MK"],
            Self::MMK => &["MM"],
            Self::MNT => &["MN"],
            Self::MOP => &["MO"],
            Self::MRU => &["MR"],
            Self::MUR => &["MU"],
            Self::MVR => &["MV"],
            Self::MWK => &["MW"],
            Self::MXN => &["MX"],
            Self::MYR => &["MY"],
            Self::MZN => &["MZ"],
            Self::NAD => &["NA"],
            Self::NGN => &["NG"],
            Self::NIO => &["NI"],
            Self::NOK => &["BV", "NO", "SJ"],
            Self::NPR => &["NP"],
            Self::NZD => &["CK", "NU", "NZ", "PN", "TK"],
            Self::OMR => &["OM"],
            Self::PAB => &["PA"],
            Self::PEN => &["PE"],
            Self::PGK => &["PG"],
            Self::PHP => &["PH"],
            Self::PKR => &["PK"],
            Self::PLN => &["PL"],
            Self::PYG => &["PY"],
            Self::QAR => &["QA"],
            Self::RON => &["RO"],
            Self::RSD => &["RS"],
            Self::RUB => &["RU"],
            Self::RWF => &["RW"],
            Self::SAR => &["SA"],
            Self::SBD => &["SB"],
            Self::SCR => &["SC"],
            Self::SDG => &["SD"],
            Self::SEK => &["SE"],
            Self::SGD => &["SG"],
            Self::SHP => &["SH"],
            Self::SLL => &["SL"],
            Self::SOS => &["SO"],
            Self::SRD => &["SR"],
            Self::SSP => &["SS"],
            Self::STD => &["ST"],
            Self::SYP => &["SY"],
            Self::SZL => &["SZ"],
            Self::THB => &["TH"],
            Self::TJS => &["TJ"],
            Self::TMT => &["TM"],
            Self::TND => &["TN"],
            Self::TOP => &["TO"],
            Self::TRY => &["TR"],
            Self::TTD => &["TT"],
            Self::TWD => &["TW"],
            Self::TZS => &["TZ"],
            Self::UAH => &["UA"],
            Self::UGX => &["UG"],
            Self::USD => &[
                "AQ", "AS", "BQ", "EC", "FM", "GU", "IO", "MH", "MP", "PR", "PW", "SV", "TC", "UM",
                "US", "VG", "VI", "ZW",
            ],
            Self::UYU => &["UY"],
            Self::UZS => &["UZ"],
            Self::VES => &["VE"],
            Self::VND => &["VN"],
            Self::VUV => &["VU"],
            Self::WST => &["WS"],
            Self::XAF => &["CF", "CG", "CM", "GA", "GQ", "TD"],
            Self::XCD => &["AG", "AI", "DM", "GD", "KN", "LC", "MS", "VC"],
            Self::XOF => &["BF", "BJ", "CI", "GW", "ML", "NE", "SN", "TG"],
            Self::XPF => &["NC", "PF", "WF"],
            Self::YER => &["YE"],
            Self::ZAR => &["ZA"],
            Self::ZMW => &["ZM"],
        }
    }
}

impl std::fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AED => "AED",
                Self::AFN => "AFN",
                Self::ALL => "ALL",
                Self::AMD => "AMD",
                Self::ANG => "ANG",
                Self::AOA => "AOA",
                Self::ARS => "ARS",
                Self::AUD => "AUD",
                Self::AWG => "AWG",
                Self::AZN => "AZN",
                Self::BAM => "BAM",
                Self::BBD => "BBD",
                Self::BDT => "BDT",
                Self::BGN => "BGN",
                Self::BHD => "BHD",
                Self::BIF => "BIF",
                Self::BMD => "BMD",
                Self::BND => "BND",
                Self::BOB => "BOB",
                Self::BRL => "BRL",
                Self::BSD => "BSD",
                Self::BTN => "BTN",
                Self::BWP => "BWP",
                Self::BYN => "BYN",
                Self::BZD => "BZD",
                Self::CAD => "CAD",
                Self::CDF => "CDF",
                Self::CHF => "CHF",
                Self::CLP => "CLP",
                Self::CNY => "CNY",
                Self::COP => "COP",
                Self::CRC => "CRC",
                Self::CUP => "CUP",
                Self::CVE => "CVE",
                Self::CZK => "CZK",
                Self::DJF => "DJF",
                Self::DKK => "DKK",
                Self::DOP => "DOP",
                Self::DZD => "DZD",
                Self::EGP => "EGP",
                Self::ERN => "ERN",
                Self::ETB => "ETB",
                Self::EUR => "EUR",
                Self::FJD => "FJD",
                Self::FKP => "FKP",
                Self::GBP => "GBP",
                Self::GEL => "GEL",
                Self::GHS => "GHS",
                Self::GIP => "GIP",
                Self::GMD => "GMD",
                Self::GNF => "GNF",
                Self::GTQ => "GTQ",
                Self::GYD => "GYD",
                Self::HKD => "HKD",
                Self::HNL => "HNL",
                Self::HTG => "HTG",
                Self::HUF => "HUF",
                Self::IDR => "IDR",
                Self::ILS => "ILS",
                Self::INR => "INR",
                Self::IQD => "IQD",
                Self::IRR => "IRR",
                Self::ISK => "ISK",
                Self::JMD => "JMD",
                Self::JOD => "JOD",
                Self::JPY => "JPY",
                Self::KES => "KES",
                Self::KGS => "KGS",
                Self::KHR => "KHR",
                Self::KMF => "KMF",
                Self::KPW => "KPW",
                Self::KRW => "KRW",
                Self::KWD => "KWD",
                Self::KYD => "KYD",
                Self::KZT => "KZT",
                Self::LAK => "LAK",
                Self::LBP => "LBP",
                Self::LKR => "LKR",
                Self::LRD => "LRD",
                Self::LSL => "LSL",
                Self::LYD => "LYD",
                Self::MAD => "MAD",
                Self::MDL => "MDL",
                Self::MGA => "MGA",
                Self::MKD => "MKD",
                Self::MMK => "MMK",
                Self::MNT => "MNT",
                Self::MOP => "MOP",
                Self::MRU => "MRU",
                Self::MUR => "MUR",
                Self::MVR => "MVR",
                Self::MWK => "MWK",
                Self::MXN => "MXN",
                Self::MYR => "MYR",
                Self::MZN => "MZN",
                Self::NAD => "NAD",
                Self::NGN => "NGN",
                Self::NIO => "NIO",
                Self::NOK => "NOK",
                Self::NPR => "NPR",
                Self::NZD => "NZD",
                Self::OMR => "OMR",
                Self::PAB => "PAB",
                Self::PEN => "PEN",
                Self::PGK => "PGK",
                Self::PHP => "PHP",
                Self::PKR => "PKR",
                Self::PLN => "PLN",
                Self::PYG => "PYG",
                Self::QAR => "QAR",
                Self::RON => "RON",
                Self::RSD => "RSD",
                Self::RUB => "RUB",
                Self::RWF => "RWF",
                Self::SAR => "SAR",
                Self::SBD => "SBD",
                Self::SCR => "SCR",
                Self::SDG => "SDG",
                Self::SEK => "SEK",
                Self::SGD => "SGD",
                Self::SHP => "SHP",
                Self::SLL => "SLL",
                Self::SOS => "SOS",
                Self::SRD => "SRD",
                Self::SSP => "SSP",
                Self::STD => "STD",
                Self::SYP => "SYP",
                Self::SZL => "SZL",
                Self::THB => "THB",
                Self::TJS => "TJS",
                Self::TMT => "TMT",
                Self::TND => "TND",
                Self::TOP => "TOP",
                Self::TRY => "TRY",
                Self::TTD => "TTD",
                Self::TWD => "TWD",
                Self::TZS => "TZS",
                Self::UAH => "UAH",
                Self::UGX => "UGX",
                Self::USD => "USD",
                Self::UYU => "UYU",
                Self::UZS => "UZS",
                Self::VES => "VES",
                Self::VND => "VND",
                Self::VUV => "VUV",
                Self::WST => "WST",
                Self::XAF => "XAF",
                Self::XCD => "XCD",
                Self::XOF => "XOF",
                Self::XPF => "XPF",
                Self::YER => "YER",
                Self::ZAR => "ZAR",
                Self::ZMW => "ZMW",
            }
        )
    }
}

impl TryFrom<&str> for CurrencyCode {
    type Error = SearchError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(SearchError::BadInput {
                expected: "a string with three characters",
            });
        }
        match value.to_uppercase().as_str() {
            "AED" => Ok(Self::AED),
            "AFN" => Ok(Self::AFN),
            "ALL" => Ok(Self::ALL),
            "AMD" => Ok(Self::AMD),
            "ANG" => Ok(Self::ANG),
            "AOA" => Ok(Self::AOA),
            "ARS" => Ok(Self::ARS),
            "AUD" => Ok(Self::AUD),
            "AWG" => Ok(Self::AWG),
            "AZN" => Ok(Self::AZN),
            "BAM" => Ok(Self::BAM),
            "BBD" => Ok(Self::BBD),
            "BDT" => Ok(Self::BDT),
            "BGN" => Ok(Self::BGN),
            "BHD" => Ok(Self::BHD),
            "BIF" => Ok(Self::BIF),
            "BMD" => Ok(Self::BMD),
            "BND" => Ok(Self::BND),
            "BOB" => Ok(Self::BOB),
            "BRL" => Ok(Self::BRL),
            "BSD" => Ok(Self::BSD),
            "BTN" => Ok(Self::BTN),
            "BWP" => Ok(Self::BWP),
            "BYN" => Ok(Self::BYN),
            "BZD" => Ok(Self::BZD),
            "CAD" => Ok(Self::CAD),
            "CDF" => Ok(Self::CDF),
            "CHF" => Ok(Self::CHF),
            "CLP" => Ok(Self::CLP),
            "CNY" => Ok(Self::CNY),
            "COP" => Ok(Self::COP),
            "CRC" => Ok(Self::CRC),
            "CUP" => Ok(Self::CUP),
            "CVE" => Ok(Self::CVE),
            "CZK" => Ok(Self::CZK),
            "DJF" => Ok(Self::DJF),
            "DKK" => Ok(Self::DKK),
            "DOP" => Ok(Self::DOP),
            "DZD" => Ok(Self::DZD),
            "EGP" => Ok(Self::EGP),
            "ERN" => Ok(Self::ERN),
            "ETB" => Ok(Self::ETB),
            "EUR" => Ok(Self::EUR),
            "FJD" => Ok(Self::FJD),
            "FKP" => Ok(Self::FKP),
            "GBP" => Ok(Self::GBP),
            "GEL" => Ok(Self::GEL),
            "GHS" => Ok(Self::GHS),
            "GIP" => Ok(Self::GIP),
            "GMD" => Ok(Self::GMD),
            "GNF" => Ok(Self::GNF),
            "GTQ" => Ok(Self::GTQ),
            "GYD" => Ok(Self::GYD),
            "HKD" => Ok(Self::HKD),
            "HNL" => Ok(Self::HNL),
            "HTG" => Ok(Self::HTG),
            "HUF" => Ok(Self::HUF),
            "IDR" => Ok(Self::IDR),
            "ILS" => Ok(Self::ILS),
            "INR" => Ok(Self::INR),
            "IQD" => Ok(Self::IQD),
            "IRR" => Ok(Self::IRR),
            "ISK" => Ok(Self::ISK),
            "JMD" => Ok(Self::JMD),
            "JOD" => Ok(Self::JOD),
            "JPY" => Ok(Self::JPY),
            "KES" => Ok(Self::KES),
            "KGS" => Ok(Self::KGS),
            "KHR" => Ok(Self::KHR),
            "KMF" => Ok(Self::KMF),
            "KPW" => Ok(Self::KPW),
            "KRW" => Ok(Self::KRW),
            "KWD" => Ok(Self::KWD),
            "KYD" => Ok(Self::KYD),
            "KZT" => Ok(Self::KZT),
            "LAK" => Ok(Self::LAK),
            "LBP" => Ok(Self::LBP),
            "LKR" => Ok(Self::LKR),
            "LRD" => Ok(Self::LRD),
            "LSL" => Ok(Self::LSL),
            "LYD" => Ok(Self::LYD),
            "MAD" => Ok(Self::MAD),
            "MDL" => Ok(Self::MDL),
            "MGA" => Ok(Self::MGA),
            "MKD" => Ok(Self::MKD),
            "MMK" => Ok(Self::MMK),
            "MNT" => Ok(Self::MNT),
            "MOP" => Ok(Self::MOP),
            "MRU" => Ok(Self::MRU),
            "MUR" => Ok(Self::MUR),
            "MVR" => Ok(Self::MVR),
            "MWK" => Ok(Self::MWK),
            "MXN" => Ok(Self::MXN),
            "MYR" => Ok(Self::MYR),
            "MZN" => Ok(Self::MZN),
            "NAD" => Ok(Self::NAD),
            "NGN" => Ok(Self::NGN),
            "NIO" => Ok(Self::NIO),
            "NOK" => Ok(Self::NOK),
            "NPR" => Ok(Self::NPR),
            "NZD" => Ok(Self::NZD),
            "OMR" => Ok(Self::OMR),
            "PAB" => Ok(Self::PAB),
            "PEN" => Ok(Self::PEN),
            "PGK" => Ok(Self::PGK),
            "PHP" => Ok(Self::PHP),
            "PKR" => Ok(Self::PKR),
            "PLN" => Ok(Self::PLN),
            "PYG" => Ok(Self::PYG),
            "QAR" => Ok(Self::QAR),
            "RON" => Ok(Self::RON),
            "RSD" => Ok(Self::RSD),
            "RUB" => Ok(Self::RUB),
            "RWF" => Ok(Self::RWF),
            "SAR" => Ok(Self::SAR),
            "SBD" => Ok(Self::SBD),
            "SCR" => Ok(Self::SCR),
            "SDG" => Ok(Self::SDG),
            "SEK" => Ok(Self::SEK),
            "SGD" => Ok(Self::SGD),
            "SHP" => Ok(Self::SHP),
            "SLL" => Ok(Self::SLL),
            "SOS" => Ok(Self::SOS),
            "SRD" => Ok(Self::SRD),
            "SSP" => Ok(Self::SSP),
            "STD" => Ok(Self::STD),
            "SYP" => Ok(Self::SYP),
            "SZL" => Ok(Self::SZL),
            "THB" => Ok(Self::THB),
            "TJS" => Ok(Self::TJS),
            "TMT" => Ok(Self::TMT),
            "TND" => Ok(Self::TND),
            "TOP" => Ok(Self::TOP),
            "TRY" => Ok(Self::TRY),
            "TTD" => Ok(Self::TTD),
            "TWD" => Ok(Self::TWD),
            "TZS" => Ok(Self::TZS),
            "UAH" => Ok(Self::UAH),
            "UGX" => Ok(Self::UGX),
            "USD" => Ok(Self::USD),
            "UYU" => Ok(Self::UYU),
            "UZS" => Ok(Self::UZS),
            "VES" => Ok(Self::VES),
            "VND" => Ok(Self::VND),
            "VUV" => Ok(Self::VUV),
            "WST" => Ok(Self::WST),
            "XAF" => Ok(Self::XAF),
            "XCD" => Ok(Self::XCD),
            "XOF" => Ok(Self::XOF),
            "XPF" => Ok(Self::XPF),
            "YER" => Ok(Self::YER),
            "ZAR" => Ok(Self::ZAR),
            "ZMW" => Ok(Self::ZMW),
            _ => Err(SearchError::BadInput {
                expected: "valid currency code",
            }),
        }
    }
}

#[cfg(feature = "iso-currency-integration")]
impl CurrencyCode {
    /// If `iso-currency-integration` feature is enabled, you can convert it to [`iso_currency::Currency`](iso_currency::Currency) enum.
    /// Note that [`CurrencyCode::STD`](crate::CurrencyCode::STD) is not supported by [iso_currency](iso_currency) library.
    pub fn to_iso_currency(&self) -> iso_currency::Currency {
        match self {
            Self::AED => iso_currency::Currency::AED,
            Self::AFN => iso_currency::Currency::AFN,
            Self::ALL => iso_currency::Currency::ALL,
            Self::AMD => iso_currency::Currency::AMD,
            Self::ANG => iso_currency::Currency::ANG,
            Self::AOA => iso_currency::Currency::AOA,
            Self::ARS => iso_currency::Currency::ARS,
            Self::AUD => iso_currency::Currency::AUD,
            Self::AWG => iso_currency::Currency::AWG,
            Self::AZN => iso_currency::Currency::AZN,
            Self::BAM => iso_currency::Currency::BAM,
            Self::BBD => iso_currency::Currency::BBD,
            Self::BDT => iso_currency::Currency::BDT,
            Self::BGN => iso_currency::Currency::BGN,
            Self::BHD => iso_currency::Currency::BHD,
            Self::BIF => iso_currency::Currency::BIF,
            Self::BMD => iso_currency::Currency::BMD,
            Self::BND => iso_currency::Currency::BND,
            Self::BOB => iso_currency::Currency::BOB,
            Self::BRL => iso_currency::Currency::BRL,
            Self::BSD => iso_currency::Currency::BSD,
            Self::BTN => iso_currency::Currency::BTN,
            Self::BWP => iso_currency::Currency::BWP,
            Self::BYN => iso_currency::Currency::BYN,
            Self::BZD => iso_currency::Currency::BZD,
            Self::CAD => iso_currency::Currency::CAD,
            Self::CDF => iso_currency::Currency::CDF,
            Self::CHF => iso_currency::Currency::CHF,
            Self::CLP => iso_currency::Currency::CLP,
            Self::CNY => iso_currency::Currency::CNY,
            Self::COP => iso_currency::Currency::COP,
            Self::CRC => iso_currency::Currency::CRC,
            Self::CUP => iso_currency::Currency::CUP,
            Self::CVE => iso_currency::Currency::CVE,
            Self::CZK => iso_currency::Currency::CZK,
            Self::DJF => iso_currency::Currency::DJF,
            Self::DKK => iso_currency::Currency::DKK,
            Self::DOP => iso_currency::Currency::DOP,
            Self::DZD => iso_currency::Currency::DZD,
            Self::EGP => iso_currency::Currency::EGP,
            Self::ERN => iso_currency::Currency::ERN,
            Self::ETB => iso_currency::Currency::ETB,
            Self::EUR => iso_currency::Currency::EUR,
            Self::FJD => iso_currency::Currency::FJD,
            Self::FKP => iso_currency::Currency::FKP,
            Self::GBP => iso_currency::Currency::GBP,
            Self::GEL => iso_currency::Currency::GEL,
            Self::GHS => iso_currency::Currency::GHS,
            Self::GIP => iso_currency::Currency::GIP,
            Self::GMD => iso_currency::Currency::GMD,
            Self::GNF => iso_currency::Currency::GNF,
            Self::GTQ => iso_currency::Currency::GTQ,
            Self::GYD => iso_currency::Currency::GYD,
            Self::HKD => iso_currency::Currency::HKD,
            Self::HNL => iso_currency::Currency::HNL,
            Self::HTG => iso_currency::Currency::HTG,
            Self::HUF => iso_currency::Currency::HUF,
            Self::IDR => iso_currency::Currency::IDR,
            Self::ILS => iso_currency::Currency::ILS,
            Self::INR => iso_currency::Currency::INR,
            Self::IQD => iso_currency::Currency::IQD,
            Self::IRR => iso_currency::Currency::IRR,
            Self::ISK => iso_currency::Currency::ISK,
            Self::JMD => iso_currency::Currency::JMD,
            Self::JOD => iso_currency::Currency::JOD,
            Self::JPY => iso_currency::Currency::JPY,
            Self::KES => iso_currency::Currency::KES,
            Self::KGS => iso_currency::Currency::KGS,
            Self::KHR => iso_currency::Currency::KHR,
            Self::KMF => iso_currency::Currency::KMF,
            Self::KPW => iso_currency::Currency::KPW,
            Self::KRW => iso_currency::Currency::KRW,
            Self::KWD => iso_currency::Currency::KWD,
            Self::KYD => iso_currency::Currency::KYD,
            Self::KZT => iso_currency::Currency::KZT,
            Self::LAK => iso_currency::Currency::LAK,
            Self::LBP => iso_currency::Currency::LBP,
            Self::LKR => iso_currency::Currency::LKR,
            Self::LRD => iso_currency::Currency::LRD,
            Self::LSL => iso_currency::Currency::LSL,
            Self::LYD => iso_currency::Currency::LYD,
            Self::MAD => iso_currency::Currency::MAD,
            Self::MDL => iso_currency::Currency::MDL,
            Self::MGA => iso_currency::Currency::MGA,
            Self::MKD => iso_currency::Currency::MKD,
            Self::MMK => iso_currency::Currency::MMK,
            Self::MNT => iso_currency::Currency::MNT,
            Self::MOP => iso_currency::Currency::MOP,
            Self::MRU => iso_currency::Currency::MRU,
            Self::MUR => iso_currency::Currency::MUR,
            Self::MVR => iso_currency::Currency::MVR,
            Self::MWK => iso_currency::Currency::MWK,
            Self::MXN => iso_currency::Currency::MXN,
            Self::MYR => iso_currency::Currency::MYR,
            Self::MZN => iso_currency::Currency::MZN,
            Self::NAD => iso_currency::Currency::NAD,
            Self::NGN => iso_currency::Currency::NGN,
            Self::NIO => iso_currency::Currency::NIO,
            Self::NOK => iso_currency::Currency::NOK,
            Self::NPR => iso_currency::Currency::NPR,
            Self::NZD => iso_currency::Currency::NZD,
            Self::OMR => iso_currency::Currency::OMR,
            Self::PAB => iso_currency::Currency::PAB,
            Self::PEN => iso_currency::Currency::PEN,
            Self::PGK => iso_currency::Currency::PGK,
            Self::PHP => iso_currency::Currency::PHP,
            Self::PKR => iso_currency::Currency::PKR,
            Self::PLN => iso_currency::Currency::PLN,
            Self::PYG => iso_currency::Currency::PYG,
            Self::QAR => iso_currency::Currency::QAR,
            Self::RON => iso_currency::Currency::RON,
            Self::RSD => iso_currency::Currency::RSD,
            Self::RUB => iso_currency::Currency::RUB,
            Self::RWF => iso_currency::Currency::RWF,
            Self::SAR => iso_currency::Currency::SAR,
            Self::SBD => iso_currency::Currency::SBD,
            Self::SCR => iso_currency::Currency::SCR,
            Self::SDG => iso_currency::Currency::SDG,
            Self::SEK => iso_currency::Currency::SEK,
            Self::SGD => iso_currency::Currency::SGD,
            Self::SHP => iso_currency::Currency::SHP,
            Self::SLL => iso_currency::Currency::SLL,
            Self::SOS => iso_currency::Currency::SOS,
            Self::SRD => iso_currency::Currency::SRD,
            Self::SSP => iso_currency::Currency::SSP,
            Self::SYP => iso_currency::Currency::SYP,
            Self::SZL => iso_currency::Currency::SZL,
            Self::THB => iso_currency::Currency::THB,
            Self::TJS => iso_currency::Currency::TJS,
            Self::TMT => iso_currency::Currency::TMT,
            Self::TND => iso_currency::Currency::TND,
            Self::TOP => iso_currency::Currency::TOP,
            Self::TRY => iso_currency::Currency::TRY,
            Self::TTD => iso_currency::Currency::TTD,
            Self::TWD => iso_currency::Currency::TWD,
            Self::TZS => iso_currency::Currency::TZS,
            Self::UAH => iso_currency::Currency::UAH,
            Self::UGX => iso_currency::Currency::UGX,
            Self::USD => iso_currency::Currency::USD,
            Self::UYU => iso_currency::Currency::UYU,
            Self::UZS => iso_currency::Currency::UZS,
            Self::VES => iso_currency::Currency::VES,
            Self::VND => iso_currency::Currency::VND,
            Self::VUV => iso_currency::Currency::VUV,
            Self::WST => iso_currency::Currency::WST,
            Self::XAF => iso_currency::Currency::XAF,
            Self::XCD => iso_currency::Currency::XCD,
            Self::XOF => iso_currency::Currency::XOF,
            Self::XPF => iso_currency::Currency::XPF,
            Self::YER => iso_currency::Currency::YER,
            Self::ZAR => iso_currency::Currency::ZAR,
            Self::ZMW => iso_currency::Currency::ZMW,
            _ => unimplemented!("Not implement by iso_currency library"),
        }
    }
}
