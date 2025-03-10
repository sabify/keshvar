// DO NOT TOUCH THIS FILE. (Auto-generated by `keshvar-code-generator/src/countries.rs`)

//! A module for country `Åland`

#[cfg(all(feature = "ax", feature = "constants"))]
/// A module to access all constant country data for `Åland`.
///
/// Note that to use this module, `constant` feature should be enabled.
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, DistanceUnit, Region, SubRegion, WeekDay,
        WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::AX;
    pub const ALPHA3: Alpha3 = Alpha3::ALA;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 358;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;
    pub const GEC: Option<GEC> = None;
    pub const INTERNATIONAL_PREFIX: &str = "";
    pub const IOC: Option<IOC> = None;
    pub const ISO_SHORT_NAME: &str = "Åland Islands";
    pub const ISO_LONG_NAME: &str = "Åland";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["sv"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["sv"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[];
    pub const NATIONAL_PREFIX: &str = "";
    pub const NATIONALITY: Option<&str> = Some("Swedish");
    pub const VEHICLE_REGISTRATION_CODE: Option<&str> = Some("AX");
    pub const NUMBER: &str = "248";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("22\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::NorthernEurope);
    pub const UN_LOCODE: &str = "AX";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Åland Islands", "Åland", "オーランド諸島", "Ålandeilanden"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    pub const G7_MEMBER: bool = false;
    pub const G20_MEMBER: bool = false;
    pub const EU_MEMBER: bool = true;
    pub const UN_MEMBER: bool = false;
    pub const EEA_MEMBER: bool = false;
    pub const DISTANCE_UNIT: DistanceUnit = DistanceUnit::Km;
    pub const POPULATION: Option<u64> = None;
    #[cfg(feature = "emojis")]
    pub const EMOJI: &str = "🇦🇽";
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Åland Islands"),
        ("af", "Ålandeilande"),
        ("ak", "Åland Islands"),
        ("am", "Åland Islands"),
        ("an", "Islas Åland"),
        ("ar", "جزر آلاند"),
        ("as", "আল\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"),
        ("ay", "Åland Islands"),
        ("az", "Åland Islands"),
        ("ba", "Åland Islands"),
        ("be", "Аландскія астравы"),
        ("bg", "Аландски острови"),
        ("bi", "Åland Islands"),
        (
            "bn",
            "অ\u{9cd}য\u{9be}ল\u{9cd}য\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ",
        ),
        (
            "bn_IN",
            "অ\u{9cd}য\u{9be}ল\u{9cd}য\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ",
        ),
        ("br", "Åland"),
        ("bs", "Åland Ostrva"),
        ("ca", "Illes Aland"),
        ("ce", "Аландан гӀайренаш"),
        ("ch", "Åland Islands"),
        ("cs", "Ålandské ostrovy"),
        ("cv", "Аландан гӀайренаш"),
        ("cy", "Ynysoedd Åland"),
        ("da", "Åland"),
        ("de", "Åland-Inseln"),
        ("dv", "Åland Islands"),
        (
            "dz",
            "ཨ\u{f71}་ལ\u{f7a}ནཌ\u{f72}་ མཚ\u{f7c}་ག\u{fb3}\u{f72}ང།",
        ),
        ("ee", "Åland Islands"),
        ("el", "Νήσοι Ώλαντ"),
        ("en", "Åland Islands"),
        ("eo", "Alando"),
        ("es", "Islas Äland"),
        ("et", "Ahvenamaa"),
        ("eu", "Åland uharteak"),
        ("fa", "جزایر آلند"),
        ("ff", "Åland Islands"),
        ("fi", "Ahvenanmaa"),
        ("fo", "Áland"),
        ("fr", "Åland, Îles"),
        ("fy", "Ålandseilannen"),
        ("ga", "Oileáin Åland"),
        ("gl", "Illas Åland"),
        ("gn", "Åland Islands"),
        ("gu", "અલ\u{ac7}ન\u{acd}ડ ટાપ\u{ac1}ઓ"),
        ("gv", "Åland"),
        ("ha", "Åland"),
        ("he", "אולנד"),
        ("hi", "ऑल\u{948}ण\u{94d}ड द\u{94d}वीपसम\u{942}ह"),
        ("hr", "Alandski otoci"),
        ("ht", "Åland Islands"),
        ("hu", "Åland-szigetek"),
        ("hy", "Ալանդյան կղզիներ"),
        ("ia", "Insulas Åland"),
        ("id", "Kepulauan Åland"),
        ("io", "Alando"),
        ("is", "Álandseyjar"),
        ("it", "Isole Åland"),
        ("iu", "Åland Islands"),
        ("ja", "オーランド諸島"),
        ("ka", "ალანდის კუნძულები"),
        ("ki", "Åland Islands"),
        ("kk", "Аланд аралдары"),
        ("kl", "Åland Islands"),
        ("km", "កោះ\u{200b}អាឡង\u{17cb}"),
        ("kn", "ಅಲಾಂಡ\u{ccd} ದ\u{ccd}ವ\u{cc0}ಪಗಳು"),
        ("ko", "올란드 제도"),
        ("ku", "Giravên Åland"),
        ("kv", "Аланд діяс"),
        ("kw", "Åland"),
        ("ky", "Аланд аралдары"),
        ("lo", "Åland Islands"),
        ("lt", "Alandai"),
        ("lv", "Olande"),
        ("mi", "Moutere Aaland"),
        ("mk", "Ајланд острови"),
        (
            "ml",
            "ലണ\u{d4d}ട\u{d4d} ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}",
        ),
        ("mn", "Åland Islands"),
        ("mr", "अल\u{902}द आयल\u{945}\u{902}डस\u{94d}"),
        ("ms", "Kepulauan Åland"),
        ("mt", "Gżejjer Åland"),
        ("my", "Åland Islands"),
        ("na", "Åland Islands"),
        ("nb", "Åland"),
        ("ne", "एल\u{94d}याण\u{94d}ड टाप\u{941}"),
        ("nl", "Ålandseilanden"),
        ("nn", "Åland"),
        ("nv", "Åland Islands"),
        ("oc", "Åland Islands"),
        ("or", "ଆଲ\u{b3e}ଣ\u{b4d}ଡ ଦ\u{b4d}ବୀପ"),
        ("pa", "ਏਲ\u{a48}\u{a02}ਡ ਟਾਪ\u{a42}"),
        ("pi", "Åland Islands"),
        ("pl", "Wyspy Alandzkie"),
        ("ps", "Åland Islands"),
        ("pt", "Ilhas Alanda"),
        ("pt_BR", "Ilhas Åland"),
        ("ro", "Insulele Åland"),
        ("ru", "Аландские острова"),
        ("rw", "Ibirwa by'Alande"),
        ("sc", "Ìsulas Åland"),
        ("sd", "Åland Islands"),
        ("si", "ඒලන\u{dca}ඩ\u{dca} ද\u{dd6}පත\u{dca}"),
        ("sk", "Ålandy"),
        ("sl", "Ålandsko otočje"),
        ("so", "Åland"),
        ("sq", "Ishujt Aland"),
        ("sr", "Острва Аланд"),
        ("sv", "Åland"),
        ("sw", "Visiwa vya Åland"),
        ("ta", "அலண\u{bcd}ட\u{bcd} த\u{bc0}வுகள\u{bcd}"),
        ("te", "అలంద\u{c4d} ఐల\u{c3e}ండ\u{c4d}స\u{c4d}"),
        ("tg", "Ҷазираҳои Аланд"),
        ("th", "หม\u{e39}\u{e48}เกาะโอล\u{e31}นด\u{e4c}"),
        ("ti", "Åland Islands"),
        ("tk", "Aland adalary"),
        ("tl", "Mga Islang Åland"),
        ("tr", "Åland Adaları"),
        ("tt", "Аланд Утравлары"),
        ("ug", "ئالاند تاقىم ئاراللىرى"),
        ("uk", "Аландські острови"),
        ("ur", "جزائر ایلانڈ"),
        ("uz", "Aland orollari"),
        ("ve", "Åland Islands"),
        ("vi", "Quần đảo A-lanh"),
        ("wa", "Iyes Åland"),
        ("wo", "Dunu Aland"),
        ("xh", "Åland Islands"),
        ("yo", "Àwọn Erékùṣù Åland"),
        ("zh_CN", "奥兰群岛"),
        ("zh_HK", "奧蘭羣島"),
        ("zh_TW", "奧蘭群島"),
        ("zu", "Åland Islands"),
    ];
    #[cfg(all(feature = "ax", feature = "geo", feature = "constants"))]
    /// GEO data as constants
    pub mod geo {
        pub const LATITUDE: f64 = 60.1785247;
        pub const LONGITUDE: f64 = 19.9156105;
        pub const MAX_LATITUDE: f64 = 60.8400009;
        pub const MAX_LONGITUDE: f64 = 21.4866841;
        pub const MIN_LATITUDE: f64 = 59.6872001;
        pub const MIN_LONGITUDE: f64 = 19.2095998;
        pub const NORTHEAST_LATITUDE: f64 = 60.8400009;
        pub const NORTHEAST_LONGITUDE: f64 = 21.4866841;
        pub const SOUTHWEST_LATITUDE: f64 = 59.6872001;
        pub const SOUTHWEST_LONGITUDE: f64 = 19.2095998;
    }
}
#[cfg(all(feature = "ax", feature = "geo"))]
/// GEO module for this country.
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    /// GEO information for this country.
    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 60.1785247,
            longitude: 19.9156105,
            max_latitude: 60.8400009,
            max_longitude: 21.4866841,
            min_latitude: 59.6872001,
            min_longitude: 19.2095998,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 60.8400009,
                    longitude: 21.4866841,
                },
                southwest: CountryGeoBound {
                    latitude: 59.6872001,
                    longitude: 19.2095998,
                },
            },
        }
    }
}

