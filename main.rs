use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread};
//use std::borrow::Cow;
//use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
//use std::io::{self, Read};


/**
 server
 */

lazy_static! {
    static ref LISTENER: TcpListener = TcpListener::bind("127.0.0.1:1315").expect("Failed to bind address");
}


fn publish_other_clients(stream: &TcpStream, msg: &str, clients: Arc<Mutex<Vec<TcpStream>>>) {

    //clients = clients.lock().unwrap();
    for client in  clients.lock().unwrap().iter() {
        //let mut tmp_stream = client.unwrap();
        if  !client.peer_addr().expect("REASON").eq(&stream.peer_addr().unwrap()) {
            let mut client = client.try_clone().unwrap();
            client.write_all(msg.as_bytes()).expect("haho");
        }
    }
}
fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                let message = String::from_utf8_lossy(&buffer[0..bytes_read]);
                let message= message.trim_end();
                println!("{}", message);
                publish_other_clients(&stream, message, Arc::clone(&clients));
                //stream.write_all("message".as_bytes()).expect("haho");

            }
            Err(_) => {
                break;
            }
        }
    }

}

fn main() {

    let clients: Vec<TcpStream> = Vec::new();
    let clients = Arc::new(Mutex::new(clients));

    let mut handles = vec![];

    for stream in LISTENER.incoming() {
        match stream {
            Ok(stream) =>  {

                let clients = Arc::clone(&clients);
                let handle = thread::spawn(move || {
                    clients.lock().unwrap().push(stream.try_clone().unwrap());
                    handle_client(stream, clients);
                });
                handles.push(handle);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
