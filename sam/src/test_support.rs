pub enum ScriptedResponse {
    Http {
        status: u16,
        body: &'static str,
    },
    TruncatedHttp {
        status: u16,
        declared_body_len: usize,
        actual_body: &'static str,
    },
    CloseConnection,
}

pub fn spawn_scripted_http_server(script: Vec<ScriptedResponse>) -> std::net::SocketAddr {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};

    fn find_header_end(request: &[u8]) -> Option<usize> {
        request.windows(4).position(|window| window == b"\r\n\r\n")
    }

    fn write_response(stream: &mut TcpStream, status: u16, declared_body_len: usize, body: &str) {
        let head: String = format!(
            "HTTP/1.1 {status} OK\r\nContent-Length: {declared_body_len}\r\nConnection: close\r\n\r\n"
        );

        let _ = stream.write_all(head.as_bytes());
        let _ = stream.write_all(body.as_bytes());
        let _ = stream.flush();
    }

    fn handle_connection(mut stream: TcpStream, action: ScriptedResponse) {
        const HEADER_TERMINATOR_LEN: usize = 4;
        const BUFFER_CAPACITY: usize = 4096;

        let mut buffer: [u8; BUFFER_CAPACITY] = [0; BUFFER_CAPACITY];
        let mut received: Vec<u8> = Vec::new();
        let header_end: usize = loop {
            let read_bytes: usize = stream.read(&mut buffer).unwrap_or(0);
            if read_bytes == 0 {
                return;
            }
            received.extend_from_slice(&buffer[..read_bytes]);

            match find_header_end(&received) {
                Some(header_end) => break header_end,
                None if received.len() == BUFFER_CAPACITY => return,
                None => continue,
            }
        };

        let headers: String = String::from_utf8_lossy(&received[..header_end]).to_lowercase();
        let declared_body_len: usize = headers
            .split("\r\n")
            .find_map(|line| line.strip_prefix("content-length:"))
            .and_then(|value| value.trim().parse().ok())
            .unwrap_or(0);
        let already_received: usize = received.len() - (header_end + HEADER_TERMINATOR_LEN);
        let mut missing_body_bytes: usize = declared_body_len.saturating_sub(already_received);

        while missing_body_bytes > 0 {
            let read_bytes: usize = stream.read(&mut buffer).unwrap_or(0);
            if read_bytes == 0 {
                break;
            }
            missing_body_bytes -= missing_body_bytes.min(read_bytes);
        }

        match action {
            ScriptedResponse::CloseConnection => {}
            ScriptedResponse::Http { status, body } => {
                write_response(&mut stream, status, body.len(), body)
            }
            ScriptedResponse::TruncatedHttp {
                status,
                declared_body_len,
                actual_body,
            } => write_response(&mut stream, status, declared_body_len, actual_body),
        }
    }

    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("A port to be bound");
    let server_addr: std::net::SocketAddr = listener.local_addr().unwrap();

    std::thread::spawn(move || {
        for action in script {
            let (stream, _) = listener
                .accept()
                .expect("accept succeeds while the script has pending responses");
            handle_connection(stream, action);
        }
    });

    server_addr
}
