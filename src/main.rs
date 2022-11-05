mod models;
mod utilities;

use std::path::Path;
use crate::utilities::{download_wallpaper, get_environment_variable, spawn_dmenu};

const API_KEY: &str = "";
const DEFAULT_CATEGORY: &str = "111";
const DEFAULT_PURITY: &str = "110";

const SORT_OPTS: &str = "date_added\nrelevance\nrandom\nviews\nfavorites\ntoplist";
const TAG_OPTS: &str = "Minimalism\nCyberpunk 2077\nfantasy girl\n#digital art\n#anime\n#4K\n#nature";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_walldir: String = format!("{}/Pictures", std::env::var("HOME").unwrap());

    let mut page = 1;
    let mut max_pages = 5;

    let query = spawn_dmenu("Search terms", TAG_OPTS);
    let categories = get_environment_variable("CATEGORIES").unwrap_or(String::from(DEFAULT_CATEGORY));
    let purity = get_environment_variable("PURITY").unwrap_or(String::from(DEFAULT_PURITY));
    let sorting = get_environment_variable("SORTING").unwrap_or_else(|| spawn_dmenu("Sort Options:", SORT_OPTS));
    let walldir = get_environment_variable("WALLDIR").unwrap_or_else(|| get_environment_variable("XDG_CACHE_HOME").unwrap_or(default_walldir));
    let download_dir = Path::new(&walldir);
    let temp_dir = download_dir.join("newwps");

    std::fs::create_dir(&temp_dir)?;
    println!("saving wallpapers to {}", temp_dir.to_str().unwrap());

    loop {
        if page > max_pages { break; }
        let last_page = download_wallpaper(page, String::from(API_KEY), categories.to_owned(), sorting.to_owned(), purity.to_owned(), query.to_owned(), String::from(temp_dir.to_str().unwrap())).await?;
        max_pages = if last_page < max_pages { last_page } else { max_pages };
        println!("Downloaded page {} of {}", page, max_pages);
        page = page + 1;
    }
    Ok(())
}

