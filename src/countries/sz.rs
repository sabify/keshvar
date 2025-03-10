// DO NOT TOUCH THIS FILE. (Auto-generated by `keshvar-code-generator/src/countries.rs`)

//! A module for country `The Kingdom of Eswatini`

#[cfg(all(feature = "sz", feature = "constants"))]
/// A module to access all constant country data for `The Kingdom of Eswatini`.
///
/// Note that to use this module, `constant` feature should be enabled.
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, DistanceUnit, Region, SubRegion, WeekDay,
        WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::SZ;
    pub const ALPHA3: Alpha3 = Alpha3::SWZ;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 268;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::SZL;
    pub const GEC: Option<GEC> = Some(GEC::WZ);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::SWZ);
    pub const ISO_SHORT_NAME: &str = "Eswatini";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Eswatini";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["en", "ss"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["en", "ss"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Swazi");
    pub const VEHICLE_REGISTRATION_CODE: Option<&str> = Some("SD");
    pub const NUMBER: &str = "748";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("[HLMS]\\d{3}");
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthernAfrica);
    pub const UN_LOCODE: &str = "SZ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] =
        &["Swaziland", "Swasiland", "Suazilandia", "スワジランド"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    pub const G7_MEMBER: bool = false;
    pub const G20_MEMBER: bool = false;
    pub const EU_MEMBER: bool = false;
    pub const UN_MEMBER: bool = true;
    pub const EEA_MEMBER: bool = false;
    pub const DISTANCE_UNIT: DistanceUnit = DistanceUnit::Km;
    pub const POPULATION: Option<u64> = Some(1201670);
    #[cfg(feature = "emojis")]
    pub const EMOJI: &str = "🇸🇿";
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Eswatini"),
        ("af", "Eswatini"),
        ("ak", "Eswatini"),
        ("am", "Eswatini"),
        ("an", "Eswatini"),
        ("ar", "إسواتيني"),
        ("as", "Eswatini"),
        ("ay", "Eswatini"),
        ("az", "Eswatini"),
        ("ba", "Eswatini"),
        ("be", "Эсваціні"),
        ("bg", "Eswatini"),
        ("bi", "Eswatini"),
        ("bn", "এসওয়\u{9be}তিনি"),
        ("bn_IN", "এসওয\u{9bc}\u{9be}ট\u{9be}নি"),
        ("br", "Eswatini"),
        ("bs", "Eswatini"),
        ("ca", "Eswatini"),
        ("ce", "Eswatini"),
        ("ch", "Eswatini"),
        ("cs", "Svazijsko"),
        ("cv", "Eswatini"),
        ("cy", "Eswatini"),
        ("da", "Eswatini"),
        ("de", "Eswatini"),
        ("dv", "Eswatini"),
        ("dz", "Eswatini"),
        ("ee", "Eswatini"),
        ("el", "Εσουατίνι"),
        ("en", "Eswatini"),
        ("eo", "Svazilando"),
        ("es", "Esuatini"),
        ("et", "Svaasimaa"),
        ("eu", "Eswatini"),
        ("fa", "پادشاهی سوازیلند"),
        ("ff", "Eswatini"),
        ("fi", "Eswatini"),
        ("fo", "Eswatini"),
        ("fr", "Eswatini"),
        ("fy", "Eswatini"),
        ("ga", "Eswatini"),
        ("gl", "Suacilandia"),
        ("gn", "Eswatini"),
        ("gu", "સ\u{acd}વાતિની"),
        ("gv", "Eswatini"),
        ("ha", "Eswatini"),
        ("he", "אסווטני"),
        ("hi", "एस\u{94d}वाटिनी"),
        ("hr", "Esvatini"),
        ("ht", "Eswatini"),
        ("hu", "Szváziföld"),
        ("hy", "Eswatini"),
        ("ia", "Eswatini"),
        ("id", "Eswatini"),
        ("io", "Eswatini"),
        ("is", "Esvatiní (Svasíland)"),
        ("it", "Eswatini"),
        ("iu", "Eswatini"),
        ("ja", "エスワティニ王国"),
        ("ka", "Eswatini"),
        ("ki", "Eswatini"),
        ("kk", "Eswatini"),
        ("kl", "Eswatini"),
        ("km", "Eswatini"),
        ("kn", "Eswatini"),
        ("ko", "에스와티니"),
        ("ku", "Eswatini"),
        ("kv", "Eswatini"),
        ("kw", "Eswatini"),
        ("ky", "Eswatini"),
        ("lo", "Eswatini"),
        ("lt", "Eswatini"),
        ("lv", "Eswatini"),
        ("mi", "Eswatini"),
        ("mk", "Eswatini"),
        ("ml", "Eswatini"),
        ("mn", "Eswatini"),
        ("mr", "इस\u{94d}वातिनी"),
        ("ms", "Eswatini"),
        ("mt", "Eswatini"),
        ("my", "Eswatini"),
        ("na", "Eswatini"),
        ("nb", "Eswatini (tidligere Swasiland)"),
        ("ne", "Eswatini"),
        ("nl", "Eswatini"),
        ("nn", "Eswatini"),
        ("nv", "Eswatini"),
        ("oc", "Eswatini"),
        ("or", "ଏସୱଆଟ\u{b3f}ନ\u{b3f}"),
        ("pa", "ਇਸਵਾਟੀਨੀ"),
        ("pi", "Eswatini"),
        ("pl", "Eswatini"),
        ("ps", "Eswatini"),
        ("pt", "Suazilândia"),
        ("pt_BR", "Suazilândia"),
        ("ro", "Eswatini"),
        ("ru", "Эсватини"),
        ("rw", "Eswatini"),
        ("sc", "Swaziland"),
        ("sd", "Eswatini"),
        ("si", "Eswatini"),
        ("sk", "Eswatini"),
        ("sl", "Eswatini"),
        ("so", "Eswatini"),
        ("sq", "Esuatini"),
        ("sr", "Есватини"),
        ("sv", "Swaziland"),
        ("sw", "Eswatini"),
        ("ta", "Eswatini"),
        ("te", "Eswatini"),
        ("tg", "Эсватини"),
        ("th", "เอสวาต\u{e34}น\u{e35}"),
        ("ti", "Eswatini"),
        ("tk", "Eswatini"),
        ("tl", "Eswatini"),
        ("tr", "Eswatini"),
        ("tt", "Eswatini"),
        ("ug", "ئېسۋاتىنى"),
        ("uk", "Есватіні"),
        ("ur", "Eswatini"),
        ("uz", "Eswatini"),
        ("ve", "Eswatini"),
        ("vi", "Eswatini"),
        ("wa", "Eswatini"),
        ("wo", "Eswatini"),
        ("xh", "Eswatini"),
        ("yo", "Eswatini"),
        ("zh_CN", "斯威士兰"),
        ("zh_HK", "斯威士蘭"),
        ("zh_TW", "史瓦帝尼"),
        ("zu", "Eswatini"),
    ];
    #[cfg(all(feature = "sz", feature = "geo", feature = "constants"))]
    /// GEO data as constants
    pub mod geo {
        pub const LATITUDE: f64 = -26.522503;
        pub const LONGITUDE: f64 = 31.465866;
        pub const MAX_LATITUDE: f64 = -25.71792;
        pub const MAX_LONGITUDE: f64 = 32.1349067;
        pub const MIN_LATITUDE: f64 = -27.317402;
        pub const MIN_LONGITUDE: f64 = 30.79064;
        pub const NORTHEAST_LATITUDE: f64 = -25.71792;
        pub const NORTHEAST_LONGITUDE: f64 = 32.1349067;
        pub const SOUTHWEST_LATITUDE: f64 = -27.317402;
        pub const SOUTHWEST_LONGITUDE: f64 = 30.79064;
    }
}
#[cfg(all(feature = "sz", feature = "geo"))]
/// GEO module for this country.
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    /// GEO information for this country.
    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: -26.522503,
            longitude: 31.465866,
            max_latitude: -25.71792,
            max_longitude: 32.1349067,
            min_latitude: -27.317402,
            min_longitude: 30.79064,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: -25.71792,
                    longitude: 32.1349067,
                },
                southwest: CountryGeoBound {
                    latitude: -27.317402,
                    longitude: 30.79064,
                },
            },
        }
    }
}

