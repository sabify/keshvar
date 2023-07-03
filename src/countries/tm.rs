// DO NOT TOUCH THIS FILE. (Auto-generated by `keshvar-code-generator/src/countries.rs`)

//! A module for country `Turkmenistan`

#[cfg(all(feature = "tm", feature = "constants"))]
/// A module to access all constant country data for `Turkmenistan`.
///
/// Note that to use this module, `constant` feature should be enabled.
pub mod consts {
    #[allow(unused_imports)]
    use crate::{
        Alpha2, Alpha3, Continent, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC, IOC,
    };

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::TM;
    pub const ALPHA3: Alpha3 = Alpha3::TKM;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 993;
    pub const CURRENCY_CODE: CurrencyCode = CurrencyCode::TMT;
    pub const GEC: Option<GEC> = Some(GEC::TX);
    pub const INTERNATIONAL_PREFIX: &str = "810";
    pub const IOC: Option<IOC> = Some(IOC::TKM);
    pub const ISO_SHORT_NAME: &str = "Turkmenistan";
    pub const ISO_LONG_NAME: &str = "Turkmenistan";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["tk"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["ru", "tk"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "8";
    pub const NATIONALITY: Option<&str> = Some("Turkmen");
    pub const NUMBER: &str = "795";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{6}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::CentralAsia);
    pub const UN_LOCODE: &str = "TM";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Turkmenistan",
        "Turkménistan",
        "Turkmenistán",
        "トルクメニスタン",
        "Turkmenia",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    pub const G7_MEMBER: bool = false;
    pub const G20_MEMBER: bool = false;
    #[cfg(feature = "emojis")]
    pub const EMOJI: &str = "🇹🇲";
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Turkmenistan"),
        ("af", "Toerkmenië"),
        ("ak", "Turkmenistan"),
        ("am", "ቱርክመኒስታን"),
        ("an", "Turkmenistan"),
        ("ar", "تركمانستان"),
        ("as", "ত\u{9c1}ৰ\u{9cd}কমেনিস\u{9cd}ত\u{9be}ন"),
        ("ay", "Turkmenistan"),
        ("az", "Türkmənistan"),
        ("ba", "Turkmenistan"),
        ("be", "Туркменістан"),
        ("bg", "Туркменистан"),
        ("bi", "Turkmenistan"),
        ("bn", "ত\u{9c1}র\u{9cd}কমেনিস\u{9cd}ত\u{9be}ন"),
        ("bn_IN", "ত\u{9c1}র\u{9cd}কমেনিস\u{9cd}ত\u{9be}ন"),
        ("br", "Turkmenistan"),
        ("bs", "Turkmenistan"),
        ("ca", "Turkmenistan"),
        ("ce", "Туркмени"),
        ("ch", "Turkmenistan"),
        ("cs", "Turkmenistán"),
        ("cv", "Туркмени"),
        ("cy", "Turkmenistan"),
        ("da", "Turkmenistan"),
        ("de", "Turkmenistan"),
        (
            "dv",
            "ތ\u{7aa}ރ\u{7aa}ކ\u{7aa}މ\u{7ac}ނ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}",
        ),
        ("dz", "ཊརཀ་མ\u{f72}་ན\u{f72}ས\u{f72}་ཏ\u{f71}ན།"),
        ("ee", "Turkmenistan"),
        ("el", "Τουρκμενιστάν"),
        ("en", "Turkmenistan"),
        ("eo", "Turkmenio"),
        ("es", "Turkmenistán"),
        ("et", "Türkmenistan"),
        ("eu", "Turkmenistan"),
        ("fa", "ترکمنستان"),
        ("ff", "Turkmenistan"),
        ("fi", "Turkmenistan"),
        ("fo", "Turkmenistan"),
        ("fr", "Turkménistan"),
        ("fy", "Turkmenistan"),
        ("ga", "An Tuircméanastáin"),
        ("gl", "Turkmenistán"),
        ("gn", "Turkmenistan"),
        ("gu", "ત\u{ac1}ર\u{acd}કમ\u{ac7}નિસ\u{acd}તાન"),
        ("gv", "Yn Turkmenistaan"),
        ("ha", "Turkmenistan"),
        ("he", "טורקמניסטן"),
        ("hi", "त\u{941}र\u{94d}कम\u{947}निस\u{94d}तान"),
        ("hr", "Turkmenistan"),
        ("ht", "Tirkmenistan"),
        ("hu", "Türkmenisztán"),
        ("hy", "Թուրքմենստան"),
        ("ia", "Turkmenistan"),
        ("id", "Turkmenistan"),
        ("io", "Turkmenistan"),
        ("is", "Túrkmenistan"),
        ("it", "Turkmenistan"),
        ("iu", "Turkmenistan"),
        ("ja", "トルクメニスタン"),
        ("ka", "თურქმენეთი"),
        ("ki", "Turkmenistan"),
        ("kk", "Түрікменстан"),
        ("kl", "Turkmenistan"),
        ("km", "ទ\u{17bd}គមេន\u{17b8}ស\u{17d2}តង\u{17cb}"),
        (
            "kn",
            "ತುರ\u{ccd}ಕ\u{ccd}ಮ\u{cc6}ನ\u{ccd}\u{200d}ಸ\u{ccd}ತಾನ\u{ccd}",
        ),
        ("ko", "투르크메니스탄"),
        ("ku", "Tirkmenîstan"),
        ("kv", "Туркменистан"),
        ("kw", "Pow Turkmen"),
        ("ky", "Түркмөнстан"),
        ("lo", "Turkmenistan"),
        ("lt", "Turkmėnistanas"),
        ("lv", "Turkmenistāna"),
        ("mi", "Turkmenistan"),
        ("mk", "Туркменистан"),
        (
            "ml",
            "ത\u{d41}ര\u{d4d}\u{200d}ക\u{d4d}\u{200c}മെനിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}",
        ),
        ("mn", "Туркменстан"),
        ("mr", "त\u{941}र\u{94d}कम\u{947}निस\u{94d}तान"),
        ("ms", "Turkmenistan"),
        ("mt", "Turkmenistan"),
        (
            "my",
            "တာ\u{1037}ခ\u{103a}မင\u{103a}နစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Turkmenistan"),
        ("nb", "Turkmenistan"),
        ("ne", "ट\u{941}र\u{94d}कम\u{947}निस\u{94d}तान"),
        ("nl", "Turkmenistan"),
        ("nn", "Turkmenistan"),
        ("nv", "Tʼóokmen Bikéyah"),
        ("oc", "Turcmenistan"),
        ("or", "ତ\u{b41}ର\u{b4d}କମେନ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"),
        ("pa", "ਤ\u{a41}ਰਕਮਸਤਾਨ"),
        ("pi", "त\u{941}र\u{94d}कमिनिस\u{94d}थान"),
        ("pl", "Turkmenistan"),
        ("ps", "ترکمنستان"),
        ("pt", "Turquemenistão"),
        ("pt_BR", "Turcomenistão"),
        ("ro", "Turkmenistan"),
        ("ru", "Туркменистан"),
        ("rw", "Turikimenisitani"),
        ("sc", "Turkmènistan"),
        ("sd", "ترڪمانستان"),
        ("si", "ත\u{dd4}ර\u{dca}ක\u{dd2}මෙන\u{dd2}ස\u{dca}ත\u{dcf}නය"),
        ("sk", "Turkménsko"),
        ("sl", "Turkmenistan"),
        ("so", "Turkmenistan"),
        ("sq", "Turkmenistan"),
        ("sr", "Туркменистан"),
        ("sv", "Turkmenistan"),
        ("sw", "Turkmenistan"),
        ("ta", "துர\u{bcd}க\u{bcd}மெனிஸ\u{bcd}த\u{bbe}ன\u{bcd}"),
        ("te", "టర\u{c4d}కమ\u{c46}న\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"),
        ("tg", "Туркманистон"),
        ("th", "เต\u{e34}ร\u{e4c}กเมน\u{e34}สถาน"),
        ("ti", "Turkmenistan"),
        ("tk", "Türkmenistan"),
        ("tl", "Turkmenistan"),
        ("tr", "Türkmenistan"),
        ("tt", "Төркмәнстан"),
        ("ug", "تۈركمەنىستان"),
        ("uk", "Туркменістан"),
        ("ur", "ترکمانستان"),
        ("uz", "Turkmaniston"),
        ("ve", "Turkmenistan"),
        ("vi", "Tuốc-mê-ni-xtanh"),
        ("wa", "Turcmenistan"),
        ("wo", "Turkumenistaan"),
        ("xh", "Turkmenistan"),
        ("yo", "Turkmẹ\u{301}nìstán"),
        ("zh_CN", "土库曼斯坦"),
        ("zh_HK", "土庫曼"),
        ("zh_TW", "土庫曼"),
        ("zu", "Turkmenistan"),
    ];
    #[cfg(all(feature = "tm", feature = "geo", feature = "constants"))]
    /// GEO data as constants
    pub mod geo {
        pub const LATITUDE: f64 = 38.969719;
        pub const LONGITUDE: f64 = 59.556278;
        pub const MAX_LATITUDE: f64 = 42.798844;
        pub const MAX_LONGITUDE: f64 = 66.70735309999999;
        pub const MIN_LATITUDE: f64 = 35.12876;
        pub const MIN_LONGITUDE: f64 = 52.3169;
        pub const NORTHEAST_LATITUDE: f64 = 42.798844;
        pub const NORTHEAST_LONGITUDE: f64 = 66.70735309999999;
        pub const SOUTHWEST_LATITUDE: f64 = 35.12876;
        pub const SOUTHWEST_LONGITUDE: f64 = 52.3169;
    }
}
#[cfg(all(feature = "tm", feature = "geo"))]
/// GEO module for this country.
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    /// GEO information for this country.
    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 38.969719,
            longitude: 59.556278,
            max_latitude: 42.798844,
            max_longitude: 66.70735309999999,
            min_latitude: 35.12876,
            min_longitude: 52.3169,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 42.798844,
                    longitude: 66.70735309999999,
                },
                southwest: CountryGeoBound {
                    latitude: 35.12876,
                    longitude: 52.3169,
                },
            },
        }
    }
}

