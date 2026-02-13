use std::path::Path;

use anyhow::{Context, Result};

use crate::config::profile::ProfileConfig;

/// Load a ProfileConfig from a TOML file.
pub fn load_config(path: &Path) -> Result<ProfileConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not open {}", path.display()))?;
    let config: ProfileConfig =
        toml::from_str(&content).with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(config)
}

/// Save a ProfileConfig to a TOML file.
pub fn save_config(config: &ProfileConfig, path: &Path) -> Result<()> {
    let content =
        toml::to_string_pretty(config).context("failed to serialize profile configuration")?;
    std::fs::write(path, content).with_context(|| format!("could not write {}", path.display()))?;
    Ok(())
}

/// Generate a starter TOML string with all sections shown as examples.
pub fn generate_starter_toml() -> String {
    r#"# gh-profile-gen profile configuration
# Uncomment and fill in the sections you want in your README.

[meta]
username = "your-github-username"
# name = "Your Display Name"

# [header]
# style = "typing_svg"  # Options: typing_svg, text, banner, wave
# typing_lines = ["Hello", "World"]
# typing_font = "Fira Code"
# typing_color = "f75c7e"
# tagline = "Your tagline here"
# banner_url = "https://example.com/banner.png"

# [about]
# role = "Your Role"
# company = "Your Company"
# current_work = "project-name"
# learning = "Something new"
# reach_me = "your@email.com"
# fun_fact = "Something fun"
# pronouns = "they/them"
# location = "City, Country"
# timezone = "UTC"

# [social]
# github = "https://github.com/username"
# twitter = "https://twitter.com/username"
# linkedin = "https://linkedin.com/in/username"
# mastodon = "https://mastodon.social/@username"
# bluesky = "https://bsky.app/profile/username"
# instagram = "https://instagram.com/username"
# youtube = "https://youtube.com/@username"
# discord = "https://discord.gg/invite"
# devto = "https://dev.to/username"
# hashnode = "https://hashnode.com/@username"
# medium = "https://medium.com/@username"
# stackoverflow = "https://stackoverflow.com/users/id"
# reddit = "https://reddit.com/u/username"
# twitch = "https://twitch.tv/username"
# website = "https://yoursite.com"
# email = "your@email.com"
# kofi = "https://ko-fi.com/username"
# rss = "https://yoursite.com/feed.xml"

# [skills]
# languages = ["Rust", "Python", "TypeScript"]
# frameworks = ["Actix", "React"]
# tools = ["Docker", "Git", "Neovim"]
# databases = ["PostgreSQL", "Redis"]
# cloud = ["AWS", "Vercel"]

# [stats]
# stats_card = true
# top_langs = true
# streak = true
# contributor_stats = false
# trophies = false
# contribution_snake = false
# profile_views = true
# theme = "tokyonight"
# hide_border = false
# top_langs_layout = "compact"
# top_langs_count = 8

# [projects]
# repos = ["username/repo1", "username/repo2"]
# display = "pin_cards"  # Options: pin_cards, markdown_table

# [blog]
# rss_urls = ["https://yoursite.com/feed.xml"]
# youtube = "https://youtube.com/@username"
# newsletter = "https://yoursite.com/newsletter"
#
# [[blog.articles]]
# title = "My First Post"
# url = "https://yoursite.com/first-post"

# [dynamic]
# spotify_uid = "your-spotify-user-id"
# wakatime = true
# github_activity = true
# stackoverflow_uid = "12345"

# [layout]
# template = "full"  # Options: minimal, full, developer_card, multi_column
# dark_mode = true
# centered = false

# [sponsors]
# github_sponsors = true
# kofi = "https://ko-fi.com/username"
# buy_me_a_coffee = "https://buymeacoffee.com/username"

# [extras]
# pgp_fingerprint = "ABCD 1234 EFGH 5678"
# xbox = "YourGamertag"
# steam = "YourSteamID"
# psn = "YourPSNID"
# certifications = ["AWS Certified Developer"]
# custom_blocks = ["Any raw markdown you want inserted"]
#
# [[extras.collapsible]]
# summary = "Click to expand"
# content = "Hidden content here"
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::profile::Meta;
    use tempfile::TempDir;

    #[test]
    fn test_save_and_load_round_trip() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("profile.toml");

        let config = ProfileConfig {
            meta: Meta {
                username: "alice".to_string(),
                name: Some("Alice".to_string()),
            },
            ..ProfileConfig::default()
        };

        save_config(&config, &path).unwrap();
        let loaded = load_config(&path).unwrap();
        assert_eq!(config, loaded);
    }

    #[test]
    fn test_load_missing_file() {
        let result = load_config(Path::new("/nonexistent/profile.toml"));
        assert!(result.is_err());
        let err_msg = format!("{:#}", result.unwrap_err());
        assert!(err_msg.contains("could not open"));
    }

    #[test]
    fn test_load_invalid_toml() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("bad.toml");
        std::fs::write(&path, "this is not valid toml [[[").unwrap();

        let result = load_config(&path);
        assert!(result.is_err());
        let err_msg = format!("{:#}", result.unwrap_err());
        assert!(err_msg.contains("failed to parse"));
    }

    #[test]
    fn test_generate_starter_toml_parses() {
        // The starter TOML should be valid (the uncommented parts)
        // Only the [meta] section is uncommented by default
        let starter = generate_starter_toml();
        assert!(starter.contains("[meta]"));
        assert!(starter.contains("username"));

        // Extract only uncommented lines and verify they parse
        let uncommented: String = starter
            .lines()
            .filter(|line: &&str| !line.starts_with('#') && !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n");
        let config: ProfileConfig =
            toml::from_str(&uncommented).expect("starter TOML should parse");
        assert_eq!(config.meta.username, "your-github-username");
    }
}
