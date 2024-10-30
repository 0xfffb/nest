
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;


struct Client {
    addr: SocketAddr,
    stream: Arc<Mutex<TcpStream>>,
}

impl Client {
    pub fn new(addr: SocketAddr, stream: Arc<Mutex<TcpStream>>) -> Self {
        Self { addr, stream }
    }
}

struct StreamPool {
    clients: HashMap<SocketAddr, Client>,
}

impl StreamPool {
    fn new() -> Self { Self { clients: HashMap::new() } }

    fn insert(&mut self, addr: SocketAddr, client: Client) {
        self.clients.insert(addr, client);
    }

    fn get(&self, addr: SocketAddr) -> Option<&Client> { self.clients.get(&addr) }

    fn remove(&mut self, addr: &SocketAddr) {
        self.clients.remove(addr);
    }

    async fn handle(&mut self, addr: SocketAddr, mut stream: TcpStream) {
        // let client = Client::new(addr, Arc::new(Mutex::new(stream)));
        // self.clients.insert(addr, client);
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await.unwrap();
        println!("Receive: {:?}, from: {}", String::from_utf8_lossy(&buffer[..n]), addr);
        stream.write(&buffer[0..n]).await.unwrap();
    }
}

#[tokio::main]
async fn main() {

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", addr);
    let mut stream_pool = StreamPool::new();
    loop {
        let (mut stream, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from {}", addr);
        tokio::spawn(&stream_pool.handle(addr, stream));
    }
}
