use std::path::PathBuf;
use crate::config::checklist::Checklists;

pub fn check_submission(submission_path: &PathBuf) -> Checklists {
    let checklists = Checklists::initialize();

    checklists
}