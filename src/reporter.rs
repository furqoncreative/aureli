use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};

use aureli::checklist_key::CHECKLIST_KEYS_COUNT;
use aureli::entities::{AutoReviewConfig, Checklists, Report};

pub fn generate_report(
    report_path: &PathBuf,
    checklists: Checklists,
    auto_review_config: AutoReviewConfig,
) {
    let is_passed = if checklists.completed_checklists_key.len() == CHECKLIST_KEYS_COUNT {
        true
    } else {
        false
    };

    let report = Report {
        submission_id: auto_review_config.id,
        rating: if is_passed { 5 } else { 0 },
        checklist_keys: checklists.completed_checklists_key,
        message: "".to_string(),
        is_passed,
    };

    let json_data = serde_json::to_string(&report).unwrap();

    if !Path::new(report_path).exists() {
        match create_dir_all(report_path) {
            Ok(_) => {}
            Err(e) => {
                panic!("Unable to create report directory: {}", e)
            }
        }
    }

    write(report_path.join("report.json"), json_data).expect("Unable to write file");
}
