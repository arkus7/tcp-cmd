use std::io::{prelude::*, BufReader};
use std::net::{Shutdown, TcpStream};

fn main() {
    let mut connection = TcpStream::connect("localhost:6969").unwrap();

    let args = std::env::args().skip(1);
    let cmd = args.collect::<Vec<_>>().join(" ");
    let message = b"cargo doc";

    connection.write_all(cmd.as_bytes()).unwrap();
    connection.flush().unwrap();

    connection.shutdown(Shutdown::Write).unwrap();

    let reader = BufReader::new(&connection);

    for line in reader.lines() {
        println!("{}", line.unwrap_or_default());
    }
}
