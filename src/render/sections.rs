use crate::config::profile::*;
use crate::services::urls;

/// Render the header section.
pub fn render_header(header: &Header, meta: &Meta) -> String {
    let style: &HeaderStyle = match &header.style {
        Some(s) => s,
        None => return render_header_text(header, meta),
    };

    match style {
        HeaderStyle::TypingSvg => render_header_typing_svg(header),
        HeaderStyle::Text => render_header_text(header, meta),
        HeaderStyle::Banner => render_header_banner(header),
        HeaderStyle::Wave => render_header_wave(meta),
    }
}

fn render_header_typing_svg(header: &Header) -> String {
    let lines: &[String] = match &header.typing_lines {
        Some(l) if !l.is_empty() => l,
        _ => return String::new(),
    };
    let font: &str = header.typing_font.as_deref().unwrap_or("Fira Code");
    let color: &str = header.typing_color.as_deref().unwrap_or("f75c7e");
    let line_strs: Vec<&str> = lines.iter().map(|s: &String| s.as_str()).collect();
    let url: String = urls::typing_svg_url(&line_strs, font, color, true);

    format!(
        "<p align=\"center\">\n  <a href=\"https://readme-typing-svg.demolab.com/\">\n    <img src=\"{}\" />\n  </a>\n</p>",
        url
    )
}

fn render_header_text(header: &Header, meta: &Meta) -> String {
    let name: &str = meta.name.as_deref().unwrap_or(&meta.username);
    let mut out = format!("## Hey! I'm {}", name);
    if let Some(tagline) = &header.tagline {
        out.push_str(&format!("\n\n{}", tagline));
    }
    out
}

fn render_header_banner(header: &Header) -> String {
    match &header.banner_url {
        Some(url) => format!("<p align=\"center\">\n  <img src=\"{}\" />\n</p>", url),
        None => String::new(),
    }
}

fn render_header_wave(meta: &Meta) -> String {
    let name: &str = meta.name.as_deref().unwrap_or(&meta.username);
    format!("# Hi there, I'm {} \u{1f44b}", name)
}

/// Render the About Me section.
pub fn render_about(about: &About) -> String {
    let mut items: Vec<String> = Vec::new();

    if let Some(role) = &about.role {
        let company_part: String = about
            .company
            .as_ref()
            .map(|c: &String| format!(" at **{}**", c))
            .unwrap_or_default();
        items.push(format!("\u{1f4bc} {}{}", role, company_part));
    }
    if let Some(work) = &about.current_work {
        items.push(format!("\u{1f52d} Currently working on **{}**", work));
    }
    if let Some(learning) = &about.learning {
        items.push(format!("\u{1f331} Learning **{}**", learning));
    }
    if let Some(reach) = &about.reach_me {
        items.push(format!("\u{1f4ac} Ask me about **{}**", reach));
    }
    if let Some(fact) = &about.fun_fact {
        items.push(format!("\u{26a1} Fun fact: {}", fact));
    }
    if let Some(pronouns) = &about.pronouns {
        items.push(format!("\u{1f600} Pronouns: {}", pronouns));
    }
    if let Some(location) = &about.location {
        items.push(format!("\u{1f4cd} {}", location));
    }

    if items.is_empty() {
        return String::new();
    }

    items
        .iter()
        .map(|item: &String| format!("- {}", item))
        .collect::<Vec<String>>()
        .join("\n")
}

/// Render the Social Links section as badges.
pub fn render_social(social: &Social) -> String {
    let platforms: Vec<(&str, &Option<String>)> = vec![
        ("github", &social.github),
        ("twitter", &social.twitter),
        ("linkedin", &social.linkedin),
        ("mastodon", &social.mastodon),
        ("bluesky", &social.bluesky),
        ("instagram", &social.instagram),
        ("youtube", &social.youtube),
        ("discord", &social.discord),
        ("devto", &social.devto),
        ("hashnode", &social.hashnode),
        ("medium", &social.medium),
        ("stackoverflow", &social.stackoverflow),
        ("reddit", &social.reddit),
        ("twitch", &social.twitch),
        ("website", &social.website),
        ("email", &social.email),
        ("kofi", &social.kofi),
        ("rss", &social.rss),
    ];

    let badges: Vec<String> = platforms
        .iter()
        .filter_map(|(platform, url_opt): &(&str, &Option<String>)| {
            let url: &String = url_opt.as_ref()?;
            let (label, logo, color): (&str, &str, &str) = urls::social_platform_info(platform)?;
            Some(urls::social_badge_markdown(label, color, logo, url))
        })
        .collect();

    if badges.is_empty() {
        return String::new();
    }

    format!("### Connect with me\n\n{}", badges.join("\n"))
}

