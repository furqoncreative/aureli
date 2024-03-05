use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};

use aureli::checklist_key::CHECKLIST_KEYS_COUNT;
use aureli::entities::{AutoReviewConfig, Checklists, Report};

pub fn generate_report(
    report_path: &PathBuf,
    checklists: Checklists,
    auto_review_config: AutoReviewConfig,
) {
    let is_submission_approved = is_submission_approved(&checklists);

    let review_message = generate_review_message(
        is_submission_approved,
        auto_review_config.submitter_name,
        &checklists.get_rejected_checklist_messages(),
    );

    let report = Report {
        submission_id: auto_review_config.id,
        rating: if is_submission_approved { 5 } else { 0 },
        checklist_keys: checklists.completed_checklists_key,
        message: review_message,
        is_passed: is_submission_approved,
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

fn generate_review_message(
    is_submission_approved: bool,
    username: String,
    rejected_checklist_messages: &[String],
) -> String {
    if is_submission_approved {
        return format!(
            "Selamat <b>{}</b>! Kamu telah memenuhi semua kriteria dan lulus dari submission ini",
            username
        );
    }

    format!(
        "Mohon maaf <b>{}</b>! Kamu belum memenuhi semua kriteria dan tidak lulus dari submission ini. \
    Berikut adalah kriteria yang belum terpenuhi: \n <ul>{}</ul> \
    Silakan diperbaiki, ya. Semangat!",
        username,
        rejected_checklist_messages.join("\n")
    )
}

fn is_submission_approved(checklists: &Checklists) -> bool {
    checklists.completed_checklists_key.len() == CHECKLIST_KEYS_COUNT
}
