use serde::{Deserialize, Serialize};

/// Top-level profile configuration. All sections except `meta` are optional.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub meta: Meta,
    pub header: Option<Header>,
    pub about: Option<About>,
    pub social: Option<Social>,
    pub skills: Option<Skills>,
    pub stats: Option<Stats>,
    pub projects: Option<Projects>,
    pub blog: Option<Blog>,
    pub dynamic: Option<Dynamic>,
    pub layout: Option<Layout>,
    pub sponsors: Option<Sponsors>,
    pub extras: Option<Extras>,
}

/// Required metadata — at minimum, the GitHub username.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub username: String,
    pub name: Option<String>,
}

/// Header section: banner, typing SVG, or text greeting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    pub style: Option<HeaderStyle>,
    pub banner_url: Option<String>,
    pub typing_lines: Option<Vec<String>>,
    pub typing_font: Option<String>,
    pub typing_color: Option<String>,
    pub tagline: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeaderStyle {
    TypingSvg,
    Text,
    Banner,
    Wave,
}

/// About Me section.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct About {
    pub role: Option<String>,
    pub company: Option<String>,
    pub current_work: Option<String>,
    pub learning: Option<String>,
    pub reach_me: Option<String>,
    pub fun_fact: Option<String>,
    pub pronouns: Option<String>,
    pub location: Option<String>,
    pub timezone: Option<String>,
}

/// Social media links.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Social {
    pub github: Option<String>,
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
    pub mastodon: Option<String>,
    pub bluesky: Option<String>,
    pub instagram: Option<String>,
    pub youtube: Option<String>,
    pub discord: Option<String>,
    pub devto: Option<String>,
    pub hashnode: Option<String>,
    pub medium: Option<String>,
    pub stackoverflow: Option<String>,
    pub reddit: Option<String>,
    pub twitch: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub kofi: Option<String>,
    pub rss: Option<String>,
}

/// Skills / Tech Stack, organized by category.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Skills {
    pub languages: Option<Vec<String>>,
    pub frameworks: Option<Vec<String>>,
    pub tools: Option<Vec<String>>,
    pub databases: Option<Vec<String>>,
    pub cloud: Option<Vec<String>>,
}

/// GitHub stats cards configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub stats_card: Option<bool>,
    pub top_langs: Option<bool>,
    pub streak: Option<bool>,
    pub contributor_stats: Option<bool>,
    pub trophies: Option<bool>,
    pub contribution_snake: Option<bool>,
    pub profile_views: Option<bool>,
    pub theme: Option<String>,
    pub hide_border: Option<bool>,
    pub top_langs_layout: Option<String>,
    pub top_langs_count: Option<u32>,
}

/// Featured projects.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Projects {
    pub repos: Option<Vec<String>>,
    pub display: Option<ProjectDisplay>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectDisplay {
    PinCards,
    MarkdownTable,
}

/// Blog / Content section.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Blog {
    pub rss_urls: Option<Vec<String>>,
    pub articles: Option<Vec<Article>>,
    pub youtube: Option<String>,
    pub newsletter: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub url: String,
}

/// Dynamic / real-time integrations.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Dynamic {
    pub spotify_uid: Option<String>,
    pub wakatime: Option<bool>,
    pub github_activity: Option<bool>,
    pub stackoverflow_uid: Option<String>,
}

/// Layout and theming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layout {
    pub template: Option<Template>,
    pub dark_mode: Option<bool>,
    pub centered: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Template {
    Minimal,
    Full,
    DeveloperCard,
    MultiColumn,
}

/// Sponsors section.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Sponsors {
    pub github_sponsors: Option<bool>,
    pub kofi: Option<String>,
    pub buy_me_a_coffee: Option<String>,
}

