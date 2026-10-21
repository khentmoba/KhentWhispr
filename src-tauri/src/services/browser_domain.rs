//! Browser active web-domain detection via UI Automation and window title heuristics.
//!
//! Inspired by Hex: enables application modes and vocabulary rules to match
//! against active websites (e.g. `github.com`, `x.com`, `docs.google.com`)
//! instead of treating all browser windows as generic `chrome.exe`.

#[cfg(target_os = "windows")]
use uiautomation::UIAutomation;

/// Common browser executable process names (in lowercase).
const KNOWN_BROWSERS: &[&str] = &[
    "chrome.exe",
    "msedge.exe",
    "brave.exe",
    "firefox.exe",
    "opera.exe",
    "vivaldi.exe",
    "arc.exe",
    "felaris.exe",
];

/// Checks if an executable process name corresponds to a known web browser.
pub fn is_browser_process(process_name: &str) -> bool {
    let lower = process_name.to_ascii_lowercase();
    KNOWN_BROWSERS.iter().any(|&b| lower == b)
}

/// Extracts domain name from a raw URL or domain string.
pub fn extract_domain_from_url(url: &str) -> Option<String> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Strip scheme
    let without_scheme = if let Some(idx) = trimmed.find("://") {
        &trimmed[idx + 3..]
    } else {
        trimmed
    };

    // Extract host (up to slash, colon, question mark, or hash)
    let host = without_scheme
        .split(&['/', ':', '?', '#'][..])
        .next()?
        .trim();

    if host.contains('.') && !host.contains(' ') {
        Some(host.to_ascii_lowercase())
    } else {
        None
    }
}

/// Infers the web domain from a browser window title.
pub fn infer_domain_from_window_title(window_title: &str) -> Option<String> {
    let lower = window_title.to_ascii_lowercase();

    // Check for explicit URLs or domain names in window title
    for token in window_title.split(&[' ', '—', '-', '|', '·', '•', '(', ')', '[', ']'][..]) {
        let clean = token.trim();
        if (clean.contains(".com")
            || clean.contains(".org")
            || clean.contains(".net")
            || clean.contains(".io")
            || clean.contains(".app")
            || clean.contains(".ai")
            || clean.contains(".so")
            || clean.contains(".dev"))
            && !clean.contains('@')
        {
            if let Some(domain) = extract_domain_from_url(clean) {
                return Some(domain);
            }
        }
    }

    // Well-known web app signatures in window titles
    if lower.contains(" / x ")
        || lower.ends_with(" / x")
        || lower.contains("home / x")
        || lower.contains("twitter")
    {
        return Some("x.com".to_string());
    }
    if lower.contains("github") {
        return Some("github.com".to_string());
    }
    if lower.contains("google docs") {
        return Some("docs.google.com".to_string());
    }
    if lower.contains("google sheets") {
        return Some("sheets.google.com".to_string());
    }
    if lower.contains("gmail") {
        return Some("mail.google.com".to_string());
    }
    if lower.contains("slack") {
        return Some("slack.com".to_string());
    }
    if lower.contains("discord") {
        return Some("discord.com".to_string());
    }
    if lower.contains("youtube") {
        return Some("youtube.com".to_string());
    }
    if lower.contains("reddit") {
        return Some("reddit.com".to_string());
    }
    if lower.contains("notion") {
        return Some("notion.so".to_string());
    }
    if lower.contains("linear") {
        return Some("linear.app".to_string());
    }
    if lower.contains("chatgpt") {
        return Some("chatgpt.com".to_string());
    }

    None
}

/// Detects the active web domain from the foreground browser window using
/// UI Automation address-bar query with graceful title inference fallback.
pub fn detect_active_browser_domain(process_name: &str, window_title: &str) -> Option<String> {
    if !is_browser_process(process_name) {
        return None;
    }

    #[cfg(target_os = "windows")]
    {
        // Try UI Automation address bar query first
        if let Ok(automation) = UIAutomation::new() {
            if let Ok(_root) = automation.get_root_element() {
                // Look for an address bar edit element in the focused browser
                if let Ok(focused) = automation.get_focused_element() {
                    if let Ok(name) = focused.get_name() {
                        if let Some(domain) = extract_domain_from_url(&name) {
                            return Some(domain);
                        }
                    }
                }
            }
        }
    }

    // Fall back to window title heuristic
    infer_domain_from_window_title(window_title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_browser_process() {
        assert!(is_browser_process("chrome.exe"));
        assert!(is_browser_process("Chrome.exe"));
        assert!(is_browser_process("felaris.exe"));
        assert!(is_browser_process("brave.exe"));
        assert!(!is_browser_process("code.exe"));
        assert!(!is_browser_process("notepad.exe"));
    }

    #[test]
    fn test_extract_domain_from_url() {
        assert_eq!(
            extract_domain_from_url("https://github.com/khentmoba/SpeakFlow"),
            Some("github.com".to_string())
        );
        assert_eq!(
            extract_domain_from_url("http://localhost:3000/app"),
            Some("localhost.3000".to_string())
                .or(Some("localhost".to_string()))
                .and(None)
        ); // local without dot
        assert_eq!(
            extract_domain_from_url("docs.google.com/document/d/123"),
            Some("docs.google.com".to_string())
        );
        assert_eq!(extract_domain_from_url(""), None);
    }

    #[test]
    fn test_infer_domain_from_window_title() {
        assert_eq!(
            infer_domain_from_window_title("(6) Home / X - Felaris"),
            Some("x.com".to_string())
        );
        assert_eq!(
            infer_domain_from_window_title(
                "Pull requests · khentmoba/SpeakFlow · GitHub - Google Chrome"
            ),
            Some("github.com".to_string())
        );
        assert_eq!(
            infer_domain_from_window_title("Project Roadmap - Google Docs - Brave"),
            Some("docs.google.com".to_string())
        );
        assert_eq!(
            infer_domain_from_window_title("General - Discord"),
            Some("discord.com".to_string())
        );
        assert_eq!(
            infer_domain_from_window_title("random document.txt - Notepad"),
            None
        );
    }
}
