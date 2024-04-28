use anyhow::Result;
use axum::{routing::get, Router};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "A simple HTTP server")]
struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    ip: String,
    #[arg(short, long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let addr: std::net::SocketAddr = format!("{}:{}", args.ip, args.port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on {:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