#[cfg(all(feature = "tm", feature = "subdivisions"))]
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
        HashMap::from([
            (
                "A",
                Subdivision {
                    name: "Ahal",
                    country_alpha2: Alpha2::TM,
                    code: "A",
                    #[cfg(feature = "geo")]
                    geo: Some(SubdivisionGeo {
                        latitude: Some(38.6399398),
                        longitude: Some(59.4720904),
                        max_latitude: Some(40.197323),
                        min_latitude: Some(35.58967),
                        max_longitude: Some(61.51145510000001),
                        min_longitude: Some(56.723392),
                    }),
                    comments: None,
                    subdivision_type: SubdivisionType::Region,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("ar", "اقليم أهال"),
                        ("az", "Ahal vilayəti"),
                        ("be", "Ахалскі велаят"),
                        ("bg", "Ахал (вилает)"),
                        ("bn", "আহ\u{9be}ল প\u{9cd}রদেশ"),
                        ("ca", "Província d’Ahal"),
                        ("ccp", "𑄃𑄦𑄣\u{11134}"),
                        ("ceb", "Ahal"),
                        ("da", "Ahal Province"),
                        ("de", "Ahal welaýaty"),
                        ("el", "Αχάλ"),
                        ("en", "Ahal"),
                        ("es", "Provincia de Ahal"),
                        ("et", "Ahali vilajett"),
                        ("fa", "استان آخال"),
                        ("fi", "Ahalin maakunta"),
                        ("fr", "Ahal"),
                        ("gu", "અહાલ પ\u{acd}રા\u{a82}ત"),
                        ("he", "מחוז אהאל"),
                        ("hi", "आख\u{93c}ाल प\u{94d}रान\u{94d}त"),
                        ("hy", "Ահալի վելայաթ"),
                        ("id", "Provinsi Ahal"),
                        ("it", "provincia di Ahal"),
                        ("ja", "アハル州"),
                        ("ka", "ახალის ველაიათი"),
                        ("kn", "ಅಹಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"),
                        ("ko", "아할 주"),
                        ("ky", "Ашгабат облусу"),
                        ("lt", "Ahalo velajatas"),
                        ("lv", "Ahala vilajets"),
                        ("mn", "Ахал муж"),
                        ("mr", "अहाल प\u{94d}रा\u{902}त"),
                        ("ms", "Provinsi Ahal"),
                        ("nb", "Ahal provins"),
                        ("nl", "Ahal"),
                        ("no", "Ahal provins"),
                        ("pl", "Wilajet achalski"),
                        ("ps", "آخال ولايت"),
                        ("pt", "Ahal"),
                        ("ro", "Provincia Ahal"),
                        ("ru", "Ахалский велаят"),
                        ("si", "අහ\u{dcf}ල\u{dca} පළ\u{dcf}ත"),
                        ("sr", "Покрајина Ахал"),
                        ("sr_Latn", "Pokrajina Ahal"),
                        ("sv", "Achal"),
                        ("ta", "ஆஹ\u{bcd}அல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"),
                        ("te", "అహల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"),
                        ("th", "เขตอะฮอล"),
                        ("tk", "Ahal"),
                        ("tr", "Ahal vilayeti"),
                        ("uk", "Ахалський велаят"),
                        ("ur", "صوبہ آخال"),
                        ("uz", "Ahal viloyati"),
                        ("vi", "Ahal (tỉnh)"),
                        ("zh", "阿哈爾州"),
                    ]),
                    unofficial_name_list: ["Akhal"].to_vec(),
                },
            ),
            (
                "B",
                Subdivision {
                    name: "Balkan",
                    country_alpha2: Alpha2::TM,
                    code: "B",
                    #[cfg(feature = "geo")]
                    geo: Some(SubdivisionGeo {
                        latitude: Some(40.9468884),
                        longitude: Some(54.4952432),
                        max_latitude: Some(42.3434979),
                        min_latitude: Some(37.309181),
                        max_longitude: Some(57.19972600000001),
                        min_longitude: Some(52.4446017),
                    }),
                    comments: None,
                    subdivision_type: SubdivisionType::Region,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("ar", "ولاية بلقان"),
                        ("az", "Balkan vilayəti"),
                        ("be", "Балканскі велаят"),
                        ("bg", "Балкан"),
                        ("bn", "বলক\u{9be}ন প\u{9cd}রদেশ"),
                        ("ca", "Província de Balkan"),
                        ("ccp", "𑄝\u{11127}𑄣\u{11134}𑄇𑄚\u{11134}"),
                        ("ceb", "Balkan"),
                        ("da", "Balkan Province"),
                        ("de", "Balkan welaýaty"),
                        ("el", "Μπαλκάν"),
                        ("en", "Balkan"),
                        ("es", "Provincia de Balkan"),
                        ("et", "Balkani vilajett"),
                        ("eu", "Balkan probintzia"),
                        ("fa", "استان بلخان"),
                        ("fi", "Balkanin maakunta"),
                        ("fr", "Balkan"),
                        ("gl", "Provincia de Balkan"),
                        ("gu", "બાલ\u{acd}કન પ\u{acd}રા\u{a82}ત"),
                        ("he", "מחוז בלקן"),
                        ("hi", "बलक\u{93c}ान प\u{94d}रान\u{94d}त"),
                        ("hr", "Balkan welaýaty"),
                        ("hu", "Balkan"),
                        ("hy", "Բալկանի վելայաթ"),
                        ("id", "Provinsi Balkan"),
                        ("it", "provincia di Balkan"),
                        ("ja", "バルカン州"),
                        ("ka", "ბალკანის ველაიათი"),
                        ("kn", "ಬಾಲ\u{ccd}ಕನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"),
                        ("ko", "발칸 주"),
                        ("lt", "Balkano velajatas"),
                        ("lv", "Balkanas vilajets"),
                        ("mn", "Балкан муж"),
                        ("mr", "बाल\u{94d}कन प\u{94d}रा\u{902}त"),
                        ("ms", "Provinsi Balkan"),
                        ("nb", "Belkan provins"),
                        ("nl", "Balkan"),
                        ("no", "Belkan provins"),
                        ("pl", "Wilajet balkański"),
                        ("ps", "بلخان ولايت"),
                        ("pt", "Balkan"),
                        ("ro", "Provincia Balkan"),
                        ("ru", "Балканский велаят"),
                        ("si", "බල\u{dca}කන\u{dca} පළ\u{dcf}ත"),
                        ("sr", "Покрајина Балкан"),
                        ("sr_Latn", "Pokrajina Balkan"),
                        ("sv", "Balkan"),
                        ("ta", "ப\u{bbe}ல\u{bcd}கன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"),
                        (
                            "te",
                            "బ\u{c3e}ల\u{c4d}కన\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}",
                        ),
                        ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}ลค\u{e31}น"),
                        ("tk", "Balkan welaýaty"),
                        ("tr", "Balkan vilayeti"),
                        ("uk", "Балканський велаят"),
                        ("ur", "صوبہ بلخان"),
                        ("uz", "Bolqon viloyati"),
                        ("vi", "Balkan"),
                        ("zh", "巴爾坎州"),
                    ]),
                    unofficial_name_list: ["Balkan"].to_vec(),
                },
            ),
            (
                "D",
                Subdivision {
                    name: "Dasoguz",
                    country_alpha2: Alpha2::TM,
                    code: "D",
                    #[cfg(feature = "geo")]
                    geo: Some(SubdivisionGeo {
                        latitude: Some(41.833333),
                        longitude: Some(59.966667),
                        max_latitude: Some(41.880745),
                        min_latitude: Some(41.788273),
                        max_longitude: Some(60.038309),
                        min_longitude: Some(59.8845004),
                    }),
                    comments: None,
                    subdivision_type: SubdivisionType::Region,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("ar", "ولاية داشوغوز"),
                        ("az", "Daşoğuz vilayəti"),
                        ("be", "Дашагузскі велаят"),
                        ("bg", "Дашховуз"),
                        ("bn", "দ\u{9be}সোগ\u{9c1}জ প\u{9cd}রদেশ"),
                        ("ca", "Província de Daşoguz"),
                        ("ccp", "𑄓𑄥\u{1112e}𑄉\u{1112a}𑄌\u{11134}"),
                        ("ceb", "Daşoguz Welaýaty"),
                        ("de", "Daşoguz welaýaty"),
                        ("en", "Daşoguz"),
                        ("es", "Provincia de Daşoguz"),
                        ("et", "Daşoguzi vilajett"),
                        ("fa", "استان داش\u{200c}اغوز"),
                        ("fi", "Daşoguzin maakunta"),
                        ("fr", "Province de Daşoguz"),
                        ("he", "מחוז דשחובוז"),
                        ("hi", "दाशोग\u{93c}\u{941}ज\u{93c} प\u{94d}रान\u{94d}त"),
                        ("hy", "Դաշողուզի վելայաթ"),
                        ("id", "Provinsi Dashhowuz"),
                        ("it", "provincia di Daşoguz"),
                        ("ja", "ダショグズ州"),
                        ("ka", "დაშოგუზის ველაიათი"),
                        ("ko", "다쇼구즈 주"),
                        ("ky", "Ташауз областы"),
                        ("lt", "Dašoguzo velajatas"),
                        ("mn", "Дашогуз муж"),
                        ("ms", "Provinsi Dashhowuz"),
                        ("nl", "Daşoguz"),
                        ("pl", "Wilajet daszoguski"),
                        ("ps", "داشوغوز ولايت"),
                        ("pt", "Daşoguz"),
                        ("ro", "Provincia Dașoguz"),
                        ("ru", "Дашогузский велаят"),
                        ("sr", "Покрајина Дашогуз"),
                        ("sr_Latn", "Pokrajina Dašoguz"),
                        ("sv", "Daşoguz"),
                        ("ta", "தகோகுஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"),
                        ("th", "จ\u{e31}งหว\u{e31}ดดาโชก\u{e38}ซ"),
                        ("tk", "Daşoguz welaýaty"),
                        ("tr", "Daşoğuz vilayeti"),
                        ("uk", "Дашогузький велаят"),
                        ("ur", "صوبہ داشوغوز"),
                        ("uz", "Dashoguz viloyati"),
                        ("vi", "Daşoguz"),
                        ("zh", "達沙古茲州"),
                    ]),
                    unofficial_name_list: [
                        "Dashhovuz",
                        "Dashhowuz",
                        "Dashkhovuz",
                        "Dashogus",
                        "Dashoguz",
                        "Daşhovuz",
                        "Dašhovuz",
                        "Tashauz",
                        "Tašauz",
                    ]
                    .to_vec(),
                },
            ),
            (
                "L",
                Subdivision {
                    name: "Lebap",
                    country_alpha2: Alpha2::TM,
                    code: "L",
                    #[cfg(feature = "geo")]
                    geo: Some(SubdivisionGeo {
                        latitude: Some(38.12724619999999),
                        longitude: Some(64.7162415),
                        max_latitude: Some(41.2915379),
                        min_latitude: Some(36.850881),
                        max_longitude: Some(66.6843031),
                        min_longitude: Some(60.18987389999999),
                    }),
                    comments: None,
                    subdivision_type: SubdivisionType::Region,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("ar", "ولاية لباب"),
                        ("az", "Lebap vilayəti"),
                        ("be", "Лябапскі велаят"),
                        ("bg", "Лебап"),
                        ("bn", "লেব\u{9be}প প\u{9cd}রদেশ"),
                        ("ca", "Província de Lebap"),
                        ("ccp", "𑄣𑄬𑄝𑄛\u{11134}"),
                        ("ceb", "Lebap"),
                        ("de", "Lebap welaýaty"),
                        ("en", "Lebap"),
                        ("es", "Provincia de Lebap"),
                        ("et", "Lebapi vilajett"),
                        ("fa", "استان لباب"),
                        ("fi", "Lebapin maakunta"),
                        ("fr", "Lebap"),
                        ("he", "מחוז לבפ"),
                        ("hi", "ल\u{947}बाप प\u{94d}रान\u{94d}त"),
                        ("hy", "Լեբապի վելայաթ"),
                        ("id", "Provinsi Lebap"),
                        ("it", "provincia di Lebap"),
                        ("ja", "レバプ州"),
                        ("ka", "ლებაპის ველაიათი"),
                        ("ko", "레바프 주"),
                        ("lt", "Lebapo velajatas"),
                        ("mn", "Лебап муж"),
                        ("ms", "Provinsi Lebap"),
                        ("nl", "Lebap"),
                        ("pl", "Wilajet lebapski"),
                        ("ps", "لب آب ولايت"),
                        ("pt", "Lebap"),
                        ("ro", "Provincia Lebap"),
                        ("ru", "Лебапский велаят"),
                        ("sr", "Покрајина Лебап"),
                        ("sr_Latn", "Pokrajina Lebap"),
                        ("sv", "Lebap"),
                        ("ta", "லெப\u{bbe}ப\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"),
                        ("th", "จ\u{e31}งหว\u{e31}ดเลบ\u{e31}ป"),
                        ("tk", "Lebap welaýaty"),
                        ("tr", "Lebap vilayeti"),
                        ("uk", "Лебапський велаят"),
                        ("ur", "صوبہ لب آب"),
                        ("uz", "Lebap viloyati"),
                        ("vi", "Lebap"),
                        ("zh", "列巴普州"),
                    ]),
                    unofficial_name_list: ["Lebap"].to_vec(),
                },
            ),
            (
                "M",
                Subdivision {
                    name: "Mary",
                    country_alpha2: Alpha2::TM,
                    code: "M",
                    #[cfg(feature = "geo")]
                    geo: Some(SubdivisionGeo {
                        latitude: Some(36.9481623),
                        longitude: Some(62.4504154),
                        max_latitude: Some(39.5136029),
                        min_latitude: Some(35.141151),
                        max_longitude: Some(64.737839),
                        min_longitude: Some(60.39503499999999),
                    }),
                    comments: None,
                    subdivision_type: SubdivisionType::Region,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("ar", "محافظة ماري"),
                        ("az", "Marı vilayəti"),
                        ("be", "Марыйскі велаят"),
                        ("bg", "Мари"),
                        ("bn", "মেরি প\u{9cd}রদেশ"),
                        ("ca", "Província de Mary"),
                        ("ccp", "𑄟𑄬𑄢\u{11128}"),
                        ("ceb", "Mary"),
                        ("da", "Mary Province"),
                        ("de", "Mary welaýaty"),
                        ("el", "Μέρι"),
                        ("en", "Mary"),
                        ("es", "Provincia de Mary"),
                        ("et", "Mary vilajett"),
                        ("fa", "استان مرو"),
                        ("fi", "Maryn maakunta"),
                        ("fr", "Mary"),
                        ("gu", "મ\u{ac7}રી પ\u{acd}રા\u{a82}ત"),
                        ("he", "מחוז מרי"),
                        ("hi", "मरी प\u{94d}रान\u{94d}त"),
                        ("hy", "Մարիի վելայաթ"),
                        ("id", "Provinsi Mary"),
                        ("it", "provincia di Mary"),
                        ("ja", "マル州"),
                        ("ka", "მარის ველაიათი"),
                        ("kk", "Мары уәлаяты"),
                        ("kn", "ಮೇರ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"),
                        ("ko", "마리 주"),
                        ("lt", "Maro velajatas"),
                        ("lv", "Mari vilajets"),
                        ("mn", "Мары муж"),
                        ("mr", "म\u{947}री प\u{94d}रा\u{902}त"),
                        ("ms", "Provinsi Mary"),
                        ("nb", "Mary provins"),
                        ("nl", "Mary"),
                        ("no", "Mary provins"),
                        ("pl", "Wilajet maryjski"),
                        ("ps", "ماري ولايت"),
                        ("pt", "Mary"),
                        ("ro", "Provincia Mary"),
                        ("ru", "Марыйский велаят"),
                        ("si", "මේර\u{dd2} පළ\u{dcf}ත"),
                        ("sr", "Покрајина Мари"),
                        ("sr_Latn", "Pokrajina Mari"),
                        ("sv", "Mary"),
                        ("ta", "மேரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"),
                        (
                            "te",
                            "మ\u{c47}ర\u{c3e} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}",
                        ),
                        ("th", "เม\u{e37}องมาตาก\u{e31}ลปา"),
                        ("tk", "Mary welaýaty"),
                        ("tr", "Mary vilayeti"),
                        ("uk", "Марийський велаят"),
                        ("ur", "صوبہ ماری"),
                        ("uz", "Mari viloyati"),
                        ("vi", "Mary"),
                        ("zh", "馬雷州"),
                    ]),
                    unofficial_name_list: ["Mary", "Merv"].to_vec(),
                },
            ),
            (
                "S",
                Subdivision {
                    name: "Aşgabat",
                    country_alpha2: Alpha2::TM,
                    code: "S",
                    #[cfg(feature = "geo")]
                    geo: None,
                    comments: None,
                    subdivision_type: SubdivisionType::CapitalCity,
                    #[cfg(feature = "translations")]
                    translations: HashMap::from([
                        ("af", "Asjchabat"),
                        ("am", "አሽጋባት"),
                        ("ar", "عشق آباد"),
                        ("az", "Aşqabad"),
                        ("be", "Ашхабад"),
                        ("bg", "Ашхабад"),
                        ("bn", "আশখ\u{9be}ব\u{9be}দ"),
                        ("bs", "Ashgabat"),
                        ("ca", "Aixkhabad"),
                        ("ccp", "𑄃𑄌\u{11134}𑄉𑄝𑄖\u{11134}"),
                        ("ceb", "Aşgabat Şäheri"),
                        ("cs", "Ašchabad"),
                        ("cy", "Ashgabat"),
                        ("da", "Asjkhabad"),
                        ("de", "Aşgabat"),
                        ("el", "Ασγκαμπάτ"),
                        ("en", "Aşgabat"),
                        ("es", "Asjabad"),
                        ("et", "Aşgabat"),
                        ("eu", "Ashgabat"),
                        ("fa", "عشق\u{200c}آباد"),
                        ("fi", "Ašgabat"),
                        ("fr", "Achgabat"),
                        ("ga", "Ashgabat"),
                        ("gl", "Achkhabad"),
                        ("gu", "અશગબાત"),
                        ("he", "אשגבאט"),
                        ("hi", "अश\u{94d}क\u{93c}ाबाद"),
                        ("hr", "Ašgabat"),
                        ("hu", "Aşgabat"),
                        ("hy", "Աշխաբադ"),
                        ("id", "Ashgabat"),
                        ("is", "Asgabat"),
                        ("it", "Aşgabat"),
                        ("ja", "アシガバート"),
                        ("ka", "აშხაბადი"),
                        ("kk", "Ашғабад"),
                        ("kn", "ಅಶ\u{ccd}ಗಾಬಾತ\u{ccd}"),
                        ("ko", "아시가바트"),
                        ("ky", "Ашхабад"),
                        ("lt", "Ašchabadas"),
                        ("lv", "Ašgabata"),
                        ("mk", "Ашхабад"),
                        ("ml", "അഷ\u{d4d}ഗ\u{d3e}ബ\u{d3e}ദ\u{d4d}"),
                        ("mn", "Ашгабат"),
                        ("mr", "अश\u{94d}गाबाद"),
                        ("ms", "Ashgabat"),
                        (
                            "my",
                            "အက\u{103a}ရ\u{103e}\u{103a}ဂါဘတ\u{103a}မြ\u{102d}\u{102f}\u{1037}",
                        ),
                        ("nb", "Asjkhabad"),
                        ("ne", "अस\u{94d}गाबत"),
                        ("nl", "Asjchabad"),
                        ("no", "Asjkhabad"),
                        ("pa", "ਅਸ\u{a3c}ਗਾਬਾਦ"),
                        ("pl", "Aszchabad"),
                        ("ps", "عشق آباد"),
                        ("pt", "Asgabate"),
                        ("ro", "Așgabat"),
                        ("ru", "Ашхабад"),
                        ("si", "ආශ\u{dca}ග\u{dcf}බ\u{dcf}ත\u{dca}"),
                        ("sk", "Ašchabad"),
                        ("sl", "Ašhabad"),
                        ("sq", "Aschgabat"),
                        ("sr", "Ашхабад"),
                        ("sr_Latn", "Ašhabad"),
                        ("sv", "Asjchabad"),
                        ("sw", "Ashgabat"),
                        ("ta", "அசுக\u{bbe}ப\u{bbe}த\u{bcd}"),
                        ("te", "అషక\u{c3e}బ\u{c3e}ద\u{c4d}"),
                        ("th", "อาชกาบ\u{e31}ต"),
                        ("tk", "Aşgabat"),
                        ("tr", "Aşkabat"),
                        ("uk", "Ашгабат"),
                        ("ur", "اشک آباد"),
                        ("uz", "Ashxobod"),
                        ("vi", "Ashgabat"),
                        ("yo", "Ashgabat"),
                        ("yo_BJ", "Ashgabat"),
                        ("yue", "阿什哈巴德"),
                        ("yue_Hans", "阿什哈巴德"),
                        ("zh", "阿什哈巴德"),
                    ]),
                    unofficial_name_list: [].to_vec(),
                },
            ),
        ])
    }
}
#[allow(unused_imports)]
use crate::{
    Alpha2, Alpha3, Continent, Country, CurrencyCode, Region, SubRegion, WeekDay, WorldRegion, GEC,
    IOC,
};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "tm")]
/// [`Country`](crate::Country) struct for this country.
pub fn new() -> Country {
    Country{
        alpha2: Alpha2::TM,
        alpha3: Alpha3::TKM,
        address_format: None,
        continent: Continent::Asia,
        country_code: 993,
        currency_code: CurrencyCode::TMT,
        gec: Some(GEC::TX),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "810",
        ioc: Some(IOC::TKM),
        iso_long_name: "Turkmenistan",
        iso_short_name: "Turkmenistan",
        official_language_list: ["tk"].to_vec(),
        spoken_language_list: ["ru", "tk"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "8",
        nationality: Some("Turkmen"),
        number: "795",
        postal_code: true,
        postal_code_format: Some("\\d{6}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::CentralAsia),
        un_locode: "TM",
        unofficial_name_list: ["Turkmenistan", "Turkménistan", "Turkmenistán", "トルクメニスタン", "Turkmenia"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "emojis")]
        emoji: "🇹🇲",
        #[cfg(feature = "translations")]
        translations: HashMap::from([("ab", "Turkmenistan"), ("af", "Toerkmenië"), ("ak", "Turkmenistan"), ("am", "ቱርክመኒስታን"), ("an", "Turkmenistan"), ("ar", "تركمانستان"), ("as", "ত\u{9c1}ৰ\u{9cd}কমেনিস\u{9cd}ত\u{9be}ন"), ("ay", "Turkmenistan"), ("az", "Türkmənistan"), ("ba", "Turkmenistan"), ("be", "Туркменістан"), ("bg", "Туркменистан"), ("bi", "Turkmenistan"), ("bn", "ত\u{9c1}র\u{9cd}কমেনিস\u{9cd}ত\u{9be}ন"), ("bn_IN", "ত\u{9c1}র\u{9cd}কমেনিস\u{9cd}ত\u{9be}ন"), ("br", "Turkmenistan"), ("bs", "Turkmenistan"), ("ca", "Turkmenistan"), ("ce", "Туркмени"), ("ch", "Turkmenistan"), ("cs", "Turkmenistán"), ("cv", "Туркмени"), ("cy", "Turkmenistan"), ("da", "Turkmenistan"), ("de", "Turkmenistan"), ("dv", "ތ\u{7aa}ރ\u{7aa}ކ\u{7aa}މ\u{7ac}ނ\u{7a8}ސ\u{7b0}ތ\u{7a7}ނ\u{7b0}"), ("dz", "ཊརཀ་མ\u{f72}་ན\u{f72}ས\u{f72}་ཏ\u{f71}ན།"), ("ee", "Turkmenistan"), ("el", "Τουρκμενιστάν"), ("en", "Turkmenistan"), ("eo", "Turkmenio"), ("es", "Turkmenistán"), ("et", "Türkmenistan"), ("eu", "Turkmenistan"), ("fa", "ترکمنستان"), ("ff", "Turkmenistan"), ("fi", "Turkmenistan"), ("fo", "Turkmenistan"), ("fr", "Turkménistan"), ("fy", "Turkmenistan"), ("ga", "An Tuircméanastáin"), ("gl", "Turkmenistán"), ("gn", "Turkmenistan"), ("gu", "ત\u{ac1}ર\u{acd}કમ\u{ac7}નિસ\u{acd}તાન"), ("gv", "Yn Turkmenistaan"), ("ha", "Turkmenistan"), ("he", "טורקמניסטן"), ("hi", "त\u{941}र\u{94d}कम\u{947}निस\u{94d}तान"), ("hr", "Turkmenistan"), ("ht", "Tirkmenistan"), ("hu", "Türkmenisztán"), ("hy", "Թուրքմենստան"), ("ia", "Turkmenistan"), ("id", "Turkmenistan"), ("io", "Turkmenistan"), ("is", "Túrkmenistan"), ("it", "Turkmenistan"), ("iu", "Turkmenistan"), ("ja", "トルクメニスタン"), ("ka", "თურქმენეთი"), ("ki", "Turkmenistan"), ("kk", "Түрікменстан"), ("kl", "Turkmenistan"), ("km", "ទ\u{17bd}គមេន\u{17b8}ស\u{17d2}តង\u{17cb}"), ("kn", "ತುರ\u{ccd}ಕ\u{ccd}ಮ\u{cc6}ನ\u{ccd}\u{200d}ಸ\u{ccd}ತಾನ\u{ccd}"), ("ko", "투르크메니스탄"), ("ku", "Tirkmenîstan"), ("kv", "Туркменистан"), ("kw", "Pow Turkmen"), ("ky", "Түркмөнстан"), ("lo", "Turkmenistan"), ("lt", "Turkmėnistanas"), ("lv", "Turkmenistāna"), ("mi", "Turkmenistan"), ("mk", "Туркменистан"), ("ml", "ത\u{d41}ര\u{d4d}\u{200d}ക\u{d4d}\u{200c}മെനിസ\u{d4d}ത\u{d3e}ന\u{d4d}\u{200d}"), ("mn", "Туркменстан"), ("mr", "त\u{941}र\u{94d}कम\u{947}निस\u{94d}तान"), ("ms", "Turkmenistan"), ("mt", "Turkmenistan"), ("my", "တာ\u{1037}ခ\u{103a}မင\u{103a}နစ\u{1039}စတန\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}"), ("na", "Turkmenistan"), ("nb", "Turkmenistan"), ("ne", "ट\u{941}र\u{94d}कम\u{947}निस\u{94d}तान"), ("nl", "Turkmenistan"), ("nn", "Turkmenistan"), ("nv", "Tʼóokmen Bikéyah"), ("oc", "Turcmenistan"), ("or", "ତ\u{b41}ର\u{b4d}କମେନ\u{b3f}ସ\u{b4d}ତ\u{b3e}ନ"), ("pa", "ਤ\u{a41}ਰਕਮਸਤਾਨ"), ("pi", "त\u{941}र\u{94d}कमिनिस\u{94d}थान"), ("pl", "Turkmenistan"), ("ps", "ترکمنستان"), ("pt", "Turquemenistão"), ("pt_BR", "Turcomenistão"), ("ro", "Turkmenistan"), ("ru", "Туркменистан"), ("rw", "Turikimenisitani"), ("sc", "Turkmènistan"), ("sd", "ترڪمانستان"), ("si", "ත\u{dd4}ර\u{dca}ක\u{dd2}මෙන\u{dd2}ස\u{dca}ත\u{dcf}නය"), ("sk", "Turkménsko"), ("sl", "Turkmenistan"), ("so", "Turkmenistan"), ("sq", "Turkmenistan"), ("sr", "Туркменистан"), ("sv", "Turkmenistan"), ("sw", "Turkmenistan"), ("ta", "துர\u{bcd}க\u{bcd}மெனிஸ\u{bcd}த\u{bbe}ன\u{bcd}"), ("te", "టర\u{c4d}కమ\u{c46}న\u{c3f}స\u{c4d}త\u{c3e}న\u{c4d}"), ("tg", "Туркманистон"), ("th", "เต\u{e34}ร\u{e4c}กเมน\u{e34}สถาน"), ("ti", "Turkmenistan"), ("tk", "Türkmenistan"), ("tl", "Turkmenistan"), ("tr", "Türkmenistan"), ("tt", "Төркмәнстан"), ("ug", "تۈركمەنىستان"), ("uk", "Туркменістан"), ("ur", "ترکمانستان"), ("uz", "Turkmaniston"), ("ve", "Turkmenistan"), ("vi", "Tuốc-mê-ni-xtanh"), ("wa", "Turcmenistan"), ("wo", "Turkumenistaan"), ("xh", "Turkmenistan"), ("yo", "Turkmẹ\u{301}nìstán"), ("zh_CN", "土库曼斯坦"), ("zh_HK", "土庫曼"), ("zh_TW", "土庫曼"), ("zu", "Turkmenistan")]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
        g7_member: false,
        g20_member: false,
    }
}
