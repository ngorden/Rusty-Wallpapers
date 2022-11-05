use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::path::Path;

use crate::models::Wallpaper;

pub async fn download_wallpaper(page: i32, api_key: String, categories: String, sorting: String, purity: String, query: String, dir: String) -> Result<i32, Box<dyn std::error::Error>> {
        let mut target: String = 
            format!("https://wallhaven.cc/api/v1/search?categories={}&sorting={}&purity={}&q={}&page={}",
                    categories, sorting, purity, query, page);

        if !api_key.is_empty() {
           target = format!("https://wallhaven.cc/api/v1/search?apikey={}&categories={}&sorting={}&purity={}&q={}&page={}",
                    api_key, categories, sorting, purity, query, page);
        };

        let response = reqwest::get(target).await?;
        let content = response.text().await?;
        let wallpaper_query: Wallpaper = serde_json::from_str(&*content).unwrap();

        for wallpaper in wallpaper_query.data {
            let wp = reqwest::get(wallpaper.path).await.expect("couldn't get wp");
            let wp_content = wp.bytes().await?;

            let path = format!("{}/{}", dir, wallpaper.id);
            write_file(&*path, wp_content.as_ref())?;
        }

        Ok(wallpaper_query.meta.last_page)
}

pub fn get_environment_variable(key: &str) -> Option<String> {
    let env_var = std::env::var(key);
    match env_var.is_ok() {
        true => { Some(env_var.unwrap()) }
        false => { None }
    }
}

pub fn spawn_dmenu(prompt: &str, options: &str) -> String {
    let process = match Command::new("dmenu")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args(["-i", "-p", prompt])
        .spawn() {
        Ok(proc) => { proc }
        Err(why) => { panic!("{}", why) }
    };

    match process.stdin.unwrap().write_all(options.as_bytes()) {
        Err(why) => panic!("{}", why),
        Ok(_) => {}
    };

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("{}", why),
        Ok(_) => {}
    };

    s
}

pub fn write_file(target: &str, content: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(target);
    let mut file = match File::create(&path) {
        Err(why) => { panic!("could not create {}", why) }
        Ok(file) => { file }
    };

    file.write_all(content).expect("couldn't write file");
    Ok(())
}
