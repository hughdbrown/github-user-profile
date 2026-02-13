use crate::config::profile::Template;

/// The sections that a template includes, in order.
#[derive(Debug, Clone, PartialEq)]
pub enum Section {
    Header,
    About,
    Social,
    Skills,
    Stats,
    Projects,
    Blog,
    Dynamic,
    Sponsors,
    Extras,
}

/// Returns the ordered list of sections for a given template.
pub fn sections_for_template(template: &Template) -> Vec<Section> {
    match template {
        Template::Minimal => vec![Section::Header, Section::About, Section::Social],
        Template::Full => vec![
            Section::Header,
            Section::About,
            Section::Social,
            Section::Skills,
            Section::Stats,
            Section::Projects,
            Section::Blog,
            Section::Dynamic,
            Section::Sponsors,
            Section::Extras,
        ],
        Template::DeveloperCard => vec![
            Section::Header,
            Section::About,
            Section::Skills,
            Section::Stats,
            Section::Projects,
            Section::Social,
        ],
        Template::MultiColumn => vec![
            Section::Header,
            Section::About,
            Section::Social,
            Section::Skills,
            Section::Stats,
            Section::Projects,
            Section::Blog,
            Section::Dynamic,
            Section::Sponsors,
            Section::Extras,
        ],
    }
}

/// Whether to use centered alignment for a template.
pub fn is_centered(template: &Template) -> bool {
    matches!(template, Template::DeveloperCard)
}

/// Whether to use multi-column layout.
pub fn is_multi_column(template: &Template) -> bool {
    matches!(template, Template::MultiColumn)
}

/// Wrap content in centered alignment HTML.
pub fn wrap_centered(content: &str) -> String {
    format!("<div align=\"center\">\n\n{}\n\n</div>", content)
}

/// Wrap sections in a multi-column HTML table.
/// Left column gets the first half of sections, right column gets the rest.
pub fn wrap_multi_column(left: &str, right: &str) -> String {
    format!(
        "<table>\n<tr>\n<td valign=\"top\" width=\"50%\">\n\n{}\n\n</td>\n<td valign=\"top\" width=\"50%\">\n\n{}\n\n</td>\n</tr>\n</table>",
        left, right
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_template_sections() {
        let sections: Vec<Section> = sections_for_template(&Template::Minimal);
        assert_eq!(
            sections,
            vec![Section::Header, Section::About, Section::Social]
        );
    }

    #[test]
    fn test_full_template_sections() {
        let sections: Vec<Section> = sections_for_template(&Template::Full);
        assert_eq!(sections.len(), 10);
        assert_eq!(sections[0], Section::Header);
        assert_eq!(sections[9], Section::Extras);
    }

    #[test]
    fn test_developer_card_template() {
        let sections: Vec<Section> = sections_for_template(&Template::DeveloperCard);
        assert_eq!(sections.len(), 6);
        // Social comes after Projects in DeveloperCard
        assert_eq!(sections[5], Section::Social);
        assert!(is_centered(&Template::DeveloperCard));
    }

    #[test]
    fn test_multi_column_template() {
        assert!(is_multi_column(&Template::MultiColumn));
        assert!(!is_multi_column(&Template::Full));
    }

    #[test]
    fn test_wrap_centered() {
        let result: String = wrap_centered("Hello");
        assert!(result.contains("<div align=\"center\">"));
        assert!(result.contains("Hello"));
        assert!(result.contains("</div>"));
    }

    #[test]
    fn test_wrap_multi_column() {
        let result: String = wrap_multi_column("Left content", "Right content");
        assert!(result.contains("<table>"));
        assert!(result.contains("Left content"));
        assert!(result.contains("Right content"));
        assert!(result.contains("width=\"50%\""));
    }
}
