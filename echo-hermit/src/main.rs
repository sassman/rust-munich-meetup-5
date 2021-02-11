#[cfg(target_os = "hermit")]
extern crate hermit_sys;

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    loop {
        let (conn, _addr) = listener.accept().unwrap();
        std::io::copy(&mut &conn, &mut &conn).unwrap();
    }
}

// use async_std::net::TcpListener;
//
// fn main() {
//     async_std::task::block_on(async {
//         let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
//         loop {
//             let (conn, _addr) = listener.accept().await.unwrap();
//             async_std::io::copy(&mut &conn, &mut &conn).await.unwrap();
//         }
//     });
// }
