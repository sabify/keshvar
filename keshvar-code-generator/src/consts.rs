use crate::{structs::CountryInfo, utils};
use anyhow::Result;
use std::{collections::HashMap, io::Write, path::PathBuf};

pub fn generate(
    destination_file: &PathBuf,
    countries_info_list: &Vec<(String, CountryInfo)>,
    continent_features: &HashMap<String, Vec<String>>,
    region_features: &HashMap<String, Vec<String>>,
    subregion_features: &HashMap<String, Vec<String>>,
    world_region_features: &HashMap<String, Vec<String>>,
) -> Result<()> {
    let mut consts_rs_file = utils::create_new_file(&destination_file, "constant")?;
    utils::write_first_comments(&mut consts_rs_file, file!())?;
    consts_rs_file.write_all(b"#[allow(unused_imports)]\n")?;
    consts_rs_file
        .write_all(b"use crate::{Alpha2, Continent, Region, SubRegion, WorldRegion};\n")?;
    consts_rs_file.write_all(b"use std::sync::LazyLock;\n")?;
    consts_rs_file.write_all(
        format!(
            "pub const ALL_COUNTRIES_COUNT: usize = {};",
            countries_info_list.len()
        )
        .as_bytes(),
    )?;
    consts_rs_file.write_all(
        r#"
pub static SUPPORTED_COUNTRIES_COUNT: LazyLock<usize> = LazyLock::new(|| SUPPORTED_ALPHA2_LIST.len());
pub static UNSUPPORTED_COUNTRIES_COUNT: LazyLock<usize> = LazyLock::new(|| ALL_COUNTRIES_COUNT - *SUPPORTED_COUNTRIES_COUNT);
"#
        .as_bytes(),
    )?;

    consts_rs_file.write_all(
        utils::country_cfg_features(
            countries_info_list
                .iter()
                .map(|(_, info)| info.feature_name.clone())
                .collect(),
            "all",
            0,
        )
        .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORT_ALL_COUNTRIES: bool = true;\n")?;
    consts_rs_file.write_all(
        utils::country_cfg_not_features(
            countries_info_list
                .iter()
                .map(|(_, info)| info.feature_name.clone())
                .collect(),
            "all",
            0,
        )
        .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORT_ALL_COUNTRIES: bool = false;\n")?;

    // consts_rs_file.write_all(
    //     b"lazy_static! { pub static ref ALL_ALPHA2_LIST: &'static [&'static str] = &[\n",
    // )?;
    // for (_, info) in countries_info_list.iter() {
    //     consts_rs_file.write_all(
    //         format!(
    //             "    {:?}, {}",
    //             info.alpha2_upper,
    //             utils::commented_country_name(info)
    //         )
    //         .as_bytes(),
    //     )?;
    // }
    // consts_rs_file.write_all(b"];}\n")?;

    consts_rs_file.write_all(
        r#"
/// A constant list containing all included [`Alpha2`](crate::Alpha2) codes.
///
/// This alphabeticaly sorted list will be created at compile-time based on included country features.
/// # Example
/// ```
/// use keshvar::{Alpha2, SUPPORTED_ALPHA2_LIST};
/// assert!(SUPPORTED_ALPHA2_LIST.contains(&Alpha2::US));
/// ```
"#
        .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORTED_ALPHA2_LIST: &[Alpha2] = &[\n")?;
    for (_, info) in countries_info_list.iter() {
        consts_rs_file
            .write_all(utils::country_cfg_feature_and_commented_name(info, 1).as_bytes())?;
        consts_rs_file.write_all(format!("    Alpha2::{},\n", info.alpha2_upper).as_bytes())?;
    }
    consts_rs_file.write_all(b"];\n")?;

    consts_rs_file.write_all(
        r#"
/// A constant list containing all included [`Alpha2`](crate::Alpha2) codes that are members of [G7](https://en.wikipedia.org/wiki/G7).
///
/// This alphabeticaly sorted list will be created at compile-time based on included country features.
/// # Example
/// ```
/// use keshvar::{Alpha2, SUPPORTED_G7_ALPHA2_LIST};
/// assert!(SUPPORTED_G7_ALPHA2_LIST.contains(&Alpha2::US));
/// ```
"#
            .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORTED_G7_ALPHA2_LIST: &[Alpha2] = &[\n")?;
    for (_, info) in countries_info_list.iter() {
        if info.g7_member.unwrap_or(false) {
            consts_rs_file
                .write_all(utils::country_cfg_feature_and_commented_name(info, 1).as_bytes())?;
            consts_rs_file.write_all(format!("    Alpha2::{},\n", info.alpha2_upper).as_bytes())?;
        }
    }
    consts_rs_file.write_all(b"];\n")?;

    consts_rs_file.write_all(
        r#"
/// A constant list containing all included [`Alpha2`](crate::Alpha2) codes that are members of [G20](https://en.wikipedia.org/wiki/G20).
///
/// This alphabeticaly sorted list will be created at compile-time based on included country features.
/// # Example
/// ```
/// use keshvar::{Alpha2, SUPPORTED_G20_ALPHA2_LIST};
/// assert!(SUPPORTED_G20_ALPHA2_LIST.contains(&Alpha2::US));
/// ```
"#
            .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORTED_G20_ALPHA2_LIST: &[Alpha2] = &[\n")?;
    for (_, info) in countries_info_list.iter() {
        if info.g20_member.unwrap_or(false) {
            consts_rs_file
                .write_all(utils::country_cfg_feature_and_commented_name(info, 1).as_bytes())?;
            consts_rs_file.write_all(format!("    Alpha2::{},\n", info.alpha2_upper).as_bytes())?;
        }
    }
    consts_rs_file.write_all(b"];\n")?;

    // consts_rs_file.write_all(
    //     b"lazy_static! { pub static ref UNSUPPORTED_ALPHA2_LIST: &'static [&'static str] = &[\n",
    // )?;
    // for (_, info) in countries_info_list.iter() {
    //     consts_rs_file.write_all(
    //         format!(
    //             "    #[cfg(not(feature = {:?}))] {}",
    //             info.feature_name,
    //             utils::commented_country_name(info)
    //         )
    //         .as_bytes(),
    //     )?;
    //     consts_rs_file.write_all(format!("    {:?},\n", info.alpha2_upper).as_bytes())?;
    // }
    // consts_rs_file.write_all(b"];}\n")?;

    consts_rs_file.write_all(
        r#"
/// A constant list containing all included [`Continent`](crate::Alpha2)s.
///
/// This alphabeticaly sorted list will be created at compile-time based on included country features.
///
/// # Example
/// ```
/// use keshvar::{Continent, SUPPORTED_CONTINENT_LIST};
/// assert!(SUPPORTED_CONTINENT_LIST.contains(&Continent::Asia));
/// ```
"#
        .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORTED_CONTINENT_LIST: &[Continent] = &[\n")?;
    let mut sorted_continent_features = continent_features.keys().collect::<Vec<_>>();
    sorted_continent_features.sort();
    for continent in sorted_continent_features {
        let feature_list = continent_features.get(continent).unwrap();
        consts_rs_file.write_all(b"    #[\n        cfg(\n            all(\n")?;
        consts_rs_file.write_all(
            feature_list
                .iter()
                .map(|x| format!("                feature = {:?}", x))
                .collect::<Vec<_>>()
                .join(",\n")
                .as_bytes(),
        )?;
        consts_rs_file.write_all(b"\n            \n)        )\n     ]\n")?;
        consts_rs_file
            .write_all(format!("    Continent::{},\n", utils::capitalize(continent)).as_bytes())?;
    }
    consts_rs_file.write_all(b"];\n")?;

    consts_rs_file.write_all(
        r#"
/// A constant list containing all included [`Region`](crate::Alpha2)s.
///
/// This alphabeticaly sorted list will be created at compile-time based on included country features.
///
/// # Example
/// ```
/// use keshvar::{Region, SUPPORTED_REGION_LIST};
/// assert!(SUPPORTED_REGION_LIST.contains(&Region::Asia));
/// ```
"#
        .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORTED_REGION_LIST: &[Region] = &[\n")?;
    let mut sorted_region_features = region_features.keys().collect::<Vec<_>>();
    sorted_region_features.sort();
    for region in sorted_region_features {
        let feature_list = region_features.get(region).unwrap();
        // ...
        //     #[cfg(all())]
        //     Region::Antarctica,
        // ...
        if !feature_list.is_empty() {
            consts_rs_file.write_all(b"    #[\n        cfg(\n            all(\n")?;
            consts_rs_file.write_all(
                feature_list
                    .iter()
                    .map(|x| format!("                feature = {:?}", x))
                    .collect::<Vec<_>>()
                    .join(",\n")
                    .as_bytes(),
            )?;
            consts_rs_file.write_all(b"\n            \n)        )\n     ]\n")?;
        }
        consts_rs_file
            .write_all(format!("    Region::{},\n", utils::capitalize(region)).as_bytes())?;
    }
    consts_rs_file.write_all(b"];\n")?;

    consts_rs_file.write_all(
        r#"
/// A constant list containing all included [`SubRegion`](crate::Alpha2)s.
///
/// This alphabeticaly sorted list will be created at compile-time based on included country features.
///
/// # Example
/// ```
/// use keshvar::{SubRegion, SUPPORTED_SUBREGION_LIST};
/// assert!(SUPPORTED_SUBREGION_LIST.contains(&SubRegion::SouthAmerica));
/// ```
"#
        .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORTED_SUBREGION_LIST: &[SubRegion] = &[\n")?;
    let mut sorted_subregion_features = subregion_features.keys().collect::<Vec<_>>();
    sorted_subregion_features.sort();
    for subregion in sorted_subregion_features {
        let feature_list = subregion_features.get(subregion).unwrap();
        consts_rs_file.write_all(b"    #[\n        cfg(\n            all(\n")?;
        consts_rs_file.write_all(
            feature_list
                .iter()
                .map(|x| format!("                feature = {:?}", x))
                .collect::<Vec<_>>()
                .join(",\n")
                .as_bytes(),
        )?;
        consts_rs_file.write_all(b"\n            \n)        )\n     ]\n")?;
        consts_rs_file
            .write_all(format!("    SubRegion::{},\n", utils::capitalize(subregion)).as_bytes())?;
    }
    consts_rs_file.write_all(b"];\n")?;

    consts_rs_file.write_all(
        r#"
/// A constant list containing all included [`WorldRegion`](crate::Alpha2)s.
///
/// This alphabeticaly sorted list will be created at compile-time based on included country features.
///
/// # Example
/// ```
/// use keshvar::{WorldRegion, SUPPORTED_WORLD_REGION_LIST};
/// assert!(SUPPORTED_WORLD_REGION_LIST.contains(&WorldRegion::APAC));
/// ```
"#
        .as_bytes(),
    )?;
    consts_rs_file.write_all(b"pub const SUPPORTED_WORLD_REGION_LIST: &[WorldRegion] = &[\n")?;
    let mut sorted_world_region_features = world_region_features.keys().collect::<Vec<_>>();
    sorted_world_region_features.sort();
    for world_region in sorted_world_region_features {
        let feature_list = world_region_features.get(world_region).unwrap();
        consts_rs_file.write_all(b"    #[\n        cfg(\n            all(\n")?;
        consts_rs_file.write_all(
            feature_list
                .iter()
                .map(|x| format!("                feature = {:?}", x))
                .collect::<Vec<_>>()
                .join(",\n")
                .as_bytes(),
        )?;
        consts_rs_file.write_all(b"\n            \n)        )\n     ]\n")?;
        consts_rs_file.write_all(
            format!(
                "    WorldRegion::{},\n",
                utils::capitalize(world_region).to_uppercase()
            )
            .as_bytes(),
        )?;
    }
    consts_rs_file.write_all(b"];\n")?;
    Ok(())
}
