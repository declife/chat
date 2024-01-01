use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::{io, thread};
use std::process;
use std::sync::{Arc, Mutex};


/**
client
*/
fn message_read(reader: TcpStream) {
    let mut buffer = [0; 512];
    let mut reader = reader;

    loop {

        match reader.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    continue;
                }

                let message = String::from_utf8_lossy(&buffer[0..bytes_read]);
                println!("{}", message);
            }
            Err(_) => {
                //to do
                break;
            }
        }
    }

}


fn main() {

    let mut handles = vec![];

    let co_stream = TcpStream::connect(("127.0.0.1", 1315));
    if co_stream.is_err() {
        //exit
        eprintln!("Error: {}", co_stream.err().unwrap());
        process::exit(1);
    }

    // read message
    let co_stream = co_stream.expect("unwrap");
    let copy_stream = co_stream.try_clone().unwrap();
    let read = thread::spawn(move || {
        message_read(copy_stream);
    });
    handles.push(read);

    // send message
    let mut addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 00);
    match co_stream.local_addr() {
        Ok(value) => {
            addr = value;
        }
        Err(_) => {}
    }
    let arc_stream = Arc::new(Mutex::new(co_stream));
    let writer = Arc::clone(&arc_stream);

    let write = thread::spawn(move || {
        message_write(writer, addr);
    });
    handles.push(write);
    // loop {
    //     thread::park();
    // }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn message_write(writer: Arc<Mutex<TcpStream>>, addr: SocketAddr) {

    let _ip = addr.ip().to_string();
    let _port = addr.port();

    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap(); // Flush the buffer to ensure the prompt is displayed
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = _ip.clone() + &_port.to_string() + ": " + &input;

        let mut writer = writer.lock().unwrap();
        writer.write_all(input.as_bytes()).expect("haha");
    }
}