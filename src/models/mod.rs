use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WallpaperData {
    pub id: String,
    url: String,
    short_url: String,
    views: i32,
    favorites: i32,
    source: String,
    purity: String,
    category: String,
    dimension_x: i32,
    dimension_y: i32,
    resolution: String,
    ratio: String,
    file_size: i32,
    file_type: String,
    created_at: String,
    colors: Vec<String>,
    pub path: String,
    thumbs: Thumbnails,
}

#[derive(Deserialize, Serialize)]
pub struct Thumbnails {
    large: String,
    original: String,
    small: String,
}

#[derive(Deserialize, Serialize)]
pub struct WallpaperMetadata {
    pub current_page: i32,
    pub last_page: i32,
    pub per_page: i32,
    pub total: i32,
    pub query: String,
    pub seed: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct Wallpaper {
    pub data: Vec<WallpaperData>,
    pub meta: WallpaperMetadata,
}
