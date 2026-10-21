//! Zero-LLM deterministic text transformation modes.
//!
//! Inspired by Hex: enables instantaneous, zero-latency text transformations
//! (casing, code-formatting, punctuation stripping, list formatting) without
//! requiring an LLM call or network access.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub enum TextTransformMode {
    #[default]
    None,
    Lowercase,
    Uppercase,
    TitleCase,
    CamelCase,
    SnakeCase,
    KebabCase,
    StripTrailingPunctuation,
    CodeWrap,
    MarkdownList,
}

#[allow(dead_code)]
impl TextTransformMode {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "lowercase" => Self::Lowercase,
            "uppercase" => Self::Uppercase,
            "titlecase" | "title_case" => Self::TitleCase,
            "camelcase" | "camel_case" => Self::CamelCase,
            "snakecase" | "snake_case" => Self::SnakeCase,
            "kebabcase" | "kebab_case" => Self::KebabCase,
            "striptrailingpunctuation" | "strip_punctuation" => Self::StripTrailingPunctuation,
            "codewrap" | "code_wrap" => Self::CodeWrap,
            "markdownlist" | "markdown_list" => Self::MarkdownList,
            _ => Self::None,
        }
    }

    pub fn apply(&self, input: &str) -> String {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return input.to_string();
        }

        match self {
            Self::None => input.to_string(),
            Self::Lowercase => {
                let mut chars = trimmed.chars();
                match chars.next() {
                    Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            }
            Self::Uppercase => trimmed.to_uppercase(),
            Self::TitleCase => trimmed
                .split_whitespace()
                .map(|word| {
                    let mut c = word.chars();
                    match c.next() {
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" "),
            Self::CamelCase => {
                let words: Vec<&str> = trimmed
                    .split(|c: char| c.is_whitespace() || c == '_' || c == '-')
                    .filter(|w| !w.is_empty())
                    .collect();
                if words.is_empty() {
                    return String::new();
                }
                let mut result = words[0].to_lowercase();
                for word in &words[1..] {
                    let mut c = word.chars();
                    if let Some(first) = c.next() {
                        result.push_str(&first.to_uppercase().to_string());
                        result.push_str(&c.as_str().to_lowercase());
                    }
                }
                result
            }
            Self::SnakeCase => {
                let words: Vec<&str> = trimmed
                    .split(|c: char| c.is_whitespace() || c == '_' || c == '-')
                    .filter(|w| !w.is_empty())
                    .collect();
                words
                    .iter()
                    .map(|w| w.to_lowercase())
                    .collect::<Vec<_>>()
                    .join("_")
            }
            Self::KebabCase => {
                let words: Vec<&str> = trimmed
                    .split(|c: char| c.is_whitespace() || c == '_' || c == '-')
                    .filter(|w| !w.is_empty())
                    .collect();
                words
                    .iter()
                    .map(|w| w.to_lowercase())
                    .collect::<Vec<_>>()
                    .join("-")
            }
            Self::StripTrailingPunctuation => trimmed
                .trim_end_matches(|c: char| {
                    c == '.' || c == ',' || c == '!' || c == '?' || c == ';' || c == ':'
                })
                .to_string(),
            Self::CodeWrap => {
                format!("`{trimmed}`")
            }
            Self::MarkdownList => {
                format!("- {trimmed}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_transforms() {
        assert_eq!(
            TextTransformMode::None.apply("Hello World."),
            "Hello World."
        );
        assert_eq!(
            TextTransformMode::Lowercase.apply("Hello World"),
            "hello World"
        );
        assert_eq!(
            TextTransformMode::Uppercase.apply("Hello World"),
            "HELLO WORLD"
        );
        assert_eq!(
            TextTransformMode::TitleCase.apply("hello world"),
            "Hello World"
        );
        assert_eq!(
            TextTransformMode::CamelCase.apply("hello world foo"),
            "helloWorldFoo"
        );
        assert_eq!(
            TextTransformMode::SnakeCase.apply("Hello World Foo"),
            "hello_world_foo"
        );
        assert_eq!(
            TextTransformMode::KebabCase.apply("Hello World Foo"),
            "hello-world-foo"
        );
        assert_eq!(
            TextTransformMode::StripTrailingPunctuation.apply("search query."),
            "search query"
        );
        assert_eq!(
            TextTransformMode::CodeWrap.apply("const x = 10;"),
            "`const x = 10;`"
        );
        assert_eq!(
            TextTransformMode::MarkdownList.apply("first item"),
            "- first item"
        );
    }
}
