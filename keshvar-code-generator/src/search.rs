use crate::{log, structs::CountryInfo, utils};
use anyhow::Result;
use std::{io::Write, path::PathBuf};

pub fn generate_mod(
    destination_file: &PathBuf,
    features_and_modules: &[(&str, &str)],
) -> Result<()> {
    let mut mod_rs_file = utils::create_new_file(destination_file, "search module")?;
    utils::write_first_comments(&mut mod_rs_file, file!())?;
    // `filename_list` is sorted so we iterate over it and lookup info from `countries_info`:
    for (feature, module) in features_and_modules {
        mod_rs_file.write_all(format!("#[cfg(feature = {:?})]\n", feature).as_bytes())?;
        mod_rs_file.write_all(format!("mod {};\n", module).as_bytes())?;
        mod_rs_file.write_all(format!("#[cfg(feature = {:?})]\n", feature).as_bytes())?;
        mod_rs_file.write_all(format!("pub use {}::*;\n\n", module).as_bytes())?;
    }
    log!("Generated {:?}", destination_file);
    Ok(())
}

pub fn generate<F>(
    destination_file: &PathBuf,
    file_title: &str,
    countries_info_list: &Vec<(String, CountryInfo)>,
    static_hashmap_name: &str,
    static_hashmap_type: &str,
    feature_name: &str,
    import_list: &[&str],
    mut iterator_function: F,
) -> Result<()>
where
    F: FnMut(&Vec<(String, CountryInfo)>) -> Vec<String>,
{
    let mut file = utils::create_new_file(destination_file, file_title)?;
    utils::write_first_comments(&mut file, file!())?;
    file.write_all(format!("#[cfg(feature = {feature_name:?})]\n").as_bytes())?;
    file.write_all(b"use hashbrown::HashMap;\n")?;
    file.write_all(format!("#[cfg(feature = {feature_name:?})]\n").as_bytes())?;
    file.write_all(b"use std::sync::LazyLock;\n")?;
    for import in import_list {
        file.write_all(format!("#[cfg(feature = {feature_name:?})]\n").as_bytes())?;
        file.write_all(format!("use {};\n", import).as_bytes())?;
    }
    file.write_all(format!("#[cfg(feature = {feature_name:?})]\n").as_bytes())?;
    file.write_all(format!("pub static {static_hashmap_name}: LazyLock<{static_hashmap_type}> = LazyLock::new(|| HashMap::from([\n").as_bytes())?;
    for data in iterator_function(countries_info_list) {
        file.write_all(data.as_bytes())?;
    }
    file.write_all(b"]));\n")?;
    log!("Generated {destination_file:?}");
    Ok(())
}
