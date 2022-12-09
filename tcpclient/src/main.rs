use std::io::{Write, Read};
use std::net::TcpStream;
use std::str;


fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3000")?;
    stream.write("hello".as_bytes())?;
    let mut buffer = [0; 5];
    stream.read(&mut buffer)?;
    println!("Got response from server {:?}", str::from_utf8(&buffer).unwrap());
    Ok(())

}
