use crate::config::profile::{Layout, ProfileConfig, Template};
use crate::render::sections;
use crate::render::templates::{
    self, Section, is_centered, is_multi_column, wrap_centered, wrap_multi_column,
};

/// Render a complete README.md from a ProfileConfig.
pub fn render(config: &ProfileConfig) -> String {
    let layout: &Layout = config.layout.as_ref().unwrap_or(&Layout {
        template: Some(Template::Full),
        dark_mode: Some(false),
        centered: Some(false),
    });
    let template: &Template = layout.template.as_ref().unwrap_or(&Template::Full);
    let centered: bool = layout.centered.unwrap_or(false) || is_centered(template);
    let ordered_sections: Vec<Section> = templates::sections_for_template(template);

    let theme: &str = config
        .stats
        .as_ref()
        .and_then(|s| s.theme.as_deref())
        .unwrap_or("default");
    let hide_border: bool = config
        .stats
        .as_ref()
        .and_then(|s| s.hide_border)
        .unwrap_or(false);

    let rendered: Vec<String> = ordered_sections
        .iter()
        .filter_map(|section: &Section| {
            let content: String = render_section(section, config, theme, hide_border);
            if content.is_empty() {
                None
            } else if centered {
                Some(wrap_centered(&content))
            } else {
                Some(content)
            }
        })
        .collect();

    if is_multi_column(template) && rendered.len() > 1 {
        let mid: usize = rendered.len().div_ceil(2);
        let left: String = rendered[..mid].join("\n\n");
        let right: String = rendered[mid..].join("\n\n");
        wrap_multi_column(&left, &right)
    } else {
        rendered.join("\n\n---\n\n")
    }
}