/// Extras: PGP, gaming, certifications, custom blocks.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Extras {
    pub pgp_fingerprint: Option<String>,
    pub xbox: Option<String>,
    pub steam: Option<String>,
    pub psn: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub custom_blocks: Option<Vec<String>>,
    pub collapsible: Option<Vec<CollapsibleSection>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollapsibleSection {
    pub summary: String,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_config_defaults() {
        let config = ProfileConfig::default();
        assert!(config.meta.username.is_empty());
        assert!(config.meta.name.is_none());
        assert!(config.header.is_none());
        assert!(config.about.is_none());
        assert!(config.social.is_none());
        assert!(config.skills.is_none());
        assert!(config.stats.is_none());
        assert!(config.projects.is_none());
        assert!(config.blog.is_none());
        assert!(config.dynamic.is_none());
        assert!(config.layout.is_none());
        assert!(config.sponsors.is_none());
        assert!(config.extras.is_none());
    }

    #[test]
    fn test_profile_config_round_trip() {
        let config = ProfileConfig {
            meta: Meta {
                username: "alice".to_string(),
                name: Some("Alice".to_string()),
            },
            header: Some(Header {
                style: Some(HeaderStyle::TypingSvg),
                banner_url: None,
                typing_lines: Some(vec!["Hello".to_string(), "World".to_string()]),
                typing_font: Some("Fira Code".to_string()),
                typing_color: Some("f75c7e".to_string()),
                tagline: Some("Rust developer".to_string()),
            }),
            about: Some(About {
                role: Some("Backend Engineer".to_string()),
                company: Some("Acme Corp".to_string()),
                current_work: Some("my-project".to_string()),
                learning: Some("WebAssembly".to_string()),
                reach_me: Some("alice@example.com".to_string()),
                fun_fact: Some("I love ferris".to_string()),
                pronouns: Some("she/her".to_string()),
                location: Some("San Francisco".to_string()),
                timezone: Some("PST".to_string()),
            }),
            social: Some(Social {
                twitter: Some("https://twitter.com/alice".to_string()),
                linkedin: Some("https://linkedin.com/in/alice".to_string()),
                ..Social::default()
            }),
            skills: Some(Skills {
                languages: Some(vec!["Rust".to_string(), "Python".to_string()]),
                frameworks: Some(vec!["Actix".to_string()]),
                tools: Some(vec!["Docker".to_string()]),
                databases: Some(vec!["PostgreSQL".to_string()]),
                cloud: Some(vec!["AWS".to_string()]),
            }),
            stats: Some(Stats {
                stats_card: Some(true),
                top_langs: Some(true),
                streak: Some(true),
                contributor_stats: None,
                trophies: None,
                contribution_snake: None,
                profile_views: Some(true),
                theme: Some("tokyonight".to_string()),
                hide_border: Some(false),
                top_langs_layout: Some("compact".to_string()),
                top_langs_count: Some(8),
            }),
            projects: Some(Projects {
                repos: Some(vec!["alice/cool-cli".to_string()]),
                display: Some(ProjectDisplay::PinCards),
            }),
            blog: Some(Blog {
                rss_urls: Some(vec!["https://alice.dev/feed.xml".to_string()]),
                articles: Some(vec![Article {
                    title: "My Post".to_string(),
                    url: "https://alice.dev/my-post".to_string(),
                }]),
                youtube: Some("https://youtube.com/@alice".to_string()),
                newsletter: Some("https://alice.dev/newsletter".to_string()),
            }),
            dynamic: Some(Dynamic {
                spotify_uid: Some("alice123".to_string()),
                wakatime: Some(true),
                github_activity: Some(true),
                stackoverflow_uid: Some("12345".to_string()),
            }),
            layout: Some(Layout {
                template: Some(Template::Full),
                dark_mode: Some(true),
                centered: Some(false),
            }),
            sponsors: Some(Sponsors {
                github_sponsors: Some(true),
                kofi: Some("https://ko-fi.com/alice".to_string()),
                buy_me_a_coffee: None,
            }),
            extras: Some(Extras {
                pgp_fingerprint: Some("ABCD1234".to_string()),
                xbox: Some("AliceGamer".to_string()),
                steam: None,
                psn: None,
                certifications: Some(vec!["AWS Certified".to_string()]),
                custom_blocks: Some(vec!["Custom markdown here".to_string()]),
                collapsible: Some(vec![CollapsibleSection {
                    summary: "More info".to_string(),
                    content: "Hidden details".to_string(),
                }]),
            }),
        };

        let toml_str: String = toml::to_string(&config).expect("serialize");
        let deserialized: ProfileConfig = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_profile_config_partial_toml() {
        let toml_str = r#"
[meta]
username = "bob"

[about]
role = "Developer"
"#;
        let config: ProfileConfig = toml::from_str(toml_str).expect("deserialize partial");
        assert_eq!(config.meta.username, "bob");
        assert_eq!(
            config.about.as_ref().unwrap().role,
            Some("Developer".to_string())
        );
        assert!(config.social.is_none());
        assert!(config.skills.is_none());
        assert!(config.stats.is_none());
    }

    #[test]
    fn test_profile_config_all_sections() {
        let toml_str = r#"
[meta]
username = "alice"
name = "Alice"

[header]
style = "typing_svg"
typing_lines = ["Hello", "World"]
typing_font = "Fira Code"
typing_color = "f75c7e"
tagline = "Rust dev"

[about]
role = "Engineer"
company = "Acme"
current_work = "project"
learning = "WASM"
reach_me = "email"
fun_fact = "fun"
pronouns = "she/her"
location = "SF"
timezone = "PST"

[social]
twitter = "https://twitter.com/alice"
linkedin = "https://linkedin.com/in/alice"

[skills]
languages = ["Rust", "Python"]
frameworks = ["Actix"]
tools = ["Docker"]
databases = ["PostgreSQL"]
cloud = ["AWS"]

[stats]
stats_card = true
top_langs = true
streak = true
theme = "tokyonight"
hide_border = false
top_langs_layout = "compact"
top_langs_count = 8

[projects]
repos = ["alice/cool-cli"]
display = "pin_cards"

[blog]
rss_urls = ["https://alice.dev/feed.xml"]
youtube = "https://youtube.com/@alice"
newsletter = "https://alice.dev/newsletter"

[[blog.articles]]
title = "My Post"
url = "https://alice.dev/my-post"

[dynamic]
spotify_uid = "alice123"
wakatime = true
github_activity = true
stackoverflow_uid = "12345"

[layout]
template = "full"
dark_mode = true
centered = false

[sponsors]
github_sponsors = true
kofi = "https://ko-fi.com/alice"

[extras]
pgp_fingerprint = "ABCD1234"
xbox = "AliceGamer"
certifications = ["AWS Certified"]
custom_blocks = ["Custom markdown"]

[[extras.collapsible]]
summary = "More info"
content = "Hidden details"
"#;
        let config: ProfileConfig = toml::from_str(toml_str).expect("deserialize all sections");
        assert_eq!(config.meta.username, "alice");
        assert!(config.header.is_some());
        assert!(config.about.is_some());
        assert!(config.social.is_some());
        assert!(config.skills.is_some());
        assert!(config.stats.is_some());
        assert!(config.projects.is_some());
        assert!(config.blog.is_some());
        assert!(config.dynamic.is_some());
        assert!(config.layout.is_some());
        assert!(config.sponsors.is_some());
        assert!(config.extras.is_some());
    }
}
