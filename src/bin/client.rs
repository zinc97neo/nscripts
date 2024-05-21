use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:10080").await?;
    let data = "Hello, server!".as_bytes();

    stream.write_all(data).await?;
    println!("Sent: {}", String::from_utf8_lossy(data));

    Ok(())
}
