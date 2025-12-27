use crate::error::FormatParseError;
use regex::Regex;

/// Build a regex from a pattern string with DOTALL flag
pub fn build_regex(pattern: &str) -> Result<Regex, FormatParseError> {
    // Pre-allocate with estimated capacity
    let mut regex_with_flags = String::with_capacity(pattern.len() + 4);
    regex_with_flags.push_str("(?s)");
    regex_with_flags.push_str(pattern);
    Regex::new(&regex_with_flags).map_err(|e| FormatParseError::RegexError(e.to_string()))
}

/// Build a case-insensitive regex from a pattern string with DOTALL flag
pub fn build_case_insensitive_regex(pattern: &str) -> Option<Regex> {
    // Pre-allocate with estimated capacity
    let mut regex_with_flags = String::with_capacity(pattern.len() + 8);
    regex_with_flags.push_str("(?s)(?i)");
    regex_with_flags.push_str(pattern);
    Regex::new(&regex_with_flags).ok()
}

/// Remove anchors and flags from a regex string for search operations
/// Returns a string slice or owned string as needed
pub fn prepare_search_regex(regex_str: &str) -> String {
    let mut start = 0;
    let mut end = regex_str.len();
    
    // Remove (?s) flag if present
    if regex_str.starts_with("(?s)") {
        start = 4;
    }
    
    // Remove ^ anchor
    if regex_str[start..].starts_with("^") {
        start += 1;
    }
    
    // Remove $ anchor
    if regex_str[..end].ends_with("$") {
        end -= 1;
    }
    
    // Only allocate if we need to modify the string
    if start > 0 || end < regex_str.len() {
        regex_str[start..end].to_string()
    } else {
        regex_str.to_string()
    }
}

/// Build a search regex (without anchors) with optional case sensitivity
pub fn build_search_regex(regex_str: &str, case_sensitive: bool) -> Result<Regex, FormatParseError> {
    let search_regex_str = prepare_search_regex(regex_str);
    
    // Pre-allocate with estimated capacity
    let capacity = search_regex_str.len() + if case_sensitive { 4 } else { 8 };
    let mut pattern = String::with_capacity(capacity);
    pattern.push_str("(?s)");
    if !case_sensitive {
        pattern.push_str("(?i)");
    }
    pattern.push_str(&search_regex_str);
    
    Regex::new(&pattern).map_err(|e| FormatParseError::RegexError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_regex() {
        let regex = build_regex(r"^test$").unwrap();
        assert!(regex.is_match("test"));
        assert!(!regex.is_match("TEST"));
        assert!(!regex.is_match("notest"));
    }

    #[test]
    fn test_build_regex_with_dotall() {
        let regex = build_regex(r"test.line").unwrap();
        assert!(regex.is_match("test\nline"));
    }

    #[test]
    fn test_build_case_insensitive_regex() {
        let regex = build_case_insensitive_regex(r"^test$").unwrap();
        assert!(regex.is_match("test"));
        assert!(regex.is_match("TEST"));
        assert!(regex.is_match("Test"));
        assert!(!regex.is_match("notest"));
    }

    #[test]
    fn test_build_case_insensitive_regex_with_dotall() {
        let regex = build_case_insensitive_regex(r"test.line").unwrap();
        assert!(regex.is_match("TEST\nLINE"));
    }

    #[test]
    fn test_prepare_search_regex_no_anchors() {
        let result = prepare_search_regex(r"test");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_prepare_search_regex_with_anchors() {
        let result = prepare_search_regex(r"^test$");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_prepare_search_regex_with_dotall() {
        let result = prepare_search_regex(r"(?s)^test$");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_prepare_search_regex_start_anchor_only() {
        let result = prepare_search_regex(r"^test");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_prepare_search_regex_end_anchor_only() {
        let result = prepare_search_regex(r"test$");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_build_search_regex_case_sensitive() {
        let regex = build_search_regex(r"^test$", true).unwrap();
        assert!(regex.is_match("test"));
        assert!(!regex.is_match("TEST"));
        // Should match anywhere in string (no anchors)
        assert!(regex.is_match("prefix test suffix"));
    }

    #[test]
    fn test_build_search_regex_case_insensitive() {
        let regex = build_search_regex(r"^test$", false).unwrap();
        assert!(regex.is_match("test"));
        assert!(regex.is_match("TEST"));
        assert!(regex.is_match("Test"));
        // Should match anywhere in string (no anchors)
        assert!(regex.is_match("prefix TEST suffix"));
    }

    #[test]
    fn test_build_search_regex_with_dotall() {
        let regex = build_search_regex(r"test.line", true).unwrap();
        assert!(regex.is_match("test\nline"));
        assert!(regex.is_match("prefix test\nline suffix"));
    }
}

