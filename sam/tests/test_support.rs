use sam::test_support::{ScriptedResponse, spawn_scripted_http_server};

use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

fn connect(addr: std::net::SocketAddr) -> TcpStream {
    let stream = TcpStream::connect(addr).expect("server should accept");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("read timeout should apply");
    stream
}

fn read_all(stream: &mut TcpStream) -> Vec<u8> {
    let mut buf = Vec::new();
    loop {
        let mut chunk = [0u8; 512];
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
        }
    }
    buf
}

#[test]
fn given_client_closing_mid_request_server_should_move_on_without_responding() {
    let addr = spawn_scripted_http_server(vec![ScriptedResponse::Http {
        status: 200,
        body: "never",
    }]);

    let mut stream = connect(addr);
    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: x\r\n")
        .expect("partial write should work");
    stream.flush().expect("flush should work");
    stream.shutdown(Shutdown::Both).expect("shutdown should work");

    let received = read_all(&mut stream);

    assert!(
        received.is_empty(),
        "Server must not answer a request whose headers never terminated"
    );
}

#[test]
fn given_header_exceeding_buffer_without_terminator_server_should_drop_connection() {
    let addr = spawn_scripted_http_server(vec![ScriptedResponse::Http {
        status: 200,
        body: "never",
    }]);

    let mut stream = connect(addr);
    let oversized = vec![b'a'; 8192];
    let _ = stream.write_all(&oversized);
    stream.flush().expect("flush should work");

    let received = read_all(&mut stream);

    assert!(
        received.is_empty(),
        "Buffer-capacity guard must drop the connection instead of blocking forever"
    );
}

#[test]
fn given_request_without_content_length_server_should_reply_immediately() {
    let addr = spawn_scripted_http_server(vec![ScriptedResponse::Http {
        status: 200,
        body: "X",
    }]);

    let mut stream = connect(addr);
    stream
        .write_all(b"POST /x HTTP/1.1\r\nHost: y\r\n\r\n")
        .expect("write should work");
    stream.flush().expect("flush should work");

    let received = String::from_utf8(read_all(&mut stream)).expect("response should be utf-8");

    assert!(received.starts_with("HTTP/1.1 200 OK"));
    assert!(received.contains("Content-Length: 1"));
    assert!(received.ends_with('X'));
}

#[test]
fn given_truncated_declared_body_server_should_send_only_actual_bytes() {
    let addr = spawn_scripted_http_server(vec![ScriptedResponse::TruncatedHttp {
        status: 200,
        declared_body_len: 1000,
        actual_body: "{\"partial\"",
    }]);

    let mut stream = connect(addr);
    stream
        .write_all(b"POST /y HTTP/1.1\r\nContent-Length: 5\r\n\r\nhello")
        .expect("write should work");
    stream.flush().expect("flush should work");

    let received = String::from_utf8(read_all(&mut stream)).expect("response should be utf-8");

    assert!(received.contains("Content-Length: 1000"));
    assert_eq!(received.rsplit('\n').next(), Some("{\"partial\""));
}

#[test]
fn given_body_arriving_after_headers_server_should_wait_for_it() {
    let addr = spawn_scripted_http_server(vec![ScriptedResponse::Http {
        status: 200,
        body: "OK!",
    }]);

    let mut stream = connect(addr);
    stream
        .write_all(b"POST /z HTTP/1.1\r\nContent-Length: 5\r\n\r\nhe")
        .expect("write should work");
    stream.flush().expect("flush should work");
    std::thread::sleep(Duration::from_millis(50));
    stream
        .write_all(b"llo")
        .expect("late body chunk should be accepted");
    stream.flush().expect("flush should work");

    let received = String::from_utf8(read_all(&mut stream)).expect("response should be utf-8");

    assert!(received.starts_with("HTTP/1.1 200 OK"));
    assert!(received.ends_with("OK!"));
}

#[test]
fn given_client_closing_mid_body_server_should_stop_waiting_and_respond() {
    let addr = spawn_scripted_http_server(vec![ScriptedResponse::Http {
        status: 200,
        body: "done",
    }]);

    let mut stream = connect(addr);
    stream
        .write_all(b"POST /w HTTP/1.1\r\nContent-Length: 50\r\n\r\nabc")
        .expect("write should work");
    stream.flush().expect("flush should work");
    stream.shutdown(Shutdown::Write).expect("half-close should work");

    let received = String::from_utf8(read_all(&mut stream)).expect("response should be utf-8");

    assert!(
        received.starts_with("HTTP/1.1 200 OK"),
        "Server must give up waiting for the missing body bytes and answer anyway"
    );
}