#[cfg(all(feature = "ax", feature = "subdivisions"))]
/// Subdivision module for this country.
pub mod subdivisions {
    #[allow(unused_imports)]
    use crate::{Alpha2, Subdivision, SubdivisionType};
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::SubdivisionGeo;

    /// Subdivisions for this country.
    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from([])
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, DistanceUnit, Region, SubRegion, VatRates,
    WeekDay, WorldRegion, GEC, IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "ax")]
/// [`Country`](crate::Country) struct for this country.
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::AX,
        alpha3: Alpha3::ALA,
        address_format: None,
        continent: Continent::Europe,
        country_code: 358,
        currency_code: CurrencyCode::EUR,
        maybe_gec: None,
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "",
        maybe_ioc: None,
        iso_long_name: "Åland",
        iso_short_name: "Åland Islands",
        official_language_list: ["sv"].to_vec(),
        spoken_language_list: ["sv"].to_vec(),
        national_destination_code_length_list: [].to_vec(),
        national_number_length_list: [].to_vec(),
        national_prefix: "",
        maybe_nationality: Some("Swedish"),
        maybe_vehicle_registration_code: Some("AX"),
        number: "248",
        postal_code: true,
        postal_code_format: Some("22\\d{3}"),
        maybe_region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        maybe_subregion: Some(SubRegion::NorthernEurope),
        un_locode: "AX",
        unofficial_name_list: ["Åland Islands", "Åland", "オーランド諸島", "Ålandeilanden"]
            .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "emojis")]
        emoji: "🇦🇽",
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Åland Islands"),
            ("af", "Ålandeilande"),
            ("ak", "Åland Islands"),
            ("am", "Åland Islands"),
            ("an", "Islas Åland"),
            ("ar", "جزر آلاند"),
            ("as", "আল\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ"),
            ("ay", "Åland Islands"),
            ("az", "Åland Islands"),
            ("ba", "Åland Islands"),
            ("be", "Аландскія астравы"),
            ("bg", "Аландски острови"),
            ("bi", "Åland Islands"),
            (
                "bn",
                "অ\u{9cd}য\u{9be}ল\u{9cd}য\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ",
            ),
            (
                "bn_IN",
                "অ\u{9cd}য\u{9be}ল\u{9cd}য\u{9be}ন\u{9cd}ড দ\u{9cd}বীপপ\u{9c1}ঞ\u{9cd}জ",
            ),
            ("br", "Åland"),
            ("bs", "Åland Ostrva"),
            ("ca", "Illes Aland"),
            ("ce", "Аландан гӀайренаш"),
            ("ch", "Åland Islands"),
            ("cs", "Ålandské ostrovy"),
            ("cv", "Аландан гӀайренаш"),
            ("cy", "Ynysoedd Åland"),
            ("da", "Åland"),
            ("de", "Åland-Inseln"),
            ("dv", "Åland Islands"),
            (
                "dz",
                "ཨ\u{f71}་ལ\u{f7a}ནཌ\u{f72}་ མཚ\u{f7c}་ག\u{fb3}\u{f72}ང།",
            ),
            ("ee", "Åland Islands"),
            ("el", "Νήσοι Ώλαντ"),
            ("en", "Åland Islands"),
            ("eo", "Alando"),
            ("es", "Islas Äland"),
            ("et", "Ahvenamaa"),
            ("eu", "Åland uharteak"),
            ("fa", "جزایر آلند"),
            ("ff", "Åland Islands"),
            ("fi", "Ahvenanmaa"),
            ("fo", "Áland"),
            ("fr", "Åland, Îles"),
            ("fy", "Ålandseilannen"),
            ("ga", "Oileáin Åland"),
            ("gl", "Illas Åland"),
            ("gn", "Åland Islands"),
            ("gu", "અલ\u{ac7}ન\u{acd}ડ ટાપ\u{ac1}ઓ"),
            ("gv", "Åland"),
            ("ha", "Åland"),
            ("he", "אולנד"),
            ("hi", "ऑल\u{948}ण\u{94d}ड द\u{94d}वीपसम\u{942}ह"),
            ("hr", "Alandski otoci"),
            ("ht", "Åland Islands"),
            ("hu", "Åland-szigetek"),
            ("hy", "Ալանդյան կղզիներ"),
            ("ia", "Insulas Åland"),
            ("id", "Kepulauan Åland"),
            ("io", "Alando"),
            ("is", "Álandseyjar"),
            ("it", "Isole Åland"),
            ("iu", "Åland Islands"),
            ("ja", "オーランド諸島"),
            ("ka", "ალანდის კუნძულები"),
            ("ki", "Åland Islands"),
            ("kk", "Аланд аралдары"),
            ("kl", "Åland Islands"),
            ("km", "កោះ\u{200b}អាឡង\u{17cb}"),
            ("kn", "ಅಲಾಂಡ\u{ccd} ದ\u{ccd}ವ\u{cc0}ಪಗಳು"),
            ("ko", "올란드 제도"),
            ("ku", "Giravên Åland"),
            ("kv", "Аланд діяс"),
            ("kw", "Åland"),
            ("ky", "Аланд аралдары"),
            ("lo", "Åland Islands"),
            ("lt", "Alandai"),
            ("lv", "Olande"),
            ("mi", "Moutere Aaland"),
            ("mk", "Ајланд острови"),
            (
                "ml",
                "ലണ\u{d4d}ട\u{d4d} ദ\u{d4d}വീപ\u{d41}കള\u{d4d}\u{200d}",
            ),
            ("mn", "Åland Islands"),
            ("mr", "अल\u{902}द आयल\u{945}\u{902}डस\u{94d}"),
            ("ms", "Kepulauan Åland"),
            ("mt", "Gżejjer Åland"),
            ("my", "Åland Islands"),
            ("na", "Åland Islands"),
            ("nb", "Åland"),
            ("ne", "एल\u{94d}याण\u{94d}ड टाप\u{941}"),
            ("nl", "Ålandseilanden"),
            ("nn", "Åland"),
            ("nv", "Åland Islands"),
            ("oc", "Åland Islands"),
            ("or", "ଆଲ\u{b3e}ଣ\u{b4d}ଡ ଦ\u{b4d}ବୀପ"),
            ("pa", "ਏਲ\u{a48}\u{a02}ਡ ਟਾਪ\u{a42}"),
            ("pi", "Åland Islands"),
            ("pl", "Wyspy Alandzkie"),
            ("ps", "Åland Islands"),
            ("pt", "Ilhas Alanda"),
            ("pt_BR", "Ilhas Åland"),
            ("ro", "Insulele Åland"),
            ("ru", "Аландские острова"),
            ("rw", "Ibirwa by'Alande"),
            ("sc", "Ìsulas Åland"),
            ("sd", "Åland Islands"),
            ("si", "ඒලන\u{dca}ඩ\u{dca} ද\u{dd6}පත\u{dca}"),
            ("sk", "Ålandy"),
            ("sl", "Ålandsko otočje"),
            ("so", "Åland"),
            ("sq", "Ishujt Aland"),
            ("sr", "Острва Аланд"),
            ("sv", "Åland"),
            ("sw", "Visiwa vya Åland"),
            ("ta", "அலண\u{bcd}ட\u{bcd} த\u{bc0}வுகள\u{bcd}"),
            ("te", "అలంద\u{c4d} ఐల\u{c3e}ండ\u{c4d}స\u{c4d}"),
            ("tg", "Ҷазираҳои Аланд"),
            ("th", "หม\u{e39}\u{e48}เกาะโอล\u{e31}นด\u{e4c}"),
            ("ti", "Åland Islands"),
            ("tk", "Aland adalary"),
            ("tl", "Mga Islang Åland"),
            ("tr", "Åland Adaları"),
            ("tt", "Аланд Утравлары"),
            ("ug", "ئالاند تاقىم ئاراللىرى"),
            ("uk", "Аландські острови"),
            ("ur", "جزائر ایلانڈ"),
            ("uz", "Aland orollari"),
            ("ve", "Åland Islands"),
            ("vi", "Quần đảo A-lanh"),
            ("wa", "Iyes Åland"),
            ("wo", "Dunu Aland"),
            ("xh", "Åland Islands"),
            ("yo", "Àwọn Erékùṣù Åland"),
            ("zh_CN", "奥兰群岛"),
            ("zh_HK", "奧蘭羣島"),
            ("zh_TW", "奧蘭群島"),
            ("zu", "Åland Islands"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
        g7_member: false,
        g20_member: false,
        eu_member: true,
        un_member: false,
        eea_member: false,
        maybe_vat_rates: None,
        distance_unit: DistanceUnit::Km,
        maybe_population: None,
    }
}
