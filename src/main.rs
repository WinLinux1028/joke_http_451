const FAVICON_ICO: &[u8] = include_bytes!("../assets/favicon.ico");
const LOGO_WEBP: &[u8] = include_bytes!("../assets/logo.webp");
const BGM_OGG: &[u8] = include_bytes!("../assets/bgm.ogg");
const INDEX_HTML: &[u8] = include_bytes!("../assets/index.html");

use std::io::Write;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufStream};

#[tokio::main]
async fn main() {
    print!("listen> ");
    std::io::stdout().flush().unwrap();
    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut buf = String::new();
    stdin.read_line(&mut buf).await.unwrap();

    let listener = tokio::net::TcpListener::bind(buf.trim()).await.unwrap();
    loop {
        let mut stream = match listener.accept().await {
            Ok((o, _)) => BufStream::new(o),
            Err(_) => continue,
        };

        tokio::spawn(async move {
            let _ = run(&mut stream).await;
        });
    }
}

async fn run<RW>(stream: &mut RW) -> Result<(), Box<dyn std::error::Error>>
where
    RW: AsyncBufRead + AsyncWrite + Unpin,
{
    let mut buf = String::new();
    stream.read_line(&mut buf).await?;
    let req: Vec<&str> = buf.trim().split(' ').collect();
    if req.len() != 3 {
        return Err("".into());
    }

    if req[1] == "/favicon.ico" {
        stream
            .write_all(b"HTTP/1.1 200 Allowed by NKVD\r\n")
            .await?;
        stream
            .write_all(b"Content-Type: image/vnd.microsoft.icon\r\n")
            .await?;
        stream
            .write_all(format!("Content-Length: {}\r\n", FAVICON_ICO.len()).as_bytes())
            .await?;
        stream.write_all(b"\r\n").await?;
        stream.write_all(FAVICON_ICO).await?;
    } else if req[1] == "/logo.webp" {
        stream
            .write_all(b"HTTP/1.1 200 Allowed by NKVD\r\n")
            .await?;
        stream.write_all(b"Content-Type: image/webp\r\n").await?;
        stream
            .write_all(format!("Content-Length: {}\r\n", LOGO_WEBP.len()).as_bytes())
            .await?;
        stream.write_all(b"\r\n").await?;
        stream.write_all(LOGO_WEBP).await?;
    } else if req[1] == "/bgm.ogg" {
        stream
            .write_all(b"HTTP/1.1 200 Allowed by NKVD\r\n")
            .await?;
        stream.write_all(b"Content-Type: audio/ogg\r\n").await?;
        stream
            .write_all(format!("Content-Length: {}\r\n", BGM_OGG.len()).as_bytes())
            .await?;
        stream.write_all(b"\r\n").await?;
        stream.write_all(BGM_OGG).await?;
    } else {
        stream
            .write_all(b"HTTP/1.1 451 Purged by USSR Goverment\r\n")
            .await?;
        stream
            .write_all(b"Content-Type: text/html; charset=utf-8\r\n")
            .await?;
        stream
            .write_all(format!("Content-Length: {}\r\n", INDEX_HTML.len()).as_bytes())
            .await?;
        stream.write_all(b"\r\n").await?;
        stream.write_all(INDEX_HTML).await?;
    }
    stream.flush().await?;

    Ok(())
}
