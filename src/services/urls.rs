/// URL builder for github-readme-stats card.
pub fn github_stats_url(username: &str, theme: &str, show_icons: bool, hide_border: bool) -> String {
    format!(
        "https://github-readme-stats.vercel.app/api?username={}&theme={}&show_icons={}&hide_border={}",
        username, theme, show_icons, hide_border
    )
}

/// URL builder for top languages card.
pub fn top_langs_url(username: &str, layout: &str, langs_count: u32, theme: &str, hide_border: bool) -> String {
    format!(
        "https://github-readme-stats.vercel.app/api/top-langs/?username={}&layout={}&langs_count={}&theme={}&hide_border={}",
        username, layout, langs_count, theme, hide_border
    )
}

/// URL builder for GitHub streak stats.
pub fn streak_stats_url(username: &str, theme: &str, hide_border: bool) -> String {
    format!(
        "https://streak-stats.demolab.com/?user={}&theme={}&hide_border={}",
        username, theme, hide_border
    )
}

/// URL builder for typing SVG animation.
pub fn typing_svg_url(lines: &[&str], font: &str, color: &str, center: bool) -> String {
    let lines_param: String = lines.join(";");
    format!(
        "https://readme-typing-svg.demolab.com/?lines={}&font={}&color={}&center={}&width=440&height=45&vCenter=true&pause=1000&size=22",
        url_encode(&lines_param),
        url_encode(font),
        color,
        center
    )
}

/// URL builder for shields.io badge.
pub fn shields_badge_url(label: &str, color: &str, logo: &str, style: &str) -> String {
    let encoded_label: String = label.replace('-', "--").replace(' ', "_");
    format!(
        "https://img.shields.io/badge/{}-{}?style={}&logo={}&logoColor=white",
        encoded_label, color, style, logo
    )
}

/// Convenience: shields.io badge with "for-the-badge" style.
pub fn skill_badge_url(label: &str, color: &str, logo: &str) -> String {
    shields_badge_url(label, color, logo, "for-the-badge")
}

/// URL builder for social link badge (for-the-badge with clickable link).
pub fn social_badge_markdown(label: &str, color: &str, logo: &str, url: &str) -> String {
    let badge: String = shields_badge_url(label, color, logo, "for-the-badge");
    format!("[![{}]({})]({})", label, badge, url)
}

/// URL builder for komarev profile views counter.
pub fn profile_views_url(username: &str) -> String {
    format!(
        "https://komarev.com/ghpvc/?username={}&color=blue&style=flat",
        username
    )
}

/// URL builder for spotify-github-profile.
pub fn spotify_url(uid: &str) -> String {
    format!(
        "https://spotify-github-profile.kittinan.vercel.app/api/view?uid={}&cover_image=true",
        uid
    )
}

/// Spotify profile badge markdown (with redirect link).
pub fn spotify_badge_markdown(uid: &str) -> String {
    let img_url: String = spotify_url(uid);
    let link_url: String = format!(
        "https://spotify-github-profile.kittinan.vercel.app/api/view?uid={}&redirect=true",
        uid
    );
    format!("[![spotify-github-profile]({})]({})", img_url, link_url)
}

/// URL builder for github-readme-stats pin card.
pub fn pin_card_url(username: &str, repo: &str, theme: &str, hide_border: bool) -> String {
    format!(
        "https://github-readme-stats.vercel.app/api/pin/?username={}&repo={}&theme={}&hide_border={}",
        username, repo, theme, hide_border
    )
}

/// URL builder for github-readme-streak-stats contributor card.
pub fn contributor_stats_url(username: &str, theme: &str, hide_border: bool) -> String {
    format!(
        "https://github-contributor-stats.vercel.app/api?username={}&theme={}&hide_border={}",
        username, theme, hide_border
    )
}

/// URL builder for github-profile-trophy.
pub fn trophies_url(username: &str, theme: &str) -> String {
    format!(
        "https://github-profile-trophy.vercel.app/?username={}&theme={}",
        username, theme
    )
}

