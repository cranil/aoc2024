use std::fs::File;

use cargo_manifest::Manifest;
use chrono::TimeZone;
use reqwest::Url;

fn get_cookie() -> String {
    let filename = format!(
        "{}/../cookie.txt",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    let cookie = std::fs::read_to_string(filename).unwrap();
    cookie.trim().to_string()
}

fn main() {
    let manifest = Manifest::from_path("Cargo.toml").unwrap();
    if let Some(package) = manifest.package {
        let name = package.name;
        let day = name[3..].parse::<u32>().unwrap();
        let url = format!("https://adventofcode.com/2024/day/{}/input", day);
        let now = chrono::Utc::now();
        let puzzle_start = chrono::Utc
            .with_ymd_and_hms(2024, 12, day, 5, 0, 0)
            .unwrap();
        if now < puzzle_start {
            println!(
                "cargo:warning=\x1B[38;2;255;0;0mYou are running this before the puzzle unlocks.\x1b[0m"
            );
            return;
        }
        let url = Url::parse(&url).unwrap();
        let client = reqwest::blocking::Client::new();
        let cookie = get_cookie();
        let req = client
            .get(url)
            .header("cookie", format!("session={}", cookie));
        let res = req.send().unwrap();
        let input = res.text().unwrap();
        if File::open("input.txt").is_ok() {
            println!(
                "cargo:warning=\x1B[38;2;0;255;0minput.txt already exists; Not overwriting.\x1b[0m"
            );
            return;
        }
        let filename = format!(
            "{}/input.txt",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );
        std::fs::write(filename, input).unwrap();
    }
}
