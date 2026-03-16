use safe_surf_core::protocol::PageContent;

#[macro_use]
extern crate libfuzzer_sys;

fuzz_target!(|data: &[u8]| {
    if let Ok(html) = std::str::from_utf8(data) {
        let content = PageContent {
            url: "http://fuzz.example".to_string(),
            html: html.to_string(),
            headers: std::collections::HashMap::new(),
        };
        let _report = safe_surf_core::risk_scorer::RiskScorer::analyze(&content);
        let sanitizer = safe_surf_core::sanitization::ContentSanitizer::default();
        let _clean = sanitizer.sanitize(html);
    }
});
