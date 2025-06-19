use std::env;
use std::io::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;


// fn handle_signal (sig: mug::Signal) {
//     match sig {
//         PLAY
//     }
// }

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break; // Connection closed
        }

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        

        // Echo back
        stream.write(&buffer[..bytes_read]).unwrap();
    }
}

pub fn sdjalksd() -> std::io::Result<()> {
    for argument in env::args() {
        if argument.as_str() == "init" {
            let listener = TcpListener::bind(mug::TCP_PORT);
            println!("Server listening on port 8080");

            for stream in listener?.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(|| handle_client(stream));
                    }
                    Err(e) => {
                        eprintln!("Connection failed: {}", e);
                    }
                }
            }
        } else if argument.as_str() == "write" {
            let mut stream = TcpStream::connect("127.0.0.1:8080")?;

            // Send data
            loop {
                let _ = stdout().flush();
                let mut s = String::new();
                stdin()
                    .read_line(&mut s)
                    .expect("Did not enter a correct string");
                if let Some('\n') = s.chars().next_back() {
                    s.pop();
                }
                if let Some('\r') = s.chars().next_back() {
                    s.pop();
                }
                stream.write(s.as_bytes())?;

                // Read response
                // let mut buffer = [0; 512];
                // let bytes_read = stream.read(&mut buffer)?;
                // let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                // println!("Server response: {}", response);
            }
        }
    }
    Ok(())
}
