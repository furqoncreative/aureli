use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::process::Command;

use glob::glob;
use log::error;

use aureli::entities::AutoReviewConfig;

pub fn find_file(submission_path: &Path, file: &str) -> Option<PathBuf> {
    let pattern = submission_path.join("**").join(file);
    for entry in glob(pattern.to_str().unwrap()).unwrap() {
        return match entry {
            Ok(path) => Some(path),
            Err(err) => {
                panic!("Error finding file: {}", err);
            }
        };
    }
    None
}

pub fn read_file(path: &Path) -> Option<String> {
    let content = read_to_string(path);

    match content {
        Ok(content) => Some(content),
        Err(_) => panic!("Error reading file: {:?}", path),
    }
}

pub fn get_auto_review_config(submission_path: &Path) -> AutoReviewConfig {
    let config_path = submission_path.join("auto-review-config.json");
    let config = serde_json::from_str(&read_file(&config_path).unwrap());

    match config {
        Ok(config) => config,
        Err(err) => panic!("Error parsing auto-review-config.json: {}", err),
    }
}

pub fn run_npm_install(path: &Path) {
    let status = Command::new("bash")
        .current_dir(path)
        .arg("-c")
        .arg("npm install")
        .status()
        .unwrap();

    if !status.success() {
        error!("npm install failed");
    }
}

pub fn run_main_js(path: &Path) {
    let status = Command::new("bash")
        .current_dir(path.parent().unwrap())
        .arg("-c")
        .arg("node main.js &")
        .status()
        .unwrap();

    if !status.success() {
        error!("run node main.js failed");
    }
}

pub fn stop_server() {
    let status = Command::new("bash")
        .arg("-c")
        .arg("kill -9 $(lsof -t -i:5000)")
        .status()
        .unwrap();

    if !status.success() {
        error!("stop node server failed");
    }
}
