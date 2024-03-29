pub mod checklist_key {
    pub const PACKAGE_JSON_EXISTS: &str = "PACKAGE_JSON_EXISTS";
    pub const MAIN_JS_EXISTS: &str = "MAIN_JS_EXISTS";
    pub const MAIN_JS_HAVE_STUDENT_ID_COMMENT: &str = "MAIN_JS_HAVE_STUDENT_ID_COMMENT";
    pub const ROOT_SHOWING_HTML: &str = "ROOT_SHOWING_HTML";
    pub const SERVE_IN_PORT_5000: &str = "SERVE_IN_PORT_5000";
    pub const HTML_CONTAIN_H_1_ELEMENT_WITH_STUDENT_ID: &str =
        "HTML_CONTAIN_H_1_ELEMENT_WITH_STUDENT_ID";

    pub const CHECKLIST_KEYS_COUNT: usize = 6;
}

pub mod entities {
    use crate::checklist_key::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    pub struct AutoReviewConfig {
        pub id: u32,
        pub submitter_id: u32,
        pub submitter_name: String,
        pub rating: u8,
        pub course_id: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Report {
        pub submission_id: u32,
        pub rating: u8,
        pub checklist_keys: Vec<String>,
        pub message: String,
        pub is_passed: bool,
    }

    #[derive(Default)]
    pub struct Checklist {
        pub status: bool,
        pub message: String,
    }

    impl Checklist {
        pub fn approve() -> Self {
            Self {
                status: true,
                message: String::from(""),
            }
        }

        pub fn reject(message: String) -> Self {
            Self {
                status: false,
                message,
            }
        }
    }

    pub struct Checklists<'a> {
        pub checklists_map: HashMap<&'a str, Checklist>,
        pub completed_checklists_key: Vec<String>,
    }

    impl Checklists<'_> {
        pub fn initialize() -> Checklists<'static> {
            Checklists {
                checklists_map: HashMap::from([
                    (PACKAGE_JSON_EXISTS, Checklist::default()),
                    (MAIN_JS_EXISTS, Checklist::default()),
                    (MAIN_JS_HAVE_STUDENT_ID_COMMENT, Checklist::reject(
                        String::from("Kriteria 'Komentar ID Anda pada main.js' tidak bisa diperiksa karena file main.js tidak ditemukan.")
                    )),
                    (ROOT_SHOWING_HTML, Checklist::reject(
                        String::from("Kriteria 'Root menampilkan HTML' tidak bisa diperiksa karena package.json tidak bisa jalankan.")
                    )),
                    (SERVE_IN_PORT_5000, Checklist::reject(
                        String::from("Kriteria 'Aplikasi Berjalan di port 5000' tidak bisa diperiksa karena package.json tidak bisa jalankan.")
                    )),
                    (HTML_CONTAIN_H_1_ELEMENT_WITH_STUDENT_ID, Checklist::reject(
                        String::from("Kriteria 'Menampilkan ID pada element H1' tidak bisa diperiksa karena project tiak bisa dijalankan.")
                    )),
                ]),
                completed_checklists_key: vec![],
            }
        }

        pub fn get_completed_checklist_keys(&self) -> Vec<String> {
            self.checklists_map
                .iter()
                .filter(|(_, checklist)| checklist.status)
                .map(|(key, _)| key.to_string())
                .collect()
        }

        pub fn get_rejected_checklist_messages(&self) -> Vec<String> {
            self.checklists_map
                .values()
                .filter(|checklist| !checklist.status)
                .map(|checklist| format!("<li>{}</li>", checklist.message))
                .collect()
        }
    }

    pub struct ChecklistResult<T> {
        pub checklist: Checklist,
        pub extra_data: Option<T>,
    }
}

pub mod cli {
    use std::path::PathBuf;

    use clap::{value_parser, Arg, Command};

    pub fn build() -> Command {
        Command::new("aureli")
            .bin_name("aureli")
            .about("Aureli (Auto Review CLI) - A Simple CLI App for Auto Review Submission")
            .version("0.0.1")
            .arg_required_else_help(true)
            .args([
                Arg::new("submission-path")
                    .short('s')
                    .long("submission-path")
                    .value_name("SUBMISSION PATH")
                    .help("Sets the submission path")
                    .value_parser(value_parser!(PathBuf))
                    .required(true),
                Arg::new("report-path")
                    .short('r')
                    .long("report-path")
                    .value_name("REPORT PATH")
                    .help("Sets the report path")
                    .value_parser(value_parser!(PathBuf))
                    .required(true),
            ])
    }
}