fn render_section(
    section: &Section,
    config: &ProfileConfig,
    theme: &str,
    hide_border: bool,
) -> String {
    match section {
        Section::Header => config
            .header
            .as_ref()
            .map(|h| sections::render_header(h, &config.meta))
            .unwrap_or_default(),
        Section::About => config
            .about
            .as_ref()
            .map(sections::render_about)
            .unwrap_or_default(),
        Section::Social => config
            .social
            .as_ref()
            .map(sections::render_social)
            .unwrap_or_default(),
        Section::Skills => config
            .skills
            .as_ref()
            .map(sections::render_skills)
            .unwrap_or_default(),
        Section::Stats => config
            .stats
            .as_ref()
            .map(|s| sections::render_stats(s, &config.meta))
            .unwrap_or_default(),
        Section::Projects => config
            .projects
            .as_ref()
            .map(|p| sections::render_projects(p, &config.meta, theme, hide_border))
            .unwrap_or_default(),
        Section::Blog => config
            .blog
            .as_ref()
            .map(sections::render_blog)
            .unwrap_or_default(),
        Section::Dynamic => config
            .dynamic
            .as_ref()
            .map(sections::render_dynamic)
            .unwrap_or_default(),
        Section::Sponsors => config
            .sponsors
            .as_ref()
            .map(|s| sections::render_sponsors(s, &config.meta))
            .unwrap_or_default(),
        Section::Extras => config
            .extras
            .as_ref()
            .map(sections::render_extras)
            .unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::profile::*;

    fn minimal_config() -> ProfileConfig {
        ProfileConfig {
            meta: Meta {
                username: "alice".to_string(),
                name: Some("Alice".to_string()),
            },
            header: Some(Header {
                style: Some(HeaderStyle::Text),
                banner_url: None,
                typing_lines: None,
                typing_font: None,
                typing_color: None,
                tagline: Some("Rust developer".to_string()),
            }),
            about: Some(About {
                role: Some("Engineer".to_string()),
                company: None,
                current_work: None,
                learning: None,
                reach_me: None,
                fun_fact: None,
                pronouns: None,
                location: None,
                timezone: None,
            }),
            social: Some(Social {
                twitter: Some("https://twitter.com/alice".to_string()),
                ..Social::default()
            }),
            layout: Some(Layout {
                template: Some(Template::Minimal),
                dark_mode: None,
                centered: None,
            }),
            ..ProfileConfig::default()
        }
    }

    fn full_config() -> ProfileConfig {
        ProfileConfig {
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
                tagline: None,
            }),
            about: Some(About {
                role: Some("Engineer".to_string()),
                company: Some("Acme".to_string()),
                current_work: Some("project".to_string()),
                learning: Some("WASM".to_string()),
                reach_me: None,
                fun_fact: Some("fun".to_string()),
                pronouns: None,
                location: None,
                timezone: None,
            }),
            social: Some(Social {
                twitter: Some("https://twitter.com/alice".to_string()),
                linkedin: Some("https://linkedin.com/in/alice".to_string()),
                ..Social::default()
            }),
            skills: Some(Skills {
                languages: Some(vec!["Rust".to_string(), "Python".to_string()]),
                tools: Some(vec!["Docker".to_string()]),
                ..Skills::default()
            }),
            stats: Some(Stats {
                stats_card: Some(true),
                streak: Some(true),
                top_langs: None,
                contributor_stats: None,
                trophies: None,
                contribution_snake: None,
                profile_views: None,
                theme: Some("tokyonight".to_string()),
                hide_border: Some(false),
                top_langs_layout: None,
                top_langs_count: None,
            }),
            projects: Some(Projects {
                repos: Some(vec!["alice/cool-cli".to_string()]),
                display: Some(ProjectDisplay::PinCards),
            }),
            blog: Some(Blog {
                rss_urls: Some(vec!["https://alice.dev/feed.xml".to_string()]),
                articles: None,
                youtube: None,
                newsletter: None,
            }),
            dynamic: Some(Dynamic {
                spotify_uid: Some("alice123".to_string()),
                wakatime: None,
                github_activity: None,
                stackoverflow_uid: None,
            }),
            layout: Some(Layout {
                template: Some(Template::Full),
                dark_mode: None,
                centered: None,
            }),
            sponsors: Some(Sponsors {
                github_sponsors: Some(true),
                ..Sponsors::default()
            }),
            extras: Some(Extras {
                pgp_fingerprint: Some("ABCD1234".to_string()),
                ..Extras::default()
            }),
        }
    }

    #[test]
    fn test_layout_minimal() {
        let config: ProfileConfig = minimal_config();
        let result: String = render(&config);
        // Minimal includes header, about, social
        assert!(result.contains("Hey! I'm Alice"));
        assert!(result.contains("Engineer"));
        assert!(result.contains("Twitter"));
        // Minimal should NOT include stats, skills, etc.
        assert!(!result.contains("### GitHub Stats"));
        assert!(!result.contains("### Tech Stack"));
    }

    #[test]
    fn test_layout_full() {
        let config: ProfileConfig = full_config();
        let result: String = render(&config);
        // Full includes everything
        assert!(result.contains("typing-svg"));
        assert!(result.contains("Engineer"));
        assert!(result.contains("Twitter"));
        assert!(result.contains("### Tech Stack"));
        assert!(result.contains("### GitHub Stats"));
        assert!(result.contains("### Featured Projects"));
        assert!(result.contains("BLOG-POST-LIST"));
        assert!(result.contains("Spotify"));
        assert!(result.contains("### Support"));
        assert!(result.contains("PGP"));
    }

    #[test]
    fn test_layout_multi_column() {
        let mut config: ProfileConfig = full_config();
        config.layout = Some(Layout {
            template: Some(Template::MultiColumn),
            dark_mode: None,
            centered: None,
        });
        let result: String = render(&config);
        assert!(result.contains("<table>"));
        assert!(result.contains("width=\"50%\""));
    }

    #[test]
    fn test_layout_centered() {
        let mut config: ProfileConfig = minimal_config();
        config.layout = Some(Layout {
            template: Some(Template::Minimal),
            dark_mode: None,
            centered: Some(true),
        });
        let result: String = render(&config);
        assert!(result.contains("<div align=\"center\">"));
    }

    #[test]
    fn test_layout_developer_card() {
        let mut config: ProfileConfig = full_config();
        config.layout = Some(Layout {
            template: Some(Template::DeveloperCard),
            dark_mode: None,
            centered: None,
        });
        let result: String = render(&config);
        // DeveloperCard is always centered
        assert!(result.contains("<div align=\"center\">"));
        // DeveloperCard has 6 sections: Header, About, Skills, Stats, Projects, Social
        assert!(result.contains("Tech Stack"));
        assert!(result.contains("GitHub Stats"));
    }

    #[test]
    fn test_render_empty_config() {
        let config = ProfileConfig {
            meta: Meta {
                username: "bob".to_string(),
                name: None,
            },
            ..ProfileConfig::default()
        };
        let result: String = render(&config);
        // With no sections configured, render should produce empty or near-empty output
        assert!(result.is_empty());
    }

    #[test]
    fn test_render_sections_joined_with_separators() {
        let config: ProfileConfig = minimal_config();
        let result: String = render(&config);
        // Non-multi-column templates join sections with ---
        assert!(result.contains("---"));
    }

    #[test]
    fn test_render_dark_light_mode() {
        // dark_mode config field is recognized (used by future image handling)
        let config = ProfileConfig {
            meta: Meta {
                username: "alice".to_string(),
                name: None,
            },
            layout: Some(Layout {
                template: Some(Template::Full),
                dark_mode: Some(true),
                centered: None,
            }),
            ..ProfileConfig::default()
        };
        let _result: String = render(&config);
        // Just verify it doesn't panic with dark_mode enabled
    }
}