/// URL builder for StackOverflow flair badge.
pub fn stackoverflow_badge_url(uid: &str) -> String {
    format!(
        "https://stackoverflow.com/users/flair/{}.png",
        uid
    )
}

/// Simple percent-encoding for URL query parameters.
fn url_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            ' ' => result.push_str("%20"),
            '+' => result.push_str("%2B"),
            '&' => result.push_str("%26"),
            '=' => result.push_str("%3D"),
            '#' => result.push_str("%23"),
            '?' => result.push_str("%3F"),
            _ => result.push(ch),
        }
    }
    result
}

/// Lookup table mapping skill names to (simple-icons slug, hex color).
/// Returns (slug, color) or None if not found.
pub fn skill_icon_lookup(skill: &str) -> Option<(&'static str, &'static str)> {
    match skill.to_lowercase().as_str() {
        // Languages
        "rust" => Some(("rust", "000000")),
        "python" => Some(("python", "3776AB")),
        "typescript" => Some(("typescript", "3178C6")),
        "javascript" => Some(("javascript", "F7DF1E")),
        "go" => Some(("go", "00ADD8")),
        "c" => Some(("c", "A8B9CC")),
        "c++" => Some(("cplusplus", "00599C")),
        "java" => Some(("openjdk", "ED8B00")),
        "kotlin" => Some(("kotlin", "7F52FF")),
        "swift" => Some(("swift", "FA7343")),
        "ruby" => Some(("ruby", "CC342D")),
        "php" => Some(("php", "777BB4")),
        "elixir" => Some(("elixir", "4B275F")),
        "haskell" => Some(("haskell", "5D4F85")),
        "scala" => Some(("scala", "DC322F")),
        "zig" => Some(("zig", "F7A41D")),
        "lua" => Some(("lua", "2C2D72")),
        "dart" => Some(("dart", "0175C2")),
        "r" => Some(("r", "276DC3")),
        "shell" | "bash" | "shell/bash" => Some(("gnubash", "4EAA25")),
        // Frameworks
        "react" => Some(("react", "61DAFB")),
        "vue" | "vue.js" => Some(("vuedotjs", "4FC08D")),
        "svelte" => Some(("svelte", "FF3E00")),
        "angular" => Some(("angular", "0F0F11")),
        "next.js" | "nextjs" => Some(("nextdotjs", "000000")),
        "actix" => Some(("actix", "000000")),
        "axum" => Some(("rust", "000000")),
        "rocket" => Some(("rocket", "D33847")),
        "django" => Some(("django", "092E20")),
        "flask" => Some(("flask", "000000")),
        "fastapi" => Some(("fastapi", "009688")),
        "rails" | "ruby on rails" => Some(("rubyonrails", "CC0000")),
        "spring" => Some(("spring", "6DB33F")),
        "express" | "express.js" => Some(("express", "000000")),
        "gin" => Some(("go", "00ADD8")),
        "echo" => Some(("go", "00ADD8")),
        // Tools
        "docker" => Some(("docker", "2496ED")),
        "git" => Some(("git", "F05032")),
        "neovim" => Some(("neovim", "57A143")),
        "vs code" | "vscode" => Some(("visualstudiocode", "007ACC")),
        "github actions" => Some(("githubactions", "2088FF")),
        "terraform" => Some(("terraform", "844FBA")),
        "ansible" => Some(("ansible", "EE0000")),
        "kubernetes" => Some(("kubernetes", "326CE5")),
        "nginx" => Some(("nginx", "009639")),
        "linux" => Some(("linux", "FCC624")),
        // Databases
        "postgresql" | "postgres" => Some(("postgresql", "4169E1")),
        "mysql" => Some(("mysql", "4479A1")),
        "sqlite" => Some(("sqlite", "003B57")),
        "mongodb" => Some(("mongodb", "47A248")),
        "redis" => Some(("redis", "FF4438")),
        "dynamodb" => Some(("amazondynamodb", "4053D6")),
        "cockroachdb" => Some(("cockroachlabs", "6933FF")),
        "clickhouse" => Some(("clickhouse", "FFCC01")),
        // Cloud/Infra
        "aws" => Some(("amazonwebservices", "232F3E")),
        "gcp" | "google cloud" => Some(("googlecloud", "4285F4")),
        "azure" => Some(("microsoftazure", "0078D4")),
        "vercel" => Some(("vercel", "000000")),
        "cloudflare" => Some(("cloudflare", "F38020")),
        "digitalocean" => Some(("digitalocean", "0080FF")),
        "fly.io" => Some(("flydotio", "24175B")),
        "heroku" => Some(("heroku", "430098")),
        _ => None,
    }
}