/// Render the Skills / Tech Stack section as badges.
pub fn render_skills(skills: &Skills) -> String {
    let mut all_badges: Vec<String> = Vec::new();

    let categories: Vec<(&str, &Option<Vec<String>>)> = vec![
        ("Languages", &skills.languages),
        ("Frameworks", &skills.frameworks),
        ("Tools", &skills.tools),
        ("Databases", &skills.databases),
        ("Cloud/Infra", &skills.cloud),
    ];

    for (_category_name, skill_list) in &categories {
        if let Some(skills_vec) = skill_list {
            for skill in skills_vec {
                let badge: String = match urls::skill_icon_lookup(skill) {
                    Some((logo, color)) => urls::skill_badge_url(skill, color, logo),
                    None => urls::skill_badge_url(skill, "333333", ""),
                };
                all_badges.push(format!("![{}]({})", skill, badge));
            }
        }
    }

    if all_badges.is_empty() {
        return String::new();
    }

    format!("### Tech Stack\n\n{}", all_badges.join("\n"))
}

/// Render the GitHub Stats section.
pub fn render_stats(stats: &Stats, meta: &Meta) -> String {
    let theme: &str = stats.theme.as_deref().unwrap_or("default");
    let hide_border: bool = stats.hide_border.unwrap_or(false);
    let mut cards: Vec<String> = Vec::new();

    if stats.stats_card.unwrap_or(false) {
        let url: String = urls::github_stats_url(&meta.username, theme, true, hide_border);
        cards.push(format!(
            "![{}'s GitHub stats]({})",
            meta.name.as_deref().unwrap_or(&meta.username),
            url
        ));
    }

    if stats.top_langs.unwrap_or(false) {
        let layout: &str = stats.top_langs_layout.as_deref().unwrap_or("compact");
        let count: u32 = stats.top_langs_count.unwrap_or(8);
        let url: String = urls::top_langs_url(&meta.username, layout, count, theme, hide_border);
        cards.push(format!("![Top Langs]({})", url));
    }

    if stats.streak.unwrap_or(false) {
        let url: String = urls::streak_stats_url(&meta.username, theme, hide_border);
        cards.push(format!("![GitHub Streak]({})", url));
    }

    if stats.contributor_stats.unwrap_or(false) {
        let url: String = urls::contributor_stats_url(&meta.username, theme, hide_border);
        cards.push(format!("![Contributor Stats]({})", url));
    }

    if stats.trophies.unwrap_or(false) {
        let url: String = urls::trophies_url(&meta.username, theme);
        cards.push(format!("![Trophies]({})", url));
    }

    if stats.profile_views.unwrap_or(false) {
        let url: String = urls::profile_views_url(&meta.username);
        cards.push(format!("![Profile Views]({})", url));
    }

    if cards.is_empty() {
        return String::new();
    }

    format!("### GitHub Stats\n\n{}", cards.join("\n\n"))
}

/// Render the Featured Projects section.
pub fn render_projects(projects: &Projects, meta: &Meta, theme: &str, hide_border: bool) -> String {
    let repos: &[String] = match &projects.repos {
        Some(r) if !r.is_empty() => r,
        _ => return String::new(),
    };

    let display: &ProjectDisplay = projects
        .display
        .as_ref()
        .unwrap_or(&ProjectDisplay::PinCards);

    let items: Vec<String> = repos
        .iter()
        .map(|repo_str: &String| {
            let (owner, repo): (&str, &str) = repo_str
                .split_once('/')
                .unwrap_or((&meta.username, repo_str));

            match display {
                ProjectDisplay::PinCards => {
                    let url: String = urls::pin_card_url(owner, repo, theme, hide_border);
                    format!(
                        "<a href=\"https://github.com/{}/{}\">\n  <img align=\"center\" src=\"{}\" />\n</a>",
                        owner, repo, url
                    )
                }
                ProjectDisplay::MarkdownTable => {
                    format!("| [{}](https://github.com/{}/{}) | |", repo, owner, repo)
                }
            }
        })
        .collect();

    let content: String = match display {
        ProjectDisplay::PinCards => items.join("\n"),
        ProjectDisplay::MarkdownTable => {
            let header = "| Project | Description |\n|---|---|";
            format!("{}\n{}", header, items.join("\n"))
        }
    };

    format!("### Featured Projects\n\n{}", content)
}

