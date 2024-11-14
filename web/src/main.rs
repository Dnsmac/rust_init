use std::{io::Read, net::{TcpListener, TcpStream}};

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for  stream in listener.incoming(){
        let stream = stream.unwrap();
        hand_connection(stream);
    }
}

fn hand_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    print!("Request :{}", String::from_utf8_lossy(&buffer[..]));
}