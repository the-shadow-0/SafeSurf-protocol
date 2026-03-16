use ammonia::Builder;
use std::collections::HashSet;

pub struct ContentSanitizer {
    builder: Builder<'static>,
}

impl Default for ContentSanitizer {
    fn default() -> Self {
        let mut builder = Builder::new();
        
        let tags: HashSet<_> = [
            "b", "i", "em", "strong", "p", "br", "div", "span", "table", "tr", "td", "th", "h1", "h2", "h3"
        ].into_iter().collect();

        let mut tag_attributes = std::collections::HashMap::new();
        tag_attributes.insert("p", ["style"].into_iter().collect::<HashSet<_>>());
        tag_attributes.insert("div", ["style"].into_iter().collect::<HashSet<_>>());
        tag_attributes.insert("span", ["style"].into_iter().collect::<HashSet<_>>());

        let clean_content_tags: HashSet<_> = [
            "script", "style", "object", "embed", "iframe", "canvas"
        ].into_iter().collect();

        builder
            .tags(tags)
            .tag_attributes(tag_attributes)
            .clean_content_tags(clean_content_tags);
        
        Self { builder }
    }
}

impl ContentSanitizer {
    pub fn sanitize(&self, html: &str) -> String {
        self.builder.clean(html).to_string()
    }

    pub fn redact_sensitive_info(&self, text: &str) -> String {
        // Redaction is a separate step from sanitization
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitization() {
        let sanitizer = ContentSanitizer::default();
        let dirty_html = "<div>Hello <script>alert('xss')</script> <img src='x' onerror='alert(1)'> <b>world</b></div>";
        let clean_html = sanitizer.sanitize(dirty_html);
        
        assert!(!clean_html.contains("<script>"));
        assert!(!clean_html.contains("onerror"));
        assert!(clean_html.contains("<b>world</b>"));
    }
}
