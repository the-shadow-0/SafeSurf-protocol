use clap::{Parser, Subcommand};
use reqwest::Client;
use safe_surf_core::protocol::{HandshakeInit, PageContent};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "safesurf")]
#[command(about = "SafeSurf Protocol CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "http://127.0.0.1:3000")]
    daemon_url: String,

    #[arg(short, long, default_value = "127.0.0.1:3001")]
    ssp_addr: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new session
    Init {
        #[arg(short, long, default_value = "0.1.0")]
        version: String,
    },
    /// Sanitize a local HTML file or string
    Sanitize {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        file: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
    /// Analyze risk score for a page
    Risk {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        file: Option<String>,
    },
    /// Fetch and analyze a remote URL
    Fetch {
        #[arg(short, long)]
        url: String,
    },
    /// Configure system-wide proxy settings
    SysSetup {
        #[arg(short, long)]
        enable: bool,
    },
    /// Check daemon health
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = Client::new();

    match &cli.command {
        Commands::Init { version } => {
            let payload = HandshakeInit {
                client_version: version.clone(),
                ephemeral_public_key: [0u8; 32],
            };
            let res = client
                .post(format!("{}/session/init", cli.daemon_url))
                .json(&payload)
                .send()
                .await?
                .text()
                .await?;
            println!("Handshake Response: {}", res);
        }
        Commands::Sanitize { url, file, content } => {
            let html = if let Some(path) = file {
                std::fs::read_to_string(path)?
            } else if let Some(c) = content {
                c.clone()
            } else {
                return Err(anyhow::anyhow!("Either file or content must be provided"));
            };

            let payload = PageContent {
                url: url.clone(),
                html,
                headers: HashMap::new(),
            };

            // Connect via SSP (Encrypted Protocol)
            use safe_surf_core::transport::EncryptedFrameCodec;
            use safe_surf_core::protocol::Message;
            use tokio::net::TcpStream;

            let mut stream = TcpStream::connect(&cli.ssp_addr).await?;
            let mut codec = EncryptedFrameCodec::new([0u8; 32]); // Mock key

            codec.write_message(&mut stream, &Message::PageContent(payload)).await?;
            if let Some(Message::RiskReport(report)) = codec.read_message::<Message, _>(&mut stream).await? {
                println!("--- Encrypted Scan Result ---");
                println!("Score: {:.2}", report.score);
                println!("Findings: {:?}", report.findings);
                println!("-----------------------------");
            }
        }
        Commands::Risk { url, file } => {
            let html = if let Some(path) = file {
                std::fs::read_to_string(path)?
            } else {
                return Err(anyhow::anyhow!("File must be provided for risk analysis"));
            };

            let payload = PageContent {
                url: url.clone(),
                html,
                headers: HashMap::new(),
            };

            let mut stream = tokio::net::TcpStream::connect(&cli.ssp_addr).await?;
            let mut codec = safe_surf_core::transport::EncryptedFrameCodec::new([0u8; 32]);
            codec.write_message(&mut stream, &safe_surf_core::protocol::Message::PageContent(payload)).await?;
            
            if let Some(safe_surf_core::protocol::Message::RiskReport(report)) = codec.read_message::<safe_surf_core::protocol::Message, _>(&mut stream).await? {
                println!("--- Encrypted Risk Analysis Report ---");
                println!("URL: {}", url);
                println!("Score: {:.2}", report.score);
                println!("Findings: {:?}", report.findings);
                println!("--------------------------------------");
            }
        }
        Commands::Fetch { url } => {
            let res = client
                .post(format!("{}/content/fetch", cli.daemon_url))
                .json(url)
                .send()
                .await?
                .json::<PageContent>()
                .await?;

            println!("Fetched {} bytes from {}", res.html.len(), url);
            
            // Now scan it via SSP
            let mut stream = tokio::net::TcpStream::connect(&cli.ssp_addr).await?;
            let mut codec = safe_surf_core::transport::EncryptedFrameCodec::new([0u8; 32]);
            codec.write_message(&mut stream, &safe_surf_core::protocol::Message::PageContent(res)).await?;
            
            if let Some(safe_surf_core::protocol::Message::RiskReport(report)) = codec.read_message::<safe_surf_core::protocol::Message, _>(&mut stream).await? {
                println!("--- Risk Analysis for Fetched Content ---");
                println!("Score: {:.2}", report.score);
                println!("Findings: {:?}", report.findings);
                println!("-----------------------------------------");
            }
        }
        Commands::Status => {
            let res = client
                .get(format!("{}/health", cli.daemon_url))
                .send()
                .await?
                .text()
                .await?;
            println!("Daemon Status: {}", res);
        }
        Commands::SysSetup { enable } => {
            if *enable {
                println!("Enabling Global Proxy (127.0.0.1:8080)...");
                // For GNOME:
                let _ = std::process::Command::new("gsettings")
                    .args(["set", "org.gnome.system.proxy", "mode", "'manual'"])
                    .status();
                let _ = std::process::Command::new("gsettings")
                    .args(["set", "org.gnome.system.proxy.http", "host", "'127.0.0.1'"])
                    .status();
                let _ = std::process::Command::new("gsettings")
                    .args(["set", "org.gnome.system.proxy.http", "port", "8080"])
                    .status();
                
                println!("\n[TIP] For CLI tools, add this to your .bashrc:");
                println!("export http_proxy=http://127.0.0.1:8080");
                println!("export https_proxy=http://127.0.0.1:8080");
            } else {
                println!("Disabling Global Proxy...");
                let _ = std::process::Command::new("gsettings")
                    .args(["set", "org.gnome.system.proxy", "mode", "'none'"])
                    .status();
            }
        }
    }

    Ok(())
}
