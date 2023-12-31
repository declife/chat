use std::io::{Read, Write};
//use std::net::{TcpListener, TcpStream};
use std::net::{TcpStream};
use std::{io, thread};
use std::process;
//use rand::Rng;

//server
fn message_read(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    //message_write(stream);
                    continue;
                }

                let message = String::from_utf8_lossy(&buffer[0..bytes_read]);
                println!("{}", message);

                //stream.write_all(message.as_bytes()).unwrap();
            }
            Err(_) => {
                //to do
                break;
            }
        }

    }

}

fn main() {

    //thread::spawn(|| {
        //let co_stream = TcpStream::connect("127.0.0.1:1315").expect("hahi");

    let co_stream = TcpStream::connect(("127.0.0.1", 1315));
    // while co_stream.is_err() {
    //     co_stream = TcpStream::connect(("127.0.0.1", 1315));
    // }
    if co_stream.is_err() {
        //exit
        eprintln!("Error: {}", co_stream.err().unwrap());
        process::exit(1);
    }

    let co_stream = co_stream.expect("success");

    let reader = co_stream.try_clone().expect("clone success");
    let writer = co_stream.try_clone().expect("clone success");
    //let writer = BufWriter::new(&co_stream);
    thread::spawn(move || {
        message_read(reader);
    });
    thread::spawn(move || {
        message_write(writer);
    });

    loop {
        thread::park();
    }
}

fn message_write(mut writer: TcpStream) {

    //let mut co_stream = TcpStream::connect(addr).expect("hahi");
    let ip = writer.local_addr().unwrap().ip().to_string();
    let port = writer.local_addr().unwrap().port();

    // Generate a random integer in the range [1, 100]
    loop {
        let mut input = String::new();
        //print!("You: ");
        io::stdout().flush().unwrap(); // Flush the buffer to ensure the prompt is displayed
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = ip.clone() + &port.to_string() + ": " + &input;
        //writer.write_all(input.as_bytes()).expect("haha");
        writer.write_all(input.as_bytes()).expect("haha");
    }
}