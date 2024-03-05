use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

use regex::Regex;

use aureli::checklist_key::{
    HTML_CONTAIN_H_1_ELEMENT_WITH_STUDENT_ID, MAIN_JS_EXISTS, MAIN_JS_HAVE_STUDENT_ID_COMMENT,
    PACKAGE_JSON_EXISTS, ROOT_SHOWING_HTML, SERVE_IN_PORT_5000,
};
use aureli::entities::{Checklist, ChecklistResult, Checklists};

use crate::utils::{find_file, read_file, run_main_js, run_npm_install, stop_server};

pub async fn grade_submission(submission_path: &PathBuf, student_id: u32) -> Checklists {
    let mut checklists = Checklists::initialize();
    let checklists_map = &mut checklists.checklists_map;

    let package_json_result = get_package_json(submission_path);
    checklists_map.insert(PACKAGE_JSON_EXISTS, package_json_result.checklist);

    let main_js_result = get_main_js(submission_path);
    let main_js_file = &main_js_result.extra_data;
    checklists_map.insert(MAIN_JS_EXISTS, main_js_result.checklist);

    if checklists_map.get(PACKAGE_JSON_EXISTS).unwrap().status {
        let project_path = package_json_result
            .extra_data
            .expect("package.json not found");

        run_npm_install(project_path.parent().unwrap());

        if main_js_file.is_some() {
            run_main_js(&main_js_file.clone().unwrap())
        }

        checklists_map.insert(SERVE_IN_PORT_5000, is_server_up().checklist);

        let html_content_result = get_html_content().await;
        checklists_map.insert(ROOT_SHOWING_HTML, html_content_result.checklist);

        if checklists_map[ROOT_SHOWING_HTML].status {
            let h1_element_result = check_h1_element_with_student_id(
                html_content_result.extra_data.unwrap().as_str(),
                student_id,
            );

            checklists_map.insert(
                HTML_CONTAIN_H_1_ELEMENT_WITH_STUDENT_ID,
                h1_element_result.checklist,
            );
        }

        stop_server();
    }

    if checklists_map.get(MAIN_JS_EXISTS).unwrap().status {
        let student_id_result =
            is_main_js_have_student_id_comment(main_js_file.clone().unwrap(), student_id);
        checklists_map.insert(MAIN_JS_HAVE_STUDENT_ID_COMMENT, student_id_result.checklist);
    } else {
        checklists_map
            .entry(MAIN_JS_EXISTS)
            .and_modify(|checklist| {
                checklist.message = "main.js not found".to_string();
            });
    }

    checklists.completed_checklists_key = checklists.get_completed_checklist_keys();

    checklists
}

fn get_package_json(submission_path: &Path) -> ChecklistResult<PathBuf> {
    let mut checklist = Checklist::default();
    let package_json = find_file(submission_path, "package.json");

    if package_json.is_none() {
        checklist.message = "package.json not found".to_string();
        return ChecklistResult {
            checklist,
            extra_data: None,
        };
    }

    checklist.status = true;
    ChecklistResult {
        checklist,
        extra_data: Some(package_json.unwrap()),
    }
}

fn get_main_js(submission_path: &Path) -> ChecklistResult<PathBuf> {
    let mut checklist = Checklist::default();
    let main_js = find_file(submission_path, "main.js");

    if main_js.is_none() {
        checklist.message = "main.js not found".to_string();
        return ChecklistResult {
            checklist,
            extra_data: None,
        };
    }

    checklist.status = true;
    ChecklistResult {
        checklist,
        extra_data: main_js,
    }
}

fn is_main_js_have_student_id_comment(main_js: PathBuf, student_id: u32) -> ChecklistResult<bool> {
    let mut checklist = Checklist::default();

    let main_js_content = read_file(main_js.as_path());
    let pattern = format!("(?://.*?|/\\*.*?\\*/).*?{}.*?", student_id);
    let regex = Regex::new(&pattern).expect("Invalid regex pattern");

    return match regex.is_match(main_js_content.unwrap().as_str()) {
        true => {
            checklist.status = true;
            ChecklistResult {
                checklist,
                extra_data: None,
            }
        }
        false => {
            checklist.message = "student id not found".to_string();
            ChecklistResult {
                checklist,
                extra_data: None,
            }
        }
    };
}

fn is_server_up() -> ChecklistResult<bool> {
    let mut checklist = Checklist::default();
    sleep(Duration::from_secs(2));

    let addr = "127.0.0.1:5000"
        .parse::<SocketAddr>()
        .expect("Invalid address");

    match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
        Ok(_) => {
            checklist.status = true;
            ChecklistResult {
                checklist,
                extra_data: Some(true),
            }
        }
        Err(err) => {
            eprintln!("Failed to connect to {}: {}", addr, err);
            checklist.message = "Port 5000 is not running".to_string();
            ChecklistResult {
                checklist,
                extra_data: None,
            }
        }
    }
}

async fn get_html_content() -> ChecklistResult<String> {
    let mut checklist = Checklist::default();
    let response = reqwest::get("http://localhost:5000").await;

    return match response {
        Ok(response) => {
            let content_type = response
                .headers()
                .get("Content-Type")
                .unwrap()
                .to_str()
                .expect("Failed to load Content-Type");

            if !content_type.contains("html") {
                checklist.message =
                    format!("Response is not HTML, but {}", content_type).to_string();
                return ChecklistResult {
                    checklist,
                    extra_data: None,
                };
            }

            checklist.status = true;
            ChecklistResult {
                checklist,
                extra_data: Some(response.text().await.unwrap()),
            }
        }
        Err(err) => {
            checklist.message = format!("Failed to get HTML content: {}", err);
            ChecklistResult {
                checklist,
                extra_data: None,
            }
        }
    };
}

fn check_h1_element_with_student_id(html_content: &str, student_id: u32) -> ChecklistResult<bool> {
    let mut checklist = Checklist::default();
    let pattern = format!("<h1>{}</h1>", student_id);
    let regex = Regex::new(&pattern).expect("Invalid regex pattern");

    match regex.is_match(html_content) {
        true => {
            checklist.status = true;
            ChecklistResult {
                checklist,
                extra_data: None,
            }
        }
        false => {
            checklist.message = "h1 element with student id not found".to_string();
            ChecklistResult {
                checklist,
                extra_data: None,
            }
        }
    }
}
