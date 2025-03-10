mod alpha2;
mod alpha3;
mod cargo_toml;
mod consts;
mod countries;
mod currency_code;
mod gec;
mod ioc;
mod population;
mod region;
mod search;
mod structs;
mod translations;
mod utils;

use structs::CountryInfo;

use anyhow::{bail, Context, Result};
use std::{collections::HashMap, env, fs, path::PathBuf, str::FromStr};

fn main() -> Result<()> {
    if env::args().len() != 3 {
        bail!("You should set input (`countries` library) and output directories");
    }
    let argument_list: Vec<_> = env::args().collect();
    let input_directory = PathBuf::from(&argument_list[1]);
    if !input_directory.exists() {
        bail!("Could not found input directory {input_directory:?}");
    }
    let output_directory = PathBuf::from(&argument_list[2]);
    // if output_directory.exists() {
    //     bail!("Output directory {output_directory:?} should be empty!");
    // }
    utils::remove_log_file();
    code_gen(input_directory, output_directory)
}

fn code_gen(input_directory: PathBuf, output_directory: PathBuf) -> Result<()> {
    let data_directory = input_directory.join("lib").join("countries").join("data");
    if !data_directory.exists() {
        bail!(
            "Could not found main data directory {data_directory:?} inside input directory {input_directory:?}"
        );
    }
    code_gen_countries(data_directory, output_directory)
}

