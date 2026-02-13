use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn cmd() -> Command {
    #[expect(deprecated)]
    Command::cargo_bin("gh-profile-gen").unwrap()
}

#[test]
fn test_cli_no_subcommand_shows_help() {
    cmd()
        .assert()
        .success()
        .stdout(predicate::str::contains("gh-profile-gen"));
}

#[test]
fn test_cli_init_creates_toml() {
    let dir = TempDir::new().unwrap();
    let output: std::path::PathBuf = dir.path().join("profile.toml");

    cmd()
        .args(["init", "-o", output.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    let content: String = std::fs::read_to_string(&output).unwrap();
    assert!(content.contains("[meta]"));
    assert!(content.contains("username"));
}

#[test]
fn test_cli_init_refuses_overwrite() {
    let dir = TempDir::new().unwrap();
    let output: std::path::PathBuf = dir.path().join("profile.toml");
    std::fs::write(&output, "existing content").unwrap();

    cmd()
        .args(["init", "-o", output.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_cli_init_force_overwrite() {
    let dir = TempDir::new().unwrap();
    let output: std::path::PathBuf = dir.path().join("profile.toml");
    std::fs::write(&output, "old content").unwrap();

    cmd()
        .args(["init", "-o", output.to_str().unwrap(), "--force"])
        .assert()
        .success();

    let content: String = std::fs::read_to_string(&output).unwrap();
    assert!(content.contains("[meta]"));
}

#[test]
fn test_cli_render_produces_readme() {
    let dir = TempDir::new().unwrap();
    let output: std::path::PathBuf = dir.path().join("README.md");

    cmd()
        .args([
            "render",
            "tests/fixtures/full.toml",
            "-o",
            output.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Wrote"));

    let content: String = std::fs::read_to_string(&output).unwrap();
    assert!(content.contains("typing-svg"));
    assert!(content.contains("Alice"));
    assert!(content.contains("Rust"));
    assert!(content.contains("GitHub Stats"));
}

#[test]
fn test_cli_render_stdout() {
    cmd()
        .args(["render", "tests/fixtures/full.toml", "--stdout"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("GitHub Stats"));
}

#[test]
fn test_cli_render_missing_file() {
    cmd()
        .args(["render", "nonexistent.toml"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("could not open"));
}

#[test]
fn test_cli_render_minimal_config() {
    cmd()
        .args(["render", "tests/fixtures/minimal.toml", "--stdout"])
        .assert()
        .success();
}

#[test]
fn test_cli_preview() {
    cmd()
        .args(["preview", "tests/fixtures/full.toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn test_full_pipeline() {
    // Render the full fixture and verify key sections are present
    let output = cmd()
        .args(["render", "tests/fixtures/full.toml", "--stdout"])
        .output()
        .unwrap();
    let content: String = String::from_utf8(output.stdout).unwrap();

    // Header
    assert!(content.contains("typing-svg.demolab.com"));
    // About
    assert!(content.contains("Backend Engineer"));
    assert!(content.contains("Acme Corp"));
    // Social
    assert!(content.contains("Twitter"));
    assert!(content.contains("twitter.com/alice"));
    // Skills
    assert!(content.contains("Tech Stack"));
    assert!(content.contains("logo=rust"));
    // Stats
    assert!(content.contains("GitHub Stats"));
    assert!(content.contains("github-readme-stats.vercel.app"));
    assert!(content.contains("streak-stats.demolab.com"));
    // Projects
    assert!(content.contains("Featured Projects"));
    assert!(content.contains("cool-cli"));
    // Blog
    assert!(content.contains("BLOG-POST-LIST"));
    assert!(content.contains("Getting Started with Rust"));
    // Dynamic
    assert!(content.contains("Spotify"));
    assert!(content.contains("WakaTime"));
    // Sponsors
    assert!(content.contains("Support"));
    assert!(content.contains("sponsors/alice"));
    // Extras
    assert!(content.contains("PGP"));
    assert!(content.contains("More about me"));
}

#[test]
fn test_init_then_render() {
    let dir = TempDir::new().unwrap();
    let toml_path: std::path::PathBuf = dir.path().join("profile.toml");
    let readme_path: std::path::PathBuf = dir.path().join("README.md");

    // Init
    cmd()
        .args(["init", "-o", toml_path.to_str().unwrap()])
        .assert()
        .success();

    // Render the generated TOML
    cmd()
        .args([
            "render",
            toml_path.to_str().unwrap(),
            "-o",
            readme_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // The starter TOML has a placeholder username, so it should produce something
    assert!(readme_path.exists());
}
