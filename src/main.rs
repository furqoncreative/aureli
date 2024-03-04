use std::path::PathBuf;

use crate::grader::grade_submission;
use crate::reporter::generate_report;
use crate::utils::get_auto_review_config;
use aureli::cli;
use aureli::entities::AutoReviewConfig;

mod grader;
mod reporter;
mod utils;

#[tokio::main]
async fn main() {
    let flags = cli::build().get_matches();

    let submission_path = flags
        .get_one::<PathBuf>("submission-path")
        .expect("source-path is required");

    let report_path = flags
        .get_one::<PathBuf>("report-path")
        .expect("report-path is required");

    let auto_review_config: AutoReviewConfig = get_auto_review_config(submission_path);

    let checklists = grade_submission(submission_path, auto_review_config.submitter_id).await;

    generate_report(report_path, checklists, auto_review_config);
}
