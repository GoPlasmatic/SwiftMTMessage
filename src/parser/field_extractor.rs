//! # Field Extractor
//!
//! Extracts individual fields from SWIFT message text.

/// Extract field content from SWIFT message text
///
/// Returns the field content and the number of characters consumed
pub fn extract_field_content(input: &str, tag: &str) -> Option<(String, usize)> {
    let field_marker = format!(":{}:", tag);

    // Find the field marker
    let field_start = input.find(&field_marker)?;

    // Start of content is after the field marker
    let content_start = field_start + field_marker.len();

    // Find the next field marker or end marker
    let remaining = &input[content_start..];

    // Look for the next field (starts with `:` and has format `:XX:` or `:XXX:`)
    let content_end = find_next_field_boundary(remaining);

    // Extract content
    let (raw_content, has_trailing_newline) = if let Some(end) = content_end {
        // end points to the newline before the next field
        // Check if there's actually a newline at that position
        let has_newline = remaining.as_bytes().get(end) == Some(&b'\n');
        (remaining[..end].to_string(), has_newline)
    } else {
        // No next field found, take everything until end marker
        // Look for block end markers: "\n-}" or "\n-\n" (trailer separator)
        // Do NOT stop at "\n-" alone as "-" can be valid field content (e.g., bullet points)
        if let Some(end_pos) = remaining.find("\n-}") {
            (remaining[..end_pos].to_string(), true)
        } else if let Some(end_pos) = remaining.find("\n-\n") {
            (remaining[..end_pos].to_string(), true)
        } else if let Some(end_pos) = remaining.find("-}") {
            (remaining[..end_pos].to_string(), false)
        } else {
            // Take all remaining content
            (remaining.to_string(), false)
        }
    };

    // Calculate consumed characters BEFORE trimming (to include newlines)
    let raw_content_len = raw_content.len();

    // Clean up the content (remove trailing newlines)
    let content = raw_content.trim_end_matches('\n').trim_end_matches('\r');

    // Calculate consumed characters including the newline after the content if present
    let consumed = field_start
        + field_marker.len()
        + raw_content_len
        + if has_trailing_newline { 1 } else { 0 };

    Some((content.to_string(), consumed))
}

/// Find the boundary of the next field
fn find_next_field_boundary(input: &str) -> Option<usize> {
    let mut chars = input.char_indices();

    while let Some((i, ch)) = chars.next() {
        if ch == '\n' {
            // Check if next character starts a field
            if let Some((_, ':')) = chars.next() {
                // This might be a field marker, verify the pattern
                let rest = &input[i + 1..];
                if is_field_marker(rest) {
                    return Some(i);
                }
            }
        }
    }

    None
}

/// Check if the text starts with a valid field marker pattern
fn is_field_marker(input: &str) -> bool {
    if !input.starts_with(':') {
        return false;
    }

    // Find the closing colon
    if let Some(close) = input[1..].find(':') {
        let tag = &input[1..close + 1];

        // Valid field tags are 2-4 characters, alphanumeric
        if (2..=4).contains(&tag.len()) {
            // Check if all characters are alphanumeric
            return tag.chars().all(|c| c.is_alphanumeric());
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_field() {
        let input = ":20:REF123\n:21:RELREF\n-";
        let (content, consumed) = extract_field_content(input, "20").unwrap();
        assert_eq!(content, "REF123");
        assert_eq!(consumed, 11); // ":20:REF123\n" - includes the newline
    }

    #[test]
    fn test_extract_multiline_field() {
        let input = ":70:LINE1\nLINE2\nLINE3\n:71A:SHA\n-";
        let (content, _consumed) = extract_field_content(input, "70").unwrap();
        assert_eq!(content, "LINE1\nLINE2\nLINE3");
    }

    #[test]
    fn test_extract_field_with_variant() {
        let input = ":50K:JOHN DOE\n123 MAIN ST\n:59:BENEFICIARY\n-";
        let (content, _) = extract_field_content(input, "50K").unwrap();
        assert_eq!(content, "JOHN DOE\n123 MAIN ST");
    }

    #[test]
    fn test_extract_last_field() {
        let input = ":20:REF123\n:71A:SHA\n-";
        let (content, _) = extract_field_content(input, "71A").unwrap();
        assert_eq!(content, "SHA");
    }

    #[test]
    fn test_field_not_found() {
        let input = ":20:REF123\n:21:RELREF\n-";
        let result = extract_field_content(input, "32A");
        assert!(result.is_none());
    }

    #[test]
    fn test_field_marker_detection() {
        assert!(is_field_marker(":20:"));
        assert!(is_field_marker(":32A:"));
        assert!(is_field_marker(":50K:"));
        assert!(!is_field_marker(":12345:")); // Too long
        assert!(!is_field_marker(":X:")); // Too short
        assert!(!is_field_marker("20:")); // No starting colon
    }
}
