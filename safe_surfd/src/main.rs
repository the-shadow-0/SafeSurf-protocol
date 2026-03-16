mod proxy;
use axum::{
    extract::{State},
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use safe_surf_core::{
    protocol::{HandshakeInit, HandshakeResponse, PageContent, RiskReport},
    sanitization::ContentSanitizer,
    session::SessionManager,
};
use std::sync::{Arc, Mutex};
use tower_http::trace::TraceLayer;
use tracing::{info, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    addr: String,

    #[arg(long, default_value = "127.0.0.1:3001")]
    ssp_addr: String,

    #[arg(long, default_value = "127.0.0.1:8080")]
    proxy_addr: String,
}

struct AppState {
    session_manager: Mutex<SessionManager>,
    sanitizer: ContentSanitizer,
    http_client: reqwest::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    // In a real scenario, this would be configured to use a Tor SOCKS5 proxy
    let http_client = reqwest::Client::builder()
        .user_agent("SafeSurf/0.1.0")
        .build()?;

    let state = Arc::new(AppState {
        session_manager: Mutex::new(SessionManager::new()),
        sanitizer: ContentSanitizer::default(),
        http_client,
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/session/init", post(init_session))
        .route("/content/sanitize", post(sanitize_content))
        .route("/content/risk", post(analyze_risk))
        .route("/content/fetch", post(fetch_content))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::clone(&state));

    info!("SafeSurf Daemon starting REST API on {}", args.addr);
    let rest_addr = args.addr.clone();
    let _rest_state = Arc::clone(&state);
    let rest_handle = tokio::spawn(async move {
        if let Ok(listener) = tokio::net::TcpListener::bind(&rest_addr).await {
            let _ = axum::serve(listener, app).await;
        }
    });

    // Start Encrypted Protocol Listener
    let ssp_addr = args.ssp_addr.clone();
    info!("SafeSurf Protocol Listener starting on {}", ssp_addr);
    let ssp_state = Arc::clone(&state);
    let ssp_handle = tokio::spawn(async move {
        if let Ok(listener) = tokio::net::TcpListener::bind(ssp_addr).await {
            loop {
                if let Ok((socket, _)) = listener.accept().await {
                    let state = Arc::clone(&ssp_state);
                    tokio::spawn(async move {
                        handle_ssp_connection(socket, state).await;
                    });
                }
            }
        }
    });

    // Start HTTP Proxy
    let proxy_addr = args.proxy_addr.clone();
    let proxy_state = Arc::clone(&state);
    let proxy_handle = tokio::spawn(async move {
        let _ = crate::proxy::run_proxy_server(&proxy_addr, proxy_state).await;
    });

    tokio::select! {
        _ = rest_handle => {},
        _ = ssp_handle => {},
        _ = proxy_handle => {},
    }

    Ok(())
}

async fn handle_ssp_connection(mut socket: tokio::net::TcpStream, state: Arc<AppState>) {
    use safe_surf_core::transport::EncryptedFrameCodec;
    use safe_surf_core::protocol::Message;

    // In a real implementation, we'd do the DH handshake here.
    // For this reference implementation, we'll use a pre-shared key or mock it.
    let mock_key = [0u8; 32]; 
    let mut codec = EncryptedFrameCodec::new(mock_key);

    while let Ok(Some(msg)) = codec.read_message::<Message, _>(&mut socket).await {
        match msg {
            Message::HandshakeInit(_init) => {
                info!("SSP Handshake Init received");
                let resp = Message::HandshakeResponse(HandshakeResponse {
                    server_version: "0.1.0".to_string(),
                    ephemeral_public_key: [0u8; 32],
                    salt: [0u8; 16],
                });
                if let Err(e) = codec.write_message(&mut socket, &resp).await {
                    error!("SSP failed to write handshake response: {:?}", e);
                    break;
                }
            }
            Message::PageContent(payload) => {
                info!("SSP Content received for URL: {}", payload.url);
                // Sanitize and return
                let _sanitized = state.sanitizer.sanitize(&payload.html);
                let risk_report = Message::RiskReport(safe_surf_core::risk_scorer::RiskScorer::analyze(&payload));
                if let Err(e) = codec.write_message(&mut socket, &risk_report).await {
                    error!("SSP failed to write risk report: {:?}", e);
                    break;
                }
            }
            _ => {
                info!("SSP received unexpected message");
            }
        }
    }
}

async fn health_check() -> &'static str {
    "SafeSurf Protocol Daemon Active"
}

async fn init_session(
    State(state): State<Arc<AppState>>,
    Json(_payload): Json<HandshakeInit>,
) -> Json<HandshakeResponse> {
    // In a real implementation, we would perform the full DH handshake here.
    // For the reference implementation, we'll simulate the response.
    let mut manager = state.session_manager.lock().unwrap();
    let salt = [0u8; 16];
    let shared_secret = [0u8; 32]; // Mocked
    let session_id = manager.create_session(&shared_secret, &salt);
    
    info!("New session created: {}", session_id);

    Json(HandshakeResponse {
        server_version: "0.1.0".to_string(),
        ephemeral_public_key: [0u8; 32], // Mocked
        salt,
    })
}

async fn sanitize_content(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PageContent>,
) -> Json<String> {
    info!("Sanitizing content for URL: {}", payload.url);
    let sanitized = state.sanitizer.sanitize(&payload.html);
    Json(sanitized)
}

async fn analyze_risk(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<PageContent>,
) -> Json<RiskReport> {
    info!("Analyzing risk for URL: {}", payload.url);
    let report = safe_surf_core::risk_scorer::RiskScorer::analyze(&payload);
    Json(report)
}

async fn fetch_content(
    State(state): State<Arc<AppState>>,
    Json(url_str): Json<String>,
) -> Result<Json<PageContent>, axum::http::StatusCode> {
    info!("Fetching content for URL: {}", url_str);
    
    let res = state.http_client.get(&url_str).send().await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let html = res.text().await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(PageContent {
        url: url_str,
        html,
        headers: std::collections::HashMap::new(),
    }))
}
