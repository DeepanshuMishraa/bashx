use std::path::PathBuf;

pub fn get_cache_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".bashx")
        .join("cache")
}


pub fn clean_cache() -> Result<(), std::io::Error> {
    let cache_dir = get_cache_dir();
    if cache_dir.exists() {
        std::fs::remove_dir_all(&cache_dir)?;
        println!("Cache directory cleaned successfully.");
    } else {
        println!("No cache directory found to clean.");
    }
    Ok(())
}
