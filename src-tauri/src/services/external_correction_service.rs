//! Short-lived, local-only learning from the field that received a dictation.
use std::sync::atomic::{AtomicU64, Ordering};

static GENERATION: AtomicU64 = AtomicU64::new(0);
const MAX_TEXT: usize = 16_384;

pub fn cancel() -> u64 {
    GENERATION.fetch_add(1, Ordering::SeqCst) + 1
}

#[derive(Debug)]
pub struct Snapshot {
    window: isize,
    field: Vec<i32>,
    text: String,
}

pub fn capture() -> Option<Snapshot> {
    capture_field(None)
}

fn capture_field(expected: Option<&Snapshot>) -> Option<Snapshot> {
    #[cfg(target_os = "windows")]
    {
        use uiautomation::{
            patterns::{UITextPattern, UIValuePattern},
            types::ControlType,
            UIAutomation,
        };
        use windows_sys::Win32::{
            System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED},
            UI::WindowsAndMessaging::GetForegroundWindow,
        };
        struct ComGuard;
        impl Drop for ComGuard {
            fn drop(&mut self) {
                unsafe { CoUninitialize() };
            }
        }
        let window = unsafe { GetForegroundWindow() } as isize;
        if window == 0 || expected.is_some_and(|s| s.window != window) {
            return None;
        }
        if unsafe { CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED as u32) } < 0 {
            return None;
        }
        let _com = ComGuard;
        let automation = UIAutomation::new_direct().ok()?;
        let focused = automation.get_focused_element().ok()?;
        if focused.is_password().ok()? || focused.get_process_id().ok()? == std::process::id() {
            return None;
        }
        let walker = automation.get_control_view_walker().ok()?;
        let mut candidate = focused.clone();
        let mut input = focused.clone();
        // Editors may focus a child of the input; track the input, not that child.
        for _ in 0..6 {
            if candidate.get_process_id().ok()? != focused.get_process_id().ok()? {
                break;
            }
            if candidate.is_password().ok()? {
                return None;
            }
            if candidate.get_control_type().ok()? == ControlType::Edit {
                input = candidate;
                break;
            }
            let Ok(parent) = walker.get_parent(&candidate) else {
                break;
            };
            candidate = parent;
        }
        let field = input.get_runtime_id().ok()?;
        if field.is_empty() || expected.is_some_and(|s| s.field != field) {
            return None;
        }
        let text = input
            .get_pattern::<UITextPattern>()
            .ok()
            .and_then(|pattern| pattern.get_document_range().ok())
            .and_then(|range| range.get_text(MAX_TEXT as i32 + 1).ok())
            .or_else(|| {
                // Never fall back to reading an ancestor document or a read-only value.
                if input.get_control_type().ok()? != ControlType::Edit {
                    return None;
                }
                let value = input.get_pattern::<UIValuePattern>().ok()?;
                if value.is_readonly().ok()? {
                    return None;
                }
                // shortcut: ValuePattern has no bounded read; reject oversized values below.
                value.get_value().ok()
            })?;
        if text.encode_utf16().count() > MAX_TEXT
            || unsafe { GetForegroundWindow() } as isize != window
            || !automation
                .compare_elements(&focused, &automation.get_focused_element().ok()?)
                .ok()?
        {
            return None;
        }
        Some(Snapshot {
            window,
            field,
            text: normalize_line_endings(&text),
        })
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = expected;
        None
    }
}

// UI Automation providers expose multiline inputs as CRLF, CR, or LF.
fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

// Require a uniquely identifiable insertion; never guess which duplicate was pasted.
fn insertion<'a>(before: &str, after: &'a str, pasted: &str) -> Option<(&'a str, &'a str)> {
    if pasted.is_empty() || pasted.chars().count() > MAX_TEXT {
        return None;
    }
    let mut found = None;
    for (start, _) in after
        .char_indices()
        .filter(|(i, _)| after[*i..].starts_with(pasted))
    {
        let prefix = &after[..start];
        let suffix = &after[start + pasted.len()..];
        if before.len() == prefix.len() + suffix.len()
            && before.starts_with(prefix)
            && before.ends_with(suffix)
        {
            if found.is_some() {
                return None;
            }
            found = Some((prefix, suffix));
        }
    }
    found
}

