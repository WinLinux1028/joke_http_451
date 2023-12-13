#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

const FAVICON_ICO: &[u8] = include_bytes!("../assets/favicon.ico");
const LOGO_WEBP: &[u8] = include_bytes!("../assets/logo.webp");
const BGM_OGG: &[u8] = include_bytes!("../assets/bgm.ogg");
const INDEX_HTML: &[u8] = include_bytes!("../assets/index.html");

use std::{io::Write, time::Duration};
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

        tokio::spawn(async move { while run(&mut stream).await.is_ok() {} });
    }
}

async fn run<RW>(stream: &mut RW) -> Result<(), Box<dyn std::error::Error>>
where
    RW: AsyncBufRead + AsyncWrite + Unpin,
{
    let mut buf = String::new();

    tokio::select! {
        result = stream.read_line(&mut buf) => { match result {
            Ok(0) => return Err("".into()),
            Err(e) => return Err(e.into()),
            Ok(_) => {}
        } }
        _ = tokio::time::sleep(Duration::from_secs(30)) => { return Err("".into()); }
    }

    let req: Vec<&str> = buf.trim().split(' ').collect();
    if req.len() != 3 {
        return Err("".into());
    }

    let write;
    if req[1] == "/favicon.ico" {
        stream
            .write_all(b"HTTP/1.1 200 Allowed by NKVD\r\n")
            .await?;
        stream
            .write_all(b"Content-Type: image/vnd.microsoft.icon\r\n")
            .await?;
        write = FAVICON_ICO;
    } else if req[1] == "/logo.webp" {
        stream
            .write_all(b"HTTP/1.1 200 Allowed by NKVD\r\n")
            .await?;
        stream.write_all(b"Content-Type: image/webp\r\n").await?;
        write = LOGO_WEBP;
    } else if req[1] == "/bgm.ogg" {
        stream
            .write_all(b"HTTP/1.1 200 Allowed by NKVD\r\n")
            .await?;
        stream.write_all(b"Content-Type: audio/ogg\r\n").await?;
        write = BGM_OGG;
    } else {
        stream
            .write_all(b"HTTP/1.1 103 I love Hakurei Reimu!!\r\n")
            .await?;
        stream
            .write_all(b"Link: </logo.webp>; rel=preload; as=image\r\n")
            .await?;
        stream
            .write_all(b"Link: </bgm.ogg>; rel=preload; as=audio\r\n")
            .await?;
        stream.write_all(b"\r\n").await?;
        stream
            .write_all(b"HTTP/1.1 451 Purged by USSR Goverment\r\n")
            .await?;
        stream
            .write_all(b"Content-Type: text/html; charset=utf-8\r\n")
            .await?;
        write = INDEX_HTML;
    }
    stream.write_all(b"Connection: keep-alive\r\n").await?;
    stream
        .write_all(b"Permissions-Policy: interest-cohort=()\r\n")
        .await?;
    stream
        .write_all(b"X-Content-Type-Options: nosniff\r\n")
        .await?;
    stream.write_all(b"X-Frame-Options: DENY\r\n").await?;
    stream
        .write_all(b"Cross-Origin-Resource-Policy: same-origin\r\n")
        .await?;
    stream
        .write_all(b"Cache-Control: public, max-age=2147483647, immutable\r\n")
        .await?;
    stream
        .write_all(format!("Content-Length: {}\r\n", write.len()).as_bytes())
        .await?;
    stream.write_all(b"\r\n").await?;
    stream.flush().await?;

    stream.write_all(write).await?;
    stream.flush().await?;

    Ok(())
}
