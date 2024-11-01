use std::io;
use std::net::SocketAddr;
use tokio::io::{stdin, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

struct Servers {

}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "0.0.0.0:8081".parse().unwrap();
    let mut stream = TcpStream::connect(addr).await.unwrap();
    let local_addr = stream.local_addr().unwrap();
    println!("Local address: {}", local_addr);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write(input.as_bytes()).await.unwrap();
        // 读取服务器的响应
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await.unwrap();
        println!("Received from server: {:?}", &buffer[0..n]);
    }
}
