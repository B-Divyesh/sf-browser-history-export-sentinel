use chrono::{DateTime, SecondsFormat, TimeZone, Utc};
use clap::{Parser, Subcommand, ValueEnum};
use rusqlite::{Connection, OpenFlags, params};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::ffi::OsStr;
use std::fmt::{self, Display};
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA_VERSION: u8 = 1;

#[derive(Parser, Debug)]
#[command(
    name = "sentinel",
    version,
    about = "Export browser history locally, then prove the export did not change.",
    long_about = "History Export Sentinel discovers Firefox and Chromium-family profiles, copies their SQLite databases to a private temporary workspace, exports normalized records, and writes hashes for later verification. It never uploads history or modifies a live browser profile.",
    after_help = "EXIT CODES:\n  0 success\n  10 no profile/history\n  11 database unreadable or changing\n  12 unsupported/corrupt database\n  13 verification mismatch\n\nTry `sentinel scan`, then `sentinel export --output ./history-archive`."
)]
struct Cli {
    /// Emit the command result as JSON for scripts.
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run a complete export and verification with bundled sample data.
    Demo {
        /// New directory for the sample export. Defaults to a unique temporary path.
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Find supported browser profiles without opening their databases.
    Scan {
        /// Treat this directory as the user home (useful for mounted backups).
        #[arg(long)]
        home: Option<PathBuf>,
    },
    /// Copy and export one profile, or every detected profile.
    Export {
        /// Profile directory or its History/places.sqlite database.
        #[arg(long)]
        profile: Option<PathBuf>,
        /// New directory that will contain profile exports and reports.
        #[arg(short, long, default_value = "history-sentinel-export")]
        output: PathBuf,
        /// Artifact format to produce.
        #[arg(long, value_enum, default_value_t = ExportFormat::Both)]
        format: ExportFormat,
        /// Treat this directory as the user home when auto-discovering.
        #[arg(long)]
        home: Option<PathBuf>,
    },
    /// Recompute hashes, counts, and date bounds from an export directory.
    Verify {
        /// Export root or a profile directory containing report.json.
        path: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum ExportFormat {
    Json,
    Csv,
    Both,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
enum Browser {
    Firefox,
    Chrome,
    Chromium,
    Edge,
    Brave,
    Vivaldi,
}

impl Browser {
    fn family(self) -> &'static str {
        match self {
            Self::Firefox => "firefox",
            _ => "chromium",
        }
    }
}

impl Display for Browser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Firefox => "firefox",
            Self::Chrome => "chrome",
            Self::Chromium => "chromium",
            Self::Edge => "edge",
            Self::Brave => "brave",
            Self::Vivaldi => "vivaldi",
        };
        f.write_str(text)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct Profile {
    browser: Browser,
    name: String,
    path: PathBuf,
    database: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HistoryRecord {
    url: String,
    title: String,
    visited_at: String,
    visit_count: i64,
    browser: String,
    profile: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Artifact {
    path: String,
    bytes: u64,
    sha256: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ExportReport {
    schema_version: u8,
    status: String,
    created_at: String,
    browser: String,
    browser_family: String,
    profile: String,
    source_database: String,
    source_database_sha256: String,
    record_count: usize,
    earliest_visit: String,
    latest_visit: String,
    artifacts: Vec<Artifact>,
}

#[derive(Debug, Serialize)]
struct ScanResult {
    status: &'static str,
    profile_count: usize,
    profiles: Vec<Profile>,
}

#[derive(Debug, Serialize)]
struct ExportResult {
    status: &'static str,
    output: PathBuf,
    profile_count: usize,
    record_count: usize,
    reports: Vec<PathBuf>,
}

#[derive(Debug, Serialize)]
struct VerifyResult {
    status: &'static str,
    reports_checked: usize,
    artifacts_checked: usize,
    record_count: usize,
}

#[derive(Debug, Serialize)]
struct DemoResult {
    status: &'static str,
    output: PathBuf,
    profile_count: usize,
    record_count: usize,
    reports_checked: usize,
    artifacts_checked: usize,
}

#[derive(Debug, Deserialize)]
struct SampleData {
    firefox: Vec<SampleVisit>,
    chromium: Vec<SampleVisit>,
}

#[derive(Debug, Deserialize)]
struct SampleVisit {
    url: String,
    title: String,
    timestamp: i64,
}

#[derive(Debug)]
struct AppError {
    kind: ErrorKind,
    message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ErrorKind {
    General,
    NoData,
    Access,
    Schema,
    Verify,
}

impl AppError {
    fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    fn code(&self) -> u8 {
        match self.kind {
            ErrorKind::General => 1,
            ErrorKind::NoData => 10,
            ErrorKind::Access => 11,
            ErrorKind::Schema => 12,
            ErrorKind::Verify => 13,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for AppError {}

#[derive(Clone, Copy)]
enum Platform {
    Linux,
    Macos,
    Windows,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("sentinel: {error}");
            ExitCode::from(error.code())
        }
    }
}

fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Command::Demo { output } => run_demo(output, cli.json),
        Command::Scan { home } => {
            let home = resolve_home(home)?;
            let profiles = discover_profiles(&home, current_platform())?;
            let result = ScanResult {
                status: if profiles.is_empty() { "empty" } else { "ok" },
                profile_count: profiles.len(),
                profiles,
            };
            if cli.json {
                print_json(&result)?;
            } else if result.profiles.is_empty() {
                println!(
                    "No supported browser profiles found under {}.",
                    home.display()
                );
                println!("Try --home with a mounted user directory, or pass --profile to export.");
            } else {
                println!("Found {} browser profile(s):", result.profile_count);
                for profile in result.profiles {
                    println!(
                        "  {:<9} {:<24} {}",
                        profile.browser,
                        profile.name,
                        profile.path.display()
                    );
                }
            }
            Ok(())
        }
        Command::Export {
            profile,
            output,
            format,
            home,
        } => {
            let profiles = if let Some(path) = profile {
                vec![profile_from_path(&path)?]
            } else {
                let home = resolve_home(home)?;
                discover_profiles(&home, current_platform())?
            };
            if profiles.is_empty() {
                return Err(AppError::new(
                    ErrorKind::NoData,
                    "no supported browser profiles found; run `sentinel scan` or pass `--profile PATH`",
                ));
            }
            let result = export_profiles(&profiles, &output, format)?;
            if cli.json {
                print_json(&result)?;
            } else {
                println!("EXPORT COMPLETE");
                println!("Profiles: {}", result.profile_count);
                println!("Records:  {}", result.record_count);
                println!("Output:   {}", result.output.display());
                println!("Next: sentinel verify {}", shell_display(&result.output));
            }
            Ok(())
        }
        Command::Verify { path } => {
            let result = verify_exports(&path)?;
            if cli.json {
                print_json(&result)?;
            } else {
                println!("VERIFIED");
                println!("Reports:   {}", result.reports_checked);
                println!("Artifacts: {}", result.artifacts_checked);
                println!("Records:   {}", result.record_count);
                println!("Every recorded hash, count, and date bound matches.");
            }
            Ok(())
        }
    }
}

fn run_demo(output: Option<PathBuf>, json: bool) -> Result<(), AppError> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let sample_root =
        env::temp_dir().join(format!("sentinel-sample-{}-{nonce}", std::process::id()));
    let output = output.unwrap_or_else(|| {
        env::temp_dir().join(format!(
            "sentinel-demo-export-{}-{nonce}",
            std::process::id()
        ))
    });
    let sample = SampleWorkspace::create(&sample_root)?;
    let profiles = sample.profiles()?;
    let exported = export_profiles(&profiles, &output, ExportFormat::Both)?;
    let verified = verify_exports(&output)?;
    let result = DemoResult {
        status: "verified",
        output: exported.output,
        profile_count: exported.profile_count,
        record_count: exported.record_count,
        reports_checked: verified.reports_checked,
        artifacts_checked: verified.artifacts_checked,
    };
    if json {
        print_json(&result)?;
    } else {
        println!("DEMO EXPORT VERIFIED");
        println!("Profiles: {} (Firefox and Chromium)", result.profile_count);
        println!("Records:  {}", result.record_count);
        println!("Files:    {}", result.artifacts_checked);
        println!("Output:   {}", result.output.display());
        println!("Sample data was isolated; no installed browser profile was read.");
    }
    Ok(())
}

struct SampleWorkspace {
    root: PathBuf,
}

impl SampleWorkspace {
    fn create(root: &Path) -> Result<Self, AppError> {
        fs::create_dir(root).map_err(general_io("create sample workspace"))?;
        set_private_directory(root)?;
        let data: SampleData = serde_json::from_str(include_str!(
            "../examples/sample-history.json"
        ))
        .map_err(|error| {
            AppError::new(
                ErrorKind::Schema,
                format!("bundled sample data is invalid: {error}"),
            )
        })?;

        let firefox = root.join("Firefox/Archive research.default");
        fs::create_dir_all(&firefox).map_err(general_io("create Firefox sample"))?;
        let firefox_db = Connection::open(firefox.join("places.sqlite")).map_err(|error| {
            AppError::new(
                ErrorKind::Schema,
                format!("cannot create Firefox sample: {error}"),
            )
        })?;
        firefox_db.execute_batch(
            "CREATE TABLE moz_places (id INTEGER PRIMARY KEY, url TEXT NOT NULL, title TEXT, visit_count INTEGER);
             CREATE TABLE moz_historyvisits (id INTEGER PRIMARY KEY, place_id INTEGER NOT NULL, visit_date INTEGER);",
        ).map_err(|error| AppError::new(ErrorKind::Schema, format!("cannot prepare Firefox sample: {error}")))?;
        for (index, visit) in data.firefox.iter().enumerate() {
            let id = index as i64 + 1;
            firefox_db
                .execute(
                    "INSERT INTO moz_places (id, url, title, visit_count) VALUES (?1, ?2, ?3, 1)",
                    params![id, visit.url, visit.title],
                )
                .map_err(|error| {
                    AppError::new(
                        ErrorKind::Schema,
                        format!("cannot write Firefox sample: {error}"),
                    )
                })?;
            firefox_db
                .execute(
                    "INSERT INTO moz_historyvisits (id, place_id, visit_date) VALUES (?1, ?1, ?2)",
                    params![id, visit.timestamp],
                )
                .map_err(|error| {
                    AppError::new(
                        ErrorKind::Schema,
                        format!("cannot write Firefox sample visit: {error}"),
                    )
                })?;
        }
        drop(firefox_db);

        let chromium = root.join("Chromium/Research profile");
        fs::create_dir_all(&chromium).map_err(general_io("create Chromium sample"))?;
        let chromium_db = Connection::open(chromium.join("History")).map_err(|error| {
            AppError::new(
                ErrorKind::Schema,
                format!("cannot create Chromium sample: {error}"),
            )
        })?;
        chromium_db.execute_batch(
            "CREATE TABLE urls (id INTEGER PRIMARY KEY, url TEXT NOT NULL, title TEXT, visit_count INTEGER);
             CREATE TABLE visits (id INTEGER PRIMARY KEY, url INTEGER NOT NULL, visit_time INTEGER);",
        ).map_err(|error| AppError::new(ErrorKind::Schema, format!("cannot prepare Chromium sample: {error}")))?;
        for (index, visit) in data.chromium.iter().enumerate() {
            let id = index as i64 + 1;
            chromium_db
                .execute(
                    "INSERT INTO urls (id, url, title, visit_count) VALUES (?1, ?2, ?3, 1)",
                    params![id, visit.url, visit.title],
                )
                .map_err(|error| {
                    AppError::new(
                        ErrorKind::Schema,
                        format!("cannot write Chromium sample: {error}"),
                    )
                })?;
            chromium_db
                .execute(
                    "INSERT INTO visits (id, url, visit_time) VALUES (?1, ?1, ?2)",
                    params![id, visit.timestamp],
                )
                .map_err(|error| {
                    AppError::new(
                        ErrorKind::Schema,
                        format!("cannot write Chromium sample visit: {error}"),
                    )
                })?;
        }
        drop(chromium_db);
        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    fn profiles(&self) -> Result<Vec<Profile>, AppError> {
        Ok(vec![
            profile_from_path(&self.root.join("Firefox/Archive research.default"))?,
            profile_from_path(&self.root.join("Chromium/Research profile"))?,
        ])
    }
}

impl Drop for SampleWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn set_private_directory(path: &Path) -> Result<(), AppError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(general_io("set private workspace permissions"))?;
    }
    Ok(())
}

fn print_json(value: &impl Serialize) -> Result<(), AppError> {
    serde_json::to_writer_pretty(io::stdout().lock(), value).map_err(|e| {
        AppError::new(
            ErrorKind::General,
            format!("could not write JSON output: {e}"),
        )
    })?;
    println!();
    Ok(())
}

fn resolve_home(home: Option<PathBuf>) -> Result<PathBuf, AppError> {
    home.or_else(|| env::var_os("HOME").map(PathBuf::from))
        .or_else(|| env::var_os("USERPROFILE").map(PathBuf::from))
        .ok_or_else(|| {
            AppError::new(
                ErrorKind::General,
                "could not determine the user home; pass --home PATH",
            )
        })
}

fn current_platform() -> Platform {
    if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::Macos
    } else {
        Platform::Linux
    }
}

fn discover_profiles(home: &Path, platform: Platform) -> Result<Vec<Profile>, AppError> {
    let (chromium_roots, firefox_roots): (Vec<(Browser, PathBuf)>, Vec<PathBuf>) = match platform {
        Platform::Linux => (
            vec![
                (Browser::Chrome, home.join(".config/google-chrome")),
                (Browser::Chromium, home.join(".config/chromium")),
                (Browser::Edge, home.join(".config/microsoft-edge")),
                (
                    Browser::Brave,
                    home.join(".config/BraveSoftware/Brave-Browser"),
                ),
                (Browser::Vivaldi, home.join(".config/vivaldi")),
            ],
            vec![home.join(".mozilla/firefox")],
        ),
        Platform::Macos => {
            let support = home.join("Library/Application Support");
            (
                vec![
                    (Browser::Chrome, support.join("Google/Chrome")),
                    (Browser::Chromium, support.join("Chromium")),
                    (Browser::Edge, support.join("Microsoft Edge")),
                    (Browser::Brave, support.join("BraveSoftware/Brave-Browser")),
                    (Browser::Vivaldi, support.join("Vivaldi")),
                ],
                vec![support.join("Firefox/Profiles")],
            )
        }
        Platform::Windows => (
            vec![
                (
                    Browser::Chrome,
                    home.join("AppData/Local/Google/Chrome/User Data"),
                ),
                (
                    Browser::Chromium,
                    home.join("AppData/Local/Chromium/User Data"),
                ),
                (
                    Browser::Edge,
                    home.join("AppData/Local/Microsoft/Edge/User Data"),
                ),
                (
                    Browser::Brave,
                    home.join("AppData/Local/BraveSoftware/Brave-Browser/User Data"),
                ),
                (
                    Browser::Vivaldi,
                    home.join("AppData/Local/Vivaldi/User Data"),
                ),
            ],
            vec![home.join("AppData/Roaming/Mozilla/Firefox/Profiles")],
        ),
    };

    let mut profiles = BTreeSet::new();
    for (browser, root) in chromium_roots {
        find_database_profiles(&root, "History", browser, 2, &mut profiles)?;
    }
    for root in firefox_roots {
        find_database_profiles(&root, "places.sqlite", Browser::Firefox, 2, &mut profiles)?;
    }
    Ok(profiles.into_iter().collect())
}

fn find_database_profiles(
    root: &Path,
    filename: &str,
    browser: Browser,
    depth: usize,
    found: &mut BTreeSet<Profile>,
) -> Result<(), AppError> {
    match fs::metadata(root) {
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => return Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(AppError::new(
                ErrorKind::Access,
                format!(
                    "cannot inspect browser profile directory {}: {error}; fix its OS permissions, then retry",
                    root.display()
                ),
            ));
        }
    }
    let direct = root.join(filename);
    if direct.is_file() {
        found.insert(Profile {
            browser,
            name: profile_name(root),
            path: root.to_path_buf(),
            database: direct,
        });
    }
    if depth == 0 {
        return Ok(());
    }
    let entries = fs::read_dir(root).map_err(|error| {
        AppError::new(
            ErrorKind::Access,
            format!(
                "cannot read browser profile directory {}: {error}; fix its OS permissions, then retry",
                root.display()
            ),
        )
    })?;
    for entry in entries {
        let entry = entry.map_err(|error| {
            AppError::new(
                ErrorKind::Access,
                format!(
                    "cannot read an entry in {}: {error}; fix its OS permissions, then retry",
                    root.display()
                ),
            )
        })?;
        let path = entry.path();
        if path.is_dir() {
            find_database_profiles(&path, filename, browser, depth - 1, found)?;
        }
    }
    Ok(())
}

fn profile_name(path: &Path) -> String {
    path.file_name()
        .and_then(OsStr::to_str)
        .filter(|name| !name.is_empty())
        .unwrap_or("profile")
        .to_string()
}

fn profile_from_path(path: &Path) -> Result<Profile, AppError> {
    let canonical = fs::canonicalize(path).map_err(|e| {
        AppError::new(
            ErrorKind::Access,
            format!(
                "cannot access profile {}: {e}; check the path and permissions",
                path.display()
            ),
        )
    })?;
    let (database, root) = if canonical.is_dir() {
        if canonical.join("places.sqlite").is_file() {
            (canonical.join("places.sqlite"), canonical.clone())
        } else if canonical.join("History").is_file() {
            (canonical.join("History"), canonical.clone())
        } else {
            return Err(AppError::new(
                ErrorKind::NoData,
                format!(
                    "{} contains neither places.sqlite nor History",
                    canonical.display()
                ),
            ));
        }
    } else {
        let root = canonical.parent().unwrap_or(Path::new(".")).to_path_buf();
        (canonical, root)
    };
    let filename = database.file_name().and_then(OsStr::to_str).unwrap_or("");
    let browser = match filename {
        "places.sqlite" => Browser::Firefox,
        "History" => Browser::Chromium,
        _ => {
            return Err(AppError::new(
                ErrorKind::Schema,
                "database must be named places.sqlite or History",
            ));
        }
    };
    Ok(Profile {
        browser,
        name: profile_name(&root),
        path: root,
        database,
    })
}

fn export_profiles(
    profiles: &[Profile],
    output: &Path,
    format: ExportFormat,
) -> Result<ExportResult, AppError> {
    if output.exists() {
        return Err(AppError::new(
            ErrorKind::General,
            format!(
                "output {} already exists; choose a new directory to avoid overwriting an archive",
                output.display()
            ),
        ));
    }
    fs::create_dir_all(output).map_err(|e| {
        AppError::new(
            ErrorKind::General,
            format!("cannot create output {}: {e}", output.display()),
        )
    })?;
    let result = (|| {
        let mut reports = Vec::new();
        let mut total_records = 0;
        let mut used_names = BTreeSet::new();
        for profile in profiles {
            let base = safe_export_name(profile);
            let mut export_name = base.clone();
            let mut suffix = 2;
            while !used_names.insert(export_name.clone()) {
                export_name = format!("{base}-{suffix}");
                suffix += 1;
            }
            let profile_output = output.join(export_name);
            fs::create_dir(&profile_output).map_err(|e| {
                AppError::new(
                    ErrorKind::General,
                    format!("cannot create {}: {e}", profile_output.display()),
                )
            })?;
            let report = export_profile(profile, &profile_output, format)?;
            total_records += report.record_count;
            reports.push(profile_output.join("report.json"));
        }
        Ok(ExportResult {
            status: "complete",
            output: output.to_path_buf(),
            profile_count: profiles.len(),
            record_count: total_records,
            reports,
        })
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(output);
    }
    result
}

fn safe_export_name(profile: &Profile) -> String {
    let raw = format!("{}-{}", profile.browser, profile.name).to_lowercase();
    let cleaned: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    let cleaned = cleaned.trim_matches('-');
    if cleaned.is_empty() {
        format!("{}-profile", profile.browser)
    } else {
        cleaned.to_string()
    }
}

struct Snapshot {
    directory: PathBuf,
    database: PathBuf,
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn copy_snapshot(source: &Path) -> Result<Snapshot, AppError> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let directory = env::temp_dir().join(format!("sentinel-{}-{nonce}", std::process::id()));
    fs::create_dir(&directory).map_err(|e| {
        AppError::new(
            ErrorKind::Access,
            format!("cannot create private temporary workspace: {e}"),
        )
    })?;
    set_private_directory(&directory)?;
    let filename = source
        .file_name()
        .ok_or_else(|| AppError::new(ErrorKind::Access, "database path has no filename"))?;
    for attempt in 1..=3 {
        let sources_before = snapshot_signature(source)?;
        let attempt_directory = directory.join(format!("attempt-{attempt}"));
        fs::create_dir(&attempt_directory).map_err(|e| {
            AppError::new(
                ErrorKind::Access,
                format!("cannot prepare temporary snapshot: {e}"),
            )
        })?;
        for (path, _, _) in &sources_before {
            let target = attempt_directory.join(path.file_name().ok_or_else(|| {
                AppError::new(ErrorKind::Access, "database path has no filename")
            })?);
            if let Err(error) = fs::copy(path, target).map_err(|e| access_copy_error(path, e)) {
                let _ = fs::remove_dir_all(&directory);
                return Err(error);
            }
        }
        let sources_after = snapshot_signature(source)?;
        if sources_before == sources_after {
            return Ok(Snapshot {
                directory,
                database: attempt_directory.join(filename),
            });
        }
        let _ = fs::remove_dir_all(&attempt_directory);
    }
    let _ = fs::remove_dir_all(&directory);
    Err(AppError::new(
        ErrorKind::Access,
        format!(
            "{} kept changing while its database set was copied; close the browser and retry",
            source.display()
        ),
    ))
}

fn snapshot_signature(source: &Path) -> Result<Vec<(PathBuf, u64, Option<SystemTime>)>, AppError> {
    let mut files = vec![source.to_path_buf()];
    for suffix in ["-wal", "-shm"] {
        let sidecar = PathBuf::from(format!("{}{}", source.display(), suffix));
        if sidecar.exists() {
            files.push(sidecar);
        }
    }
    files
        .into_iter()
        .map(|path| {
            let metadata = fs::metadata(&path).map_err(|e| access_copy_error(&path, e))?;
            Ok((path, metadata.len(), metadata.modified().ok()))
        })
        .collect()
}

fn access_copy_error(path: &Path, error: io::Error) -> AppError {
    AppError::new(
        ErrorKind::Access,
        format!(
            "cannot read/copy {}: {error}; close the browser or fix OS permissions, then retry",
            path.display()
        ),
    )
}

fn export_profile(
    profile: &Profile,
    output: &Path,
    format: ExportFormat,
) -> Result<ExportReport, AppError> {
    let snapshot = copy_snapshot(&profile.database)?;
    let source_database_sha256 = sha256_file(&snapshot.database)?;
    let records = read_records(&snapshot.database, profile)?;
    if records.is_empty() {
        return Err(AppError::new(
            ErrorKind::NoData,
            format!(
                "{} contains no history visits; no export was written",
                profile.database.display()
            ),
        ));
    }
    let earliest_visit = records
        .first()
        .expect("checked nonempty")
        .visited_at
        .clone();
    let latest_visit = records.last().expect("checked nonempty").visited_at.clone();
    let mut artifacts = Vec::new();
    if matches!(format, ExportFormat::Json | ExportFormat::Both) {
        let path = output.join("history.json");
        write_json_records(&path, &records)?;
        artifacts.push(artifact_for(&path, "history.json")?);
    }
    if matches!(format, ExportFormat::Csv | ExportFormat::Both) {
        let path = output.join("history.csv");
        write_csv_records(&path, &records)?;
        artifacts.push(artifact_for(&path, "history.csv")?);
    }
    let report = ExportReport {
        schema_version: SCHEMA_VERSION,
        status: "complete".into(),
        created_at: now_rfc3339(),
        browser: profile.browser.to_string(),
        browser_family: profile.browser.family().into(),
        profile: profile.name.clone(),
        source_database: profile
            .database
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("database")
            .into(),
        source_database_sha256,
        record_count: records.len(),
        earliest_visit,
        latest_visit,
        artifacts,
    };
    let report_path = output.join("report.json");
    let writer = BufWriter::new(File::create(&report_path).map_err(general_io("create report"))?);
    serde_json::to_writer_pretty(writer, &report)
        .map_err(|e| AppError::new(ErrorKind::General, format!("cannot write report: {e}")))?;
    Ok(report)
}

fn read_records(database: &Path, profile: &Profile) -> Result<Vec<HistoryRecord>, AppError> {
    let connection = Connection::open_with_flags(
        database,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| {
        AppError::new(
            ErrorKind::Schema,
            format!(
                "cannot open copied {} database: {e}",
                profile.browser.family()
            ),
        )
    })?;
    if profile.browser == Browser::Firefox {
        read_firefox(&connection, profile)
    } else {
        read_chromium(&connection, profile)
    }
}

fn read_firefox(
    connection: &Connection,
    profile: &Profile,
) -> Result<Vec<HistoryRecord>, AppError> {
    let sql = "SELECT p.url, COALESCE(p.title, ''), v.visit_date, COALESCE(p.visit_count, 0) \
               FROM moz_historyvisits v JOIN moz_places p ON p.id = v.place_id \
               WHERE v.visit_date IS NOT NULL ORDER BY v.visit_date, v.id";
    query_records(connection, sql, profile, firefox_timestamp)
}

fn read_chromium(
    connection: &Connection,
    profile: &Profile,
) -> Result<Vec<HistoryRecord>, AppError> {
    let sql = "SELECT u.url, COALESCE(u.title, ''), v.visit_time, COALESCE(u.visit_count, 0) \
               FROM visits v JOIN urls u ON u.id = v.url \
               WHERE v.visit_time IS NOT NULL ORDER BY v.visit_time, v.id";
    query_records(connection, sql, profile, chromium_timestamp)
}

fn query_records(
    connection: &Connection,
    sql: &str,
    profile: &Profile,
    timestamp: fn(i64) -> Result<String, AppError>,
) -> Result<Vec<HistoryRecord>, AppError> {
    let mut statement = connection.prepare(sql).map_err(|e| {
        AppError::new(
            ErrorKind::Schema,
            format!(
                "{} history schema is unsupported or corrupt: {e}",
                profile.browser.family()
            ),
        )
    })?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .map_err(|e| {
            AppError::new(
                ErrorKind::Schema,
                format!("cannot query copied history database: {e}"),
            )
        })?;
    let mut records = Vec::new();
    for row in rows {
        let (url, title, raw_time, visit_count) =
            row.map_err(|e| AppError::new(ErrorKind::Schema, format!("invalid history row: {e}")))?;
        records.push(HistoryRecord {
            url,
            title,
            visited_at: timestamp(raw_time)?,
            visit_count,
            browser: profile.browser.to_string(),
            profile: profile.name.clone(),
        });
    }
    Ok(records)
}

fn firefox_timestamp(micros: i64) -> Result<String, AppError> {
    timestamp_from_micros(micros)
}

fn chromium_timestamp(micros_since_1601: i64) -> Result<String, AppError> {
    const EPOCH_DELTA_MICROS: i64 = 11_644_473_600_000_000;
    timestamp_from_micros(
        micros_since_1601
            .checked_sub(EPOCH_DELTA_MICROS)
            .ok_or_else(|| {
                AppError::new(
                    ErrorKind::Schema,
                    "Chromium visit timestamp is outside the supported range",
                )
            })?,
    )
}

fn timestamp_from_micros(micros: i64) -> Result<String, AppError> {
    let seconds = micros.div_euclid(1_000_000);
    let nanos = (micros.rem_euclid(1_000_000) as u32) * 1_000;
    let time: DateTime<Utc> = Utc.timestamp_opt(seconds, nanos).single().ok_or_else(|| {
        AppError::new(
            ErrorKind::Schema,
            format!("visit timestamp {micros} is outside the supported range"),
        )
    })?;
    Ok(time.to_rfc3339_opts(SecondsFormat::Micros, true))
}

fn write_json_records(path: &Path, records: &[HistoryRecord]) -> Result<(), AppError> {
    let writer = BufWriter::new(File::create(path).map_err(general_io("create JSON export"))?);
    serde_json::to_writer_pretty(writer, records)
        .map_err(|e| AppError::new(ErrorKind::General, format!("cannot write JSON export: {e}")))
}

fn write_csv_records(path: &Path, records: &[HistoryRecord]) -> Result<(), AppError> {
    let mut writer = csv::Writer::from_path(path)
        .map_err(|e| AppError::new(ErrorKind::General, format!("cannot create CSV export: {e}")))?;
    for record in records {
        writer.serialize(record).map_err(|e| {
            AppError::new(ErrorKind::General, format!("cannot write CSV export: {e}"))
        })?;
    }
    writer
        .flush()
        .map_err(|e| AppError::new(ErrorKind::General, format!("cannot finish CSV export: {e}")))
}

fn artifact_for(path: &Path, relative: &str) -> Result<Artifact, AppError> {
    Ok(Artifact {
        path: relative.into(),
        bytes: fs::metadata(path)
            .map_err(general_io("inspect artifact"))?
            .len(),
        sha256: sha256_file(path)?,
    })
}

fn sha256_file(path: &Path) -> Result<String, AppError> {
    let mut reader = BufReader::new(File::open(path).map_err(general_io("open file for hashing"))?);
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = reader.read(&mut buffer).map_err(general_io("hash file"))?;
        if read == 0 {
            break;
        }
        hash.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn verify_exports(root: &Path) -> Result<VerifyResult, AppError> {
    if !root.exists() {
        return Err(AppError::new(
            ErrorKind::Verify,
            format!("{} does not exist", root.display()),
        ));
    }
    let mut reports = Vec::new();
    collect_reports(root, 0, &mut reports)?;
    if reports.is_empty() {
        return Err(AppError::new(
            ErrorKind::Verify,
            format!("no report.json found under {}", root.display()),
        ));
    }
    let mut artifact_count = 0;
    let mut record_count = 0;
    for report_path in &reports {
        let file = File::open(report_path).map_err(verify_io("open report"))?;
        let report: ExportReport = serde_json::from_reader(BufReader::new(file)).map_err(|e| {
            AppError::new(
                ErrorKind::Verify,
                format!("invalid {}: {e}", report_path.display()),
            )
        })?;
        if report.schema_version != SCHEMA_VERSION
            || report.status != "complete"
            || report.record_count == 0
        {
            return Err(AppError::new(
                ErrorKind::Verify,
                format!(
                    "{} has an unsupported or incomplete report",
                    report_path.display()
                ),
            ));
        }
        let base = report_path.parent().unwrap_or(root);
        let mut bounds_checked = false;
        for artifact in &report.artifacts {
            let relative = safe_relative(&artifact.path)?;
            let path = base.join(relative);
            let metadata = fs::metadata(&path).map_err(verify_io("inspect artifact"))?;
            if metadata.len() != artifact.bytes || sha256_file_verify(&path)? != artifact.sha256 {
                return Err(AppError::new(
                    ErrorKind::Verify,
                    format!(
                        "{} changed after export (size or SHA-256 mismatch)",
                        path.display()
                    ),
                ));
            }
            let (count, earliest, latest) = if artifact.path.ends_with(".json") {
                inspect_json_export(&path)?
            } else if artifact.path.ends_with(".csv") {
                inspect_csv_export(&path)?
            } else {
                return Err(AppError::new(
                    ErrorKind::Verify,
                    format!("unsupported artifact type: {}", artifact.path),
                ));
            };
            if count != report.record_count
                || earliest != report.earliest_visit
                || latest != report.latest_visit
            {
                return Err(AppError::new(
                    ErrorKind::Verify,
                    format!(
                        "{} does not match the report count/date bounds",
                        path.display()
                    ),
                ));
            }
            bounds_checked = true;
            artifact_count += 1;
        }
        if !bounds_checked {
            return Err(AppError::new(
                ErrorKind::Verify,
                format!("{} records no export artifacts", report_path.display()),
            ));
        }
        record_count += report.record_count;
    }
    Ok(VerifyResult {
        status: "verified",
        reports_checked: reports.len(),
        artifacts_checked: artifact_count,
        record_count,
    })
}

fn collect_reports(path: &Path, depth: usize, found: &mut Vec<PathBuf>) -> Result<(), AppError> {
    if path.is_file() {
        if path.file_name() == Some(OsStr::new("report.json")) {
            found.push(path.to_path_buf());
        }
        return Ok(());
    }
    if depth > 3 {
        return Ok(());
    }
    for entry in fs::read_dir(path).map_err(verify_io("read export directory"))? {
        let child = entry.map_err(verify_io("read export entry"))?.path();
        if child.is_dir() {
            collect_reports(&child, depth + 1, found)?;
        } else if child.file_name() == Some(OsStr::new("report.json")) {
            found.push(child);
        }
    }
    found.sort();
    Ok(())
}

fn safe_relative(value: &str) -> Result<&Path, AppError> {
    let path = Path::new(value);
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(AppError::new(
            ErrorKind::Verify,
            format!("unsafe artifact path in report: {value}"),
        ));
    }
    Ok(path)
}

fn sha256_file_verify(path: &Path) -> Result<String, AppError> {
    sha256_file(path).map_err(|e| AppError::new(ErrorKind::Verify, e.message))
}

fn inspect_json_export(path: &Path) -> Result<(usize, String, String), AppError> {
    let file = File::open(path).map_err(verify_io("open JSON artifact"))?;
    let records: Vec<HistoryRecord> =
        serde_json::from_reader(BufReader::new(file)).map_err(|e| {
            AppError::new(
                ErrorKind::Verify,
                format!("invalid JSON artifact {}: {e}", path.display()),
            )
        })?;
    record_bounds(&records, path)
}

fn inspect_csv_export(path: &Path) -> Result<(usize, String, String), AppError> {
    let mut reader = csv::Reader::from_path(path).map_err(|e| {
        AppError::new(
            ErrorKind::Verify,
            format!("invalid CSV artifact {}: {e}", path.display()),
        )
    })?;
    let mut records = Vec::new();
    for record in reader.deserialize() {
        records.push(record.map_err(|e| {
            AppError::new(
                ErrorKind::Verify,
                format!("invalid CSV row in {}: {e}", path.display()),
            )
        })?);
    }
    record_bounds(&records, path)
}

fn record_bounds(
    records: &[HistoryRecord],
    path: &Path,
) -> Result<(usize, String, String), AppError> {
    let earliest = records.iter().map(|r| r.visited_at.as_str()).min();
    let latest = records.iter().map(|r| r.visited_at.as_str()).max();
    match (earliest, latest) {
        (Some(first), Some(last)) => Ok((records.len(), first.into(), last.into())),
        _ => Err(AppError::new(
            ErrorKind::Verify,
            format!("{} is empty", path.display()),
        )),
    }
}

fn general_io(action: &'static str) -> impl FnOnce(io::Error) -> AppError {
    move |e| AppError::new(ErrorKind::General, format!("could not {action}: {e}"))
}

fn verify_io(action: &'static str) -> impl FnOnce(io::Error) -> AppError {
    move |e| AppError::new(ErrorKind::Verify, format!("could not {action}: {e}"))
}

fn now_rfc3339() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn shell_display(path: &Path) -> String {
    let value = path.display().to_string();
    if value.contains([' ', '\t', '\n', '\'', '"']) {
        format!("'{value}'")
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn converts_firefox_and_chromium_epochs() {
        assert_eq!(firefox_timestamp(0).unwrap(), "1970-01-01T00:00:00.000000Z");
        assert_eq!(
            chromium_timestamp(11_644_473_600_000_000).unwrap(),
            "1970-01-01T00:00:00.000000Z"
        );
    }

    #[test]
    fn rejects_unsafe_artifact_paths() {
        assert!(safe_relative("history.json").is_ok());
        assert!(safe_relative("../history.json").is_err());
        assert!(safe_relative("/tmp/history.json").is_err());
    }

    #[test]
    fn discovers_layouts_on_all_supported_operating_systems() {
        let temp = TempDir::new().unwrap();
        let cases = [
            (
                Platform::Linux,
                ".mozilla/firefox/a.default/places.sqlite",
                ".config/google-chrome/Default/History",
            ),
            (
                Platform::Macos,
                "Library/Application Support/Firefox/Profiles/a.default/places.sqlite",
                "Library/Application Support/Google/Chrome/Default/History",
            ),
            (
                Platform::Windows,
                "AppData/Roaming/Mozilla/Firefox/Profiles/a.default/places.sqlite",
                "AppData/Local/Google/Chrome/User Data/Default/History",
            ),
        ];
        for (index, (platform, firefox, chromium)) in cases.into_iter().enumerate() {
            let home = temp.path().join(format!("home-{index}"));
            for relative in [firefox, chromium] {
                let file = home.join(relative);
                fs::create_dir_all(file.parent().unwrap()).unwrap();
                File::create(file).unwrap();
            }
            let found = discover_profiles(&home, platform).unwrap();
            assert_eq!(found.len(), 2);
            assert!(found.iter().any(|p| p.browser == Browser::Firefox));
            assert!(found.iter().any(|p| p.browser == Browser::Chrome));
        }
    }

    #[test]
    fn claim_copy_safety() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("History");
        let connection = Connection::open(&source).unwrap();
        connection
            .pragma_update(None, "journal_mode", "WAL")
            .unwrap();
        connection
            .execute_batch(
                "CREATE TABLE urls (id INTEGER PRIMARY KEY, url TEXT, title TEXT, visit_count INTEGER);
                 CREATE TABLE visits (id INTEGER PRIMARY KEY, url INTEGER, visit_time INTEGER);
                 INSERT INTO urls VALUES (1, 'https://copy.example/', 'Copy test', 1);
                 INSERT INTO visits VALUES (1, 1, 13344473600000000);",
            )
            .unwrap();
        let before = fs::read(&source).unwrap();
        let snapshot = copy_snapshot(&source).unwrap();
        assert!(snapshot.database.is_file());
        assert!(snapshot.database.with_file_name("History-wal").is_file());
        assert_eq!(fs::read(&source).unwrap(), before);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&snapshot.directory)
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o700
            );
        }
        let snapshot_directory = snapshot.directory.clone();
        drop(snapshot);
        assert!(!snapshot_directory.exists());
        drop(connection);
    }
}
