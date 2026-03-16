use safe_surf_core::protocol::PageContent;
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();
    let daemon_url = "http://127.0.0.1:3000";

    println!("SafeSurf SDK Example Starting...");

    // 1. Check a synthetic page for risk
    let content = PageContent {
        url: "http://malicious.example".to_string(),
        html: "<html><body><h1>Login</h1><input type='password' /></body></html>".to_string(),
        headers: std::collections::HashMap::new(),
    };

    let report = client
        .post(format!("{}/content/risk", daemon_url))
        .json(&content)
        .send()
        .await?
        .json::<safe_surf_core::protocol::RiskReport>()
        .await?;

    println!("Risk Score: {}", report.score);
    println!("Findings: {:?}", report.findings);
    println!("Recommendation: {}", report.recommended_action);

    Ok(())
}
