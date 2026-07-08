use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use toy_pake::{derive_shared_secret, parse_request};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request = String::new();
    reader.read_line(&mut request)?;

    let response = match parse_request(&request) {
        Ok((password, salt, local_ephemeral, peer_ephemeral)) => {
            let secret = derive_shared_secret(&password, &salt, &local_ephemeral, &peer_ephemeral);
            let mut hex = String::new();
            for byte in secret {
                use std::fmt::Write as _;
                let _ = write!(&mut hex, "{:02x}", byte);
            }
            format!("{hex}\n")
        }
        Err(err) => format!("error: {err}\n"),
    };

    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    println!("toy_pake server listening on 0.0.0.0:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream) {
                    eprintln!("connection error: {err}");
                }
            }
            Err(err) => eprintln!("accept error: {err}"),
        }
    }

    Ok(())
}
