//! Part 1: Input Sanitization
//!
//! Use the `ammonia` crate to strip dangerous HTML from user input while allowing
//! a safe subset of formatting tags.

/// Sanitize an HTML string, allowing only safe formatting tags.
///
/// Allowed tags: `<b>`, `<i>`, `<em>`, `<strong>`, `<p>`, `<br>`, `<ul>`, `<ol>`, `<li>`, `<a>`.
/// Allowed attributes: `href` on `<a>` only (restricted to http/https URLs).
/// Everything else is stripped.
///
/// TODO: Implement using `ammonia::Builder`.
///
/// # Examples
///
/// ```
/// use security_hardening::sanitize::sanitize_html;
///
/// let input = r#"<p>Hello</p><script>alert('xss')</script>"#;
/// let clean = sanitize_html(input);
/// assert!(clean.contains("<p>Hello</p>"));
/// assert!(!clean.contains("<script>"));
/// ```
pub fn sanitize_html(_input: &str) -> String {
    todo!("Implement HTML sanitization with ammonia")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allows_safe_formatting_tags() {
        let input = "<p><strong>Bold</strong> and <em>italic</em></p>";
        let result = sanitize_html(input);
        assert!(result.contains("<strong>"));
        assert!(result.contains("<em>"));
        assert!(result.contains("<p>"));
    }

    #[test]
    fn test_strips_script_tags() {
        let input = r#"Hello<script>alert('xss')</script>World"#;
        let result = sanitize_html(input);
        assert!(!result.contains("<script>"));
        assert!(!result.contains("alert"));
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));
    }

    #[test]
    fn test_strips_onclick_attributes() {
        let input = r#"<p onclick="alert('xss')">Click me</p>"#;
        let result = sanitize_html(input);
        assert!(!result.contains("onclick"));
        assert!(result.contains("Click me"));
    }

    #[test]
    fn test_strips_javascript_uri() {
        let input = r#"<a href="javascript:alert('xss')">Link</a>"#;
        let result = sanitize_html(input);
        assert!(!result.contains("javascript:"));
    }

    #[test]
    fn test_strips_iframe() {
        let input = r#"<iframe src="https://evil.com"></iframe>Content"#;
        let result = sanitize_html(input);
        assert!(!result.contains("<iframe"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_allows_safe_links() {
        let input = r#"<a href="https://example.com">Safe link</a>"#;
        let result = sanitize_html(input);
        assert!(result.contains("https://example.com"));
        assert!(result.contains("Safe link"));
    }
}
