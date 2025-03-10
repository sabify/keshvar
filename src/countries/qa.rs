// DO NOT TOUCH THIS FILE. (Auto-generated by `keshvar-code-generator/src/countries.rs`)

//! A module for country `The State of Qatar`

#[cfg(all(feature = "qa", feature = "constants"))]
/// A module to access all constant country data for `The State of Qatar`.
///
/// Note that to use this module, `constant` feature should be enabled.
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, DistanceUnit, Region, SubRegion, WeekDay,
        WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::QA;
    pub const ALPHA3: Alpha3 = Alpha3::QAT;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 974;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::QAR;
    pub const GEC: Option<GEC> = Some(GEC::QA);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<IOC> = Some(IOC::QAT);
    pub const ISO_SHORT_NAME: &str = "Qatar";
    pub const ISO_LONG_NAME: &str = "The State of Qatar";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ar"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[7];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Qatari");
    pub const VEHICLE_REGISTRATION_CODE: Option<&str> = Some("Q");
    pub const NUMBER: &str = "634";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Sunday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAsia);
    pub const UN_LOCODE: &str = "QA";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Qatar", "قطر", "Katar", "カタール"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    pub const G7_MEMBER: bool = false;
    pub const G20_MEMBER: bool = false;
    pub const EU_MEMBER: bool = false;
    pub const UN_MEMBER: bool = true;
    pub const EEA_MEMBER: bool = false;
    pub const DISTANCE_UNIT: DistanceUnit = DistanceUnit::Km;
    pub const POPULATION: Option<u64> = Some(2695122);
    #[cfg(feature = "emojis")]
    pub const EMOJI: &str = "🇶🇦";
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Qatar"),
        ("af", "Katar"),
        ("ak", "Qatar"),
        ("am", "ቃጣር"),
        ("an", "Qatar"),
        ("ar", "قطر"),
        ("as", "ক\u{9be}ট\u{9be}ৰ"),
        ("ay", "Qatar"),
        ("az", "Katar"),
        ("ba", "Qatar"),
        ("be", "Катар"),
        ("bg", "Катар"),
        ("bi", "Qatar"),
        ("bn", "ক\u{9be}ত\u{9be}র"),
        ("bn_IN", "ক\u{9be}ত\u{9be}র"),
        ("br", "Katar"),
        ("bs", "Katar"),
        ("ca", "Qatar"),
        ("ce", "КъатӀар"),
        ("ch", "Qatar"),
        ("cs", "Katar"),
        ("cv", "КъатӀар"),
        ("cy", "Qatar"),
        ("da", "Qatar"),
        ("de", "Katar"),
        ("dv", "ޤ\u{7a6}ޠ\u{7a6}ރ\u{7aa}"),
        ("dz", "ཀ་ཊར།"),
        ("ee", "Qatar"),
        ("el", "Κατάρ"),
        ("en", "Qatar"),
        ("eo", "Kataro"),
        ("es", "Catar"),
        ("et", "Katar"),
        ("eu", "Qatar"),
        ("fa", "قطر"),
        ("ff", "Qatar"),
        ("fi", "Qatar"),
        ("fo", "Katar"),
        ("fr", "Qatar"),
        ("fy", "Katar"),
        ("ga", "Catar"),
        ("gl", "Qatar"),
        ("gn", "Qatar"),
        ("gu", "કતાર"),
        ("gv", "Yn Chatar"),
        ("ha", "Qatar"),
        ("he", "קטר"),
        ("hi", "क\u{93c}तर"),
        ("hr", "Katar"),
        ("ht", "Katar"),
        ("hu", "Katar"),
        ("hy", "Կատար"),
        ("ia", "Qatar"),
        ("id", "Qatar"),
        ("io", "Katar"),
        ("is", "Katar"),
        ("it", "Qatar"),
        ("iu", "Qatar"),
        ("ja", "カタール"),
        ("ka", "კატარი"),
        ("ki", "Qatar"),
        ("kk", "Катар"),
        ("kl", "Qatar"),
        ("km", "កាតារ"),
        ("kn", "ಕತಾರ\u{ccd}"),
        ("ko", "카타르"),
        ("ku", "Qatar"),
        ("kv", "Катар"),
        ("kw", "Katar"),
        ("ky", "Катар"),
        ("lo", "ປະເທດກາຕາ"),
        ("lt", "Kataras"),
        ("lv", "Katara"),
        ("mi", "Qatar"),
        ("mk", "Катар"),
        ("ml", "ഖത\u{d4d}തര\u{d4d}\u{200d}"),
        ("mn", "Катар"),
        ("mr", "कतार"),
        ("ms", "Qatar"),
        ("mt", "Qatar"),
        ("my", "ကာတာန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
        ("na", "Qatar"),
        ("nb", "Qatar"),
        ("ne", "कतार"),
        ("nl", "Qatar"),
        ("nn", "Qatar"),
        ("nv", "Qatar"),
        ("oc", "Qatar"),
        ("or", "କତ\u{b3e}ର"),
        ("pa", "ਕਤਰ"),
        ("pi", "कतार"),
        ("pl", "Katar"),
        ("ps", "قطر"),
        ("pt", "Catar"),
        ("pt_BR", "Catar"),
        ("ro", "Qatar"),
        ("ru", "Катар"),
        ("rw", "Katari"),
        ("sc", "Catar"),
        ("sd", "Qatar"),
        ("si", "කට\u{dcf}ර\u{dca}"),
        ("sk", "Katar"),
        ("sl", "Katar"),
        ("so", "Qadar"),
        ("sq", "Katar"),
        ("sr", "Катар"),
        ("sv", "Qatar"),
        ("sw", "Qatar"),
        ("ta", "கத\u{bbe}ர\u{bcd}"),
        ("te", "కత\u{c3e}ర\u{c4d}"),
        ("tg", "Қатар"),
        ("th", "กาตาร\u{e4c}"),
        ("ti", "Qatar"),
        ("tk", "Katar"),
        ("tl", "Qatar"),
        ("tr", "Katar"),
        ("tt", "Катар"),
        ("ug", "قاتار"),
        ("uk", "Катар"),
        ("ur", "قطر"),
        ("uz", "Qatar"),
        ("ve", "Qatar"),
        ("vi", "Ca-tă"),
        ("wa", "Katar"),
        ("wo", "Xatar"),
        ("xh", "Qatar"),
        ("yo", "Katar"),
        ("zh_CN", "卡塔尔"),
        ("zh_HK", "卡塔爾"),
        ("zh_TW", "卡達"),
        ("zu", "Qatar"),
    ];
    #[cfg(all(feature = "qa", feature = "geo", feature = "constants"))]
    /// GEO data as constants
    pub mod geo {
        pub const LATITUDE: f64 = 25.354826;
        pub const LONGITUDE: f64 = 51.183884;
        pub const MAX_LATITUDE: f64 = 26.2171;
        pub const MAX_LONGITUDE: f64 = 51.7144001;
        pub const MIN_LATITUDE: f64 = 24.471118;
        pub const MIN_LONGITUDE: f64 = 50.7211001;
        pub const NORTHEAST_LATITUDE: f64 = 26.2171;
        pub const NORTHEAST_LONGITUDE: f64 = 51.7144001;
        pub const SOUTHWEST_LATITUDE: f64 = 24.471118;
        pub const SOUTHWEST_LONGITUDE: f64 = 50.7211001;
    }
}
#[cfg(all(feature = "qa", feature = "geo"))]
/// GEO module for this country.
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    /// GEO information for this country.
    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 25.354826,
            longitude: 51.183884,
            max_latitude: 26.2171,
            max_longitude: 51.7144001,
            min_latitude: 24.471118,
            min_longitude: 50.7211001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 26.2171,
                    longitude: 51.7144001,
                },
                southwest: CountryGeoBound {
                    latitude: 24.471118,
                    longitude: 50.7211001,
                },
            },
        }
    }
}