/// Render the Blog / Content section.
pub fn render_blog(blog: &Blog) -> String {
    let mut sections: Vec<String> = Vec::new();

    // RSS markers for GitHub Action
    if let Some(rss_urls) = &blog.rss_urls
        && !rss_urls.is_empty()
    {
        sections.push(
            "<!-- BLOG-POST-LIST:START -->\n<!-- BLOG-POST-LIST:END -->".to_string(),
        );
    }

    // Manual articles
    if let Some(articles) = &blog.articles
        && !articles.is_empty()
    {
        let list: String = articles
            .iter()
            .enumerate()
            .map(|(i, a): (usize, &Article)| format!("{}. [{}]({})", i + 1, a.title, a.url))
            .collect::<Vec<String>>()
            .join("\n");
        sections.push(list);
    }

    // YouTube
    if let Some(youtube) = &blog.youtube {
        sections.push(format!(
            "**YouTube**: [My Channel]({})",
            youtube
        ));
    }

    // Newsletter
    if let Some(newsletter) = &blog.newsletter {
        sections.push(format!(
            "**Newsletter**: [Subscribe]({})",
            newsletter
        ));
    }

    if sections.is_empty() {
        return String::new();
    }

    format!(
        "### Latest Blog Posts\n\n{}",
        sections.join("\n\n")
    )
}

/// Render the Dynamic / Real-time section.
pub fn render_dynamic(dynamic: &Dynamic) -> String {
    let mut items: Vec<String> = Vec::new();

    if let Some(uid) = &dynamic.spotify_uid {
        items.push(format!(
            "### Spotify\n\n{}",
            urls::spotify_badge_markdown(uid)
        ));
    }

    if dynamic.wakatime.unwrap_or(false) {
        items.push("### WakaTime\n\n<!-- WAKATIME:START -->\n<!-- WAKATIME:END -->".to_string());
    }

    if dynamic.github_activity.unwrap_or(false) {
        items.push("### Recent Activity\n\n<!--START_SECTION:activity-->\n<!--END_SECTION:activity-->".to_string());
    }

    if let Some(uid) = &dynamic.stackoverflow_uid {
        let url: String = urls::stackoverflow_badge_url(uid);
        items.push(format!(
            "### StackOverflow\n\n[![StackOverflow]({})](https://stackoverflow.com/users/{})",
            url, uid
        ));
    }

    items.join("\n\n")
}

/// Render the Sponsors section.
pub fn render_sponsors(sponsors: &Sponsors, meta: &Meta) -> String {
    let mut items: Vec<String> = Vec::new();

    if sponsors.github_sponsors.unwrap_or(false) {
        items.push(format!(
            "[![Sponsor](https://img.shields.io/badge/Sponsor-EA4AAA?style=for-the-badge&logo=githubsponsors&logoColor=white)](https://github.com/sponsors/{})",
            meta.username
        ));
    }

    if let Some(kofi) = &sponsors.kofi {
        items.push(format!(
            "[![Ko-fi](https://img.shields.io/badge/Ko--fi-FF5E5B?style=for-the-badge&logo=kofi&logoColor=white)]({})",
            kofi
        ));
    }

    if let Some(bmac) = &sponsors.buy_me_a_coffee {
        items.push(format!(
            "[![Buy Me a Coffee](https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black)]({})",
            bmac
        ));
    }

    if items.is_empty() {
        return String::new();
    }

    format!("### Support\n\n{}", items.join("\n"))
}