#[cfg(all(feature = "sz", feature = "subdivisions"))]
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
        HashMap::from(
            [

                (
                    "HH",
                    Subdivision{
                        name: "Hhohho",
                        country_alpha2: Alpha2::SZ,
                        code: "HH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.1365662), longitude: Some(31.3541631), max_latitude: Some(-25.7196479), min_latitude: Some(-26.475104), max_longitude: Some(31.760738), min_longitude: Some(30.908245)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هوهو"), ("be", "Хоха"), ("bg", "Хохо"), ("bn", "হোহো জেল\u{9be}"), ("ca", "Hhohho"), ("ccp", "𑄦\u{1112e}𑄦\u{1112e}"), ("ceb", "Hhohho District"), ("cs", "Hhohho"), ("da", "Hhohho District"), ("de", "Hhohho"), ("el", "Χνόχο"), ("en", "Hhohho"), ("es", "Hhohho"), ("fa", "ناحیه هووهو"), ("fi", "Hhohho"), ("fr", "Hhohho"), ("gu", "હહહો જિલ\u{acd}લો"), ("he", "מחוז האהוהאהו"), ("hi", "होहो जिला"), ("hy", "Հոհո"), ("id", "Distrik Hhohho"), ("it", "distretto di Hhohho"), ("ja", "ホホ"), ("ka", "ჰოჰო"), ("kn", "ಹ\u{ccd}ಹ\u{ccb} ಹ\u{ccd}ಹ\u{ccb} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "호호 구"), ("lt", "Hhohho rajonas"), ("lv", "Hoho reģions"), ("mr", "होहो जिल\u{94d}हा"), ("ms", "Hhohho District"), ("nb", "Hhohho"), ("nl", "Hhohho"), ("no", "Hhohho"), ("pl", "Dystrykt Hhohho"), ("pt", "Hhohho"), ("ro", "Districtul Hhohho"), ("ru", "Хохо"), ("si", "හොහ\u{dca}හෝ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Хохо"), ("sr_Latn", "Hoho"), ("sv", "Hhohho"), ("ta", "ஹ\u{bcd}ஹஒஹொ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "హ\u{c4b}హ\u{c4d}హ\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "จ\u{e31}งหว\u{e31}ดโฮโฮ"), ("tr", "Hhohho District"), ("uk", "Хохо"), ("ur", "ہوہو علاقہ"), ("vi", "Quận Hhohho"), ("zh", "霍霍"), ("zu", "IHhohho")]),
                        unofficial_name_list: ["Hhohho"].to_vec(),
                    }
                ),
                (
                    "LU",
                    Subdivision{
                        name: "Lubombo",
                        country_alpha2: Alpha2::SZ,
                        code: "LU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.7851773), longitude: Some(31.8107079), max_latitude: Some(-25.93021), min_latitude: Some(-27.1706039), max_longitude: Some(32.13726), min_longitude: Some(31.4920601)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة لوبومبو"), ("bg", "Лубомбо"), ("bn", "ল\u{9c1}বোম\u{9cd}ব\u{9c1} জেল\u{9be}"), ("ca", "Lubombo"), ("ccp", "𑄣\u{1112a}𑄝\u{1112a}𑄟\u{11134}𑄝\u{1112e}"), ("ceb", "Lubombo District"), ("da", "Lubombo District"), ("de", "Lubombo"), ("el", "Λουμπόμπο"), ("en", "Lubombo"), ("es", "Lubombo"), ("fa", "ناحیه لوبومبو"), ("fi", "Lubombo"), ("fr", "Lubombo"), ("gu", "લ\u{ac1}મોમ\u{acd}બો જિલ\u{acd}લો"), ("he", "מחוז לובומבו"), ("hi", "ल\u{941}बोम\u{94d}बो जिला"), ("hy", "Լուբոմբո"), ("id", "Distrik Lubombo"), ("it", "distretto di Lubombo"), ("ja", "ルボンボ"), ("ka", "ლუბომბო"), ("kn", "ಲುಬ\u{cca}ಂಬಾ ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "루봄보 구"), ("lt", "Lubombo rajonas"), ("lv", "Lubombo reģions"), ("mr", "ल\u{941}ब\u{94d}म\u{94d}बो जिल\u{94d}हा"), ("ms", "Lubombo District"), ("nb", "Lubombo"), ("nl", "Lubombo"), ("no", "Lubombo"), ("pl", "Dystrykt Lubombo"), ("pt", "Lubombo"), ("ro", "Districtul Lubombo"), ("ru", "Лубомбо"), ("si", "ල\u{dd4}බෝම\u{dca}බෝ ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Лубомбо"), ("sr_Latn", "Lubombo"), ("sv", "Lubombo"), ("ta", "லுப\u{bbe}ம\u{bcd}போ ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "లుబ\u{c4b}ంబ\u{c4b} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ล\u{e39}บอมโบ"), ("tr", "Lubombo District"), ("uk", "Лубомбо"), ("ur", "لوبومبو علاقہ"), ("vi", "Quận Lubombo"), ("zh", "卢邦博")]),
                        unofficial_name_list: ["Lebombo"].to_vec(),
                    }
                ),
                (
                    "MA",
                    Subdivision{
                        name: "Manzini",
                        country_alpha2: Alpha2::SZ,
                        code: "MA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.644588), longitude: Some(31.3541631), max_latitude: Some(-26.1589679), min_latitude: Some(-26.8945191), max_longitude: Some(31.706045), min_longitude: Some(30.794107)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة منزيني"), ("bg", "Манзини"), ("bn", "ম\u{9be}ঞ\u{9cd}জিনি জেল\u{9be}"), ("ca", "Manzini"), ("ccp", "𑄟𑄚\u{11134}𑄎\u{11128}𑄚\u{11128}"), ("ceb", "Manzini District"), ("da", "Manzini District"), ("de", "Manzini"), ("el", "Μανζίνι"), ("en", "Manzini"), ("es", "Distrito de Manzini"), ("fa", "منطقه مانزینی"), ("fi", "Manzini"), ("fr", "Manzini"), ("gu", "મ\u{ac7}ન\u{acd}ઝીની જિલ\u{acd}લો"), ("he", "מחוז מנזיני"), ("hi", "म\u{948}न\u{94d}ज\u{93c}ीनी प\u{94d}रद\u{947}श"), ("hy", "Մանզինի"), ("id", "Distrik Manzini"), ("it", "distretto di Manzini"), ("ja", "マンジニ地方"), ("ka", "მანზინის რეგიონი"), ("kn", "ಮಂಜ\u{cbf}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "만지니 구"), ("lt", "Manzinio rajonas"), ("lv", "Manzini reģions"), ("mr", "मा\u{902}झिनी जिल\u{94d}हा"), ("ms", "Manzini District"), ("nb", "Manzini"), ("nl", "Manzini"), ("no", "Manzini"), ("pl", "Dystrykt Manzini"), ("pt", "Manzini"), ("ro", "Districtul Manzini, Swaziland"), ("ru", "Манзини"), ("si", "මන\u{dca}ස\u{dd2}න\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Манзини (покрајина)"), ("sr_Latn", "Manzini (pokrajina)"), ("sv", "Manzini"), ("ta", "மஞ\u{bcd}சினி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "మ\u{c3e}ంజ\u{c3f}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ม\u{e31}นซ\u{e34}น\u{e35}"), ("tr", "Manzini District"), ("uk", "Манзіні"), ("ur", "مانزینی علاقہ"), ("vi", "Quận Manzini"), ("zh", "曼齊尼區")]),
                        unofficial_name_list: ["Manzini"].to_vec(),
                    }
                ),
                (
                    "SH",
                    Subdivision{
                        name: "Shiselweni",
                        country_alpha2: Alpha2::SZ,
                        code: "SH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-26.9827577), longitude: Some(31.3541631), max_latitude: Some(-26.75172), min_latitude: Some(-27.317101), max_longitude: Some(31.980196), min_longitude: Some(30.902433)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شيزلويني"), ("bg", "Шиселвени"), ("bn", "শিসেলওয\u{9bc}েনি জেল\u{9be}"), ("ca", "Shiselweni"), ("ccp", "𑄥\u{1112d}𑄥𑄬𑄣\u{11134}𑄤𑄚\u{11128}"), ("ceb", "Shiselweni District"), ("da", "Shiselweni District"), ("de", "Shiselweni"), ("el", "Σισελουένι"), ("en", "Shiselweni"), ("es", "Shishelweni"), ("fa", "ناحیه شیزلونی"), ("fi", "Shiselweni"), ("fr", "Shiselweni"), ("gu", "શીસ\u{ac7}લવ\u{ac7}ની જિલ\u{acd}લો"), ("he", "מחוז שיסלוואני"), ("hi", "शिस\u{947}ल\u{94d}व\u{947}नी जिला"), ("hy", "Շիսելվենի"), ("id", "Distrik Shiselweni"), ("it", "distretto di Shiselweni"), ("ja", "シセルウェニ"), ("ka", "შისელვენი"), ("kn", "ಶ\u{cbf}ಸ\u{cc6}ಲ\u{ccd}ವ\u{cc6}ನ\u{cbf} ಜ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "시셀웨니 구"), ("lt", "Šiselvenio rajonas"), ("lv", "Šiselveni reģions"), ("mr", "शीसलन\u{947}णी जिल\u{94d}हा"), ("ms", "Shiselweni District"), ("nb", "Shiselweni"), ("nl", "Shiselweni"), ("no", "Shiselweni"), ("pl", "Dystrykt Shiselweni"), ("pt", "Shishelweni"), ("ro", "Districtul Shiselweni"), ("ru", "Шиселвени"), ("si", "ශ\u{dd2}සේල\u{dca}වෙන\u{dd2} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sr", "Шиселвени"), ("sr_Latn", "Šiselveni"), ("sv", "Shiselweni"), ("ta", "ஷிசெலவெனி ம\u{bbe}வட\u{bcd}டம\u{bcd}"), ("te", "ష\u{c3f}ష\u{c46}ల\u{c4d}వ\u{c47}న\u{c3f} జ\u{c3f}ల\u{c4d}ల\u{c3e}"), ("th", "ช\u{e34}เซลเวน\u{e35}"), ("tr", "Shiselweni District"), ("uk", "Шіселвені"), ("ur", "شیزلوینی علاقہ"), ("vi", "Quận Shiselweni"), ("zh", "希塞卢韦尼")]),
                        unofficial_name_list: ["Shiselweni"].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, DistanceUnit, Region, SubRegion, VatRates,
    WeekDay, WorldRegion, GEC, IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "sz")]