fn passage<'a>(text: &'a str, prefix: &str, suffix: &str) -> Option<&'a str> {
    if text.len() < prefix.len() + suffix.len() {
        return None;
    }
    text.strip_prefix(prefix)?.strip_suffix(suffix)
}

fn tokens(text: &str) -> Vec<&str> {
    let mut start = 0;
    let mut result = Vec::new();
    let mut word = None;
    for (index, ch) in text.char_indices() {
        let next = ch.is_alphanumeric();
        if word.is_some_and(|previous| previous != next) {
            result.push(&text[start..index]);
            start = index;
        }
        word = Some(next);
    }
    result.push(&text[start..]);
    result
}

// Only a single short word substitution with a small edit distance is eligible.
// shortcut: whitespace/phrase edits and long terms are skipped; expand only with labeled examples.
fn correction(before: &str, after: &str) -> Option<(String, String)> {
    let original = tokens(before);
    let updated = tokens(after);
    if original.len() != updated.len() {
        return None;
    }
    let mut changes = original.iter().zip(&updated).filter(|(a, b)| a != b);
    let (&from, &to) = changes.next()?;
    if changes.next().is_some()
        || !from.chars().all(char::is_alphanumeric)
        || !to.chars().all(char::is_alphanumeric)
        || !from.chars().any(char::is_alphabetic)
        || !to.chars().any(char::is_alphabetic)
        || !from
            .chars()
            .filter(|c| c.is_numeric())
            .eq(to.chars().filter(|c| c.is_numeric()))
    {
        return None;
    }
    let a: Vec<char> = from.chars().collect();
    let b: Vec<char> = to.chars().collect();
    if a.is_empty() || b.is_empty() || a.len() > 24 || b.len() > 24 {
        return None;
    }
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, left) in a.iter().enumerate() {
        let mut diagonal = row[0];
        row[0] = i + 1;
        for (j, right) in b.iter().enumerate() {
            let previous = row[j + 1];
            row[j + 1] = (diagonal + usize::from(left != right))
                .min(row[j] + 1)
                .min(previous + 1);
            diagonal = previous;
        }
    }
    let swapped = a.len() == b.len()
        && (0..a.len().saturating_sub(1)).any(|i| {
            a[i] == b[i + 1] && a[i + 1] == b[i] && a[..i] == b[..i] && a[i + 2..] == b[i + 2..]
        });
    let distance = if swapped { 1 } else { row[b.len()] };
    if distance == 0 || distance > 2 || distance * 2 > a.len().max(b.len()) {
        return None;
    }
    Some((from.to_string(), to.to_string()))
}

/// Apply only unambiguous, user-confirmed spelling rules, without chaining replacements.
pub fn apply_spelling_rules(
    text: &str,
    profile: &crate::state::user_profile::UserProfile,
) -> String {
    use crate::state::user_profile::CorrectionSource;
    if !profile.external_correction_learning {
        return text.to_string();
    }
    let rules: Vec<_> = profile
        .correction_patterns
        .iter()
        .filter(|rule| {
            rule.source == CorrectionSource::User
                && correction(&rule.original, &rule.corrected).is_some()
        })
        .collect();
    tokens(text)
        .into_iter()
        .map(|token| {
            let mut matches = rules.iter().filter(|rule| rule.original == token);
            let Some(first) = matches.next() else {
                return token;
            };
            if matches.any(|rule| rule.corrected != first.corrected) {
                return token;
            }
            first.corrected.as_str()
        })
        .collect()
}

fn corrections(before: &str, after: &str) -> Option<Vec<(String, String)>> {
    let original = tokens(before);
    let updated = tokens(after);
    if updated.len() < original.len() {
        return None;
    }
    if updated.len() > original.len()
        && original
            .last()
            .is_some_and(|last| last.chars().any(char::is_alphanumeric))
        && updated[original.len()].chars().any(char::is_alphanumeric)
    {
        return None;
    }
    let last_idx = original.len().saturating_sub(1);
    let pairs: Option<Vec<_>> = original
        .iter()
        .enumerate()
        .zip(updated.iter().take(original.len()))
        .filter(|&((i, a), b)| {
            if a == b {
                return false;
            }
            !(i == last_idx
                && updated.len() > original.len()
                && !a.chars().any(char::is_alphanumeric)
                && b.starts_with(*a))
        })
        .map(|((_, a), b)| correction(a, b))
        .collect();
    pairs.filter(|pairs| !pairs.is_empty())
}