fn code_gen_countries(data_directory: PathBuf, output_directory: PathBuf) -> Result<()> {
    let src_directory = output_directory.join("src");
    if !src_directory.exists() {
        fs::create_dir(src_directory.clone())
            .context(format!("Could not create directory {src_directory:?}"))?;
    }
    let countries_directory = src_directory.join("countries");
    if !countries_directory.exists() {
        fs::create_dir(countries_directory.clone()).context(format!(
            "Could not create directory {countries_directory:?}"
        ))?;
    }
    let search_directory = src_directory.join("search");
    if !search_directory.exists() {
        fs::create_dir(search_directory.clone())
            .context(format!("Could not create directory {search_directory:?}"))?;
    }
    let input_countries_directory = data_directory.join("countries");
    let mut filename_list = fs::read_dir(&input_countries_directory)
        .context(format!(
            "Could not read directory {input_countries_directory:?}"
        ))?
        .map(|x| x.unwrap().path())
        .collect::<Vec<_>>();
    filename_list.sort();
    // Load countries info:
    let mut countries_info_list = Vec::new();
    let mut found_empty = Vec::new();
    for filename in filename_list.clone() {
        let (country_name, info) = load_and_decode_country_info(
            data_directory.clone(),
            PathBuf::from(filename.file_name().unwrap()),
        )?;
        // Check which option has empty value set:
        for (field, field_name) in [
            (&info.address_format, "address_format"),
            (&info.gec, "gec"),
            (&info.ioc, "ioc"),
            (&info.nationality, "nationality"),
            (&info.vehicle_registration_code, "vehicle_registration_code"),
            (&info.postal_code_format, "postal_code_format"),
            (&info.region, "region"),
            (&info.subregion, "subregion"),
        ] {
            if field.is_none() {
                found_empty.push(field_name);
            }
        }
        countries_info_list.push((country_name, info))
    }
    countries_info_list.sort_by_key(|(name, _)| name.clone());

    let input_translations_directory = data_directory.join("translations");
    let mut translation_filename_list = fs::read_dir(&input_translations_directory)
        .context(format!(
            "Could not read directory {input_translations_directory:?}"
        ))?
        .map(|x| x.unwrap().path())
        .collect::<Vec<_>>();
    translation_filename_list.sort();
    for filename in translation_filename_list {
        let language = match filename
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(['-', '.'])
            .collect::<Vec<&str>>()[..]
        {
            ["countries", language, "yaml"] => language.to_string(),
            _ => bail!(
                "Could not decode translation filename {:?} -> {:?}",
                filename,
                filename
                    .to_str()
                    .unwrap()
                    .split(['-', '.'])
                    .collect::<Vec<&str>>()
            ),
        };
        let translations = translations::load_translations(&filename)?;
        for (alpha2, translation) in translations {
            countries_info_list.iter_mut().for_each(|(_, info)| {
                if info.alpha2_upper == alpha2 {
                    info.translation_list
                        .push((language.clone(), translation.clone()));
                }
            });
        }
    }
    countries_info_list.iter_mut().for_each(|(_, info)| {
        info.translation_list
            .sort_by_key(|(language, _)| language.clone());
    });

    // Bellow options are going to be Option<_>;
    // So at least for one country each option should be set to nothing.
    for field_name in [
        "address_format",
        "gec",
        "ioc",
        "nationality",
        "postal_code_format",
        "region",
        "subregion",
    ] {
        if !found_empty.contains(&field_name) {
            bail!(
                "Option `{}` in all country YAML files has a value set.",
                field_name
            )
        }
    }
    // Categorize countries for `region`, `subregion`, and `world region`:
    let mut region_features = HashMap::new();
    // No country is in region `Antarctica`:
    region_features.insert("antarctica".to_string(), Vec::new());
    let mut subregion_features = HashMap::new();
    let mut world_region_features = HashMap::new();
    let mut continent_features = HashMap::new();
    let mut country_feature_list = Vec::new();
    countries_info_list.iter().for_each(|(_, info)| {
        let country_feature_name = &info.feature_name;
        if let Some(ref region) = info.region {
            let region_feature = utils::to_cargo_feature_name(region);
            if !region_features.contains_key(&region_feature) {
                region_features.insert(region_feature.clone(), Vec::new());
            }
            region_features
                .get_mut(&region_feature)
                .unwrap()
                .push(country_feature_name.clone());
        }
        if let Some(ref subregion) = info.subregion {
            let subregion_feature = utils::to_cargo_feature_name(subregion);
            if !subregion_features.contains_key(&subregion_feature) {
                subregion_features.insert(subregion_feature.clone(), Vec::new());
            }
            subregion_features
                .get_mut(&subregion_feature)
                .unwrap()
                .push(country_feature_name.clone());
        }
        let continent = utils::to_cargo_feature_name(&info.continent);
        if !continent_features.contains_key(&continent) {
            continent_features.insert(continent.clone(), Vec::new());
        }
        continent_features
            .get_mut(&continent)
            .unwrap()
            .push(country_feature_name.clone());
        let world_region = utils::to_cargo_feature_name(&info.world_region);
        if !world_region_features.contains_key(&world_region) {
            world_region_features.insert(world_region.clone(), Vec::new());
        }
        world_region_features
            .get_mut(&world_region)
            .unwrap()
            .push(country_feature_name.clone());
        country_feature_list.push(country_feature_name.clone());
    });
    // Sort features:
    region_features.iter_mut().for_each(|(_, list)| list.sort());
    continent_features
        .iter_mut()
        .for_each(|(_, list)| list.sort());
    subregion_features
        .iter_mut()
        .for_each(|(_, list)| list.sort());
    world_region_features
        .iter_mut()
        .for_each(|(_, list)| list.sort());
    country_feature_list.sort();
    let mut subdivision_type_name_list = Vec::new();
    for (_, info) in countries_info_list.iter() {
        for (_, subdivision) in info.subdivisions.iter() {
            let subdivision_type_name = subdivision._type.to_lowercase();
            if !subdivision_type_name_list.contains(&subdivision_type_name) {
                subdivision_type_name_list.push(subdivision_type_name.clone())
            }
        }
    }
    population::read_from_file("population.csv", &mut countries_info_list)?;
    log!("Region features: {:?}", region_features);
    log!("Continent features: {:?}", continent_features);
    log!("SubRegion features: {:?}", subregion_features);
    log!("World Region features: {:?}", world_region_features);
    log!("Country features: {:?}", country_feature_list);
    log!(
        "Detected subdivision types: {:?}",
        subdivision_type_name_list
    );

    let countries_mod_rs_filename = countries_directory.join("mod.rs");
    countries::generate_mod(&countries_mod_rs_filename, &countries_info_list)?;
    // Generate codes for each country:
    countries_info_list.iter().try_for_each(|(name, info)| {
        let module_filename =
            countries_directory.join(PathBuf::from(name.to_lowercase()).with_extension("rs"));
        countries::generate_country(&module_filename, info)
    })?;

    let mut search_features_and_modules = Vec::new();

    let search_iso_short_name_rs_filename = search_directory.join("iso_short_name.rs");
    let search_iso_shortname_feature = "search-iso-short-name";
    search::generate(
        &search_iso_short_name_rs_filename,
        "search iso shortname",
        &countries_info_list,
        "SUPPORTED_ISO_SHORT_NAMES",
        "HashMap<&'static str, Alpha2>",
        search_iso_shortname_feature,
        &["crate::Alpha2"],
        |countries_info_list| {
            countries_info_list
                .iter()
                .map(|(_, info)| {
                    utils::country_cfg_feature_and_commented_name(info, 1)
                        + format!(
                            "    ({:?}, Alpha2::{}),\n",
                            info.iso_short_name.to_lowercase(),
                            info.alpha2_upper
                        )
                        .as_str()
                })
                .collect()
        },
    )?;
    search_features_and_modules.push((search_iso_shortname_feature, "iso_short_name"));

    let search_iso_long_name_rs_filename = search_directory.join("iso_long_name.rs");
    let search_iso_long_name_feature = "search-iso-long-name";
    search::generate(
        &search_iso_long_name_rs_filename,
        "search iso longname",
        &countries_info_list,
        "SUPPORTED_ISO_LONG_NAMES",
        "HashMap<&'static str, Alpha2>",
        search_iso_long_name_feature,
        &["crate::Alpha2"],
        |countries_info_list| {
            countries_info_list
                .iter()
                .map(|(_, info)| {
                    utils::country_cfg_feature_and_commented_name(info, 1)
                        + format!(
                            "    ({:?}, Alpha2::{}),\n",
                            info.iso_long_name.to_lowercase(),
                            info.alpha2_upper
                        )
                        .as_str()
                })
                .collect()
        },
    )?;
    search_features_and_modules.push((search_iso_long_name_feature, "iso_long_name"));

    let search_iso_number_rs_filename = search_directory.join("number.rs");
    let search_iso_number_feature = "search-iso-number";
    search::generate(
        &search_iso_number_rs_filename,
        "search iso number",
        &countries_info_list,
        "SUPPORTED_COUNTRY_NUMBERS",
        "HashMap<usize, Alpha2>",
        search_iso_number_feature,
        &["crate::Alpha2"],
        |countries_info_list| {
            countries_info_list
                .iter()
                .map(|(_, info)| {
                    utils::country_cfg_feature_and_commented_name(info, 1)
                        + format!(
                            "    ({}, Alpha2::{}),\n",
                            usize::from_str(&info.number).unwrap(),
                            info.alpha2_upper,
                        )
                        .as_str()
                })
                .collect()
        },
    )?;
    search_features_and_modules.push((search_iso_number_feature, "number"));

    let search_iso_country_code_rs_filename = search_directory.join("country_code.rs");
    let search_iso_country_code_feature = "search-country-code";
    search::generate(
        &search_iso_country_code_rs_filename,
        "search iso number",
        &countries_info_list,
        "SUPPORTED_COUNTRY_CODE",
        "HashMap<usize, Alpha2>",
        search_iso_country_code_feature,
        &["crate::Alpha2"],
        |countries_info_list| {
            countries_info_list
                .iter()
                .map(|(_, info)| {
                    utils::country_cfg_feature_and_commented_name(info, 1)
                        + format!(
                            "    ({}, Alpha2::{}),\n",
                            info.country_code, info.alpha2_upper
                        )
                        .as_str()
                })
                .collect()
        },
    )?;
    search_features_and_modules.push((search_iso_country_code_feature, "country_code"));

    let search_translations_rs_filename = search_directory.join("translations.rs");
    let search_translations_feature = "search-translations";
    search::generate(
        &search_translations_rs_filename,
        "search iso number",
        &countries_info_list,
        "SUPPORTED_COUNTRY_TRANSLATED_NAMES",
        "HashMap<&'static str, Alpha2>",
        search_translations_feature,
        &["crate::Alpha2"],
        |countries_info_list| {
            let mut all_name_list: Vec<_> = countries_info_list
                .iter()
                .map(|(_, info)| {
                    let mut name_list = Vec::from([
                        info.iso_short_name.clone().to_lowercase(),
                        info.iso_long_name.clone().to_lowercase(),
                    ]);
                    name_list.extend(
                        info.unofficial_names
                            .clone()
                            .into_iter()
                            .map(|name| name.to_lowercase()),
                    );
                    name_list.extend(
                        info.translation_list
                            .clone()
                            .iter()
                            .map(|(_, name)| name.clone()),
                    );
                    let mut name_list: Vec<_> = name_list
                        .into_iter()
                        .map(|name| {
                            (
                                utils::country_cfg_feature_and_commented_name(&info, 1),
                                info.alpha2_upper.clone(),
                                name.to_lowercase(),
                            )
                        })
                        .collect();
                    name_list.sort_by_key(|(_, _, x)| x.clone());
                    name_list
                })
                .flatten()
                .collect();
            all_name_list.sort_by_key(|(_, _, x)| x.clone());
            all_name_list
                .into_iter()
                .map(|(feature_name, alpha2, name)| {
                    format!("{}    ({:?}, Alpha2::{}),\n", feature_name, name, alpha2)
                })
                .collect()
        },
    )?;
    search_features_and_modules.push((search_translations_feature, "translations"));

    let search_mod_rs_filename = search_directory.join("mod.rs");
    search::generate_mod(&search_mod_rs_filename, &search_features_and_modules)?;

    let region_rs_filename = src_directory.join("region.rs");
    region::generate(
        &region_rs_filename,
        &countries_info_list,
        &continent_features,
        &region_features,
        &subregion_features,
        &world_region_features,
        &subdivision_type_name_list,
    )?;

    let gec_rs_filename = src_directory.join("gec.rs");
    gec::generate(&gec_rs_filename, &countries_info_list)?;

    let ioc_rs_filename = src_directory.join("ioc.rs");
    ioc::generate(&ioc_rs_filename, &countries_info_list)?;

    let currency_code_rs_filename = src_directory.join("currency_code.rs");
    currency_code::generate(&currency_code_rs_filename, &countries_info_list)?;

    let alpha2_rs_filename = src_directory.join("alpha2.rs");
    alpha2::generate(&alpha2_rs_filename, &countries_info_list)?;

    let alpha3_rs_filename = src_directory.join("alpha3.rs");
    alpha3::generate(&alpha3_rs_filename, &countries_info_list)?;

    let consts_rs_filename = src_directory.join("consts.rs");
    consts::generate(
        &consts_rs_filename,
        &countries_info_list,
        &continent_features,
        &region_features,
        &subregion_features,
        &world_region_features,
    )?;

    let cargo_toml_filename = output_directory.join("Cargo.toml");
    cargo_toml::generate(
        &cargo_toml_filename,
        &countries_info_list,
        &country_feature_list,
        &continent_features,
        &region_features,
        &subregion_features,
        &world_region_features,
    )?;
    Ok(())
}

fn load_and_decode_country_info(
    data_directory: PathBuf,
    yaml_filename: PathBuf,
) -> Result<(String, CountryInfo)> {
    let country_filename = data_directory.join("countries").join(&yaml_filename);
    let (name, mut info) = structs::load_country_info(&country_filename)?;
    let subdivision_filename = data_directory.join("subdivisions").join(&yaml_filename);
    info.subdivisions = structs::load_country_subdivisions(&subdivision_filename)?;
    Ok((name, info))
}
