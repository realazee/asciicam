#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let port: u16 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8000);
    server::serve(port);
}

#[cfg(not(target_arch = "wasm32"))]
mod server {
    use std::io::{BufRead, BufReader, Write};
    use std::net::{TcpListener, TcpStream};

    const INDEX_HTML: &str = include_str!("../web/index.html");
    const WASM_JS: &str = include_str!("../web/pkg/asciicam.js");
    const WASM_BIN: &[u8] = include_bytes!("../web/pkg/asciicam_bg.wasm");

    pub fn serve(port: u16) {
        let listener = TcpListener::bind(("127.0.0.1", port)).expect("failed to bind 127.0.0.1");
        eprintln!("asciicam ready at http://127.0.0.1:{port}");
        for stream in listener.incoming().flatten() {
            std::thread::spawn(move || handle(stream));
        }
    }

    fn handle(mut stream: TcpStream) {
        let target = match read_target(&mut stream) {
            Some(t) => t,
            None => {
                let _ = respond(
                    &mut stream,
                    400,
                    "Bad Request",
                    "text/plain",
                    b"bad request",
                );
                return;
            }
        };
        let path = target.split('?').next().unwrap_or("/");
        let (status, reason, mime, body): (u16, &str, &str, &[u8]) = match path {
            "/" | "/index.html" => (200, "OK", "text/html; charset=utf-8", INDEX_HTML.as_bytes()),
            "/pkg/asciicam.js" => (
                200,
                "OK",
                "application/javascript; charset=utf-8",
                WASM_JS.as_bytes(),
            ),
            "/pkg/asciicam_bg.wasm" => (200, "OK", "application/wasm", WASM_BIN),
            _ => (404, "Not Found", "text/plain", b"not found"),
        };
        let _ = respond(&mut stream, status, reason, mime, body);
    }

    fn read_target(stream: &mut TcpStream) -> Option<String> {
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).ok()?;
        let mut parts = line.split_whitespace();
        let method = parts.next()?;
        let target = parts.next()?;
        if method != "GET" {
            return None;
        }
        loop {
            let mut h = String::new();
            if reader.read_line(&mut h).ok()? == 0 || h == "\r\n" || h == "\n" {
                break;
            }
        }
        Some(target.to_string())
    }

    fn respond(
        stream: &mut TcpStream,
        code: u16,
        reason: &str,
        content_type: &str,
        body: &[u8],
    ) -> std::io::Result<()> {
        write!(
            stream,
            "HTTP/1.1 {code} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
            body.len()
        )?;
        stream.write_all(body)?;
        stream.flush()
    }
}