pub fn watch(app: tauri::AppHandle, before: Snapshot, pasted: String, generation: u64) {
    use crate::{
        services::profile_service,
        state::{user_profile::CorrectionSource, AppState},
    };
    use std::time::{Duration, Instant};
    use tauri::Manager;

    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let pasted = normalize_line_endings(&pasted);
        let active = || {
            GENERATION.load(Ordering::SeqCst) == generation
                && state.with_profile(|p| p.external_correction_learning)
                && state.recording.recording.lock().is_none()
        };
        // Clipboard consumers may apply the paste asynchronously.
        let mut baseline = None;
        for _ in 0..5 {
            std::thread::sleep(Duration::from_millis(200));
            if !active() {
                return;
            }
            let Some(current) = capture_field(Some(&before)) else {
                return;
            };
            if insertion(&before.text, &current.text, &pasted).is_some() {
                baseline = Some(current.text);
                break;
            }
            if current.text != before.text {
                return;
            }
        }
        let Some(baseline) = baseline else {
            return;
        };
        let Some((prefix, suffix)) = insertion(&before.text, &baseline, &pasted) else {
            return;
        };
        let deadline = Instant::now() + Duration::from_secs(60);
        let mut learned = std::collections::HashSet::new();
        let mut latest = pasted.clone();
        let mut changed_at = Instant::now();
        while Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(400));
            if !active() {
                return;
            }
            let Some(current) = capture_field(Some(&before)) else {
                continue;
            };
            let Some(edited) = passage(&current.text, prefix, suffix) else {
                continue;
            };
            if edited != latest {
                latest = edited.to_string();
                changed_at = Instant::now();
                continue;
            }
            if latest == pasted || changed_at.elapsed() < Duration::from_secs(2) {
                continue;
            }
            let Some(pairs) = corrections(&pasted, &latest) else {
                continue;
            };
            let pairs: Vec<_> = pairs
                .into_iter()
                .filter(|pair| learned.insert(pair.clone()))
                .collect();
            if pairs.is_empty() {
                continue;
            }
            // Do not invoke the LLM: surrounding document text never leaves this process.
            profile_service::update_profile_and_schedule(&state, |profile| {
                if profile.external_correction_learning
                    && GENERATION.load(Ordering::SeqCst) == generation
                {
                    profile_service::learn_from_structured(
                        profile,
                        &pairs,
                        &[],
                        CorrectionSource::User,
                    );
                }
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learning_defaults_on_and_preserves_an_explicit_opt_out() {
        use crate::state::user_profile::UserProfile;
        assert!(UserProfile::default().external_correction_learning);
        let mut legacy = serde_json::to_value(UserProfile::default()).unwrap();
        legacy
            .as_object_mut()
            .unwrap()
            .remove("external_correction_learning");
        let mut profile: UserProfile = serde_json::from_value(legacy).unwrap();
        assert!(profile.external_correction_learning);
        profile.external_correction_learning = false;
        let loaded: UserProfile =
            serde_json::from_str(&serde_json::to_string(&profile).unwrap()).unwrap();
        assert!(!loaded.external_correction_learning);
    }

    #[test]
    fn learned_spelling_is_applied_without_ai_or_replacement_chains() {
        use crate::services::profile_service::learn_from_structured;
        use crate::state::user_profile::{CorrectionSource, UserProfile};
        let mut profile = UserProfile {
            external_correction_learning: true,
            ..Default::default()
        };
        let pairs = corrections("teh Speekflow app", "the Speakflow app").unwrap();
        learn_from_structured(&mut profile, &pairs, &[], CorrectionSource::User);
        assert_eq!(
            apply_spelling_rules("teh Speekflow, not Speekflows.", &profile),
            "the Speakflow, not Speekflows."
        );
        learn_from_structured(
            &mut profile,
            &[("Speakflow".into(), "Speakfloo".into())],
            &[],
            CorrectionSource::User,
        );
        assert_eq!(apply_spelling_rules("Speekflow", &profile), "Speakflow");
        learn_from_structured(
            &mut profile,
            &[("Speekflow".into(), "Speekfloo".into())],
            &[],
            CorrectionSource::User,
        );
        assert_eq!(apply_spelling_rules("Speekflow", &profile), "Speekflow");
        profile.external_correction_learning = false;
        assert_eq!(apply_spelling_rules("teh", &profile), "teh");
    }

    #[test]
    fn multiple_edits_must_all_be_small_word_corrections() {
        assert_eq!(corrections("teh recieve", "the receive").unwrap().len(), 2);
        assert_eq!(
            corrections("teh Speekflow", "the Speakflow is great").unwrap(),
            vec![
                ("teh".into(), "the".into()),
                ("Speekflow".into(), "Speakflow".into()),
            ]
        );
        assert_eq!(
            corrections("Use configuraton.", "Use configuration. Next sentence").unwrap(),
            vec![("configuraton".into(), "configuration".into())]
        );
        assert_eq!(corrections("hello", "hello there"), None);
        assert_eq!(corrections("teh hello!", "the hello?"), None);
        assert_eq!(corrections("teh hello", "the goodbye"), None);
        assert_eq!(corrections("hello", "hello"), None);
    }

    #[test]
    fn matches_multiline_pastes_across_windows_text_providers() {
        let before = normalize_line_endings("Before\r\n\r\nafter");
        let pasted = normalize_line_endings("teh\nrecieve");
        for raw in [
            "Before\r\nteh\r\nrecieve\r\nafter",
            "Before\rteh\rrecieve\rafter",
            "Before\nteh\nrecieve\nafter",
        ] {
            let baseline = normalize_line_endings(raw);
            let (prefix, suffix) = insertion(&before, &baseline, &pasted).unwrap();
            let edited = normalize_line_endings("Before\r\nthe\r\nreceive\r\nafter");
            let edited = passage(&edited, prefix, suffix).unwrap();
            assert_eq!(corrections(&pasted, edited).unwrap().len(), 2);
        }
    }

    #[test]
    fn identifies_only_the_inserted_passage() {
        assert_eq!(
            insertion("Before  after", "Before teh after", "teh"),
            Some(("Before ", " after"))
        );
        assert_eq!(insertion("hi", "hihi", "hi"), None);
        assert_eq!(insertion("a", "aaa", "aa"), None);
        assert_eq!(insertion("selected", "replacement", "replacement"), None);
        assert_eq!(insertion("", "", ""), None);
        assert_eq!(
            passage("Before the after", "Before ", " after"),
            Some("the")
        );
        assert_eq!(passage("Different the after", "Before ", " after"), None);
        assert_eq!(passage("short", "long prefix", "suffix"), None);
    }

    #[test]
    fn learns_whole_words_not_character_fragments() {
        assert_eq!(
            correction("Use Speekflow today.", "Use Speakflow today."),
            Some(("Speekflow".into(), "Speakflow".into()))
        );
        assert_eq!(correction("teh", "the"), Some(("teh".into(), "the".into())));
        assert_eq!(
            correction("recieve", "receive"),
            Some(("recieve".into(), "receive".into()))
        );
        assert_eq!(
            correction("café", "cafe"),
            Some(("café".into(), "cafe".into()))
        );
    }

    #[test]
    fn rejects_rewrites_additions_deletions_and_punctuation() {
        for (before, after) in [
            ("hello", "hello there"),
            ("hello there", "hello"),
            ("send it", "keep it"),
            ("hello!", "hello?"),
            ("one two", "won too"),
            ("hello", "hello"),
            ("", ""),
            ("word", ""),
            ("a", "b"),
            ("1000", "2000"),
            ("2025", "2026"),
            ("GPT4", "GPT5"),
        ] {
            assert_eq!(correction(before, after), None, "{before} -> {after}");
        }
    }
}
