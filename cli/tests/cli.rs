use rusqlite::Connection;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

fn sentinel(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sentinel"))
        .args(args)
        .output()
        .expect("run sentinel")
}

fn firefox_fixture(profile: &Path, with_visits: bool) -> PathBuf {
    fs::create_dir_all(profile).unwrap();
    let database = profile.join("places.sqlite");
    let connection = Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE moz_places (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL,
                title TEXT,
                visit_count INTEGER
            );
            CREATE TABLE moz_historyvisits (
                id INTEGER PRIMARY KEY,
                place_id INTEGER NOT NULL,
                visit_date INTEGER
            );",
        )
        .unwrap();
    if with_visits {
        connection.execute("INSERT INTO moz_places VALUES (1, 'https://example.test/one', 'One, with comma', 2)", []).unwrap();
        connection
            .execute(
                "INSERT INTO moz_places VALUES (2, 'https://example.test/two', NULL, 1)",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO moz_historyvisits VALUES (1, 1, 1700000000000000)",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO moz_historyvisits VALUES (2, 1, 1700000001000000)",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO moz_historyvisits VALUES (3, 2, 1700000002000000)",
                [],
            )
            .unwrap();
    }
    database
}

fn chromium_fixture(profile: &Path) -> PathBuf {
    fs::create_dir_all(profile).unwrap();
    let database = profile.join("History");
    let connection = Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE urls (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL,
                title TEXT,
                visit_count INTEGER
            );
            CREATE TABLE visits (
                id INTEGER PRIMARY KEY,
                url INTEGER NOT NULL,
                visit_time INTEGER
            );
            INSERT INTO urls VALUES (1, 'https://chromium.test/', 'Chromium fixture', 2);
            INSERT INTO visits VALUES (1, 1, 13344473600000000);
            INSERT INTO visits VALUES (2, 1, 13344473601000000);",
        )
        .unwrap();
    database
}

#[test]
fn exports_and_verifies_firefox_json_and_csv() {
    let temp = TempDir::new().unwrap();
    let profile = temp.path().join("fixture.default-release");
    firefox_fixture(&profile, true);
    let output = temp.path().join("archive");

    let export = sentinel(&[
        "--json",
        "export",
        "--profile",
        profile.to_str().unwrap(),
        "--output",
        output.to_str().unwrap(),
        "--format",
        "both",
    ]);
    assert!(
        export.status.success(),
        "{}",
        String::from_utf8_lossy(&export.stderr)
    );
    let result: Value = serde_json::from_slice(&export.stdout).unwrap();
    assert_eq!(result["record_count"], 3);

    let profile_output = output.join("firefox-fixture-default-release");
    assert!(profile_output.join("history.json").is_file());
    assert!(profile_output.join("history.csv").is_file());
    assert!(profile_output.join("report.json").is_file());

    let verify = sentinel(&["--json", "verify", output.to_str().unwrap()]);
    assert!(
        verify.status.success(),
        "{}",
        String::from_utf8_lossy(&verify.stderr)
    );
    let checked: Value = serde_json::from_slice(&verify.stdout).unwrap();
    assert_eq!(checked["status"], "verified");
    assert_eq!(checked["artifacts_checked"], 2);
}

#[test]
fn exports_chromium_and_detects_tampering() {
    let temp = TempDir::new().unwrap();
    let profile = temp.path().join("Default");
    chromium_fixture(&profile);
    let output = temp.path().join("archive");
    let export = sentinel(&[
        "export",
        "--profile",
        profile.to_str().unwrap(),
        "--output",
        output.to_str().unwrap(),
        "--format",
        "json",
    ]);
    assert!(
        export.status.success(),
        "{}",
        String::from_utf8_lossy(&export.stderr)
    );

    let artifact = output.join("chromium-default/history.json");
    fs::write(&artifact, "[]\n").unwrap();
    let verify = sentinel(&["verify", output.to_str().unwrap()]);
    assert_eq!(verify.status.code(), Some(13));
    assert!(String::from_utf8_lossy(&verify.stderr).contains("changed after export"));
}

#[test]
fn refuses_empty_and_malformed_databases_without_leaving_an_archive() {
    let temp = TempDir::new().unwrap();
    let empty_profile = temp.path().join("empty.default");
    firefox_fixture(&empty_profile, false);
    let empty_output = temp.path().join("empty-archive");
    let empty = sentinel(&[
        "export",
        "--profile",
        empty_profile.to_str().unwrap(),
        "--output",
        empty_output.to_str().unwrap(),
    ]);
    assert_eq!(empty.status.code(), Some(10));
    assert!(String::from_utf8_lossy(&empty.stderr).contains("no history visits"));
    assert!(!empty_output.exists());

    let malformed_profile = temp.path().join("malformed");
    fs::create_dir_all(&malformed_profile).unwrap();
    Connection::open(malformed_profile.join("History")).unwrap();
    let malformed_output = temp.path().join("malformed-archive");
    let malformed = sentinel(&[
        "export",
        "--profile",
        malformed_profile.to_str().unwrap(),
        "--output",
        malformed_output.to_str().unwrap(),
    ]);
    assert_eq!(malformed.status.code(), Some(12));
    assert!(String::from_utf8_lossy(&malformed.stderr).contains("schema"));
    assert!(!malformed_output.exists());
}

#[test]
fn scan_reports_detected_profile_and_empty_state() {
    let temp = TempDir::new().unwrap();
    let home = temp.path().join("home");
    firefox_fixture(&home.join(".mozilla/firefox/test.default"), true);
    let found = sentinel(&["--json", "scan", "--home", home.to_str().unwrap()]);
    assert!(found.status.success());
    let result: Value = serde_json::from_slice(&found.stdout).unwrap();
    assert_eq!(result["profile_count"], 1);

    let empty_home = temp.path().join("empty-home");
    fs::create_dir(&empty_home).unwrap();
    let empty = sentinel(&["--json", "scan", "--home", empty_home.to_str().unwrap()]);
    assert!(empty.status.success());
    let result: Value = serde_json::from_slice(&empty.stdout).unwrap();
    assert_eq!(result["status"], "empty");
}
