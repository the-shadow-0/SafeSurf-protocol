use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Method, StatusCode};
use hyper_util::rt::TokioIo;
use http_body_util::Full;
use bytes::Bytes;
use std::sync::Arc;
use tracing::{info, error};
use safe_surf_core::protocol::PageContent;

pub async fn run_proxy_server(addr: &str, state: Arc<crate::AppState>) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| { error!("Failed to bind proxy to {}: {}", addr, e); e })?;
    info!("SafeSurf HTTP Proxy listening on {}", addr);

    loop {
        let (stream, _) = match listener.accept().await {
            Ok(s) => s,
            Err(e) => {
                error!("Proxy failed to accept connection: {}", e);
                continue;
            }
        };
        let io = TokioIo::new(stream);
        let state = Arc::clone(&state);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(io, service_fn(move |req| {
                    proxy_handler(req, Arc::clone(&state))
                }))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn proxy_handler(
    req: Request<hyper::body::Incoming>,
    state: Arc<crate::AppState>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    info!("Proxy Request: {} {}", req.method(), req.uri());

    if req.method() == Method::CONNECT {
        // CONNECT is for HTTPS tunneling. 
        // In a production safety tool, we might MITM here, 
        // but for this reference, we'll just log and deny or provide a basic tunnel.
        // For simplicity and safety, let's focus on intercepting HTTP first.
        let mut resp = Response::new(Full::new(Bytes::from("HTTPS Tunneling via CONNECT not yet implemented in this reference. Please use HTTP for sanitization demonstration.")));
        *resp.status_mut() = StatusCode::NOT_IMPLEMENTED;
        return Ok(resp);
    }

    // Handle standard HTTP Proxy requests
    let url = req.uri().to_string();
    
    // Fetch content via the daemon's secure fetcher
    let res = match state.http_client.get(&url).send().await {
        Ok(res) => res,
        Err(e) => {
            error!("Failed to fetch URL: {}", e);
            let mut resp = Response::new(Full::new(Bytes::from(format!("Failed to fetch: {}", e))));
            *resp.status_mut() = StatusCode::BAD_GATEWAY;
            return Ok(resp);
        }
    };

    let html = res.text().await.unwrap_or_default();
    
    // Run Sanitization & Risk Analysis
    let sanitized = state.sanitizer.sanitize(&html);
    let _report = safe_surf_core::risk_scorer::RiskScorer::analyze(&PageContent {
        url: url.clone(),
        html: html.clone(),
        headers: std::collections::HashMap::new(),
    });

    // Provide a "Safety Wrapped" response
    let body_str = if sanitized != html {
        format!(
            "<!-- SafeSurf: This page has been sanitized for your safety -->\n{}",
            sanitized
        )
    } else {
        sanitized
    };

    Ok(Response::new(Full::new(Bytes::from(body_str))))
}
