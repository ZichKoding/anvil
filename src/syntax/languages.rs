use tree_sitter::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LangId {
    Rust,
    Python,
    JavaScript,
    Json,
    Toml,
    Markdown,
}

impl LangId {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Self::Rust),
            "py" | "pyi" => Some(Self::Python),
            "js" | "jsx" | "mjs" | "cjs" => Some(Self::JavaScript),
            "json" | "jsonc" => Some(Self::Json),
            "toml" => Some(Self::Toml),
            "md" | "markdown" => Some(Self::Markdown),
            _ => None,
        }
    }

    pub fn language(&self) -> Language {
        match self {
            Self::Rust => tree_sitter_rust::LANGUAGE.into(),
            Self::Python => tree_sitter_python::LANGUAGE.into(),
            Self::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
            Self::Json => tree_sitter_json::LANGUAGE.into(),
            Self::Toml => tree_sitter_toml_ng::LANGUAGE.into(),
            Self::Markdown => tree_sitter_md::LANGUAGE.into(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Python => "Python",
            Self::JavaScript => "JavaScript",
            Self::Json => "JSON",
            Self::Toml => "TOML",
            Self::Markdown => "Markdown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Known extensions ---

    #[test]
    fn test_from_extension_rs_is_rust() {
        assert_eq!(LangId::from_extension("rs"), Some(LangId::Rust));
    }

    #[test]
    fn test_from_extension_py_is_python() {
        assert_eq!(LangId::from_extension("py"), Some(LangId::Python));
    }

    #[test]
    fn test_from_extension_pyi_is_python() {
        assert_eq!(LangId::from_extension("pyi"), Some(LangId::Python));
    }

    #[test]
    fn test_from_extension_js_is_javascript() {
        assert_eq!(LangId::from_extension("js"), Some(LangId::JavaScript));
    }

    #[test]
    fn test_from_extension_jsx_is_javascript() {
        assert_eq!(LangId::from_extension("jsx"), Some(LangId::JavaScript));
    }

    #[test]
    fn test_from_extension_mjs_is_javascript() {
        assert_eq!(LangId::from_extension("mjs"), Some(LangId::JavaScript));
    }

    #[test]
    fn test_from_extension_cjs_is_javascript() {
        assert_eq!(LangId::from_extension("cjs"), Some(LangId::JavaScript));
    }

    #[test]
    fn test_from_extension_json_is_json() {
        assert_eq!(LangId::from_extension("json"), Some(LangId::Json));
    }

    #[test]
    fn test_from_extension_jsonc_is_json() {
        assert_eq!(LangId::from_extension("jsonc"), Some(LangId::Json));
    }

    #[test]
    fn test_from_extension_toml_is_toml() {
        assert_eq!(LangId::from_extension("toml"), Some(LangId::Toml));
    }

    #[test]
    fn test_from_extension_md_is_markdown() {
        assert_eq!(LangId::from_extension("md"), Some(LangId::Markdown));
    }

    #[test]
    fn test_from_extension_markdown_is_markdown() {
        assert_eq!(LangId::from_extension("markdown"), Some(LangId::Markdown));
    }

    // --- Unknown extensions ---

    #[test]
    fn test_from_extension_unknown_returns_none() {
        assert_eq!(LangId::from_extension("xyz"), None);
    }

    #[test]
    fn test_from_extension_empty_string_returns_none() {
        assert_eq!(LangId::from_extension(""), None);
    }

    #[test]
    fn test_from_extension_case_sensitive_uppercase_returns_none() {
        // extensions are matched case-sensitively; "RS" is not "rs"
        assert_eq!(LangId::from_extension("RS"), None);
    }

    #[test]
    fn test_from_extension_txt_returns_none() {
        assert_eq!(LangId::from_extension("txt"), None);
    }

    #[test]
    fn test_from_extension_html_returns_none() {
        assert_eq!(LangId::from_extension("html"), None);
    }

    // --- name() sanity checks (no terminal needed) ---

    #[test]
    fn test_name_rust() {
        assert_eq!(LangId::Rust.name(), "Rust");
    }

    #[test]
    fn test_name_python() {
        assert_eq!(LangId::Python.name(), "Python");
    }

    #[test]
    fn test_name_javascript() {
        assert_eq!(LangId::JavaScript.name(), "JavaScript");
    }

    #[test]
    fn test_name_json() {
        assert_eq!(LangId::Json.name(), "JSON");
    }

    #[test]
    fn test_name_toml() {
        assert_eq!(LangId::Toml.name(), "TOML");
    }

    #[test]
    fn test_name_markdown() {
        assert_eq!(LangId::Markdown.name(), "Markdown");
    }
}
