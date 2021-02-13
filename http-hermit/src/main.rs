/// Example is derived from rusty-hermit's http example
/// https://github.com/hermitcore/rusty-hermit/tree/master/examples/httpd
#[cfg(target_os = "hermit")]
extern crate hermit_sys;
use tiny_http::Server;
use ascii::AsciiString;

fn main() {
    let crab = vec![0xF0 as u8, 0x9F as u8, 0xA6 as u8, 0x80 as u8];
    let text = format!(
        "Hello from RustyHermit {}",
        String::from_utf8(crab).unwrap_or_default()
    );

    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Now listening on port 8080");

    for rq in server.incoming_requests() {
        println!("Client connected: {}", rq.remote_addr());
        let response = tiny_http::Response::from_string(text.to_string())
            .with_status_code(200)
            .with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: AsciiString::from_ascii("text/plain; charset=utf8").unwrap(),
            });
        let _ = rq.respond(response);
    }
}
