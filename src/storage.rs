use crate::models::Film;
use std::fs;
use std::path::PathBuf;

const DATA_FILE: &str = "films.json";

pub fn get_data_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("film_list");
    fs::create_dir_all(&path).ok();
    path.push(DATA_FILE);
    path
}
pub fn load() -> Vec<Film> {
    let local_path = PathBuf::from("films.json");
    let path = if local_path.exists() {
        local_path
    } else {
        get_data_path()
    };
    if let Ok(data) = fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn save(films: &[Film]) -> Result<(), Box<dyn std::error::Error>> {
    // let path = get_data_path();

    let localpath = PathBuf::from("films.json");
    let data = serde_json::to_string_pretty(films)?;
    fs::write(localpath, data)?;
    Ok(())
}
