use std::io::{Write, Read};
use std::net::TcpListener;


fn main() -> std::io::Result<()> {
    let connection_listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("Running on port 3000");

    for stream in connection_listener.incoming() {
        let mut stream = stream?;
        println!("Connection established");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        stream.write(&mut buffer)?;
    }
    Ok(())
}
