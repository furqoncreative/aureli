# Aureli

Aureli (Auto Review CLI) - A Simple CLI App for Auto Review Submission.

## Requirement 
- Linux
- NodeJS >= v14
- Rust [Install via [rustup.rs](https://rustup.rs/)] (Optional)

## Usage
To run aureli, run the following command in your terminal:

1. Clone the repository:

```bash
git clone https://github.com/furqoncreative/aureli
```
2. Change directory to the project folder:
```bash
cd aureli
```
3. Run the CLI :

#### With Rust (Cargo)

```bash
cargo run -- -s <SUBMISSION PATH> -r <REPORT PATH>
```
#### Without Rust
```bash
bin/aureli -s <SUBMISSION PATH> -r <REPORT PATH>
```

#### Options
```
Options:
  -s, --submission-path <SUBMISSION PATH>  Sets the submission path (required)
  -r, --report-path <REPORT PATH>          Sets the report path (required)
  -h, --help                               Print help
  -V, --version        
```

## Example

```bash
bin/aureli -s submissions/submission-1 -r report/submission-1
```
### Output (report.json)
```json
{
  "submission_id": 1,
  "rating": 5,
  "checklist_keys": [
    "MAIN_JS_EXISTS",
    "ROOT_SHOWING_HTML",
    "MAIN_JS_HAVE_STUDENT_ID_COMMENT",
    "SERVE_IN_PORT_5000",
    "HTML_CONTAIN_H_1_ELEMENT_WITH_STUDENT_ID",
    "PACKAGE_JSON_EXISTS"
  ],
  "message": "Selamat <b>Ujang</b>! Kamu telah memenuhi semua kriteria dan lulus dari submission ini",
  "is_passed": true
}
```
 