#[cfg(all(feature = "qa", feature = "subdivisions"))]
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
                    "DA",
                    Subdivision{
                        name: "Ad Dawhah",
                        country_alpha2: Alpha2::QA,
                        code: "DA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.2916097), longitude: Some(51.5304368), max_latitude: Some(25.4125783), min_latitude: Some(25.1954283), max_longitude: Some(51.6281212), min_longitude: Some(51.4307964)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Doha"), ("am", "ዶሃ"), ("ar", "الدوحة"), ("az", "Doha"), ("be", "Горад Доха"), ("bg", "Доха"), ("bn", "দোহ\u{9be}"), ("bs", "Doha"), ("ca", "Doha"), ("ccp", "𑄘\u{1112e}𑄦"), ("ceb", "Doha"), ("cs", "Dauhá"), ("cy", "Doha"), ("da", "Doha"), ("de", "Doha"), ("el", "Ντόχα"), ("en", "Doha"), ("es", "Doha"), ("et", "Doha"), ("eu", "Doha"), ("fa", "دوحه"), ("fi", "Doha"), ("fr", "Doha"), ("ga", "Doha"), ("gl", "Doha"), ("gu", "દોહા"), ("ha", "Doha"), ("ha_NE", "Doha"), ("he", "דוחה"), ("hi", "दोहा"), ("hr", "Doha"), ("hu", "Doha"), ("hy", "Դոհա"), ("id", "Doha"), ("is", "Doha"), ("it", "Doha"), ("ja", "ドーハ"), ("jv", "Doha"), ("ka", "დოჰა"), ("kk", "Доха"), ("km", "ដ\u{17bc}ហា"), ("kn", "ದ\u{cca}ಹಾ"), ("ko", "도하"), ("ky", "Доха"), ("lt", "Doha"), ("lv", "Doha"), ("mk", "Доха"), ("ml", "ദോഹ"), ("mn", "Доха"), ("mr", "दोहा"), ("ms", "Doha"), ("my", "ဒ\u{102d}\u{102f}ဟာမြ\u{102d}\u{102f}\u{1037}"), ("nb", "Doha"), ("ne", "दोहा"), ("nl", "Doha"), ("no", "Doha"), ("or", "ଦୋହ\u{b3e}"), ("pa", "ਦ\u{a4b}ਹਾ"), ("pl", "Doha"), ("ps", "دوها"), ("pt", "Doha"), ("ro", "Doha"), ("ru", "Доха"), ("sd", "دوحہ"), ("si", "\u{200d}දෝහ\u{dcf}"), ("sk", "Dauha"), ("sl", "Doha"), ("so", "Dooxa"), ("sq", "Doha"), ("sr", "Доха"), ("sr_Latn", "Doha"), ("sv", "Doha"), ("sw", "Doha"), ("ta", "தோக\u{bbe}"), ("te", "ద\u{c4b}హ\u{c3e}"), ("th", "โดฮา"), ("tk", "Doha"), ("tr", "Doha"), ("uk", "Доха"), ("ur", "دوحہ"), ("uz", "Doha"), ("vi", "Doha"), ("yo", "Doha"), ("yo_BJ", "Doha"), ("yue", "多哈"), ("yue_Hans", "多哈"), ("zh", "多哈")]),
                        unofficial_name_list: ["Dawhah", "Doha", "Doha", "Doha", "Doha", "ad-Dawhah", "ad-Dawh\u{328}ah"].to_vec(),
                    }
                ),
                (
                    "KH",
                    Subdivision{
                        name: "Al Khawr",
                        country_alpha2: Alpha2::QA,
                        code: "KH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.6804078), longitude: Some(51.4968502), max_latitude: Some(25.6959881), min_latitude: Some(25.6203229), max_longitude: Some(51.5287971), min_longitude: Some(51.4761828)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الخور والدخيرة"), ("bn", "আল খোর"), ("ccp", "𑄃𑄣\u{11134} 𑄈\u{1112e}𑄢\u{11134}"), ("ceb", "Baladīyat al Khawr wa adh Dhakhīrah"), ("da", "Al Khor"), ("de", "Al-Khor"), ("el", "Αλ Κορ"), ("en", "Al Khor"), ("es", "Jor"), ("fa", "الخور والدخیرة"), ("fi", "Al-Khawr"), ("fr", "Al Khawr"), ("gu", "અલ ખોર"), ("he", "אל-ח׳ור"), ("hi", "अल खोर"), ("id", "Al Khor"), ("it", "Al Khawr"), ("ja", "アル＝ハウル"), ("kn", "ಅಲ\u{ccd} ಖ\u{ccb}ರ\u{ccd}"), ("ko", "알코르"), ("lt", "Chauras"), ("lv", "Haura"), ("mk", "Хор"), ("mr", "अल ख\u{941}र"), ("ms", "Al Khor"), ("nb", "Al Khor"), ("nl", "Al Khawr"), ("no", "Al Khor"), ("pl", "Al-Chaur"), ("pt", "Al Khor"), ("ro", "Al Khor"), ("ru", "Эль-Хаур"), ("si", "අල\u{dca} ඛොර\u{dca}"), ("sv", "Al Khor"), ("ta", "அல\u{bcd} க\u{bcd}ஹோர\u{bcd}"), ("te", "అల\u{c4d} ఖ\u{c4b}ర\u{c4d}"), ("th", "อ\u{e31}ล คอร\u{e4c}"), ("tr", "Al Khor"), ("uk", "Аль-Хор"), ("ur", "الخور و الدخیرہ"), ("vi", "Al Khor"), ("zh", "艾科爾")]),
                        unofficial_name_list: ["H\u{331}ūr", "al-H\u{331}awr", "al-Khawr", "al-Khour"].to_vec(),
                    }
                ),
                (
                    "MS",
                    Subdivision{
                        name: "Madinat ash Shamal",
                        country_alpha2: Alpha2::QA,
                        code: "MS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(26.1156079), longitude: Some(51.2218451), max_latitude: Some(26.123153), min_latitude: Some(26.1038857), max_longitude: Some(51.23826039999999), min_longitude: Some(51.2056447)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية الشمال"), ("bn", "ম\u{9be}দিন\u{9be}ত আস শ\u{9be}ম\u{9be}ল"), ("ca", "Madinat ash Shamal"), ("ccp", "𑄟𑄓\u{11128}𑄚𑄖\u{11134} 𑄃𑄌\u{11134} 𑄥\u{11133}𑄠𑄟\u{11127}𑄣\u{11134}"), ("da", "Madinat ash Shamal"), ("de", "Asch-Schamal"), ("el", "Μαντινάτ ας-Σαμάλ"), ("en", "Madinat ash Shamal"), ("es", "Madinat ash Shamal"), ("fa", "الشمال"), ("fi", "Al-Šamal"), ("fr", "Ash Shamal"), ("gu", "મદીનાત એશ શમલ"), ("he", "בלדית א-שמאל"), ("hi", "मदिनत ऐश शमल"), ("id", "Madinat ash-Shamal"), ("it", "Madinat ash Shamal"), ("ja", "アッ＝シャマール"), ("kn", "ಮಡ\u{cbf}ನಾತ\u{ccd} ಆಶ\u{ccd} ಶಾಮಲ\u{ccd}"), ("ko", "아시샤말"), ("lt", "Šamalas"), ("lv", "Medīnat Eš Šamāla"), ("mk", "Шамал"), ("mr", "मदिनत आश शामल"), ("ms", "Madinat ash Shamal"), ("nb", "Ash Shamal"), ("nl", "Ash Shamal"), ("no", "Ash Shamal"), ("pl", "Asz-Szamal"), ("pt", "Madinat ash Shamal"), ("ro", "Madinat ash Shamal"), ("ru", "Аш-Шамаль"), ("si", "මඩ\u{dd2}න\u{dcf}ට\u{dca} ඇශ\u{dca} ශමල\u{dca}"), ("sv", "Madinat ash Shamal"), ("ta", "ம\u{bbe}டினட\u{bcd} ஆஷ\u{bcd} ஷ\u{bbe}மல\u{bcd}"), ("te", "మ\u{c3e}డ\u{c3f}న\u{c3e}ట\u{c4d} ఆష\u{c4d} ష\u{c3e}మ\u{c3e}ల\u{c4d}"), ("th", "มาด\u{e34}เนท แอช ชาร\u{e4c}มาล"), ("tr", "Medinet eş Şemal"), ("uk", "Аш-Шамаль"), ("ur", "بلدیہ شمال"), ("vi", "Madinat ash Shamal"), ("zh", "北部区")]),
                        unofficial_name_list: ["ash-Shamal"].to_vec(),
                    }
                ),
                (
                    "RA",
                    Subdivision{
                        name: "Ar Rayyan",
                        country_alpha2: Alpha2::QA,
                        code: "RA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.2089787), longitude: Some(51.095836), max_latitude: Some(25.8087775), min_latitude: Some(24.471186), max_longitude: Some(51.53700079999999), min_longitude: Some(50.7500553)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية الريان"), ("bg", "Ар Райян"), ("bn", "আল র\u{9be}য\u{9bc}য\u{9bc}\u{9be}ন পৌরসভ\u{9be}"), ("ca", "Ar Rayyan"), ("ccp", "𑄃𑄣\u{11134} 𑄢𑄬𑄠𑄚\u{11134}"), ("ceb", "Baladīyat ar Rayyān"), ("cs", "Al Raján"), ("cy", "Bwrdeistref Al Rayyan"), ("da", "Al-Rayyan"), ("de", "Ar-Rayyan"), ("el", "Αλ Ραγιάν"), ("en", "Al Rayyan"), ("es", "Rayán"), ("fa", "ریان"), ("fi", "Al-Rayyan"), ("fr", "Al Rayyan"), ("ga", "Al Rayyan"), ("gl", "Ar Rayyan"), ("gu", "અલ ર\u{ac7}યાન મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("he", "מחוז א-ריאן"), ("hi", "अल रय\u{94d}यान"), ("id", "Ar-Rayyan"), ("it", "Ar Rayyan"), ("ja", "ライヤーン"), ("kn", "ಅಲ\u{ccd} ರ\u{cc7}ಯಾನ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "알라이얀"), ("lt", "Rajanas"), ("lv", "Rajana"), ("mk", "Рајан"), ("mr", "अल र\u{947}यान म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Ar Rayyan"), ("nb", "Al Rayyan kommune"), ("nl", "Ar Rayyan"), ("no", "Al Rayyan kommune"), ("pl", "Ar-Rajjan"), ("pt", "Al Rayyan"), ("ro", "Al Rayyan"), ("ru", "Ар-Райян"), ("si", "අල\u{dca} රය\u{dca}ය\u{dcf}න\u{dca} ම\u{dd4}න\u{dd2}ස\u{dd2}පල\u{dd2}ට\u{dd2}"), ("sv", "Al Rayyan"), ("ta", "ஆள\u{bcd} ரேய\u{bbe}ன\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "అల\u{c4d} రయ\u{c3e}న\u{c4d} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "อ\u{e31}ล ระห\u{e4c}ย\u{e31}น"), ("tr", "Er Reyyan"), ("uk", "Ар-Раян"), ("ur", "ریان، قطر"), ("vi", "Al Rayyan"), ("zh", "阿爾拉揚體育會")]),
                        unofficial_name_list: ["Rayyan", "ar-Rayyan"].to_vec(),
                    }
                ),
                (
                    "SH",
                    Subdivision{
                        name: "الشحانية",
                        country_alpha2: Alpha2::QA,
                        code: "SH",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الشحانية"), ("de", "Asch-Schahaniyya"), ("en", "Ash Shīḩānīyah"), ("es", "Municipio de Al Shahaniya"), ("fa", "شحانیه"), ("fr", "Al-Shahaniya"), ("gl", "Al Shahaniya"), ("ja", "シャハーニーヤ"), ("pt", "Al-Shahaniya"), ("ro", "Al-Shahaniya"), ("ru", "Эш-Шахания"), ("so", "Al-Shahaniya"), ("tr", "El Şahaniye")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "US",
                    Subdivision{
                        name: "Umm Salal",
                        country_alpha2: Alpha2::QA,
                        code: "US",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.3981114), longitude: Some(51.42508489999999), max_latitude: Some(25.4387563), min_latitude: Some(25.3710324), max_longitude: Some(51.4667945), min_longitude: Some(51.3913962)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية أم صلال"), ("bn", "উম\u{9cd}ম স\u{9be}ল\u{9be}ল পৌরসভ\u{9be}"), ("ccp", "𑄅\u{1112a}𑄟\u{11134} 𑄥𑄣𑄣\u{11134}"), ("cy", "Bwrdeistref Umm Salal"), ("da", "Umm Salal"), ("de", "Umm Salal"), ("el", "Ουμ Σαλάλ"), ("en", "Umm Salal"), ("es", "Umm Salal"), ("fa", "ام صلال"), ("fi", "Umm Salal"), ("fr", "Umm Salal"), ("gu", "ઉમ સલાલ મ\u{acd}ય\u{ac1}નિસિપાલિટી"), ("hi", "उम\u{94d}म सलाल नगरपालिका"), ("id", "Kotamadya Umm Salal"), ("it", "Umm Salal"), ("ja", "ウンム・サラール"), ("kn", "ಉಮ\u{ccd} ಸಲಾಲ\u{ccd} ಪುರಸಭ\u{cc6}"), ("ko", "움살랄"), ("lt", "Um Salalas"), ("lv", "Umm-Salālas pašvaldība"), ("mk", "Ум Салал"), ("mr", "उम\u{94d}म सलाल म\u{94d}य\u{941}न\u{94d}सिपाल\u{94d}टी"), ("ms", "Umm Salal Municipality"), ("nb", "Umm Salal Kommune"), ("nl", "Umm Salal"), ("no", "Umm Salal Kommune"), ("pl", "Umm Salal"), ("pt", "Umm Salal"), ("ro", "Umm Salal"), ("ru", "Умм-Салаль"), ("si", "උම\u{dca} සලල\u{dca} නගර සභ\u{dcf}ව"), ("so", "Umm Salal"), ("sv", "Umm Salal Kommun"), ("ta", "உள\u{bcd}ளம\u{bcd} சல\u{bbe}ல\u{bcd} நகர\u{bbe}ட\u{bcd}சி"), ("te", "ఉమ\u{c4d} సల\u{c3e}య\u{c3f} మున\u{c4d}స\u{c3f}ప\u{c3e}ల\u{c3f}ట\u{c40}"), ("th", "อ\u{e31}ม สาลาล ม\u{e39}น\u{e34}พ\u{e34}ซ\u{e34}พ\u{e31}ลล\u{e34}ต\u{e35}\u{e49}"), ("tr", "Umm Selal"), ("uk", "Умм-Салаль"), ("ur", "ام صلال"), ("vi", "Đô thị tự trị Umm Salal"), ("zh", "烏姆錫拉勒")]),
                        unofficial_name_list: ["Umm Shalal"].to_vec(),
                    }
                ),
                (
                    "WA",
                    Subdivision{
                        name: "Al Wakrah",
                        country_alpha2: Alpha2::QA,
                        code: "WA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(25.1768157), longitude: Some(51.6048431), max_latitude: Some(25.233427), min_latitude: Some(24.473726), max_longitude: Some(51.6284578), min_longitude: Some(51.1108249)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الوكرة"), ("bg", "Ал-Уакра"), ("bn", "আল ওয\u{9bc}\u{9be}কর\u{9be}হ"), ("ca", "Al Wakrah"), ("ccp", "𑄃𑄣\u{11134} 𑄤𑄇\u{11134}𑄢𑄦\u{11134}"), ("ceb", "Al Wakrah"), ("cs", "Al Wakrah"), ("da", "Al Wakrah"), ("de", "Al-Wakra"), ("el", "Αλ Γουάκρα"), ("en", "Al Wakrah"), ("es", "Al Wakrah"), ("fa", "الوکره"), ("fi", "Al-Wakra"), ("fr", "Al Wakrah"), ("ga", "Al Wakrah"), ("gu", "અલ વક\u{acd}ર\u{acd}રાહ"), ("he", "אל-וכרה"), ("hi", "अल वकरा"), ("hu", "Al-Vakra"), ("id", "Al-Wakrah"), ("it", "Al Wakrah"), ("ja", "アル＝ワクラ"), ("kn", "ಅಲ\u{ccd} ವಕ\u{ccd}ರಾ"), ("ko", "알와크라"), ("lt", "Vakra"), ("lv", "Vakra"), ("mk", "Вакра"), ("mr", "अल वक\u{94d}र\u{94d}राह"), ("ms", "Al Wakrah"), ("nb", "Al Wakrah"), ("nl", "Al Wakrah"), ("no", "Al Wakrah"), ("pl", "Al-Wakra"), ("pt", "Al-Wakrah"), ("ru", "Аль-Вакра"), ("si", "අල\u{dca} වක\u{dca}\u{200d}ර\u{dcf}"), ("sv", "Al Wakrah"), ("ta", "அல\u{bcd} வக\u{bcd}ரஹ"), ("te", "అల\u{c4d} వ\u{c3e}క\u{c4d}ర\u{c3e}హ\u{c4d}"), ("th", "อ\u{e31}ล วาครา"), ("tr", "El Vakra"), ("uk", "Аль-Вакра"), ("ur", "الوکرہ"), ("vi", "Al Wakrah"), ("zh", "沃克拉")]),
                        unofficial_name_list: ["Wakra", "Wakrah"].to_vec(),
                    }
                ),
                (
                    "ZA",
                    Subdivision{
                        name: "بلدية الضعاين",
                        country_alpha2: Alpha2::QA,
                        code: "ZA",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Municipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بلدية الضعاين"), ("bn", "আল দ\u{9be}য\u{9bc}েন"), ("ccp", "𑄃𑄣\u{11134} 𑄘𑄬𑄠𑄚\u{11134}"), ("da", "Al Daayen"), ("el", "Αλ Ντααγιέν"), ("en", "Al Daayen"), ("es", "Al Daayen"), ("fa", "بلدیة الضعاین"), ("fi", "Al-Dayyan"), ("fr", "Al Daayen"), ("gu", "અલ દાય\u{ac7}ન"), ("he", "א-דעאין"), ("hi", "अल डाय\u{947}न"), ("id", "Al Daayen"), ("it", "Al Daayen"), ("ja", "アッ＝ザアーイン"), ("kn", "ಅಲ\u{ccd} ಡಾಯ\u{cc6}ನ\u{ccd}"), ("ko", "알다옌"), ("lt", "Al Daajenas"), ("lv", "Zaājīna"), ("mk", "Дајен"), ("mr", "अल दाय\u{947}न"), ("ms", "Al Daayen"), ("nb", "Al Daayen"), ("nl", "Al Daayen"), ("no", "Al Daayen"), ("pl", "Ad-Da’ajin"), ("pt", "Al Daayen"), ("ro", "Al Daayen"), ("ru", "Аль-Дайиан"), ("si", "අල\u{dca} ඩ\u{dcf}යෙන\u{dca}"), ("sv", "Al Dayeen"), ("ta", "அல\u{bcd} ட\u{bbe}யென\u{bcd}"), ("te", "అల\u{c4d} డ\u{c3e}య\u{c46}న\u{c4d}"), ("th", "ออล\u{e4c}ไดอเยน"), ("tr", "Ed Daayin"), ("uk", "Аль-Даїян"), ("ur", "بلدیہ الضعاین"), ("vi", "Al Daayen"), ("zh", "戴揚")]),
                        unofficial_name_list: [].to_vec(),
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
#[cfg(feature = "qa")]
/// [`Country`](crate::Country) struct for this country.
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::QA,
        alpha3: Alpha3::QAT,
        address_format: Some("{{recipient}}\n{{street}}\n{{postalcode}} {{city}}\n{{country}}"),
        continent: Continent::Asia,
        country_code: 974,
        currency_code: CurrencyCode::QAR,
        maybe_gec: Some(GEC::QA),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        maybe_ioc: Some(IOC::QAT),
        iso_long_name: "The State of Qatar",
        iso_short_name: "Qatar",
        official_language_list: ["ar"].to_vec(),
        spoken_language_list: ["ar"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [7].to_vec(),
        national_prefix: "None",
        maybe_nationality: Some("Qatari"),
        maybe_vehicle_registration_code: Some("Q"),
        number: "634",
        postal_code: false,
        postal_code_format: None,
        maybe_region: Some(Region::Asia),
        start_of_week: WeekDay::Sunday,
        maybe_subregion: Some(SubRegion::WesternAsia),
        un_locode: "QA",
        unofficial_name_list: ["Qatar", "قطر", "Katar", "カタール"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "emojis")]
        emoji: "🇶🇦",
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Qatar"),
            ("af", "Katar"),
            ("ak", "Qatar"),
            ("am", "ቃጣር"),
            ("an", "Qatar"),
            ("ar", "قطر"),
            ("as", "ক\u{9be}ট\u{9be}ৰ"),
            ("ay", "Qatar"),
            ("az", "Katar"),
            ("ba", "Qatar"),
            ("be", "Катар"),
            ("bg", "Катар"),
            ("bi", "Qatar"),
            ("bn", "ক\u{9be}ত\u{9be}র"),
            ("bn_IN", "ক\u{9be}ত\u{9be}র"),
            ("br", "Katar"),
            ("bs", "Katar"),
            ("ca", "Qatar"),
            ("ce", "КъатӀар"),
            ("ch", "Qatar"),
            ("cs", "Katar"),
            ("cv", "КъатӀар"),
            ("cy", "Qatar"),
            ("da", "Qatar"),
            ("de", "Katar"),
            ("dv", "ޤ\u{7a6}ޠ\u{7a6}ރ\u{7aa}"),
            ("dz", "ཀ་ཊར།"),
            ("ee", "Qatar"),
            ("el", "Κατάρ"),
            ("en", "Qatar"),
            ("eo", "Kataro"),
            ("es", "Catar"),
            ("et", "Katar"),
            ("eu", "Qatar"),
            ("fa", "قطر"),
            ("ff", "Qatar"),
            ("fi", "Qatar"),
            ("fo", "Katar"),
            ("fr", "Qatar"),
            ("fy", "Katar"),
            ("ga", "Catar"),
            ("gl", "Qatar"),
            ("gn", "Qatar"),
            ("gu", "કતાર"),
            ("gv", "Yn Chatar"),
            ("ha", "Qatar"),
            ("he", "קטר"),
            ("hi", "क\u{93c}तर"),
            ("hr", "Katar"),
            ("ht", "Katar"),
            ("hu", "Katar"),
            ("hy", "Կատար"),
            ("ia", "Qatar"),
            ("id", "Qatar"),
            ("io", "Katar"),
            ("is", "Katar"),
            ("it", "Qatar"),
            ("iu", "Qatar"),
            ("ja", "カタール"),
            ("ka", "კატარი"),
            ("ki", "Qatar"),
            ("kk", "Катар"),
            ("kl", "Qatar"),
            ("km", "កាតារ"),
            ("kn", "ಕತಾರ\u{ccd}"),
            ("ko", "카타르"),
            ("ku", "Qatar"),
            ("kv", "Катар"),
            ("kw", "Katar"),
            ("ky", "Катар"),
            ("lo", "ປະເທດກາຕາ"),
            ("lt", "Kataras"),
            ("lv", "Katara"),
            ("mi", "Qatar"),
            ("mk", "Катар"),
            ("ml", "ഖത\u{d4d}തര\u{d4d}\u{200d}"),
            ("mn", "Катар"),
            ("mr", "कतार"),
            ("ms", "Qatar"),
            ("mt", "Qatar"),
            ("my", "ကာတာန\u{102d}\u{102f}င\u{103a}င\u{1036}"),
            ("na", "Qatar"),
            ("nb", "Qatar"),
            ("ne", "कतार"),
            ("nl", "Qatar"),
            ("nn", "Qatar"),
            ("nv", "Qatar"),
            ("oc", "Qatar"),
            ("or", "କତ\u{b3e}ର"),
            ("pa", "ਕਤਰ"),
            ("pi", "कतार"),
            ("pl", "Katar"),
            ("ps", "قطر"),
            ("pt", "Catar"),
            ("pt_BR", "Catar"),
            ("ro", "Qatar"),
            ("ru", "Катар"),
            ("rw", "Katari"),
            ("sc", "Catar"),
            ("sd", "Qatar"),
            ("si", "කට\u{dcf}ර\u{dca}"),
            ("sk", "Katar"),
            ("sl", "Katar"),
            ("so", "Qadar"),
            ("sq", "Katar"),
            ("sr", "Катар"),
            ("sv", "Qatar"),
            ("sw", "Qatar"),
            ("ta", "கத\u{bbe}ர\u{bcd}"),
            ("te", "కత\u{c3e}ర\u{c4d}"),
            ("tg", "Қатар"),
            ("th", "กาตาร\u{e4c}"),
            ("ti", "Qatar"),
            ("tk", "Katar"),
            ("tl", "Qatar"),
            ("tr", "Katar"),
            ("tt", "Катар"),
            ("ug", "قاتار"),
            ("uk", "Катар"),
            ("ur", "قطر"),
            ("uz", "Qatar"),
            ("ve", "Qatar"),
            ("vi", "Ca-tă"),
            ("wa", "Katar"),
            ("wo", "Xatar"),
            ("xh", "Qatar"),
            ("yo", "Katar"),
            ("zh_CN", "卡塔尔"),
            ("zh_HK", "卡塔爾"),
            ("zh_TW", "卡達"),
            ("zu", "Qatar"),
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
        maybe_population: Some(2695122),
    }
}
