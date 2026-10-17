use std::{
    io::{Error, Read},
    net::TcpListener,
    thread,
};

fn tcp_handle_stream(mut stream: std::net::TcpStream) {
    // get the peer address, unwrap and create owned data from borrow (clone)
    let address: std::net::SocketAddr = stream.peer_addr().unwrap().to_owned();
    println!("[info] {address} connected");

    // create a array buffer of size 1024 and fill with zeros
    let mut buffer: [u8; 1024] = [0u8; 1024];

    // start stream bytes read and pull theses bytes on buffer
    loop {
        match stream.read(&mut buffer) {
            // WHEN: peer closed the connection (TCP FIN received)
            Ok(0) => {
                println!("[info] {address} disconnected");
                break;
            }
            // WHEN: read the bytes from buffer
            Ok(bytes_read) => {
                let message: String = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
                println!("[info] {address} says: {message}");
            }
            // WHEN: any error happen
            Err(error) => {
                eprintln!("[error] {address}: {error}");
                break;
            }
        }
    }
}

fn main() -> Result<(), Error> {
    // open a tcp connection in random open ports (localhost:0)
    let listener: TcpListener = TcpListener::bind("localhost:0")?;

    // get the socket address with port (opened randomly)
    let socket_address: std::net::SocketAddr = listener.local_addr()?;
    println!("[info] tcp server started and listening on {socket_address}");

    // iterate over the connections being received
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // ===================================================================================================
                // LEARNING NOTE
                // ===================================================================================================
                // Without `move`, the closure would borrow `stream`. This is necessary, becauese
                // `thread::spawn` can be runs on a different thread that may outlive the current one.
                //
                // POINTS:
                // - we can't borrow across threads
                // - after `move`, `stream` in `main` function is gone, it now lives (not borrewed) on the new thread
                thread::spawn(move || tcp_handle_stream(stream));
            }
            Err(error) => eprintln!("[error] {error}"),
        }
    }

    Ok(())
}