/// [`Country`](crate::Country) struct for this country.
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::SZ,
        alpha3: Alpha3::SWZ,
        address_format: None,
        continent: Continent::Africa,
        country_code: 268,
        currency_code: CurrencyCode::SZL,
        maybe_gec: Some(GEC::WZ),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        maybe_ioc: Some(IOC::SWZ),
        iso_long_name: "The Kingdom of Eswatini",
        iso_short_name: "Eswatini",
        official_language_list: ["en", "ss"].to_vec(),
        spoken_language_list: ["en", "ss"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        maybe_nationality: Some("Swazi"),
        maybe_vehicle_registration_code: Some("SD"),
        number: "748",
        postal_code: true,
        postal_code_format: Some("[HLMS]\\d{3}"),
        maybe_region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        maybe_subregion: Some(SubRegion::SouthernAfrica),
        un_locode: "SZ",
        unofficial_name_list: ["Swaziland", "Swasiland", "Suazilandia", "スワジランド"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "emojis")]
        emoji: "🇸🇿",
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Eswatini"),
            ("af", "Eswatini"),
            ("ak", "Eswatini"),
            ("am", "Eswatini"),
            ("an", "Eswatini"),
            ("ar", "إسواتيني"),
            ("as", "Eswatini"),
            ("ay", "Eswatini"),
            ("az", "Eswatini"),
            ("ba", "Eswatini"),
            ("be", "Эсваціні"),
            ("bg", "Eswatini"),
            ("bi", "Eswatini"),
            ("bn", "এসওয়\u{9be}তিনি"),
            ("bn_IN", "এসওয\u{9bc}\u{9be}ট\u{9be}নি"),
            ("br", "Eswatini"),
            ("bs", "Eswatini"),
            ("ca", "Eswatini"),
            ("ce", "Eswatini"),
            ("ch", "Eswatini"),
            ("cs", "Svazijsko"),
            ("cv", "Eswatini"),
            ("cy", "Eswatini"),
            ("da", "Eswatini"),
            ("de", "Eswatini"),
            ("dv", "Eswatini"),
            ("dz", "Eswatini"),
            ("ee", "Eswatini"),
            ("el", "Εσουατίνι"),
            ("en", "Eswatini"),
            ("eo", "Svazilando"),
            ("es", "Esuatini"),
            ("et", "Svaasimaa"),
            ("eu", "Eswatini"),
            ("fa", "پادشاهی سوازیلند"),
            ("ff", "Eswatini"),
            ("fi", "Eswatini"),
            ("fo", "Eswatini"),
            ("fr", "Eswatini"),
            ("fy", "Eswatini"),
            ("ga", "Eswatini"),
            ("gl", "Suacilandia"),
            ("gn", "Eswatini"),
            ("gu", "સ\u{acd}વાતિની"),
            ("gv", "Eswatini"),
            ("ha", "Eswatini"),
            ("he", "אסווטני"),
            ("hi", "एस\u{94d}वाटिनी"),
            ("hr", "Esvatini"),
            ("ht", "Eswatini"),
            ("hu", "Szváziföld"),
            ("hy", "Eswatini"),
            ("ia", "Eswatini"),
            ("id", "Eswatini"),
            ("io", "Eswatini"),
            ("is", "Esvatiní (Svasíland)"),
            ("it", "Eswatini"),
            ("iu", "Eswatini"),
            ("ja", "エスワティニ王国"),
            ("ka", "Eswatini"),
            ("ki", "Eswatini"),
            ("kk", "Eswatini"),
            ("kl", "Eswatini"),
            ("km", "Eswatini"),
            ("kn", "Eswatini"),
            ("ko", "에스와티니"),
            ("ku", "Eswatini"),
            ("kv", "Eswatini"),
            ("kw", "Eswatini"),
            ("ky", "Eswatini"),
            ("lo", "Eswatini"),
            ("lt", "Eswatini"),
            ("lv", "Eswatini"),
            ("mi", "Eswatini"),
            ("mk", "Eswatini"),
            ("ml", "Eswatini"),
            ("mn", "Eswatini"),
            ("mr", "इस\u{94d}वातिनी"),
            ("ms", "Eswatini"),
            ("mt", "Eswatini"),
            ("my", "Eswatini"),
            ("na", "Eswatini"),
            ("nb", "Eswatini (tidligere Swasiland)"),
            ("ne", "Eswatini"),
            ("nl", "Eswatini"),
            ("nn", "Eswatini"),
            ("nv", "Eswatini"),
            ("oc", "Eswatini"),
            ("or", "ଏସୱଆଟ\u{b3f}ନ\u{b3f}"),
            ("pa", "ਇਸਵਾਟੀਨੀ"),
            ("pi", "Eswatini"),
            ("pl", "Eswatini"),
            ("ps", "Eswatini"),
            ("pt", "Suazilândia"),
            ("pt_BR", "Suazilândia"),
            ("ro", "Eswatini"),
            ("ru", "Эсватини"),
            ("rw", "Eswatini"),
            ("sc", "Swaziland"),
            ("sd", "Eswatini"),
            ("si", "Eswatini"),
            ("sk", "Eswatini"),
            ("sl", "Eswatini"),
            ("so", "Eswatini"),
            ("sq", "Esuatini"),
            ("sr", "Есватини"),
            ("sv", "Swaziland"),
            ("sw", "Eswatini"),
            ("ta", "Eswatini"),
            ("te", "Eswatini"),
            ("tg", "Эсватини"),
            ("th", "เอสวาต\u{e34}น\u{e35}"),
            ("ti", "Eswatini"),
            ("tk", "Eswatini"),
            ("tl", "Eswatini"),
            ("tr", "Eswatini"),
            ("tt", "Eswatini"),
            ("ug", "ئېسۋاتىنى"),
            ("uk", "Есватіні"),
            ("ur", "Eswatini"),
            ("uz", "Eswatini"),
            ("ve", "Eswatini"),
            ("vi", "Eswatini"),
            ("wa", "Eswatini"),
            ("wo", "Eswatini"),
            ("xh", "Eswatini"),
            ("yo", "Eswatini"),
            ("zh_CN", "斯威士兰"),
            ("zh_HK", "斯威士蘭"),
            ("zh_TW", "史瓦帝尼"),
            ("zu", "Eswatini"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
        g7_member: false,
        g20_member: false,
        eu_member: false,
        un_member: true,
        eea_member: false,
        maybe_vat_rates: None,
        distance_unit: DistanceUnit::Km,
        maybe_population: Some(1201670),
    }
}
