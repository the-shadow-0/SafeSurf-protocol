use crate::protocol::{PageContent, RiskReport};
use regex::Regex;

pub struct RiskScorer;

impl RiskScorer {
    pub fn analyze(content: &PageContent) -> RiskReport {
        let mut score: f32 = 0.0;
        let mut findings = Vec::new();

        // 1. Check for sensitive keywords (phishing markers)
        let phishing_patterns = vec![
            (Regex::new(r"login|password|signin|account").unwrap(), 0.2, "Contains authentication keywords"),
            (Regex::new(r"urgent|verify|locked|suspended").unwrap(), 0.1, "Contains urgency/scareword markers"),
        ];

        for (re, weight, msg) in phishing_patterns {
            if re.is_match(&content.html.to_lowercase()) {
                score += weight;
                findings.push(msg.to_string());
            }
        }

        // 2. Check for suspicious forms
        if content.html.contains("<form") && (content.html.contains("type=\"password\"") || content.html.contains("type='password'")) {
            score += 0.4;
            findings.push("Contains password input form".to_string());
        }

        // 3. Check for external resources (leaking metadata)
        if content.html.contains("src=\"http") || content.html.contains("src='http") {
             // In a deep web context, external resource loading is highly suspicious
             score += 0.3;
             findings.push("Loads external resources (potential tracking)".to_string());
        }

        let recommended_action = if score > 0.7 {
            "CRITICAL: Do not provide credentials. Page is highly suspicious."
        } else if score > 0.3 {
            "WARNING: Use caution. Inspect sources before interacting."
        } else {
            "Neutral: No immediate threats detected."
        };

        RiskReport {
            score: score.min(1.0),
            findings,
            recommended_action: recommended_action.to_string(),
        }
    }
}
