use log::{error, info, LevelFilter};
use simplelog::*;
use std::fs::{self, File};
use std::path::Path;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

enum Message {
    spawn(String),
}

const LOG_FILE_PATH: &str = "/home/zinc/.wmbc.log";

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = init_log() {
        error!("failed to initialize log: {}", e);
    };
    if let Err(e) = start_behavior_controller().await {
        error!("failed to start behavior controller: {}", e);
    };
    Ok(())
}

async fn start_behavior_controller() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:10080").await?;
    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("new connection: {}", addr);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        break;
                    }
                    Ok(n) => {
                        info!("received {}", String::from_utf8_lossy(&buf[..n]));
                    }
                    Err(e) => {
                        error!("failed to read from socket: {}", e);
                        break;
                    }
                }
            }
        });
    }
}

fn init_log() -> Result<()> {
    if Path::new(LOG_FILE_PATH).exists() {
        fs::remove_file(LOG_FILE_PATH).unwrap();
    }
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(LOG_FILE_PATH)?,
        ),
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ])?;
    Ok(())
}
