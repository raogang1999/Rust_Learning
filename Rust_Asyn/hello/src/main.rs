use std::fs::read;
use std::thread;
use std::thread::*;
use std::io::{prelude::*,BufReader,Write};
use std::net::TcpStream;
use futures::join;
use futures::executor;

fn use_server(server:&str,port:u16,content:&str)->std::io::Result<()>{
    let mut stream = TcpStream::connect((server,port))?;
    let _ = stream.write(content.as_bytes());
    let mut reader = BufReader::new(&stream);
    let mut buffer = Vec::new();
    reader.read_until(b'\n',&mut buffer)?;
    println!("recv from server: {}",std::str::from_utf8(&buffer).unwrap());
    Ok(())
}
async fn async_use_server(server:&str,port:u16,content:&str){
    use_server(server,port,content).unwrap();
}
async fn use_all_server(){
    let f1 = async_use_server("127.0.0.1",8051,"use server1 127.0.0.1:8001");
    let f2 = async_use_server("127.0.0.1",8052,"use server1 127.0.0.1:8002");
    join!(f1,f2);
}
fn main() -> std::io::Result<()>{
    let f = use_all_server();
    executor::block_on(f);
    Ok(())
}