/// Render the Extras section.
pub fn render_extras(extras: &Extras) -> String {
    let mut items: Vec<String> = Vec::new();

    if let Some(pgp) = &extras.pgp_fingerprint {
        items.push(format!(
            "![PGP](https://img.shields.io/badge/PGP-{}-333333?style=flat-square&logo=gnuprivacyguard&logoColor=white)",
            pgp.replace(' ', "%20")
        ));
    }

    // Gaming profiles
    let mut gaming: Vec<String> = Vec::new();
    if let Some(xbox) = &extras.xbox {
        gaming.push(format!("- **Xbox**: {}", xbox));
    }
    if let Some(steam) = &extras.steam {
        gaming.push(format!("- **Steam**: {}", steam));
    }
    if let Some(psn) = &extras.psn {
        gaming.push(format!("- **PSN**: {}", psn));
    }
    if !gaming.is_empty() {
        items.push(format!("**Gaming**\n\n{}", gaming.join("\n")));
    }

    // Certifications
    if let Some(certs) = &extras.certifications
        && !certs.is_empty()
    {
        let list: String = certs
            .iter()
            .map(|c: &String| format!("- {}", c))
            .collect::<Vec<String>>()
            .join("\n");
        items.push(format!("**Certifications**\n\n{}", list));
    }

    // Collapsible sections
    if let Some(collapsibles) = &extras.collapsible {
        for section in collapsibles {
            items.push(format!(
                "<details>\n<summary>{}</summary>\n\n{}\n\n</details>",
                section.summary, section.content
            ));
        }
    }

    // Custom markdown blocks
    if let Some(blocks) = &extras.custom_blocks {
        for block in blocks {
            items.push(block.clone());
        }
    }

    items.join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_meta() -> Meta {
        Meta {
            username: "alice".to_string(),
            name: Some("Alice".to_string()),
        }
    }

    #[test]
    fn test_render_header_typing_svg() {
        let header = Header {
            style: Some(HeaderStyle::TypingSvg),
            banner_url: None,
            typing_lines: Some(vec!["Hello".to_string(), "World".to_string()]),
            typing_font: Some("Fira Code".to_string()),
            typing_color: Some("f75c7e".to_string()),
            tagline: None,
        };
        let result: String = render_header(&header, &test_meta());
        assert!(result.contains("<p align=\"center\">"));
        assert!(result.contains("<img src=\"https://readme-typing-svg.demolab.com/"));
        assert!(result.contains("Hello;World"));
        assert!(result.contains("f75c7e"));
    }

    #[test]
    fn test_render_header_text() {
        let header = Header {
            style: Some(HeaderStyle::Text),
            banner_url: None,
            typing_lines: None,
            typing_font: None,
            typing_color: None,
            tagline: Some("Rust developer".to_string()),
        };
        let result: String = render_header(&header, &test_meta());
        assert!(result.contains("## Hey! I'm Alice"));
        assert!(result.contains("Rust developer"));
    }

    #[test]
    fn test_render_header_banner() {
        let header = Header {
            style: Some(HeaderStyle::Banner),
            banner_url: Some("https://example.com/banner.png".to_string()),
            typing_lines: None,
            typing_font: None,
            typing_color: None,
            tagline: None,
        };
        let result: String = render_header(&header, &test_meta());
        assert!(result.contains("https://example.com/banner.png"));
        assert!(result.contains("<p align=\"center\">"));
    }

    #[test]
    fn test_render_header_wave() {
        let header = Header {
            style: Some(HeaderStyle::Wave),
            banner_url: None,
            typing_lines: None,
            typing_font: None,
            typing_color: None,
            tagline: None,
        };
        let result: String = render_header(&header, &test_meta());
        assert!(result.contains("# Hi there, I'm Alice"));
    }

    #[test]
    fn test_render_about_section() {
        let about = About {
            role: Some("Backend Engineer".to_string()),
            company: Some("Acme".to_string()),
            current_work: Some("my-project".to_string()),
            learning: Some("WebAssembly".to_string()),
            reach_me: Some("Rust, CLI tools".to_string()),
            fun_fact: Some("I love ferris".to_string()),
            pronouns: Some("she/her".to_string()),
            location: Some("San Francisco".to_string()),
            timezone: None,
        };
        let result: String = render_about(&about);
        assert!(result.contains("Backend Engineer"));
        assert!(result.contains("at **Acme**"));
        assert!(result.contains("Currently working on **my-project**"));
        assert!(result.contains("Learning **WebAssembly**"));
        assert!(result.contains("Fun fact: I love ferris"));
        assert!(result.contains("Pronouns: she/her"));
        assert!(result.contains("San Francisco"));
    }

    #[test]
    fn test_render_about_empty() {
        let about = About {
            role: None,
            company: None,
            current_work: None,
            learning: None,
            reach_me: None,
            fun_fact: None,
            pronouns: None,
            location: None,
            timezone: None,
        };
        let result: String = render_about(&about);
        assert!(result.is_empty());
    }

    #[test]
    fn test_render_social_badges() {
        let social = Social {
            twitter: Some("https://twitter.com/alice".to_string()),
            linkedin: Some("https://linkedin.com/in/alice".to_string()),
            ..Social::default()
        };
        let result: String = render_social(&social);
        assert!(result.contains("### Connect with me"));
        assert!(result.contains("Twitter"));
        assert!(result.contains("https://twitter.com/alice"));
        assert!(result.contains("LinkedIn"));
        assert!(result.contains("https://linkedin.com/in/alice"));
        assert!(result.contains("for-the-badge"));
    }

    #[test]
    fn test_render_social_empty() {
        let social = Social::default();
        let result: String = render_social(&social);
        assert!(result.is_empty());
    }

    #[test]
    fn test_render_skills_badges() {
        let skills = Skills {
            languages: Some(vec!["Rust".to_string(), "Python".to_string()]),
            tools: Some(vec!["Docker".to_string()]),
            ..Skills::default()
        };
        let result: String = render_skills(&skills);
        assert!(result.contains("### Tech Stack"));
        assert!(result.contains("![Rust]"));
        assert!(result.contains("![Python]"));
        assert!(result.contains("![Docker]"));
        assert!(result.contains("logo=rust"));
        assert!(result.contains("logo=python"));
    }

    #[test]
    fn test_render_skills_unknown_tech() {
        let skills = Skills {
            languages: Some(vec!["ObscureLang".to_string()]),
            ..Skills::default()
        };
        let result: String = render_skills(&skills);
        assert!(result.contains("![ObscureLang]"));
        assert!(result.contains("333333")); // fallback color
    }

    #[test]
    fn test_render_stats_cards() {
        let stats = Stats {
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
        };
        let result: String = render_stats(&stats, &test_meta());
        assert!(result.contains("### GitHub Stats"));
        assert!(result.contains("Alice's GitHub stats"));
        assert!(result.contains("github-readme-stats.vercel.app"));
        assert!(result.contains("streak-stats.demolab.com"));
    }

    #[test]
    fn test_render_stats_empty() {
        let stats = Stats {
            stats_card: Some(false),
            streak: Some(false),
            top_langs: None,
            contributor_stats: None,
            trophies: None,
            contribution_snake: None,
            profile_views: None,
            theme: None,
            hide_border: None,
            top_langs_layout: None,
            top_langs_count: None,
        };
        let result: String = render_stats(&stats, &test_meta());
        assert!(result.is_empty());
    }

    #[test]
    fn test_render_featured_projects() {
        let projects = Projects {
            repos: Some(vec!["alice/cool-cli".to_string(), "alice/other-lib".to_string()]),
            display: Some(ProjectDisplay::PinCards),
        };
        let result: String = render_projects(&projects, &test_meta(), "tokyonight", false);
        assert!(result.contains("### Featured Projects"));
        assert!(result.contains("https://github.com/alice/cool-cli"));
        assert!(result.contains("github-readme-stats.vercel.app/api/pin/"));
    }

    #[test]
    fn test_render_featured_projects_table() {
        let projects = Projects {
            repos: Some(vec!["alice/cool-cli".to_string()]),
            display: Some(ProjectDisplay::MarkdownTable),
        };
        let result: String = render_projects(&projects, &test_meta(), "tokyonight", false);
        assert!(result.contains("| Project | Description |"));
        assert!(result.contains("cool-cli"));
    }

    #[test]
    fn test_render_blog_markers() {
        let blog = Blog {
            rss_urls: Some(vec!["https://alice.dev/feed.xml".to_string()]),
            articles: None,
            youtube: None,
            newsletter: None,
        };
        let result: String = render_blog(&blog);
        assert!(result.contains("<!-- BLOG-POST-LIST:START -->"));
        assert!(result.contains("<!-- BLOG-POST-LIST:END -->"));
    }

    #[test]
    fn test_render_manual_blog_posts() {
        let blog = Blog {
            rss_urls: None,
            articles: Some(vec![
                Article {
                    title: "First Post".to_string(),
                    url: "https://alice.dev/first".to_string(),
                },
                Article {
                    title: "Second Post".to_string(),
                    url: "https://alice.dev/second".to_string(),
                },
            ]),
            youtube: None,
            newsletter: None,
        };
        let result: String = render_blog(&blog);
        assert!(result.contains("### Latest Blog Posts"));
        assert!(result.contains("1. [First Post](https://alice.dev/first)"));
        assert!(result.contains("2. [Second Post](https://alice.dev/second)"));
    }

    #[test]
    fn test_render_sponsors() {
        let sponsors = Sponsors {
            github_sponsors: Some(true),
            kofi: Some("https://ko-fi.com/alice".to_string()),
            buy_me_a_coffee: None,
        };
        let result: String = render_sponsors(&sponsors, &test_meta());
        assert!(result.contains("### Support"));
        assert!(result.contains("github.com/sponsors/alice"));
        assert!(result.contains("ko-fi.com/alice"));
    }

    #[test]
    fn test_render_extras_pgp() {
        let extras = Extras {
            pgp_fingerprint: Some("ABCD1234".to_string()),
            ..Extras::default()
        };
        let result: String = render_extras(&extras);
        assert!(result.contains("PGP"));
        assert!(result.contains("ABCD1234"));
    }

    #[test]
    fn test_render_collapsible_section() {
        let extras = Extras {
            collapsible: Some(vec![CollapsibleSection {
                summary: "More info".to_string(),
                content: "Hidden details".to_string(),
            }]),
            ..Extras::default()
        };
        let result: String = render_extras(&extras);
        assert!(result.contains("<details>"));
        assert!(result.contains("<summary>More info</summary>"));
        assert!(result.contains("Hidden details"));
        assert!(result.contains("</details>"));
    }

    #[test]
    fn test_render_custom_markdown() {
        let extras = Extras {
            custom_blocks: Some(vec!["Custom **markdown** here".to_string()]),
            ..Extras::default()
        };
        let result: String = render_extras(&extras);
        assert!(result.contains("Custom **markdown** here"));
    }

    #[test]
    fn test_render_empty_sections_omitted() {
        let social = Social::default();
        assert!(render_social(&social).is_empty());

        let skills = Skills::default();
        assert!(render_skills(&skills).is_empty());

        let stats = Stats {
            stats_card: None,
            top_langs: None,
            streak: None,
            contributor_stats: None,
            trophies: None,
            contribution_snake: None,
            profile_views: None,
            theme: None,
            hide_border: None,
            top_langs_layout: None,
            top_langs_count: None,
        };
        assert!(render_stats(&stats, &test_meta()).is_empty());

        let projects = Projects::default();
        assert!(render_projects(&projects, &test_meta(), "default", false).is_empty());

        let blog = Blog::default();
        assert!(render_blog(&blog).is_empty());

        let extras = Extras::default();
        assert!(render_extras(&extras).is_empty());
    }

    #[test]
    fn test_render_dynamic_spotify() {
        let dynamic = Dynamic {
            spotify_uid: Some("USER".to_string()),
            wakatime: None,
            github_activity: None,
            stackoverflow_uid: None,
        };
        let result: String = render_dynamic(&dynamic);
        assert!(result.contains("### Spotify"));
        assert!(result.contains("uid=USER"));
    }

    #[test]
    fn test_render_dynamic_wakatime() {
        let dynamic = Dynamic {
            spotify_uid: None,
            wakatime: Some(true),
            github_activity: None,
            stackoverflow_uid: None,
        };
        let result: String = render_dynamic(&dynamic);
        assert!(result.contains("<!-- WAKATIME:START -->"));
    }

    #[test]
    fn test_render_dynamic_github_activity() {
        let dynamic = Dynamic {
            spotify_uid: None,
            wakatime: None,
            github_activity: Some(true),
            stackoverflow_uid: None,
        };
        let result: String = render_dynamic(&dynamic);
        assert!(result.contains("<!--START_SECTION:activity-->"));
    }
}