/// Social platform metadata: (display_label, logo_slug, badge_color).
pub fn social_platform_info(platform: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match platform.to_lowercase().as_str() {
        "github" => Some(("GitHub", "github", "181717")),
        "twitter" | "x" => Some(("Twitter", "x", "000000")),
        "linkedin" => Some(("LinkedIn", "linkedin", "0077B5")),
        "mastodon" => Some(("Mastodon", "mastodon", "6364FF")),
        "bluesky" => Some(("Bluesky", "bluesky", "0085FF")),
        "instagram" => Some(("Instagram", "instagram", "E4405F")),
        "youtube" => Some(("YouTube", "youtube", "FF0000")),
        "discord" => Some(("Discord", "discord", "5865F2")),
        "devto" | "dev.to" => Some(("Dev.to", "devdotto", "0A0A0A")),
        "hashnode" => Some(("Hashnode", "hashnode", "2962FF")),
        "medium" => Some(("Medium", "medium", "000000")),
        "stackoverflow" => Some(("StackOverflow", "stackoverflow", "F58025")),
        "reddit" => Some(("Reddit", "reddit", "FF4500")),
        "twitch" => Some(("Twitch", "twitch", "9146FF")),
        "website" => Some(("Website", "googlechrome", "4285F4")),
        "email" => Some(("Email", "gmail", "EA4335")),
        "kofi" | "ko-fi" => Some(("Ko-fi", "kofi", "FF5E5B")),
        "rss" => Some(("RSS", "rss", "FFA500")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_stats_url() {
        let url: String = github_stats_url("alice", "tokyonight", true, false);
        assert_eq!(
            url,
            "https://github-readme-stats.vercel.app/api?username=alice&theme=tokyonight&show_icons=true&hide_border=false"
        );
    }

    #[test]
    fn test_top_langs_url() {
        let url: String = top_langs_url("alice", "compact", 8, "tokyonight", false);
        assert_eq!(
            url,
            "https://github-readme-stats.vercel.app/api/top-langs/?username=alice&layout=compact&langs_count=8&theme=tokyonight&hide_border=false"
        );
    }

    #[test]
    fn test_streak_stats_url() {
        let url: String = streak_stats_url("alice", "tokyonight", false);
        assert_eq!(
            url,
            "https://streak-stats.demolab.com/?user=alice&theme=tokyonight&hide_border=false"
        );
    }

    #[test]
    fn test_typing_svg_url() {
        let url: String = typing_svg_url(&["Hello", "World"], "Fira Code", "f75c7e", true);
        assert!(url.starts_with("https://readme-typing-svg.demolab.com/"));
        assert!(url.contains("lines=Hello;World"));
        assert!(url.contains("font=Fira%20Code"));
        assert!(url.contains("color=f75c7e"));
        assert!(url.contains("center=true"));
    }

    #[test]
    fn test_shields_badge_url() {
        let url: String = shields_badge_url("Rust", "000000", "rust", "for-the-badge");
        assert_eq!(
            url,
            "https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"
        );
    }

    #[test]
    fn test_shields_badge_url_with_spaces_and_dashes() {
        let url: String = shields_badge_url("Vue.js", "4FC08D", "vuedotjs", "for-the-badge");
        assert!(url.contains("Vue.js-4FC08D"));

        let url: String = shields_badge_url("C++", "00599C", "cplusplus", "for-the-badge");
        assert!(url.contains("C++-00599C"));
    }

    #[test]
    fn test_profile_views_url() {
        let url: String = profile_views_url("alice");
        assert_eq!(
            url,
            "https://komarev.com/ghpvc/?username=alice&color=blue&style=flat"
        );
    }

    #[test]
    fn test_spotify_url() {
        let url: String = spotify_url("USER");
        assert_eq!(
            url,
            "https://spotify-github-profile.kittinan.vercel.app/api/view?uid=USER&cover_image=true"
        );
    }

    #[test]
    fn test_pin_card_url() {
        let url: String = pin_card_url("alice", "cool-cli", "tokyonight", false);
        assert_eq!(
            url,
            "https://github-readme-stats.vercel.app/api/pin/?username=alice&repo=cool-cli&theme=tokyonight&hide_border=false"
        );
    }

    #[test]
    fn test_contributor_stats_url() {
        let url: String = contributor_stats_url("alice", "tokyonight", false);
        assert_eq!(
            url,
            "https://github-contributor-stats.vercel.app/api?username=alice&theme=tokyonight&hide_border=false"
        );
    }

    #[test]
    fn test_trophies_url() {
        let url: String = trophies_url("alice", "tokyonight");
        assert_eq!(
            url,
            "https://github-profile-trophy.vercel.app/?username=alice&theme=tokyonight"
        );
    }

    #[test]
    fn test_skill_badge_url() {
        let url: String = skill_badge_url("Rust", "000000", "rust");
        assert!(url.contains("style=for-the-badge"));
        assert!(url.contains("logo=rust"));
    }

    #[test]
    fn test_social_badge_markdown() {
        let md: String = social_badge_markdown("Twitter", "000000", "x", "https://twitter.com/alice");
        assert!(md.starts_with("[![Twitter]"));
        assert!(md.contains("https://twitter.com/alice"));
        assert!(md.contains("style=for-the-badge"));
    }

    #[test]
    fn test_skill_icon_lookup() {
        assert_eq!(skill_icon_lookup("Rust"), Some(("rust", "000000")));
        assert_eq!(skill_icon_lookup("python"), Some(("python", "3776AB")));
        assert_eq!(skill_icon_lookup("TYPESCRIPT"), Some(("typescript", "3178C6")));
        assert_eq!(skill_icon_lookup("Docker"), Some(("docker", "2496ED")));
        assert_eq!(skill_icon_lookup("PostgreSQL"), Some(("postgresql", "4169E1")));
        assert_eq!(skill_icon_lookup("AWS"), Some(("amazonwebservices", "232F3E")));
        assert!(skill_icon_lookup("unknown-tech").is_none());
    }

    #[test]
    fn test_social_platform_info() {
        assert_eq!(social_platform_info("twitter"), Some(("Twitter", "x", "000000")));
        assert_eq!(social_platform_info("LinkedIn"), Some(("LinkedIn", "linkedin", "0077B5")));
        assert_eq!(social_platform_info("github"), Some(("GitHub", "github", "181717")));
        assert!(social_platform_info("nonexistent").is_none());
    }

    #[test]
    fn test_spotify_badge_markdown() {
        let md: String = spotify_badge_markdown("USER");
        assert!(md.contains("spotify-github-profile"));
        assert!(md.contains("uid=USER"));
        assert!(md.contains("redirect=true"));
    }

    #[test]
    fn test_stackoverflow_badge_url() {
        let url: String = stackoverflow_badge_url("12345");
        assert_eq!(url, "https://stackoverflow.com/users/flair/12345.png");
    }

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("Hello World"), "Hello%20World");
        assert_eq!(url_encode("a+b"), "a%2Bb");
        assert_eq!(url_encode("key=val&key2=val2"), "key%3Dval%26key2%3Dval2");
    }
}
